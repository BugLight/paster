use crate::config::GetByKey;

use super::*;

use anyhow::{Context, Error, Result};
use either::Either;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PastebinConfig {
    /// Developer API key
    pub dev_key: String,
    /// User key to paste as a logged in member
    pub user_key: Option<String>,
}

struct Pastebin {
    dev_key: String,
    user_key: Option<String>,

    client: Client,
}

impl Paste for Pastebin {
    fn paste(&self, mut source: Box<dyn Read>) -> Result<String> {
        let mut text = String::new();
        source.read_to_string(&mut text)?;

        let mut data = reqwest::blocking::multipart::Form::new()
            .text("api_option", "paste")
            .text("api_dev_key", self.dev_key.clone())
            .text("api_paste_code", text);
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
            Ok(resp.text()?)
        } else {
            Err(Error::msg(format!(
                "pastebin.com responded with error code {}",
                resp.status().as_str()
            )))
        }
    }
}

impl From<PastebinConfig> for Box<dyn Paste> {
    fn from(val: PastebinConfig) -> Self {
        Box::new(Pastebin {
            dev_key: val.dev_key,
            user_key: val.user_key,
            client: Client::new(),
        })
    }
}

impl GetByKey for PastebinConfig {
    fn get_by_key(&mut self, key: &str) -> Result<Either<&mut String, &mut dyn GetByKey>> {
        if key == "dev_key" {
            Ok(Either::Left(&mut self.dev_key))
        } else if key == "user_key" {
            Ok(Either::Left(self.user_key.get_or_insert(String::from(""))))
        } else {
            Err(Error::msg("Key not found"))
        }
    }
}
