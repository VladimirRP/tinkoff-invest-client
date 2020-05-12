#![allow(clippy::redundant_field_names, non_shorthand_field_patterns)]

pub mod domain;
mod errors;
mod market;
mod operations;
mod orders;
mod portfolio;
mod sandbox;
mod user;

use crate::domain::*;
pub use crate::errors::Error;
pub use crate::market::Market;
pub use crate::operations::Operations;
pub use crate::orders::Orders;
pub use crate::portfolio::Portfolio;
pub use crate::sandbox::Sandbox;
pub use crate::user::User;
use futures::future;
use futures::sink::Sink;
use futures::stream::Stream;
use futures_util::sink::SinkExt;
use futures_util::StreamExt;
use reqwest::{Client, StatusCode};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

pub struct TinkoffInvestClient {
    http_client: Client,
    endpoint: String,
    token: String,
}

impl<'a> TinkoffInvestClient {
    pub fn new(http_client: Client, endpoint: &str, token: &str) -> Self {
        Self {
            http_client: http_client,
            endpoint: endpoint.to_string(),
            token: token.to_string(),
        }
    }

    pub async fn get_stream(
        &self,
        ws_endpoint: &str,
    ) -> Result<
        (
            impl Sink<OutcomeEvent, Error = Error>,
            impl Stream<Item = Result<IncomeEvent, Error>>,
        ),
        Error,
    > {
        let request = http::Request::builder()
            .method("GET")
            .uri(ws_endpoint)
            .header("Authorization", "Bearer ".to_owned() + &self.token)
            .body(())?;
        let (ws_stream, _) = connect_async(request).await?;

        let (write, read) = ws_stream.split();

        let write = write.sink_err_into::<Error>().with(|e: OutcomeEvent| {
            let message = match e {
                OutcomeEvent::Ping(b) => Ok(Message::Ping(b)),
                OutcomeEvent::Pong(b) => Ok(Message::Pong(b)),
                _ => serde_json::to_string(&e)
                    .map_err(Error::from)
                    .map(Message::text),
            };
            future::ready(message)
        });

        let read = read.then(|e| {
            e.map_err(Error::from)
                .and_then(|e| match e {
                    Message::Text(text) => {
                        serde_json::from_str::<IncomeEvent>(&text).map_err(Error::from)
                    }
                    Message::Binary(b) => Ok(IncomeEvent::Binary(b)),
                    Message::Close(frame) => {
                        dbg!(frame);
                        Ok(IncomeEvent::Close)
                    }
                    Message::Ping(b) => Ok(IncomeEvent::Ping(b)),
                    Message::Pong(b) => Ok(IncomeEvent::Pong(b)),
                })
                .map(future::ok)
                .unwrap_or_else(future::err)
        });

        Ok((write, read))
    }

    async fn make_get_request<T: DeserializeOwned>(
        &self,
        uri: &str,
        query: &HashMap<&str, &str>,
    ) -> Result<Response<T>, Error> {
        let endpoint = self.endpoint.clone() + uri;

        let result = self
            .http_client
            .get(&endpoint)
            .header("Content-Type", "application/json")
            .header("Authorization", "Bearer ".to_owned() + &self.token)
            .query(&query)
            .send()
            .await?;

        let status = result.status();
        let text = result.text().await?;

        if status == StatusCode::OK {
            let resp: Response<T> = serde_json::from_str(&text)?;
            Ok(resp)
        } else {
            Err(Error::GeneralError {
                description: format!("Got unexpected response, status={} text={}", status, text),
            })
        }
    }

    async fn make_post_request<T: DeserializeOwned>(
        &self,
        uri: &str,
        query: &HashMap<&str, &str>,
        body: &str,
    ) -> Result<Response<T>, Error> {
        let endpoint = self.endpoint.clone() + uri;

        let result = self
            .http_client
            .post(&endpoint)
            .header("Content-Type", "application/json")
            .header("Authorization", "Bearer ".to_owned() + &self.token)
            .query(&query)
            .body(body.to_string())
            .send()
            .await?;

        let status = result.status();
        let text = result.text().await?;

        if status == StatusCode::OK {
            let resp: Response<T> = serde_json::from_str(&text)?;
            Ok(resp)
        } else {
            Err(Error::GeneralError {
                description: format!("Got unexpected response, status={} text={}", status, text),
            })
        }
    }
}
