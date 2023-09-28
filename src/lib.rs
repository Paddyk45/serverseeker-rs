#![allow(unused)]
use std::{io::Error, vec};

mod models;
use models::*;
pub use models::{
    MaxOnlinePlayers, ServerInfoBuilder, ServerInfoBuilderError, ServerInfoPlayer,
    ServerSeekerClient, ServerSoftware, ServersBuilder, ServersBuilderError, WhereisBuilder,
    WhereisBuilderError, WhereisServer,
};

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

//      |\      _,,,---,,_
//ZZZzz /,`.-'`'    -.  ;-;;,_
//     |,4-  ) )-,_. ,\ (  `'-'
//    '---''(_/--'  `-'\_)
