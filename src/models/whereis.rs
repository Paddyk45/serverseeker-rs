use derive_builder::Builder;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Serialize, Builder, Default)]
#[builder(name = "WhereisBuilder", public, setter(strip_option), default)]
pub(crate) struct WhereisParams<T: Into<String> + Default> {
    /// Your api_key
    #[builder(setter(skip))]
    pub api_key: Option<String>,

    /// The name of the player you want to find
    pub name: Option<T>,

    /// The uuid of the player you want to find
    pub uuid: Option<T>,
}

/// A server in the results
#[derive(Deserialize, Debug)]
pub struct WhereisServer {
    /// The last time the player was seen on the server (unix timestamp)
    pub last_seen: i64,

    /// The name of the player
    pub name: String,

    /// The ip:port of the server
    pub server: String,

    /// The uuid of the player
    pub uuid: String,
}

/// The data array from the response
#[derive(Deserialize, Debug)]
pub(crate) struct WhereisData {
    /// An array of servers the player was seen on. Limited to 1000
    pub data: Vec<WhereisServer>,
}
