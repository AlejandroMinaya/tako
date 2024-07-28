use crate::core::tasks::Oswald;
use crate::adapters::SQLiteStore;

mod core;
mod adapters;
mod ports;

fn main() {
    let mngr = Oswald::new(Box::new(SQLiteStore::new("sqlite://playground.sqlite".to_owned())));
}
