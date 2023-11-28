use crate::ServerSeekerClient;
use derive_builder::Builder;
use reqwest::Method;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Clone)]
pub enum MaxOnlinePlayers {
    Num(u16),
    Inf,
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
#[derive(Serialize, Clone)]
pub enum ServerSoftware {
    Any,
    Vanilla,
    Paper,
    Spigot,
    Bukkit,
}

/// The search parameters
#[derive(Serialize, Builder, Clone, Default)]
#[builder(name = "ServersBuilder", public, setter(strip_option), default)]
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
    #[builder(setter(into))]
    pub description: Option<String>,

    /// The [protocol version](https://wiki.vg/Protocol_version_numbers) of the server
    pub protocol: Option<i32>,

    /// The server should have been online after this unix timestamp Defaults to showing all servers which were online at last ping.
    pub online_after: Option<u16>,

    /// The software of the server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software: Option<ServerSoftware>,

    /// The country code of the server. See [here](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)
    #[builder(setter(into))]
    pub country_code: Option<String>,

    /// The AS number of the server. You can get it easily from ipinfo. See [here](https://en.wikipedia.org/wiki/Autonomous_system_(Internet))
    pub asn: Option<i16>,

    /// If Forge servers should be excluded
    pub ignore_modded: Option<bool>
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
    pub max_players: i64,

    /// The amount of players online during the last scan
    pub online_players: i64,

    /// The [protocol version](https://wiki.vg/Protocol_version_numbers) of the server
    pub protocol: i64,

    /// The ip:port of the server
    pub server: String,

    /// The minecraft version of the server
    pub version: String,
}

/// The data array from the response
#[derive(Deserialize, Debug)]
pub(crate) struct ServersData {
    pub data: Vec<ServersServer>,
}

impl ServerSeekerClient {
    /// Get a list of random servers, optionally with criteria
    pub async fn servers(
        &self,
        builder: &ServersBuilder,
    ) -> anyhow::Result<Vec<ServersServer>> {
        let mut params = builder.build()?;
        Ok(self
            .request::<ServersData, _, _>("/servers", params, Method::POST)
            .await?
            .data)
    }
}
