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

impl From<DebugConfig> for Box<dyn Paste> {
    fn from(_val: DebugConfig) -> Self {
        Box::new(Debug)
    }
}
