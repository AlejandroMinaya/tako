use crate::app::tasks::Oswald;

mod app;
mod infra;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let oswald = Oswald::default();

    #[cfg(feature = "wasm_app")]
    if cfg!(feature = "wasm_app") {
        infra::wasm_app::start(oswald).await.unwrap();
    }

    #[cfg(feature = "rpc")]
    if cfg!(feature = "rpc") {
        println!("rpc not implemented yet");
    }

    Ok(())
}
