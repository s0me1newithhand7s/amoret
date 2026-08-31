/* imports */

use serde::Deserialize;

use std::path::PathBuf;

use clap::Parser;

use directories::ProjectDirs;

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ActivityType {
    Playing,
    Listening,
    Watching,
    Competing,
}

/* structs */

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct ActivityTimestamps {
    pub start: Option<u64>,
    pub end: Option<u64>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct ActivityAssets {
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub large_url: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
    pub small_url: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct ActivityParty {
    pub id: Option<String>,
    pub size: Option<[i32; 2]>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct ActivitySecrets {
    pub join: Option<String>,
    pub spectate: Option<String>,
    pub r#match: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct ActivityButton {
    pub label: String,
    pub url: String,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct DiscordConfig {
    pub client_id: u64,
    pub activity_type: ActivityType,
    pub details: Option<String>,
    pub state: Option<String>,
    pub instance: Option<bool>,

    pub timestamps: Option<ActivityTimestamps>,
    pub assets: Option<ActivityAssets>,
    pub party: Option<ActivityParty>,
    pub secrets: Option<ActivitySecrets>,
    pub buttons: Option<Vec<ActivityButton>>,
}

#[derive(Parser, Debug)]
pub struct ConfigArgs {
    #[arg(
        short,
        long,
        value_name = "FILE",
        help = "Path to config file. If unused - fallbacks to OS-depended default."
    )]
    pub config: Option<PathBuf>,
}

/* fns */

pub fn load(path: &PathBuf) -> Option<DiscordConfig> {
    let extension = path.extension()?.to_str()?;

    match extension {
        "toml" => {
            let contents = match std::fs::read_to_string(path) {
                Ok(c) => c,
                Err(_) => return None,
            };

            match toml::from_str::<DiscordConfig>(&contents) {
                Ok(cfg) => Some(cfg),
                Err(e) => {
                    eprintln!("[Config Error] Invalid TOML syntax: {e}");
                    None
                }
            }
        }

        "scm" | "steel" => {
            let mut engine = steel::steel_vm::engine::Engine::new();
            let script = match std::fs::read_to_string(path) {
                Ok(c) => c,
                Err(_) => return None,
            };

            let results = match engine.run(script) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("[Config Error] amoret found scheme runtime error: {e}");
                    return None;
                }
            };

            let evaluated_value = match results.last() {
                Some(v) => v,
                None => {
                    eprintln!("[Config Error] Scheme script returned no values.");
                    return None;
                }
            };

            if let steel::rvals::SteelVal::HashMapV(map) = evaluated_value {
                let client_id = match map.get(&steel::rvals::SteelVal::SymbolV("client_id".into()))
                {
                    Some(steel::rvals::SteelVal::IntV(id)) => *id as u64,
                    _ => {
                        eprintln!("[Config Error] amoret requires a valid integer 'client_id' in scheme config.");
                        return None;
                    }
                };

                let get_string = |key: &str| -> Option<String> {
                    match map.get(&steel::rvals::SteelVal::SymbolV(key.into())) {
                        Some(steel::rvals::SteelVal::StringV(s)) => Some(s.to_string()),
                        _ => None,
                    }
                };

                Some(DiscordConfig {
                    client_id,
                    activity_type: match get_string("activity_type").as_deref() {
                        Some("listening") => crate::amoret::config::ActivityType::Listening,
                        Some("watching") => crate::amoret::config::ActivityType::Watching,
                        Some("competing") => crate::amoret::config::ActivityType::Competing,
                        _ => crate::amoret::config::ActivityType::Playing,
                    },
                    details: get_string("details"),
                    state: get_string("state"),
                    instance: match map.get(&steel::rvals::SteelVal::SymbolV("instance".into())) {
                        Some(steel::rvals::SteelVal::BoolV(b)) => Some(*b),
                        _ => None,
                    },
                    timestamps: None,
                    assets: None,
                    party: None,
                    secrets: None,
                    buttons: None,
                })
            } else {
                eprintln!("[Config Error] amoret requires the scheme script to return a hash map collection.");
                None
            }
        }
        _ => {
            eprintln!("[Config Error] amoret does not support '.{extension}' formats.");
            None
        }
    }
}

pub fn resolve_path(cli_path: Option<PathBuf>) -> PathBuf {
    if let Some(p) = cli_path {
        return p;
    }

    if let Some(proj_dirs) = ProjectDirs::from("com", "amoret", "amoret") {
        let config_dir = proj_dirs.config_dir();

        let scm_path = config_dir.join("config.scm");
        if scm_path.exists() {
            return scm_path;
        }
        return config_dir.join("config.toml");
    }

    PathBuf::from("config.toml")
}
