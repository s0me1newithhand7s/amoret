/* imports */

use clap::Parser;
use clap_verbosity_flag::Verbosity;

use crate::amoret::config::ConfigArgs;

/* structs */

#[derive(Parser, Debug)]
#[command(author, version, about = "Discord RPC client on Rust.")]
pub struct Cli {
    #[command(flatten)]
    pub config: ConfigArgs,

    #[command(flatten)]
    pub verbose: Verbosity,

    #[arg(short, long, help = "Run as a background daemon.")]
    pub daemon: bool,

    #[arg(
        long,
        help = "Validate configuration file and exit.",
        conflicts_with_all = &["daemon", "reload"]
    )]
    pub validate: bool,

    #[arg(
        short = 'R',
        long,
        help = "Kill the running daemon instance and start a new one.",
        conflicts_with_all = &["daemon", "validate"]
    )]
    pub reload: bool,
}
