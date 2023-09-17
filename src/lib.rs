#![allow(unused)]
use std::{io::Error, vec};

mod models;
use models::*;
pub use models::{
    MaxOnlinePlayers, ServerInfoBuilder, ServerInfoBuilderError, ServerInfoPlayer,
    ServerSeekerClient, ServerSoftware, ServersBuilder, ServersBuilderError, WhereisBuilder,
    WhereisBuilderError, WhereisServer,
};

use derive_builder::Builder;
use serde::Deserialize;

const API_URL: &str = "https://api.serverseeker.net";

/// A response from the ServerSeeker API
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum APIResponse<T> {
    Error(APIError),
    Data(T),
}

impl ServerSeekerClient {
    pub fn new(api_key: String) -> Self {
        let client = reqwest::Client::new();
        ServerSeekerClient { client, api_key }
    }
}

impl ServerSeekerClient {
    /// Get all servers a player was on during a scan
    pub async fn whereis(
        &self,
        builder: &WhereisBuilder,
    ) -> Result<Vec<WhereisServer>, failure::Error> {
        let url = format!("{API_URL}/whereis");
        let mut params = builder.build()?;
        params.api_key = Some(self.api_key.clone());
        let body = serde_json::to_string(&params).unwrap();
        let res = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?;
        let data: APIResponse<WhereisData> = serde_json::from_str(&res.text().await?)?;
        match data {
            APIResponse::Data(d) => Ok(d.data),
            APIResponse::Error(e) => Err(failure::err_msg(e.error)),
            _ => Err(failure::err_msg("An unknown error occured")),
        }
    }

    /// Get a list of random servers, optionally with criteria
    pub async fn servers(
        &self,
        builder: &ServersBuilder,
    ) -> Result<Vec<ServersServer>, failure::Error> {
        let url = format!("{API_URL}/servers");
        let mut params = builder.build()?;
        params.api_key = Some(self.api_key.clone());
        let body = serde_json::to_string(&params).unwrap();
        let res = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?;
        let data: APIResponse<ServersData> = serde_json::from_str(&res.text().await?)?;
        match data {
            APIResponse::Data(d) => Ok(d.data),
            APIResponse::Error(e) => Err(failure::err_msg(e.error)),
            _ => Ok(Vec::new()),
        }
    }

    /// Get information about a server
    pub async fn server_info(
        &self,
        builder: &ServerInfoBuilder,
    ) -> Result<ServerInfoInfo, failure::Error> {
        let url = format!("{API_URL}/server_info");
        let mut params = builder.build()?;
        params.api_key = Some(self.api_key.clone());
        let body = serde_json::to_string(&params).unwrap();
        let res = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?;
        let info: APIResponse<ServerInfoInfo> = serde_json::from_str(&res.text().await?)?;
        match info {
            APIResponse::Data(info) => Ok(info),
            APIResponse::Error(e) => Err(failure::err_msg(e.error)),
            _ => Err(failure::err_msg("An unknown error occured")),
        }
    }
}

//      |\      _,,,---,,_
//ZZZzz /,`.-'`'    -.  ;-;;,_
//     |,4-  ) )-,_. ,\ (  `'-'
//    '---''(_/--'  `-'\_)
