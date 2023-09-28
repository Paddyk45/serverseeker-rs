use derive_builder::Builder;
use serde::{Deserialize, Serialize, Serializer};
use crate::ServerSeekerClient;

/// The server ip/port
#[derive(Serialize, Builder, Default)]
#[builder(name = "ServerInfoBuilder", public, setter(strip_option), default)]
pub struct ServerInfoParams<T: Into<String> + Default> {
    /// Your api_key
    #[builder(setter(skip))]
    pub api_key: Option<String>,

    /// The IP of the server
    pub ip: T,

    /// The port of the server (default=25565)
    pub port: Option<u16>,
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
    pub players: Vec<ServerInfoPlayer>,
}

/// A player that was seen on a server
#[derive(Serialize, Deserialize, Debug)]
pub struct ServerInfoPlayer {
    /// The last time the player was seen on the server (unix timestamp)
    pub last_seen: i64,

    /// The name of the player
    pub name: String,

    /// The uuid of the player
    pub uuid: String,
}

impl ServerSeekerClient {
    /// Get information about a server
    pub async fn server_info<T: Into<String> + Default + Clone + Serialize>(
        &self,
        builder: &ServerInfoBuilder<T>,
    ) -> anyhow::Result<ServerInfoInfo> {
        let mut params = builder.build()?;
        params.api_key = Some(self.api_key.clone());
        Ok(self.request::<ServerInfoInfo, _, _>("/server_info", params).await?)
    }
}
