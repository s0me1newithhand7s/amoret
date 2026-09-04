/* mods */

mod amoret;

/* imports */

use clap::Parser;
use log::error;
use std::io::IsTerminal;
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

    let interactive = std::io::stdin().is_terminal();

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

    if args.reload && interactive {
        println!("amoret is reloading daemon...");

        if let Some(ref path) = pid_path {
            if path.exists() {
                if let Ok(pid_str) = std::fs::read_to_string(path) {
                    if let Ok(target_pid) = pid_str.trim().parse::<usize>() {
                        let mut sys = System::new();
                        sys.refresh_all();

                        if let Some(process) = sys.process(Pid::from(target_pid)) {
                            let name = process.name().to_string_lossy();
                            if name.contains("amoret") {
                                if !process.kill() {
                                    error!("amoret canot stop old instance with PID {target_pid}");
                                }
                            } else {
                                log::warn!(
                                    "amoret refuses to kill PID {target_pid}: it is `{name}`, not amoret"
                                );
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

        std::thread::sleep(std::time::Duration::from_millis(200));
        println!("amoret successfully reloaded.");
    }

    if (args.daemon || args.reload) && interactive {
        if let Err(e) = daemonize(&pid_path) {
            error!("amoret canot daemonize: {e}");
            std::process::exit(1);
        }
    }

    if let Err(e) = amoret::run(args.config.config).await {
        error!("amoret died: {e}");
        remove_pid_file_if_mine(&pid_path);
        std::process::exit(1);
    }

    remove_pid_file_if_mine(&pid_path);
}

fn daemonize(pid_path: &Option<std::path::PathBuf>) -> Result<(), std::io::Error> {
    let exe = std::env::current_exe()?;

    let mut cmd = std::process::Command::new(exe);
    cmd.args(std::env::args_os().skip(1))
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());

    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0);
    }

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const DETACHED_PROCESS: u32 = 0x0000_0008;
        const CREATE_NEW_PROCESS_GROUP: u32 = 0x0000_0200;
        cmd.creation_flags(DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP);
    }

    let child = cmd.spawn()?;

    if let Some(ref path) = pid_path {
        if let Some(parent) = path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                error!("amoret cannot create runtime directory: {e}");
            }
        }
        if let Err(e) = std::fs::write(path, child.id().to_string()) {
            error!("amoret cannot save PID file: {e}");
        }
    }

    eprintln!("amoret is daemonized");
    std::process::exit(0);
}

fn remove_pid_file_if_mine(pid_path: &Option<std::path::PathBuf>) {
    if let Some(ref path) = pid_path {
        if let Ok(mine) = std::fs::read_to_string(path) {
            if mine.trim() == std::process::id().to_string() {
                let _ = std::fs::remove_file(path);
            }
        }
    }
}
