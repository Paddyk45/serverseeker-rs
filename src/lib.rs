#![allow(unused)]
use std::io::Error;

pub mod models;
use models::*;

use exitfailure::ExitFailure;
use minreq::head;

const API_URL: &str = "https://serverseeker.damcraft.de/api/v1";

impl ServerSeekerClient {
    pub fn new(api_key: String) -> Result<Self, Error> {
        Ok(ServerSeekerClient{api_key})
    }
}

impl ServerSeekerClient {
    /// Get all servers a player was on during a scan
    pub async fn whereis<F>(&self, f: F) -> Result<Vec<WhereisServer>, ExitFailure> 
    where F: FnOnce(WhereisBuilder) -> WhereisBuilder
    {
        let url = format!("{API_URL}/whereis");
        let mut builder = WhereisBuilder::new();
        builder.params.api_key = Some(self.api_key.clone());
        let f_builder = f(builder);
        let params = f_builder.build();
        let body = serde_json::to_string(&params).unwrap();
        let res = minreq::post(url)
            .with_header("Content-Type", "application/json")
            .with_body(body)
            .send()?;
        let data: WhereisData = serde_json::from_str(res.as_str().unwrap())?;
        Ok(data.data)
    }

    /// Get a list of random servers, optionally with criteria
    pub async fn servers<F>(&self, f: F) -> Result<Vec<ServersServer>, ExitFailure>
    where F: FnOnce(ServersBuilder) -> ServersBuilder
    {
        let url = format!("{API_URL}/servers");
        let mut builder = ServersBuilder::new();
        builder.params.api_key = Some(self.api_key.clone());
        let f_builder = f(builder);
        let params = f_builder.build();
        let body = serde_json::to_string(&params).unwrap();
        let res = minreq::post(url)
            .with_header("Content-Type", "application/json")
            .with_body(body)
            .send()?;
        let data: ServersData = serde_json::from_str(res.as_str().unwrap())?;
        Ok(data.data)
    }

    /// Get information about a server
    pub async fn server_info<F>(&self, f: F) -> Result<ServerInfoInfo, ExitFailure>
    where F: FnOnce(ServerInfoBuilder) -> ServerInfoBuilder
    {
        let url = format!("{API_URL}/server_info");
        let mut builder = ServerInfoBuilder::new();
        builder.params.api_key = Some(self.api_key.clone());
        let f_builder = f(builder);
        let params = f_builder.build();
        let body = serde_json::to_string(&params).unwrap();
        let res = minreq::post(url)
            .with_header("Content-Type", "application/json")
            .with_body(body)
            .send()?;
        let info: ServerInfoInfo = serde_json::from_str(res.as_str().unwrap())?;
        Ok(info)
    }
}

impl ServersBuilder {
    pub fn new() -> Self {
        Self {
            params: ServersParams {
                api_key: None,
                online_players: None, 
                max_players: None, 
                cracked: None, 
                description: None, 
                protocol: None, 
                online_after: None 
            }
        }
    }

    pub fn online_players(mut self, value: u16) -> Self {
        self.params.online_players = Some(value);
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

    pub fn build(self) -> ServersParams {
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

    pub fn build(self) -> WhereisParams {
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

    pub fn build(self) -> ServerInfoParams {
        self.params
    }
}

//      |\      _,,,---,,_
//ZZZzz /,`.-'`'    -.  ;-;;,_
//     |,4-  ) )-,_. ,\ (  `'-'
//    '---''(_/--'  `-'\_) 