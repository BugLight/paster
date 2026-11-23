use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum HttpApiMethod {
    Get,
    Post
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpApiConfig {
    endpoint_url: String,
    method: HttpApiMethod,
    authorization: Option<String>,
    text_key: String,
    syntax_key: String,
}
