/* imports */

use clap::Parser;
use clap_verbosity_flag::Verbosity;

use crate::amoret::config::ConfigArgs;
use crate::amoret::plugins::PluginArgs;

/* structs */

#[derive(Parser, Debug)]
#[command(author, version, about = "Discord RPC client on Rust.")]
pub struct Cli {
    #[command(flatten)]
    pub config: ConfigArgs,

    #[command(flatten)]
    pub plugins: PluginArgs,

    #[command(flatten)]
    pub verbose: Verbosity,

    #[arg(short, long, help = "Run as a background daemon.")]
    pub daemon: bool,
}
