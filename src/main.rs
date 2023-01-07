use std::{
    env,
    net::SocketAddr,
    sync::{Arc, RwLock},
};

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use bdk::{bitcoin::Network, database::SqliteDatabase, wallet::AddressIndex, Wallet};
use serde::Serialize;

#[derive(Serialize)]
struct AddressResponse {
    address: String,
    index: u32,
}

struct AppState {
    wallet: Wallet<SqliteDatabase>,
}

// lololol
unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}

impl AppState {
    fn new(descriptor: String) -> anyhow::Result<Self> {
        // Set up bdk
        let wallet = Wallet::new(
            &descriptor,
            None,
            Network::Testnet,
            SqliteDatabase::new("./paypaul.db"),
        )?;

        Ok(AppState { wallet })
    }
}

type SharedState = Arc<RwLock<AppState>>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from various sources.
    dotenv::from_filename(".env.local").ok();
    dotenv::from_filename(".env").ok();
    dotenv::dotenv().ok();

    let descriptor = env::var("WALLET_DESCRIPTOR")?;

    let state = AppState::new(descriptor)?;

    let shared_state: SharedState = Arc::new(RwLock::new(state));

    // Set up Axum
    let app = Router::new()
        .route("/hello", get(hello_handler))
        .route("/address", get(new_address_handler))
        .with_state(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
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
    let wallet = &state.read().unwrap().wallet;
    let address = wallet.get_address(AddressIndex::New)?;
    let address_response = AddressResponse {
        address: address.address.to_string(),
        index: address.index,
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
