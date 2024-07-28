use crate::core::tasks::Oswald;
use crate::adapters::SQLiteStore;

mod core;
mod adapters;
mod ports;

#[tokio::main]
async fn main() {
    let mut mngr = Oswald::new(Box::new(SQLiteStore::new("sqlite://playground.sqlite".to_owned())));
    match mngr.load().await {
        Ok(()) => {
            mngr.get_all_tasks().into_iter().for_each(|task| println!("{:?}", task))
        },
        Err(e) => {
            println!("Couldn't load database {:?}", e);
        }
    };
}
