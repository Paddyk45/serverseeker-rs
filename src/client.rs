use anyhow::{bail, Error};
use derive_builder::Builder;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use reqwest::{ClientBuilder, Method};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize, Serializer};
use std::any::{type_name, TypeId};
use std::fmt::format;
use thiserror::Error;
use crate::request::APIResponse;
use crate::ServerSeekerError;

/// A ServerSeeker client which stores the api key
pub struct ServerSeekerClient {
    pub client: reqwest::Client,
    pub api_key: String,
}

impl ServerSeekerClient {
    pub const API_URL: &'static str = "https://api.serverseeker.net";

    pub fn new<T: ToString>(api_key: T) -> Self {
        let mut client_builder = ClientBuilder::new();
        let mut hm = HeaderMap::new();
        hm.append(AUTHORIZATION, api_key.to_string().parse().unwrap());
        let client = client_builder.default_headers(hm).build().unwrap();

        ServerSeekerClient {
            client,
            api_key: api_key.to_string(),
        }
    }

    /// Create a new ServerSeekerClient and check if the api key is valid
    pub async fn new_checked<T: ToString>(
        api_key: T,
    ) -> Result<ServerSeekerClient, ServerSeekerError> {
        let res = reqwest::Client::new()
            .post(format!("{}{}", Self::API_URL, "/user_info"))
            .header("Authorization", api_key.to_string())
            .send()
            .await
            .expect("Failed to send request");
        match res.status().as_u16() {
            200 => {
                let mut client_builder = ClientBuilder::new();
                let mut hm = HeaderMap::new();
                hm.append(AUTHORIZATION, api_key.to_string().parse().unwrap());
                let client = client_builder.default_headers(hm).build().unwrap();

                Ok(ServerSeekerClient {
                    client,
                    api_key: api_key.to_string(),
                })
            }
            400 | 401 => Err(ServerSeekerError::InvalidApiKey),
            _ => unreachable!("That shouldn't have happened"),
        }
    }

    pub(crate) async fn request<T: DeserializeOwned, P: Serialize, E: ToString>(
        &self,
        endpoint: E,
        params: P,
        method: Method
    ) -> anyhow::Result<T> {
        let res: APIResponse<T> = self
            .client.request(method, format!("{}{}", Self::API_URL, endpoint.to_string()))
            .header("Content-Type", "application/json")
            .json(&params)
            .send()
            .await?
            .json()
            .await?;
        match res {
            APIResponse::Data(d) => Ok(d),
            APIResponse::Error(e) => bail!(e),
            _ => bail!("An unknown error occurred"),
        }
    }
}