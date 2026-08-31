/* mods */

mod amoret;

/* imports */

use clap::Parser;
use log::error;
use sysinfo::{Pid, System};

/* fns */

#[tokio::main]
async fn main() {
    let args = amoret::cli::Cli::parse();

    if let Err(e) = simplelog::SimpleLogger::init(
        args.verbose.log_level_filter(),
        simplelog::Config::default(),
    ) {
        eprintln!("amoret canot log: {e}");
        std::process::exit(1);
    }

    let pid_path = directories::ProjectDirs::from("com", "amoret", "amoret")
        .map(|p| p.config_dir().join("amoret.pid"));

    if args.validate {
        let path = amoret::config::resolve_path(args.config.config.clone());
        println!("amoret is checking configuration path: {}", path.display());

        match amoret::config::load(&path) {
            Some(_) => {
                println!("amoret found configuration is valid.");
                std::process::exit(0);
            }
            None => {
                eprintln!("amoret found configuration error.");
                std::process::exit(1);
            }
        }
    }

    if args.reload {
        println!("amoret is reloading daemon...");

        if let Some(ref path) = pid_path {
            if path.exists() {
                if let Ok(pid_str) = std::fs::read_to_string(path) {
                    if let Ok(target_pid) = pid_str.trim().parse::<usize>() {
                        let mut sys = System::new();
                        sys.refresh_all(); // Ensures correct process metadata across OS types

                        if let Some(process) = sys.process(Pid::from(target_pid)) {
                            if !process.kill() {
                                error!("amoret canot stop old instance with PID {target_pid}");
                            }
                        } else {
                            log::warn!("amoret found no active old instance with PID {target_pid}");
                        }
                    } else {
                        error!("amoret canot parse PID file");
                    }
                } else {
                    error!("amoret canot read PID file");
                }

                if let Err(e) = std::fs::remove_file(path) {
                    error!("amoret canot remove old PID file: {e}");
                }
            }
        }

        let child_args: Vec<String> = std::env::args()
            .skip(1)
            .filter(|a| a != "--reload" && a != "-R")
            .collect();

        let exe = match std::env::current_exe() {
            Ok(p) => p,
            Err(e) => {
                error!("amoret not found: {e}");
                std::process::exit(1);
            }
        };

        if let Err(e) = std::process::Command::new(exe).args(&child_args).spawn() {
            error!("amoret canot spawn new daemon: {e}");
            std::process::exit(1);
        }

        println!("amoret successfully reloaded.");
        std::process::exit(0);
    }

    if let Some(ref path) = pid_path {
        if let Some(parent) = path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                error!("amoret cannot create runtime directory: {e}");
            }
        }
        if let Err(e) = std::fs::write(path, std::process::id().to_string()) {
            error!("amoret cannot save PID file: {e}");
        }
    }

    if let Err(e) = amoret::run().await {
        error!("amoret died: {e}");
        if let Some(ref path) = pid_path {
            let _ = std::fs::remove_file(path);
        }
        std::process::exit(1);
    }

    if let Some(ref path) = pid_path {
        let _ = std::fs::remove_file(path);
    }
}
