use crate::core::tasks::Oswald;
use crate::adapters::SQLiteStore;

mod core;
mod adapters;
mod clients;
mod ports;

#[tokio::main]
async fn main() {
    let oswald = Oswald::new(SQLiteStore::new("sqlite://playground.sqlite".to_owned()));
    clients::api::start(oswald).await
}
