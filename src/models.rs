use serde::{Deserialize, Serialize};


#[derive(Serialize, Debug)]
pub struct WhereisParams {
    pub api_key: Option<String>,
    pub name: Option<String>,
    pub uuid: Option<String>
}

pub struct WhereisBuilder {
    pub params: WhereisParams
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WhereisServer {
    pub last_seen: i64,
    pub name: String,
    pub server: String,
    pub uuid: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WhereisData {
    pub data: Vec<WhereisServer>
}



#[derive(Serialize, Debug)]
pub struct ServersParams {
    pub api_key: Option<String>,
    pub online_players: Option<u16>,
    pub max_players: Option<u16>,
    pub cracked: Option<bool>,
    pub description: Option<String>,
    pub protocol: Option<i32>,
    pub online_after: Option<u16>
}

pub struct ServersBuilder {
    pub params: ServersParams
}

/// A server from the server search
#[derive(Serialize, Deserialize, Debug)]
pub struct ServersServer {
    /// If the server has online mode enabled
    pub cracked: Option<bool>,
    /// The MOTD of the server
    pub description: String,
    /// Unix timestamp of when the server was last scanned online
    pub last_seen: i64,
    /// The maximum amount of players on the server
    pub max_players: u32,
    /// The amount of online players at the time of the scan
    pub online_players: u32,
    /// The [protocol version](https://wiki.vg/Protocol_version_numbers) of the server
    pub protocol: i64,
    /// The ip:port of the server
    pub server: String,
    /// The minecraft version of the server
    pub version: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServersData {
    pub data: Vec<ServersServer>
}