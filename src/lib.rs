#![allow(unused)]
use std::{io::Error, vec};

mod request;
mod client;

pub use request::{
    MaxOnlinePlayers, ServerInfoBuilder, ServerInfoBuilderError, ServerInfoPlayer,
    ServerSoftware, ServersBuilder, ServersBuilderError, ServersServer,
    WhereisBuilder, WhereisBuilderError, WhereisServer,
};

pub use client::ServerSeekerClient;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Deserialize)]
#[serde(untagged)]
pub enum ServerSeekerError {
    // Only for ServerSeekerClient::new_checked()
    #[error("Invalid api_key")]
    InvalidApiKey,

    #[error("API returned error: {error:0}")]
    APIError { error: String },
}

//      |\      _,,,---,,_
//ZZZzz /,`.-'`'    -.  ;-;;,_
//     |,4-  ) )-,_. ,\ (  `'-'
//    '---''(_/--'  `-'\_)
