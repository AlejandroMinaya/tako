use crate::core::tasks::Oswald;
use crate::adapters::SQLiteStore;

mod core;
mod adapters;
mod clients;
mod ports;

#[tokio::main]
async fn main() {
    let data_store = SQLiteStore::new("sqlite://playground.sqlite".to_owned());
    let mut oswald = Oswald::new();
    oswald.load(&data_store).await.expect("Couldn't load from data store");
    clients::api::start(oswald).await
}
