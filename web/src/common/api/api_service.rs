use super::models::LoginCredential;
use super::models::{OutputId, NewPost};
use reqwest::{Client, StatusCode};
use reqwest::Error;

pub const API_DEV_URL: &str = "http://0.0.0.0:4003/v1";

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

    pub async fn create_post(&self, new_post: &NewPost) -> Result<OutputId, Error> {
        self.client.post(format!("{}/{}", API_DEV_URL, "post"))
            .json(new_post)
            .send()
            .await
            .unwrap()
            .json::<OutputId>()
            .await
    }

    pub async fn login(&self, credentials: &LoginCredential) -> Result<String, Error> {
        let login_res = self.client.post(format!("{}/{}", API_DEV_URL, "login"))
            .json(credentials)
            .send()
            .await;

        match login_res {
            Ok(res) => {
                match res.status() {
                    StatusCode::OK => {
                        let token_res = res.bytes().await;
                        match token_res {
                            Ok(token_bytes) => Ok(String::from_utf8_lossy(&token_bytes).to_string()),
                            Err(e) => Err(e)
                        }                        
                    },
                    _ => Err(res.error_for_status().err().unwrap())
                } 
            },
            Err(e) => Err(e)
        }               
    }
}