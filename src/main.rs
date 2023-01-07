use std::env;

use bdk::{
    bitcoin::Network,
    database::{MemoryDatabase, SqliteDatabase},
    wallet::AddressIndex,
    Wallet,
};

fn main() -> anyhow::Result<()> {
    // Load environment variables from various sources.
    dotenv::from_filename(".env.local").ok();
    dotenv::from_filename(".env").ok();
    dotenv::dotenv().ok();

    let descriptor = env::var("WALLET_DESCRIPTOR")?;
    println!("Descriptor: {}", descriptor);

    // Set up bdk
    let wallet = Wallet::new(
        &descriptor,
        None,
        Network::Testnet,
        SqliteDatabase::new("./paypaul.db"),
    )?;

    let address = wallet.get_address(AddressIndex::New)?;

    println!("Address #{}: {}", address.index, address.address);

    Ok(())
}
