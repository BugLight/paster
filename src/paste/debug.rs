use super::*;

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

impl From<DebugConfig> for Box<dyn Paste> {
    fn from(_val: DebugConfig) -> Self {
        Box::new(Debug)
    }
}
