use serde::{Deserialize, Serialize};

/// A ServerSeeker client which stores the api key
pub struct ServerSeekerClient {
    pub api_key: String
}

// For .whereis():
/// The search parameters
#[derive(Serialize)]
pub struct WhereisParams {
    /// Your api_key
    pub api_key: Option<String>,
    /// The name of the player you want to find
    pub name: Option<String>,
    /// The uuid of the player you want to find
    pub uuid: Option<String>
}

/// Builder
pub struct WhereisBuilder {
    pub params: WhereisParams
}

/// A server in the results
#[derive(Deserialize, Debug)]
pub struct WhereisServer {
    /// The last time the player was seen on the server (unix timestamp)
    pub last_seen: i64,
    /// The name of the player
    pub name: String,
    /// The ip and port of the server
    pub server: String,
    /// The uuid of the player 
    pub uuid: String
}

/// The data array from the response
#[derive(Deserialize)]
pub struct WhereisData {
    /// An array of servers the player was seen on. Limited to 1000
    pub data: Vec<WhereisServer>
}


// For .servers():
/// The search parameters
#[derive(Serialize)]
pub struct ServersParams {
    /// Your api_key
    pub api_key: Option<String>,
    /// The minimum amount of online players the server should have
    pub online_players: Option<u16>,
    /// The maximum amount of players the server should hold
    pub max_players: Option<u16>,
    /// Whether the server should be cracked or not (onlime mode disabled)
    pub cracked: Option<bool>,
    /// What the description aka MOTD of the servers should contain
    pub description: Option<String>,
    /// The [protocol version](https://wiki.vg/Protocol_version_numbers) of the server
    pub protocol: Option<i32>,
    /// The server should have been online after this unix timestamp Defaults to showing all servers which were online at last ping.
    pub online_after: Option<u16>
}

/// Builder
pub struct ServersBuilder {
    pub params: ServersParams
}

/// A server in the results
#[derive(Deserialize, Debug)]
pub struct ServersServer {
    /// Whether the server is cracked or not. None if unknown
    pub cracked: Option<bool>,
    /// The MOTD of the server
    pub description: String,
    /// The last time the server was seen (unix timestamp)
    pub last_seen: i64,
    /// The maximum amount of players the server can hold
    pub max_players: u32,
    /// The amount of players online during the last scan
    pub online_players: u32,
    /// The [protocol version](https://wiki.vg/Protocol_version_numbers) of the server
    pub protocol: i64,
    /// The ip:port of the server
    pub server: String,
    /// The minecraft version of the server
    pub version: String
}

/// The data array from the response
#[derive(Deserialize, Debug)]
pub struct ServersData {
    pub data: Vec<ServersServer>
}


// For .server_info()
/// The server ip/port
#[derive(Serialize)]
pub struct ServerInfoParams {
    /// Your api_key
    pub api_key: Option<String>,
    /// The IP of the server
    pub ip: String,
    /// The port of the server (default=25565)
    pub port: Option<u16>
}

/// Builder
pub struct ServerInfoBuilder {
    pub params: ServerInfoParams
}

/// The information about the server
#[derive(Serialize, Deserialize, Debug)]
pub struct ServerInfoInfo {
    /// Whether the server is cracked or not. None if unknown
    pub cracked: Option<bool>,
    /// The description (MOTD) of the server
    pub description: String,
    /// The last time the server was seen (unix timestamp)
    pub last_seen: i64,
    /// The maximum amount of players the server can hold
    pub max_players: u32,
    /// The amount of players online during the last scan
    pub online_players: u16,
    /// The [protocol version](https://wiki.vg/Protocol_version_numbers) of the server
    pub protocol: i64,
    /// The minecraft version of the server
    pub version: String,
    /// An array of when which players were seen on the server. Limited to 1000
    pub players: Vec<ServerInfoPlayer>
}

/// A player that was seen on a server
#[derive(Serialize, Deserialize, Debug)]
pub struct ServerInfoPlayer {
    /// The last time the player was seen on the server (unix timestamp)
    pub last_seen: i64,
    /// The name of the player
    pub name: String,
    /// The uuid of the player
    pub uuid: String
}