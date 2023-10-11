#![no_std]
#![cfg_attr(feature = "std", feature(std))]

#[cfg(feature = "std")]
use std::{io::Error, vec};

#[cfg(not(feature = "std"))]
use {
    core::fmt::Error,
    alloc::vec::Vec,
};

mod models;
use models::*;
pub use models::{
    MaxOnlinePlayers, ServerInfoBuilder, ServerInfoBuilderError, ServerInfoPlayer,
    ServerSeekerClient, ServerSeekerError, ServerSoftware, ServersBuilder, ServersBuilderError,
    WhereisBuilder, WhereisBuilderError, WhereisServer,
};

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

//      |\      _,,,---,,_
//ZZZzz /,`.-'`'    -.  ;-;;,_
//     |,4-  ) )-,_. ,\ (  `'-'
//    '---''(_/--'  `-'\_)
