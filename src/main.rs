use std::net::ToSocketAddrs;

use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use tokio::net::unix::SocketAddr;

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
        let address = "127.0.0.1:8008".to_socket_addrs()?;
    }

    Ok(())
}
