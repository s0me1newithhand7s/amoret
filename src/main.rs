/* mods */

mod amoret;

/* imports */

use clap::Parser;
use log::error;

/* fns */

#[tokio::main]
async fn main() {
    let args = amoret::cli::Cli::parse();
    if let Err(e) = simplelog::SimpleLogger::init(
        args.verbose.log_level_filter(),
        simplelog::Config::default()
    ) {
        eprintln!("amoret canot log: {e}");
        std::process::exit(1);
    }

    if args.daemon {
        let exe = match std::env::current_exe() {
            Ok(p) => p,
            Err(e) => {
                error!("amoret not found: {e}");
                std::process::exit(1);
            }
        };
        
        let child_args: Vec<String> = std::env::args()
            .skip(1)
            .filter(|a| a != "--daemon" && a != "-d")
            .collect();

        let mut cmd = std::process::Command::new(exe);
        cmd.args(&child_args)
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
            cmd.creation_flags(0x00000008 | 0x00000200);
        }

        if let Err(e) = cmd.spawn() {
            error!("amoret died: {e}");
            std::process::exit(1);
        }

        std::process::exit(0); 
    }

    if let Err(e) = amoret::run().await {
        error!("amoret died: {e}");
        std::process::exit(1);
    }
}
