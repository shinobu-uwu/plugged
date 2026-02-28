use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about = "Play an audio file when a USB device is connected/disconnected", long_about = None)]
pub struct CliArgs {}

#[cfg_attr(test, allow(clippy::panic))]
#[cfg(test)]
mod tests {
    use super::*;
    use clap::error::ErrorKind;

    #[test]
    fn parse_defaults() {
        match CliArgs::try_parse_from(["plugged"]) {
            Ok(_args) => {}
            Err(err) => panic!("args should parse: {err}"),
        }
    }

    #[test]
    fn help_flag_is_available() {
        match CliArgs::try_parse_from(["plugged", "--help"]) {
            Ok(_) => panic!("--help should not parse into args"),
            Err(err) => assert_eq!(err.kind(), ErrorKind::DisplayHelp),
        }
    }
}
