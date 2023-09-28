use derive_builder::Builder;
use serde::{Deserialize, Serialize, Serializer};

/// A ServerSeeker client which stores the api key
pub struct ServerSeekerClient{
    pub client: reqwest::Client,
    pub api_key: String,
}

/// An error
#[derive(Deserialize, Debug)]
pub(crate) struct APIError {
    pub error: String,
}
