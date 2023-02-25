Very bad readme

# STEP ZERO

- rustup
- rust analyzer

# STEP ONE

make an app that imports bdk, reads xpub from environment variable, outputs first unused address and quits

sample descriptor from bdk:

wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/\*)

https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html

- cargo new
- hello world
- read descriptor from env
  cargo add dotenv for reading from file
  cargo add anyhow for handling error

- add bdk

string vs &str!
magic of ? again

- return address on each run
  import AddressIndex
  last unused
  print address index and address.address

# STEP TWO

make it so the app uses sqlite to persist. should return a new address on each run

- feature sqlite
  cargo add bdk --features sqlite

          SqliteDatabase::new("./paypaul.db"),

AddressIndex::New

https://sqliteviewer.app/

# STEP THREE

make an axum server that returns address on each run

why axum? because axum is made by tokio and they're winning the async war

cargo add axum
cargo add serde --features derive
cargo add serde_json
cargo add tokio --features full

https://github.com/tokio-rs/axum/blob/main/examples/hello-world/src/main.rs

return helloworld first

- make main async
- copy and paste from the helloworld example basically

http://localhost:3000/hello

return a hardcoded address as json with index (probably wouldn't return an index in prod but just for learning serde)

#[derive(Serialize)]

add address route: .route("/address", get(new_address_handler));

nice we get json back!

put wallet into "shared state"
https://docs.rs/axum/latest/axum/extract/struct.State.html

Wallet is generic!!!!

struct AppState {
wallet: Wallet<SqliteDatabase>,
}

type SharedState = Arc<RwLock<AppState>>;

https://doc.rust-lang.org/nomicon/send-and-sync.html

unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}

cargo add axum --features macros
