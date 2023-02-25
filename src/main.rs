use std::{
    env,
    net::SocketAddr,
    sync::{Arc, RwLock},
};

use axum::{
    extract::State,
    http::{self, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use bdk::{
    bitcoin::{Amount, Network},
    database::SqliteDatabase,
    wallet::AddressIndex,
    Wallet,
};
use serde::Serialize;
use tonic_lnd::LightningClient;
use tower_http::cors::{Any, CorsLayer};

use crate::bip21::create_bip_21;

mod bip21;

#[derive(Serialize)]
struct AddressResponse {
    address: String,
    bolt11: String,
    index: u32,
    bip21: String,
}

struct AppState {
    wallet: Wallet<SqliteDatabase>,
    lnd_client: LightningClient,
}

// lololol
unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}

impl AppState {
    fn new(
        descriptor: String,
        network: String,
        db_path: String,
        lnd_client: LightningClient,
    ) -> anyhow::Result<Self> {
        let parsed_network = network.parse::<Network>()?;
        // Set up bdk
        let wallet = Wallet::new(
            &descriptor,
            None,
            parsed_network,
            SqliteDatabase::new(db_path),
        )?;

        Ok(AppState { wallet, lnd_client })
    }
}

type SharedState = Arc<RwLock<AppState>>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let process_id = std::process::id();

    dbg!(process_id);

    // Load environment variables from various sources.
    dotenv::from_filename(".env.local").ok();
    dotenv::from_filename(".env").ok();
    dotenv::dotenv().ok();

    let descriptor = env::var("WALLET_DESCRIPTOR")?;
    let network = env::var("NETWORK")?;
    let db_path = env::var("DB_PATH")?;
    let lnd_address = env::var("LND_ADDRESS")?;
    let mac_path = env::var("LND_MACAROON_PATH")?;
    let tls_path = env::var("LND_TLS_CERT_PATH")?;

    let client = tonic_lnd::connect(lnd_address, 10009, tls_path, mac_path)
        .await
        .expect("failed to connect")
        .lightning()
        .clone();

    let state = AppState::new(descriptor, network, db_path, client)?;

    let shared_state: SharedState = Arc::new(RwLock::new(state));

    // Set up Axum
    let app = Router::new()
        .route("/hello", get(hello_handler))
        .route("/address", get(new_address_handler))
        .with_state(shared_state)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_headers(vec![http::header::CONTENT_TYPE])
                .allow_methods([Method::GET, Method::POST]),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn hello_handler() -> &'static str {
    "hello!"
}

#[axum::debug_handler]
async fn new_address_handler(
    State(state): State<SharedState>,
) -> Result<impl IntoResponse, AppError> {
    // Onchain
    let address = &state
        .read()
        .unwrap()
        .wallet
        .get_address(AddressIndex::New)?;

    dbg!(address);

    // Lightning
    // Hard thing is to use this mutable "Client" thing without Rust getting mad
    // Can't use these on the same line because Rust \o/
    let mut client = state.read().unwrap().lnd_client.clone();

    let bolt11 = client
        .add_invoice(tonic_lnd::lnrpc::Invoice {
            value: 1000,
            ..tonic_lnd::lnrpc::Invoice::default()
        })
        .await
        .unwrap()
        .into_inner()
        .payment_request;

    dbg!(bolt11.clone());

    let bip21 = create_bip_21(
        address.address.clone(),
        bolt11.clone(),
        Amount::from_sat(1000),
        "heyo".to_string(),
    );

    let address_response = AddressResponse {
        address: address.address.to_string(),
        index: address.index,
        bolt11,
        bip21,
    };

    Ok(Json(address_response))
}

// Make our own error that wraps `anyhow::Error`.
struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
