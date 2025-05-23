use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Link {
    pub url: String,
    pub title: String
}

impl Link {
    pub fn new(url: String, title: String) -> Self {
        Self { url, title }
    }
}
