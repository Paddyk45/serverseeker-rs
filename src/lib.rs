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
    Data(T),
}

impl ServerSeekerClient {
    pub fn new(api_key: String) -> Self {
        let client = reqwest::Client::new();
        ServerSeekerClient {client, api_key}
    }
}

impl ServerSeekerClient {
    /// Get all servers a player was on during a scan
    pub async fn whereis<F>(&self, f: F) -> Result<Vec<WhereisServer>, failure::Error> 
    where F: FnOnce(WhereisBuilder) -> WhereisBuilder
    {
        let url = format!("{API_URL}/whereis");
        let mut builder = WhereisBuilder::new();
        builder.params.api_key = Some(self.api_key.clone());
        let f_builder = f(builder);
        let params = f_builder.build();
        let body = serde_json::to_string(&params).unwrap();
        let res = self.client.post(url)
            .header("Content-Type", "application/json")
            .body(body)
            .send().await?;
        let data: APIResponse<WhereisData> = serde_json::from_str(&res.text().await?)?;
        match data { 
            APIResponse::Data(d) => Ok(d.data),
            APIResponse::Error(e) => Err(failure::err_msg(e.error)),
            _ => Err(failure::err_msg("An unknown error occured"))
        }
    }

    /// Get a list of random servers, optionally with criteria
    pub async fn servers<F>(&self, f: F) -> Result<Vec<ServersServer>, failure::Error>
    where F: Fn(ServersBuilder) -> ServersBuilder
    {
        let url = format!("{API_URL}/servers");
        let mut builder = ServersBuilder::new();
        builder.params.api_key = Some(self.api_key.clone());
        let f_builder = f(builder);
        let params = f_builder.build();
        let body = serde_json::to_string(&params).unwrap();
        let res = self.client.post(url)
            .header("Content-Type", "application/json")
            .body(body)
            .send().await?;
        let data: APIResponse<ServersData> = serde_json::from_str(&res.text().await?)?;
        match data {
            APIResponse::Data(d) => Ok(d.data),
            APIResponse::Error(e) => Err(failure::err_msg(e.error)),
            _ => Ok(Vec::new())
        }
    }

    /// Get information about a server
    pub async fn server_info<F>(&self, f: F) -> Result<ServerInfoInfo, failure::Error>
    where F: FnOnce(ServerInfoBuilder) -> ServerInfoBuilder
    {
        let url = format!("{API_URL}/server_info");
        let mut builder = ServerInfoBuilder::new();
        builder.params.api_key = Some(self.api_key.clone());
        let f_builder = f(builder);
        let params = f_builder.build();
        let body = serde_json::to_string(&params).unwrap();
        let res = self.client.post(url)
            .header("Content-Type", "application/json")
            .body(body)
            .send().await?;
        let info: APIResponse<ServerInfoInfo> = serde_json::from_str(&res.text().await?)?;
        match info {
            APIResponse::Data(info) => Ok(info),
            APIResponse::Error(e) => Err(failure::err_msg(e.error)),
            _ => Err(failure::err_msg("An unknown error occured"))
        }
    }
}

impl ServersBuilder {
    pub fn new() -> Self {
        Self {
            params: ServersParams {
                api_key: None,
                num_online_players: None, 
                range_online_players: None,
                max_players: None, 
                cracked: None, 
                description: None, 
                protocol: None, 
                online_after: None,
                software: None,
                country_code: None,
                asn: None
            }
        }
    }
    pub fn num_online_players(mut self, value: u16) -> Self {
        self.params.num_online_players = Some(value);
        self
    }

    pub fn range_online_players(mut self, value: (u16, MaxOnlinePlayers)) -> Self {
        self.params.range_online_players = Some(value);
        self
    }

    pub fn max_players(mut self, value: u16) -> Self {
        self.params.max_players = Some(value);
        self
    }

    pub fn cracked(mut self, value: bool) -> Self {
        self.params.cracked = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.params.description = Some(value);
        self
    }

    pub fn protocol(mut self, value: i32) -> Self {
        self.params.protocol = Some(value);
        self
    }

    pub fn online_after(mut self, value: u16) -> Self {
        self.params.online_after = Some(value);
        self
    }

    pub fn software(mut self, value: ServerSoftware) -> Self {
        self.params.software = Some(value);
        self
    }

    pub fn country_code(mut self, value: String) -> Self {
        self.params.country_code = Some(value);
        self
    }

    pub fn asn(mut self, value: i16) -> Self {
        self.params.asn = Some(value);
        self
    }

    fn build(self) -> ServersParams {
        self.params
    }
}

impl WhereisBuilder {
    pub fn new() -> Self {
        Self {
            params: WhereisParams {
                api_key: None, 
                name: None, 
                uuid: None
            }
        }
    }

    pub fn name(mut self, value: String) -> Self{
        self.params.name = Some(value);
        self
    }

    pub fn uuid(mut self, value: String) -> Self{
        self.params.uuid = Some(value);
        self
    }

    fn build(self) -> WhereisParams {
        self.params
    }
}

impl ServerInfoBuilder {
    pub fn new() -> Self {
        Self {
            params: ServerInfoParams {
                api_key: None, 
                ip: String::new(), 
                port: None
            }
        }
    }

    pub fn ip(mut self, value: String) -> Self{
        self.params.ip = value;
        self
    }

    pub fn port(mut self, value: u16) -> Self{
        self.params.port = Some(value);
        self
    }

    fn build(self) -> ServerInfoParams {
        self.params
    }
}

//      |\      _,,,---,,_
//ZZZzz /,`.-'`'    -.  ;-;;,_
//     |,4-  ) )-,_. ,\ (  `'-'
//    '---''(_/--'  `-'\_) 
