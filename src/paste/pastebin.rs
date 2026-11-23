use super::*;

use anyhow::{Context, Error, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};


// -------------------------------------------------------------------------------------- Public --


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PastebinConfig {
    /// Developer API key
    pub dev_key: String,
    /// User key to paste as a logged in member
    pub user_key: Option<String>,
}


// ---------------------------------------------------------------------------------------- Impl --


impl Default for PastebinConfig {
    fn default() -> Self {
        PastebinConfig {
            dev_key: String::from("<your dev API key>"),
            user_key: None,
        }
    }
}

struct Pastebin {
    dev_key: String,
    user_key: Option<String>,

    client: Client,
}

impl PasteDestination for Pastebin {
    fn paste(&self, buf: PasteBuffer) -> Result<Paste> {
        let mut data = reqwest::blocking::multipart::Form::new()
            .text("api_option", "paste")
            .text("api_dev_key", self.dev_key.clone())
            .text("api_paste_code", buf.text);
        if let Some(user_key) = &self.user_key {
            data = data.text("api_user_key", user_key.clone());
        }

        let resp = self
            .client
            .post("https://pastebin.com/api/api_post.php")
            .multipart(data)
            .send()
            .with_context(|| "Failed to send request to pastebin.com")?;
        if resp.status().is_success() {
            let uri = resp.text()?;
            Ok(Paste {
                name: "Untitled".to_string(),
                uri
            })
        } else {
            Err(Error::msg(format!(
                "pastebin.com responded with error code {}",
                resp.status().as_str()
            )))
        }
    }
}

impl From<PastebinConfig> for Box<dyn PasteDestination> {
    fn from(val: PastebinConfig) -> Self {
        Box::new(Pastebin {
            dev_key: val.dev_key,
            user_key: val.user_key,
            client: Client::new(),
        })
    }
}
