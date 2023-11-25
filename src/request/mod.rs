mod server_info;

use serde::Deserialize;
use thiserror::Error;
pub use server_info::*;

mod servers;
pub use servers::*;

mod stats;
pub use stats::*;

mod user_info;
pub use user_info::*;

mod whereis;
pub use whereis::*;
use crate::ServerSeekerError;

/// A response from the ServerSeeker API
#[derive(Debug, Error, Deserialize)]
#[serde(untagged)]
pub(crate) enum APIResponse<T> {
    Error(ServerSeekerError),
    Data(T),
}
