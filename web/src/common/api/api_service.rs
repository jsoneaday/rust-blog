use serde::Deserialize;
use super::models::NewPost;
use reqwest::Client;
use reqwest::Error;
use derive_more::Display;

pub const API_DEV_URL: &str = "http://0.0.0.0:4003/v1";

#[derive(Deserialize, Display, Debug)]
pub struct OutputId {
    pub id: i64
}

#[derive(Clone, Debug, Default)]
pub struct ApiService {
    client: Client
}

impl ApiService {
    pub fn new() -> Self {
        ApiService {
            client: Client::new()
        }
    }

    pub async fn create_post(&self, new_post: NewPost) -> Result<OutputId, Error> {
        self.client.post(format!("{}/{}", API_DEV_URL, "post"))
            .json(&new_post)
            .send()
            .await
            .unwrap()
            .json::<OutputId>()
            .await
    }
}