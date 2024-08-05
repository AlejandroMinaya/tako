use std::env;
use crate::core::tasks::Oswald;
use crate::adapters::SQLiteStore;

mod core;
mod adapters;
mod clients;
mod ports;

#[tokio::main]
async fn main() {
    let conn_str = env::var("DATABASE_URL_SQLITE").expect("Expected to find DATABASE_URL_SQLITE envar");
    let oswald = Oswald::new(SQLiteStore::new(conn_str));
    clients::wasm_app::start(oswald).await.unwrap();
}
