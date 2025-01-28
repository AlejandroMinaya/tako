use crate::app::tasks::Oswald;

mod adapters;
mod app;
mod clients;
mod ports;

#[tokio::main]
async fn main() {
    let oswald = Oswald::new(ports::DummyStore);
    clients::wasm_app::start(oswald).await.unwrap();
}
