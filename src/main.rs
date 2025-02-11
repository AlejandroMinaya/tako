use crate::app::tasks::Oswald;

mod app;
mod infra;

#[tokio::main]
async fn main() {
    let oswald = Oswald::default();
    infra::wasm_app::start(oswald).await.unwrap();
}
