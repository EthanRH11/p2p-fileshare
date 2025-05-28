mod cli;
mod chunk;
mod net;
mod peer;
mod protocol;

use anyhow::Result;
use tracing_subscriber;
use tokio::net::UdpSocket;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<()> {
    // 1. parse CLI args into config
    let cfg = cli::parse_args();

    // 2. initialize logging
    tracing_subscriber::fmt::init();
    tracing::info!("Starting p2p-fileshare (mode = {:?})", cfg.mode);

    // 3. Spawn background tasks
    // UDP listener for peer announcements
    let udp_handle = tokio::spawn(async move {
        if let Err(e) = net::run_udp_listener(&cfg).await {
            tracing::error!("UDP Listener failed: {:#?}", e);
        }
    });

    // TCP Listener for incoming chunk request
    let tcp_handler = tokio::spawn(async move {
        if let Err(e) = net::run_tcp_listener(&cfg).await {
            tracing::error!("TCP Listener failed: {:#?}", e);
        }
    });

    // Periodic announce task
    let announce_handle =  tokio::spawn(async move {
        if let Err(e) = peer::run_announce_task(&cfg).await {
            tracing::error!("Announce task failed: {:#?}", e);
        }
    });

    // 4. Enter command loop (share, get, peers, exit) 
    cli::run_command_loop(&cfg).await?;

    udp_handle.abort();
    tcp_handler.abort();
    announce_handle.abort();

    tracing::info!("Shutting down.");
    Ok(())
}