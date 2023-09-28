use std::any::{type_name, TypeId};
use std::fmt::format;
use anyhow::{bail, Error};
use derive_builder::Builder;
use serde::{Deserialize, Serialize, Serializer};
use serde::de::DeserializeOwned;
use thiserror::Error;
use crate::models::*;
use crate::models::{ServersServer, WhereisData};


/// A ServerSeeker client which stores the api key
pub struct ServerSeekerClient {
    pub client: reqwest::Client,
    pub api_key: String,
}

impl ServerSeekerClient {
    pub const API_URL: &'static str = "https://api.serverseeker.net";

    pub fn new<T: ToString>(api_key: T) -> Self {
        let client = reqwest::Client::new();
        ServerSeekerClient {
            client,
            api_key: api_key.to_string(),
        }
    }

    pub(crate) async fn request<T: DeserializeOwned, P: Serialize, E: ToString>(&self, endpoint: E, params: P) -> anyhow::Result<T> {
        let res: APIResponse<T> = self
            .client
            .post(format!("{}{}", Self::API_URL, endpoint.to_string()))
            .header("Content-Type", "application/json")
            .json(&params)
            .send()
            .await?
            .json()
            .await?;
        match res {
            APIResponse::Data(d) => Ok(d),
            APIResponse::Error(e) => anyhow::bail!(e),
            _ => bail!("An unknown error occured"),
        }
    }
}

/// A response from the ServerSeeker API
#[derive(Debug, Error, Deserialize)]
#[serde(untagged)]
enum APIResponse<T> {
    Error(APIError),
    Data(T),
}

/// An error
#[derive(Deserialize, Error, Debug)]
pub(crate) enum APIError {
    #[error("API returned error: {0}")]
    Error(String)
}
