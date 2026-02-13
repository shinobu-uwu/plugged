use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about = "Play an audio file when a USB device is connected/disconnected", long_about = None)]
pub struct CliArgs {}
