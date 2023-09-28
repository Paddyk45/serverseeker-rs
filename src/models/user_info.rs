use std::fmt::format;
use serde::Deserialize;
use crate::ServerSeekerClient;

#[derive(Deserialize, Debug)]
pub struct UserInfo {
    pub discord_id: String,
    pub discord_username: String,
    pub discord_avatar_url: String,
    pub requests_per_day_server_info: u16,
    pub requests_per_day_servers: u16,
    pub requests_per_day_whereis: u16,
    pub requests_made_server_info: u16,
    pub requests_made_servers: u16,
    pub requests_made_whereis: u16
}

impl ServerSeekerClient {
    pub async fn user_info(&self) -> anyhow::Result<UserInfo> {
        Ok(self.client.get(format!("{}/user_info", Self::API_URL)).send().await?.json().await?)
    }
}