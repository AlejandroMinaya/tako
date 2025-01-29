use crate::app::tasks::Oswald;

mod app;
mod clients;

#[tokio::main]
async fn main() {
    let oswald = Oswald::default();
    clients::wasm_app::start(oswald).await.unwrap();
}
