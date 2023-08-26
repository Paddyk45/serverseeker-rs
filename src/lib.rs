#![allow(unused)]
use std::{io::Error, vec};

mod models;
pub use models::*;

use serde::Deserialize;

const API_URL: &str = "https://api.serverseeker.net";

/// A response from the ServerSeeker API
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum APIResponse<T> {
    Error(APIError),
    Data(T)
}

impl ServerSeekerClient {
    pub fn new(api_key: String) -> Self {
        ServerSeekerClient{api_key}
    }
}

impl ServerSeekerClient {
    /// Get all servers a player was on during a scan
    pub async fn whereis(&self, params: WhereisParams) -> Result<WhereisServers, failure::Error>
    {
        let url = format!("{API_URL}/whereis");
        let body = serde_json::to_string(&params).unwrap();
        let res = minreq::post(url)
            .with_header("Authorization", format!("Bearer {}", self.api_key))
            .with_header("Content-Type", "application/json")
            .with_body(body)
            .send()?;
        let data: APIResponse<WhereisServers> = serde_json::from_str(res.as_str().unwrap())?;
        match data {
            APIResponse::Data(d) => Ok(d),
            APIResponse::Error(e) => Err(failure::err_msg(e.error)),
            _ => Err(failure::err_msg("Invalid Response"))
        }
    }

    /// Get a list of random servers, optionally with criteria
    pub async fn servers(&self, params: ServersParams) -> Result<ServersServers, failure::Error>
    {
        let url = format!("{API_URL}/servers");
        let body = serde_json::to_string(&params).unwrap();
        let res = minreq::post(url)
            .with_header("Authorization", format!("Bearer {}", self.api_key))
            .with_header("Content-Type", "application/json")
            .with_body(body)
            .send()?;
        let data: APIResponse<ServersServers> = serde_json::from_str(res.as_str().unwrap())?;
        match data {
            APIResponse::Data(d) => Ok(d),
            APIResponse::Error(e) => Err(failure::err_msg(e.error)),
            _ => Err(failure::err_msg("Invalid Response"))
        }
    }

    /// Get information about a server
    pub async fn server_info(&self, params: ServerInfoParams) -> Result<ServerInfoServers, failure::Error>
    {
        let url = format!("{API_URL}/server_info");
        let body = serde_json::to_string(&params).unwrap();
        let res = minreq::post(url)
            .with_header("Authorization", format!("Bearer {}", self.api_key))
            .with_header("Content-Type", "application/json")
            .with_body(body)
            .send()?;
        let info: APIResponse<ServerInfoServers> = serde_json::from_str(res.as_str().unwrap())?;
        match info {
            APIResponse::Data(info) => Ok(info),
            APIResponse::Error(e) => Err(failure::err_msg(e.error)),
            _ => Err(failure::err_msg("Invalid Response"))
        }
    }
}

//      |\      _,,,---,,_
//ZZZzz /,`.-'`'    -.  ;-;;,_
//     |,4-  ) )-,_. ,\ (  `'-'
//    '---''(_/--'  `-'\_) 
