/* imports */

use steel::steel_vm::{engine::Engine, register_fn::RegisterFn};

use clap::Parser;

use std::path::PathBuf;

use tokio::sync::watch;

use crate::amoret::config::DiscordConfig;

/* structs */

#[allow(dead_code)]
pub struct Plugins;

#[derive(Parser, Debug)]
pub struct PluginArgs {
    #[arg(
        short,
        long,
        value_name = "SCRIPT",
        help = "Path to steel script to be plugged."
    )]
    pub plugins: Option<PathBuf>,
}

/* fns */

#[allow(dead_code)]
impl Plugins {
    pub fn run(script_path: PathBuf, tx: watch::Sender<Option<DiscordConfig>>, base_cfg: DiscordConfig) {
        std::thread::spawn(move || {
            let mut engine = Engine::new();

            engine.register_fn("set_state", move |state: String| {
                let mut new_cfg = base_cfg.clone();
                new_cfg.state = Some(state);
                let _ = tx.send(Some(new_cfg));
            });

            match std::fs::read_to_string(&script_path) {
                Ok(script) => {
                    if let Err(e) = engine.run(script) {
                        eprintln!("[Plugin Error] amoret died: {:?}", e);
                    }
                }
                Err(e) => eprintln!("[Plugin Error] amoret failed to read script: {}", e),
            }
        });
    }
}
