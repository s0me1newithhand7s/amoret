/* imports */

use tokio::{
    sync::watch,
    time::{Duration, sleep},
};

use std::path::PathBuf;

use clap::Parser;

use discord_rich_presence::{
    DiscordIpc, DiscordIpcClient,
    activity::{
        Activity, ActivityType as RpcActivityType, Assets, Button, Party, Secrets, Timestamps,
    },
};

use crate::amoret::cli::Cli;
use crate::amoret::config::{self, DiscordConfig};

/* fns */

async fn watch_loop(path: PathBuf, tx: watch::Sender<Option<DiscordConfig>>) {
    let mut last_config: Option<DiscordConfig> = None;

    loop {
        match config::load(&path) {
            Some(cfg) if Some(&cfg) != last_config.as_ref() => {
                last_config = Some(cfg.clone());
                let _ = tx.send(last_config.clone());
            }
            _ => {}
        }
        sleep(Duration::from_secs(3)).await;
    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let path = config::resolve_path(args.config.config);

    let (tx, mut rx) = watch::channel(None);
    tokio::spawn(watch_loop(path, tx));

    let mut ipc_client: Option<DiscordIpcClient> = None;
    let mut active_client_id: Option<u64> = None;

    while rx.changed().await.is_ok() {
        let Some(cfg) = rx.borrow().clone() else {
            continue;
        };

        if active_client_id != Some(cfg.client_id) {
            if let Some(mut old_client) = ipc_client.take() {
                let _ = old_client.close();
            }

            let client_id_str = cfg.client_id.to_string();

            let mut new_client = DiscordIpcClient::new(&client_id_str);

            match new_client.connect() {
                Ok(_) => {
                    log::info!("Connected to Discord IPC!");
                    ipc_client = Some(new_client);
                    active_client_id = Some(cfg.client_id);
                }
                Err(e) => log::error!("Failed to connect to Discord IPC: {e}"),
            }
        }

        if let Some(ref mut client) = ipc_client {
            let activity = build_activity(&cfg);

            if let Err(e) = client.set_activity(activity) {
                log::error!("Failed to set activity: {e}");
            } else {
                log::info!("Activity set successfully!");
            }
        }
    }

    Ok(())
}

fn build_activity(cfg: &DiscordConfig) -> Activity<'_> {
    let mut activity = Activity::default();

    let act_type = match cfg.activity_type {
        crate::amoret::config::ActivityType::Playing => RpcActivityType::Playing,
        crate::amoret::config::ActivityType::Listening => RpcActivityType::Listening,
        crate::amoret::config::ActivityType::Watching => RpcActivityType::Watching,
        crate::amoret::config::ActivityType::Competing => RpcActivityType::Competing,
    };
    activity = activity.activity_type(act_type);

    if let Some(state) = &cfg.state {
        activity = activity.state(state.clone());
    }
    if let Some(details) = &cfg.details {
        activity = activity.details(details.clone());
    }

    if let Some(assets) = &cfg.assets {
        let mut a = Assets::default();
        if let Some(img) = &assets.large_image {
            a = a.large_image(img.clone());
        }
        if let Some(txt) = &assets.large_text {
            a = a.large_text(txt.clone());
        }

        if let Some(img) = &assets.small_image {
            a = a.small_image(img.clone());
        }
        if let Some(txt) = &assets.small_text {
            a = a.small_text(txt.clone());
        }
        activity = activity.assets(a);
    }

    if let Some(ts) = &cfg.timestamps {
        let mut t = Timestamps::default();
        if let Some(start) = ts.start {
            t = t.start(start as i64);
        }
        if let Some(end) = ts.end {
            t = t.end(end as i64);
        }
        activity = activity.timestamps(t);
    }

    if let Some(party) = &cfg.party {
        let mut p = Party::default();
        if let Some(id) = &party.id {
            p = p.id(id.clone());
        }
        if let Some(size) = party.size {
            p = p.size(size);
        }
        activity = activity.party(p);
    }

    if let Some(secrets) = &cfg.secrets {
        let mut s = Secrets::default();
        if let Some(j) = &secrets.join {
            s = s.join(j.clone());
        }
        if let Some(sp) = &secrets.spectate {
            s = s.spectate(sp.clone());
        }
        if let Some(m) = &secrets.r#match {
            s = s.r#match(m.clone());
        }
        activity = activity.secrets(s);
    }

    if let Some(buttons) = cfg.buttons.clone() {
        let mapped_buttons: Vec<Button> = buttons
            .into_iter()
            .map(|b| Button::new(b.label, b.url))
            .collect();
        activity = activity.buttons(mapped_buttons);
    }

    activity
}
