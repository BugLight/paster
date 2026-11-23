use std::collections::HashMap;

use crate::paste::{debug, pastebin, PasteDestination};
use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PasterConfig {
    /// Config format version
    pub version: String,
    /// Default destination name
    pub default: String,
    /// Different paste destinations
    pub dest: HashMap<String, DestinationConfig>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum DestinationConfig {
    /// pastebin.com destination
    Pastebin(pastebin::PastebinConfig),
    /// debug destination (echoes pasted content)
    Debug(debug::DebugConfig),
}

impl From<DestinationConfig> for Box<dyn PasteDestination> {
    fn from(val: DestinationConfig) -> Self {
        match val {
            // TODO: There must be some clever way to do it
            DestinationConfig::Pastebin(x) => x.into(),
            DestinationConfig::Debug(x) => x.into(),
        }
    }
}

impl Default for PasterConfig {
    fn default() -> Self {
        let default = String::from("pastebin");
        let mut destinations = HashMap::new();
        destinations.insert(
            default.clone(),
            DestinationConfig::Pastebin(pastebin::PastebinConfig::default()),
        );

        PasterConfig {
            version: String::from("1"),
            default,
            dest: destinations,
        }
    }
}

pub fn update_config_value(config: &mut serde_yaml::Value, key: &str, value: String) -> Result<()> {
    if let serde_yaml::Value::Mapping(map) = config {
        if let Some((head, tail)) = key.split_once('.') {
            let config = map
                .get_mut(head)
                .ok_or(Error::msg("Unknown key: ".to_string() + head))?;
            update_config_value(config, tail, value)
        } else {
            match map.get_mut(key) {
                Some(serde_yaml::Value::String(val)) => {
                    *val = value;
                    Ok(())
                }
                Some(_) => Err(Error::msg("Expected string")),
                None => Err(Error::msg("Unknown key: ".to_string() + key)),
            }
        }
    } else {
        Err(Error::msg("Expected mapping"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_plain() -> Result<()> {
        let mut config = serde_yaml::to_value(PasterConfig::default())?;
        update_config_value(&mut config, "default", String::from("test"))?;

        let config: PasterConfig = serde_yaml::from_value(config)?;
        assert_eq!(config.default, "test");
        Ok(())
    }

    #[test]
    fn update_nested() -> Result<()> {
        let mut config = serde_yaml::to_value(PasterConfig::default())?;
        update_config_value(&mut config, "dest.pastebin.dev_key", String::from("test"))?;

        let config: PasterConfig = serde_yaml::from_value(config)?;
        match config.dest.get("pastebin").unwrap() {
            DestinationConfig::Pastebin(pastebin::PastebinConfig {
                dev_key,
                user_key: _,
            }) => {
                assert_eq!(dev_key, "test");
            }
            DestinationConfig::Debug(_) => panic!(),
        }
        Ok(())
    }

    #[test]
    fn update_unknown_key() -> Result<()> {
        let mut config = serde_yaml::to_value(PasterConfig::default())?;
        assert!(update_config_value(&mut config, "unknown", String::from("test")).is_err());
        Ok(())
    }

    #[test]
    fn update_plain_as_nested() -> Result<()> {
        let mut config = serde_yaml::to_value(PasterConfig::default())?;
        assert!(
            update_config_value(&mut config, "default.something", String::from("test")).is_err()
        );
        Ok(())
    }

    #[test]
    fn update_nested_as_plain() -> Result<()> {
        let mut config = serde_yaml::to_value(PasterConfig::default())?;
        assert!(update_config_value(&mut config, "dest", String::from("test")).is_err());
        Ok(())
    }
}
