/* imports */

use serde::Deserialize;

use std::path::PathBuf;

use clap::Parser;

use directories::ProjectDirs;

/* enums */

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
        help = "Path to config file. If unused - fallbacks to OS-depended default.")]
    pub config: Option<PathBuf>,
}

/* fns */

pub fn load(path: &PathBuf) -> Option<DiscordConfig> {
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

pub fn resolve_path(cli_path: Option<PathBuf>) -> PathBuf {
    if let Some(p) = cli_path {
        return p;
    }

    if let Some(proj_dirs) = ProjectDirs::from("com", "amoret", "amoret") {
        let config_dir = proj_dirs.config_dir();
        return config_dir.join("config.toml");
    }

    PathBuf::from("config.toml")
}
