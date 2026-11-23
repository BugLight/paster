use anyhow::Result;

use std::fmt::Display;


pub mod debug;
pub mod pastebin;


// -------------------------------------------------------------------------------------- Public --


pub struct Paste {
    pub name: String,
    pub uri: String,
}

#[derive(Default)]
pub struct PasteBuffer {
    pub text: String,
    pub name: Option<String>,
    pub syntax: Option<String>,
}

pub trait PasteDestination {
    /// Pastes text from the buffer and returns a new paste on success
    ///
    /// Implementations might not support and ignore optional fields.
    /// Implementations shall accept any combination of buffer fields.
    fn paste(&self, buf: PasteBuffer) -> Result<Paste>;
}


// ---------------------------------------------------------------------------------------- Impl --


impl PasteBuffer {
    pub fn empty() -> PasteBuffer {
        Self::default()
    }
}

impl Display for Paste {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.uri)?;
        Ok(())
    }
}
