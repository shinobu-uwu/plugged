mod cli;
mod config;
mod player;

use anyhow::{Context, Result, anyhow};
use clap::Parser;
use tracing::info;
use tracing_subscriber::EnvFilter;
use udev::{
    EventType, MonitorBuilder,
    mio::{Events, Interest, Poll, Token},
};

use crate::{cli::CliArgs, config::Config, player::Player};

const UDEV_TOKEN: Token = Token(0);
const APP_NAME: &str = "plugged";
const CONFIG_NAME: &str = "config";

fn main() -> Result<()> {
    init();
    let _ = CliArgs::parse();
    let config: Config = confy::load(APP_NAME, CONFIG_NAME)?;
    let connected_sound = std::fs::read(config.sounds.connected)
        .with_context(|| anyhow!("Failed to load connected sound"))?;
    let disconnected_sound = std::fs::read(config.sounds.disconnected)
        .with_context(|| anyhow!("Failed to load disconnected sound"))?;
    let player = Player::new(connected_sound, disconnected_sound)?;
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);
    let mut monitor = MonitorBuilder::new()?.match_subsystem("usb")?.listen()?;
    poll.registry()
        .register(&mut monitor, UDEV_TOKEN, Interest::READABLE)?;

    info!("Starting plugged daemon");

    loop {
        poll.poll(&mut events, None)?;

        for event in events.iter() {
            if event.token() == UDEV_TOKEN {
                while let Some(udev_event) = monitor.iter().next() {
                    if udev_event.devtype() != Some(std::ffi::OsStr::new("usb_device")) {
                        continue;
                    }

                    match udev_event.event_type() {
                        EventType::Add => {
                            info!("Add event received: {:#?}", udev_event);
                            player.play_plugged();
                        }
                        EventType::Remove => {
                            info!("Remove event received: {:#?}", udev_event);
                            player.play_unplugged();
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn init() {
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
}
