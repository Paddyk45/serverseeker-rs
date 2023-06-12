use std::io::Error;

mod models;
use models::*;

use exitfailure::ExitFailure;
use reqwest::header::CONTENT_TYPE;

const API_URL: &str = "https://serverseeker.damcraft.de/api/v1";

pub struct ServerSeekerClient {
    client: reqwest::Client,
    api_key: String
}

impl ServerSeekerClient {
    pub fn new(api_key: String) -> Result<Self, Error> {
        let client = reqwest::Client::new();
        Ok(ServerSeekerClient{client, api_key})
    }
}

impl ServerSeekerClient {
    /// Returns a Vect of WhereisServers
    /// 
    /// playername: the name of the player you want to get recent servers from
    pub async fn whereis(&self, playername: &str) -> Result<Vec<WhereisServer>, ExitFailure> {
        let url = format!("{API_URL}/whereis");
        let body = format!(r#"{{ "api_key": "{}","name": "{}" }}"#, self.api_key, playername);
        let res = self.client.post(url)
            .header(CONTENT_TYPE, "application/json")
            .body(body)
            .send().await?
            .text().await
            .unwrap();
        let data: WhereisServerData = serde_json::from_str(&res)?;
        Ok(data.data)
    }
}