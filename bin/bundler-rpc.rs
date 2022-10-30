use anyhow::Result;
use clap::Parser;
use jsonrpsee::{core::server::rpc_module::Methods, http_server::HttpServerBuilder, tracing::info};
use std::future::pending;
use tracing_subscriber;

use aa_bundler::rpc::{eth::EthApiServerImpl, eth_api::EthApiServer};

#[derive(Parser)]
#[clap(
    name = "AA - Bundler RPC",
    about = "RPC server for EIP-4337 Account Abstraction Bundler"
)]
pub struct Opt {
    #[clap(long, default_value = "127.0.0.1:3001")]
    pub rpc_listen_address: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt: Opt = Opt::parse();

    tracing_subscriber::fmt::init();

    let http_server = HttpServerBuilder::default()
        .build(&opt.rpc_listen_address)
        .await?;

    let mut api = Methods::new();
    api.merge(
        EthApiServerImpl {
            call_gas_limit: 100_000_000,
        }
        .into_rpc(),
    )
    .unwrap();

    let _http_server_handle = http_server.start(api.clone())?;
    info!("HTTP server listening on {}", opt.rpc_listen_address);

    pending().await
}