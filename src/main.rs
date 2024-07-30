use crate::core::tasks::Oswald;
use crate::adapters::SQLiteStore;

mod core;
mod adapters;
mod clients;

#[tokio::main]
async fn main() {
    let data_store = SQLiteStore::new("sqlite://playground.sqlite".to_owned());
    let mut oswald = Oswald::new();

    match oswald.load(&data_store).await {
        Ok(()) => {
            oswald.get_all_tasks().into_iter().for_each(|task| println!("{:?}", task))
        },
        Err(e) => {
            println!("Couldn't load database {:?}", e);
        }
    };

    clients::api::start().await
}
