use super::models::{LoginCredential, LoginResponse, UpdatePost, Post};
use super::models::{OutputId, NewPost};
use reqwest::header::HeaderMap;
use reqwest::{Client, StatusCode};
use reqwest::Error;

#[derive(Clone, Debug, Default)]
pub struct ApiService {
    client: Client,
    api_url: String
}

impl ApiService {
    pub fn new() -> Self {
        // todo: replace with dynamic env variable when ready
        let api_url = "https://127.0.0.1:4003/v1".to_string();

        ApiService {
            client: Client::new(),
            api_url
        }
    }

    pub async fn create_post(&self, new_post: &NewPost, token: String) -> Result<OutputId, Error> {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

        let post_resp = self.client.post(format!("{}/{}", self.api_url, "post"))
            .headers(headers)
            .json(new_post)
            .send()
            .await;

        match post_resp {
            Ok(output_id) => output_id.json::<OutputId>().await,
            Err(e) => Err(e)
        }
    }

    pub async fn update_post(&self, update_post: &UpdatePost, token: String) -> Result<(), Error> {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

        let post_resp = self.client.post(format!("{}/{}", self.api_url, "update_post"))
            .headers(headers)
            .json(update_post)
            .send()
            .await;

        match post_resp {
            Ok(resp) => {
                match resp.status() {
                    StatusCode::NO_CONTENT => Ok(()),
                    _ => Err(resp.error_for_status().err().unwrap())
                }
            },
            Err(e) => Err(e)
        }
    }

    pub async fn login(&self, credentials: &LoginCredential) -> Result<LoginResponse, Error> {
        let login_res = self.client.post(format!("{}/{}", self.api_url, "login"))
            .json(credentials)
            .send()
            .await;

        match login_res {
            Ok(res) => {
                match res.status() {
                    StatusCode::OK => res.json::<LoginResponse>().await,
                    _ => Err(res.error_for_status().err().unwrap())
                } 
            },
            Err(e) => Err(e)
        }               
    }

    pub async fn get_latest_posts(&self, last_offset: i32) -> Result<Vec<Post>, Error> {
        let posts = self.client.get(format!("{}/{}/10/{}", self.api_url, "post", last_offset))
            .send()
            .await;

        match posts {
            Ok(res) => {
                match res.status() {
                    StatusCode::OK => res.json::<Vec<Post>>().await,
                    _ => Err(res.error_for_status().err().unwrap())
                }
            },
            Err(e) => Err(e)
        }
    }

    pub async fn get_post(&self, post_id: i64) -> Result<Option<Post>, Error> {
        let post_resp = self.client.get(format!("{}/{}/{}", self.api_url, "post", post_id))
            .send()
            .await;

        match post_resp {
            Ok(res) => {
                match res.status() {
                    StatusCode::OK => res.json::<Option<Post>>().await,
                    _ => Err(res.error_for_status().err().unwrap())
                }
            },
            Err(e) => Err(e)
        }
    }
}