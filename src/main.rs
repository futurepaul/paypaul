use std::{
    env,
    net::SocketAddr,
    sync::{Arc, RwLock},
};

use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};
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

async fn new_address_handler(State(state): State<SharedState>) -> impl IntoResponse {
    let wallet = &state.read().unwrap().wallet;
    let address = wallet.get_address(AddressIndex::New).unwrap();
    let address_response = AddressResponse {
        address: address.address.to_string(),
        index: address.index,
    };

    Json(address_response)
}
