use crate::config::GetByKey;

use super::*;

use anyhow::Error;
use either::Either;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DebugConfig;

struct Debug;

impl Paste for Debug {
    fn paste(&self, mut source: Box<dyn Read>) -> Result<String> {
        let mut text = String::new();
        source.read_to_string(&mut text)?;
        Ok(text)
    }
}

impl GetByKey for DebugConfig {
    fn get_by_key(&mut self, _key: &str) -> Result<Either<&mut String, &mut dyn GetByKey>> {
        Err(Error::msg(""))
    }
}

impl Into<Box<dyn Paste>> for DebugConfig {
    fn into(self) -> Box<dyn Paste> {
        Box::new(Debug)
    }
}
