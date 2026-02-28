mod cli;
mod config;
mod notify;
mod player;

use anyhow::{Context, Result, anyhow};
use clap::Parser;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;
use udev::{
    EventType, MonitorBuilder,
    mio::{Events, Interest, Poll, Token},
};

use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use crate::{cli::CliArgs, config::Config, notify::Notifier, player::Player};

const UDEV_TOKEN: Token = Token(0);
const APP_NAME: &str = "plugged";
const CONFIG_NAME: &str = "config";

fn main() -> Result<()> {
    init()?;
    let _ = CliArgs::parse();

    let config: Config = confy::load(APP_NAME, CONFIG_NAME)?;
    let player = if config.sounds.enable {
        let connected_sound = std::fs::read(config.sounds.connected)
            .with_context(|| anyhow!("Failed to load connected sound"))?;
        let disconnected_sound = std::fs::read(config.sounds.disconnected)
            .with_context(|| anyhow!("Failed to load disconnected sound"))?;
        Some(Player::new(connected_sound, disconnected_sound)?)
    } else {
        None
    };
    let notifier = Notifier::new(config.notifications.enable, config.notifications.format)?;

    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);
    let mut monitor = MonitorBuilder::new()?.match_subsystem("usb")?.listen()?;
    let running = Arc::new(AtomicBool::new(true));
    let shutdown_flag = Arc::clone(&running);

    ctrlc::set_handler(move || {
        shutdown_flag.store(false, Ordering::SeqCst);
    })
    .with_context(|| anyhow!("Failed to set Ctrl-C handler"))?;

    poll.registry()
        .register(&mut monitor, UDEV_TOKEN, Interest::READABLE)?;

    info!("Starting plugged daemon");

    while running.load(Ordering::SeqCst) {
        poll.poll(&mut events, Some(Duration::from_millis(500)))?;

        for event in events.iter() {
            if event.token() == UDEV_TOKEN {
                while let Some(udev_event) = monitor.iter().next() {
                    if udev_event.devtype() != Some(std::ffi::OsStr::new("usb_device")) {
                        continue;
                    }

                    match udev_event.event_type() {
                        EventType::Add => {
                            info!("Add event received: {:#?}", udev_event);
                            if let Some(player) = &player {
                                player.play_plugged();
                            }

                            if let Err(err) =
                                notifier.notify(&device_name(&udev_event), "connected")
                            {
                                warn!("Failed to send notification: {err}");
                            }
                        }
                        EventType::Remove => {
                            info!("Remove event received: {:#?}", udev_event);
                            if let Some(player) = &player {
                                player.play_unplugged();
                            }

                            if let Err(err) =
                                notifier.notify(&device_name(&udev_event), "disconnected")
                            {
                                warn!("Failed to send notification: {err}");
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    info!("Shutting down plugged daemon");
    Ok(())
}

fn init() -> Result<()> {
    let default_level = if cfg!(debug_assertions) {
        "info"
    } else {
        "warn"
    };

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_level)),
        )
        .compact()
        .init();
    Ok(())
}

fn device_name(event: &udev::Event) -> String {
    const KEYS: [&str; 3] = ["ID_MODEL_FROM_DATABASE", "ID_MODEL", "DEVNAME"];

    for key in KEYS {
        if let Some(value) = event.property_value(key) {
            return value.to_string_lossy().into_owned();
        }
    }

    event.sysname().to_string_lossy().into_owned()
}
