use std::collections::HashMap;

use anyhow::{Error, Result};
use either::Either;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PasterConfig {
    /// Default destination name
    default: String,
    /// Different paste destinations
    dest: HashMap<String, DestinationConfig>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DestinationConfig {
    /// pastebin.com destination
    Pastebin {
        /// Developer API key
        dev_key: String,
        /// User key to paste as a logged in member
        user_key: Option<String>,
    },
}

trait GetByKey {
    fn get_by_key(&mut self, key: &str) -> Result<Either<&mut String, &mut dyn GetByKey>>;
}

// TODO: Make derive macro
impl GetByKey for PasterConfig {
    fn get_by_key(&mut self, key: &str) -> Result<Either<&mut String, &mut dyn GetByKey>> {
        if key == "default" {
            Ok(Either::Left(&mut self.default))
        } else if key == "dest" {
            Ok(Either::Right(&mut self.dest))
        } else {
            Err(Error::msg("Key not found"))
        }
    }
}

impl<T> GetByKey for HashMap<String, T>
where
    T: GetByKey,
{
    fn get_by_key(&mut self, key: &str) -> Result<Either<&mut String, &mut dyn GetByKey>> {
        self.get_mut(key)
            .map(|x| Either::<&mut String, &mut dyn GetByKey>::Right(x))
            .ok_or(Error::msg("Key not found"))
    }
}

impl GetByKey for DestinationConfig {
    fn get_by_key(&mut self, key: &str) -> Result<Either<&mut String, &mut dyn GetByKey>> {
        match self {
            DestinationConfig::Pastebin {
                ref mut dev_key,
                ref mut user_key,
            } => {
                if key == "dev_key" {
                    Ok(Either::Left(dev_key))
                } else if key == "user_key" {
                    Ok(Either::Left(user_key.get_or_insert(String::from(""))))
                } else {
                    Err(Error::msg("Key not found"))
                }
            }
        }
    }
}

impl Default for PasterConfig {
    fn default() -> Self {
        let default = String::from("pastebin");
        let mut destinations = HashMap::new();
        destinations.insert(
            default.clone(),
            DestinationConfig::Pastebin {
                dev_key: String::from("<your dev API key>"),
                user_key: None,
            },
        );

        PasterConfig {
            default,
            dest: destinations,
        }
    }
}

pub fn update_config_value(config: &mut PasterConfig, key: &str, value: String) -> Result<()> {
    update_value_by_key(config, key, value)
}

fn update_value_by_key(config: &mut dyn GetByKey, key: &str, value: String) -> Result<()> {
    match key.split_once('.') {
        Some((left, right)) => match config.get_by_key(left)? {
            Either::Left(_) => Err(Error::msg("Expected nested structure but got plain")),
            Either::Right(subconfig) => update_value_by_key(subconfig, right, value),
        },
        None => match config.get_by_key(key)? {
            Either::Left(val) => {
                *val = value;
                Ok(())
            }
            Either::Right(_) => Err(Error::msg("Expected plain value but got nested")),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_plain() -> Result<()> {
        let mut config = PasterConfig::default();
        update_value_by_key(&mut config, "default", String::from("test"))?;
        assert_eq!(config.default, "test");
        Ok(())
    }

    #[test]
    fn update_nested() -> Result<()> {
        let mut config = PasterConfig::default();
        update_value_by_key(&mut config, "dest.pastebin.dev_key", String::from("test"))?;
        match config.dest.get("pastebin").unwrap() {
            DestinationConfig::Pastebin {
                dev_key,
                user_key: _,
            } => {
                assert_eq!(dev_key, "test");
            }
        }
        Ok(())
    }

    #[test]
    fn update_unknown_key() {
        let mut config = PasterConfig::default();
        assert!(update_value_by_key(&mut config, "unknown", String::from("test")).is_err());
    }

    #[test]
    fn update_plain_as_nested() {
        let mut config = PasterConfig::default();
        assert!(
            update_value_by_key(&mut config, "default.something", String::from("test")).is_err()
        );
    }

    #[test]
    fn update_nested_as_plain() {
        let mut config = PasterConfig::default();
        assert!(update_value_by_key(&mut config, "dest", String::from("test")).is_err());
    }
}
