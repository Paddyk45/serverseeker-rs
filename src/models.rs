use derive_builder::Builder;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Value;

/// A ServerSeeker client which stores the api key
pub struct ServerSeekerClient {
    pub api_key: String
}

/// An error
#[derive(Deserialize, Debug)]
pub struct APIError {
    pub error: String
}

/// A response
#[derive(Deserialize, Debug)]
pub struct APIData {
    pub data: Value
}

#[derive(Clone, Serialize)]
pub enum UsernameOrUuid {
    Username(String),
    Uuid(String)
}

// For .whereis():
#[derive(Clone, Builder, Serialize)]
#[builder(setter(into))]
pub struct WhereisParams {
    /// The name or uuid of the player you want to find
    pub player: UsernameOrUuid,
}

#[derive(Clone, Deserialize, Debug)]
pub struct WhereisServers(pub Vec<WhereisServer>);

/// A server in the results
#[derive(Clone, Deserialize, Debug)]
pub struct WhereisServer {
    /// The last time the player was seen on the server (unix timestamp)
    pub last_seen: i64,
    /// The name of the player
    pub name: String,
    /// The ip:port of the server
    pub server: String,
    /// The uuid of the player 
    pub uuid: String
}

// For .servers():
#[derive(Clone)]
pub enum MaxOnlinePlayers {
    Num(u16),
    Inf
}



impl Serialize for MaxOnlinePlayers {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            MaxOnlinePlayers::Num(value) => serializer.serialize_u16(*value),
            MaxOnlinePlayers::Inf => serializer.serialize_str("inf"),
        }
    }
}

/// Software of a server
#[derive(Clone, Serialize)]
pub enum ServerSoftware {
    Any,
    Vanilla,
    Paper,
    Spigot,
    Bukkit
}

/// The search parameters
#[derive(Clone, Builder, Serialize)]
#[builder(setter(into))]
pub struct ServersParams {
    /// The amount of online players the server should have
    #[serde(rename = "online_players")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_online_players: Option<u16>,
    /// Min-Max range of online players
    #[serde(rename = "online_players")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_online_players: Option<(u16, MaxOnlinePlayers)>,
    /// The maximum amount of players the server should hold
    pub max_players: Option<u16>,
    /// Whether the server should be cracked or not (onlime mode disabled)
    pub cracked: Option<bool>,
    /// What the description aka MOTD of the servers should contain
    pub description: Option<String>,
    /// The [protocol version](https://wiki.vg/Protocol_version_numbers) of the server
    pub protocol: Option<i32>,
    /// The server should have been online after this unix timestamp Defaults to showing all servers which were online at last ping.
    pub online_after: Option<u16>,
    /// The software of the server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software: Option<ServerSoftware>,
    /// The country code of the server. See [here](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)
    pub country_code: Option<String>,
    /// The AS number of the server. You can get it easily from ipinfo. See [here](https://en.wikipedia.org/wiki/Autonomous_system_(Internet))
    pub asn: Option<i16>
}

#[derive(Clone, Deserialize, Debug)]
pub struct ServersServers(pub Vec<ServersServer>);

/// A server in the results
#[derive(Clone, Deserialize, Debug)]
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
    pub version: String,
}


// For .server_info()
/// The server ip/port
#[derive(Clone, Builder, Serialize)]
#[builder(setter(into))]
pub struct ServerInfoParams {
    /// The IP of the server
    pub ip: String,
    /// The port of the server (default=25565)
    pub port: Option<u16>
}

/// The information about the server
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ServerInfo {
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
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ServerInfoPlayer {
    /// The last time the player was seen on the server (unix timestamp)
    pub last_seen: i64,
    /// The name of the player
    pub name: String,
    /// The uuid of the player
    pub uuid: String
}