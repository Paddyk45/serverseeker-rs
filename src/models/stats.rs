use std::fmt::format;
use serde::Deserialize;
use crate::ServerSeekerClient;

#[derive(Deserialize, Debug)]
pub struct Stats {
    /// The amount of servers in the database
    pub server_count: u32,
    /// How many player records are in the database
    pub player_history_count: u64,
    /// How many of the servers  in the database are cracked
    pub cracked_servers: u32,
    /// How many of the servers in the database are online
    pub currently_online: u32
}

impl ServerSeekerClient {
    pub async fn stats(&self) -> anyhow::Result<Stats> {
        Ok(self.client.get(format!("{}/stats", Self::API_URL)).send().await?.json().await?)
    }
}