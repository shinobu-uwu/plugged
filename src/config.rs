use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub sounds: Sounds,
    #[serde(default)]
    pub notifications: Notifications,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sounds {
    #[serde(default = "default_true")]
    pub enable: bool,
    pub connected: PathBuf,
    pub disconnected: PathBuf,
}

impl Default for Sounds {
    fn default() -> Self {
        Self {
            enable: true,
            connected: PathBuf::new(),
            disconnected: PathBuf::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notifications {
    pub enable: bool,
    pub format: String,
}

impl Default for Notifications {
    fn default() -> Self {
        Self {
            enable: true,
            format: String::from("Device {{device_name}} {{action}}."),
        }
    }
}

fn default_true() -> bool {
    true
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
        assert!(config.sounds.enable);
        assert_eq!(config.sounds.connected, PathBuf::from("/tmp/plug.ogg"));
        assert_eq!(config.sounds.disconnected, PathBuf::from("/tmp/unplug.ogg"));
    }

    #[test]
    fn serialize_roundtrip() {
        let config = Config {
            sounds: Sounds {
                enable: true,
                connected: PathBuf::from("/tmp/plug.ogg"),
                disconnected: PathBuf::from("/tmp/unplug.ogg"),
            },
            notifications: Notifications::default(),
        };

        let encoded = match toml::to_string(&config) {
            Ok(encoded) => encoded,
            Err(err) => panic!("config should serialize: {err}"),
        };
        let decoded: Config = match toml::from_str(&encoded) {
            Ok(decoded) => decoded,
            Err(err) => panic!("config should deserialize: {err}"),
        };
        assert!(decoded.sounds.enable);
        assert_eq!(decoded.sounds.connected, PathBuf::from("/tmp/plug.ogg"));
        assert_eq!(
            decoded.sounds.disconnected,
            PathBuf::from("/tmp/unplug.ogg")
        );
    }
}
