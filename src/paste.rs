use std::io::Read;

use anyhow::Result;

pub trait Paste {
    /// Pastes text from source and returns URL to it on success
    fn paste(&self, source: Box<dyn Read>) -> Result<String>;
}

pub mod debug;
pub mod pastebin;
