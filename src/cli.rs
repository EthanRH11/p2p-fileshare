use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser, Debug)]
#[clap(name = "p2p-fileshare", version)]
pub struct Config {
    // Operation mode (announce, share, get, peers)
    #[clap(subcommand)]
    pub mode: Mode,

    // UDP port for peer discovery
    #[clap(long, default_value = "6001")]
    pub udp_port: u16,

    // TCP port for chunk requests
    #[clap(long, default_value = "6000")]
    pub tcp_port: u16,

    // Chunk size in bytes
    #[clap(long, default_value = "65536")]
    pub chunk_size: usize,

    //More global flags in the future...
}

// Sub commands
#[derive(Subcommand, Debug)]
pub enum Mode {
    // Announce yourself on the network
    Announce {}

    // Share a file 
    Share {
        // Path to the file to share
        #[clap(parse(from_os_str))]
        file: std::path::Pathbuf,
    },

    // Download a file
    Get {
        // Name of file
        filename: String,

        //Destination directory
        #[clap(parse(from_os_str), long, default_value = ".")]
        out_dir: std::path::Pathbuf,
    },

    //List known peers
    Peers{},
}

//Parse the CLI args into a config
pub fn parse_args() -> Config {
    Config::parse()
}

pub async fn run_command_loop(cfg: &Config) -> anyhow::Result<()> {
    
}