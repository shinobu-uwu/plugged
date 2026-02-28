use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub sounds: Sounds,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Sounds {
    pub connected: PathBuf,
    pub disconnected: PathBuf,
}

#[cfg_attr(test, allow(clippy::panic))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_config_from_toml() {
        let input = r#"
            [sounds]
            connected = "/tmp/plug.ogg"
            disconnected = "/tmp/unplug.ogg"
        "#;

        let config: Config = match toml::from_str(input) {
            Ok(config) => config,
            Err(err) => panic!("config should parse: {err}"),
        };
        assert_eq!(config.sounds.connected, PathBuf::from("/tmp/plug.ogg"));
        assert_eq!(
            config.sounds.disconnected,
            PathBuf::from("/tmp/unplug.ogg")
        );
    }

    #[test]
    fn serialize_roundtrip() {
        let config = Config {
            sounds: Sounds {
                connected: PathBuf::from("/tmp/plug.ogg"),
                disconnected: PathBuf::from("/tmp/unplug.ogg"),
            },
        };

        let encoded = match toml::to_string(&config) {
            Ok(encoded) => encoded,
            Err(err) => panic!("config should serialize: {err}"),
        };
        let decoded: Config = match toml::from_str(&encoded) {
            Ok(decoded) => decoded,
            Err(err) => panic!("config should deserialize: {err}"),
        };
        assert_eq!(decoded.sounds.connected, PathBuf::from("/tmp/plug.ogg"));
        assert_eq!(
            decoded.sounds.disconnected,
            PathBuf::from("/tmp/unplug.ogg")
        );
    }
}
