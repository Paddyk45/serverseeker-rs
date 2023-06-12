use serde::{Deserialize, Serialize};


/// A server from the server search
#[derive(Serialize, Deserialize, Debug)]
pub struct DiscoveredServer {
    /// If the server has online mode enabled
    pub cracked: bool,
    /// The MOTD of the server
    pub description: String,
    /// Unix timestamp of when the server was last scanned online
    pub last_seen: i64,
    /// The maximum amount of players on the server
    pub max_players: u32,
    /// The amount of online players at the time of the scan
    pub online_players: u32,
    /// The [protocol version](https://wiki.vg/Protocol_version_numbers) of the server
    pub protocol: u16,
    pub ip: String,
    pub version: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WhereisServer {
    pub last_seen: i64,
    pub name: String,
    pub server: String,
    pub uuid: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WhereisServerData {
    pub data: Vec<WhereisServer>
}

