use super::*;


use arboard::Clipboard;
use serde::{Deserialize, Serialize};


// -------------------------------------------------------------------------------------- Public --


#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DebugConfig;


// ---------------------------------------------------------------------------------------- Impl --


struct Debug;

impl PasteDestination for Debug {
    fn paste(&self, buf: PasteBuffer) -> Result<Paste> {
        let mut clipboard = Clipboard::new()?;
        clipboard.set_text(buf.text)?;
        Ok(Paste {
            name: "Unnamed".to_string(),
            uri: "".to_string()
        })
    }
}

impl From<DebugConfig> for Box<dyn PasteDestination> {
    fn from(_val: DebugConfig) -> Self {
        Box::new(Debug)
    }
}
