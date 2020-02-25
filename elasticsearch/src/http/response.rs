//! HTTP response components

use crate::error::Error;
use http::header::HeaderMap;
use http::StatusCode;
use serde::de::DeserializeOwned;

/// A response from Elasticsearch
pub struct Response(surf::Response);

impl Response {
    /// Creates a new instance of an Elasticsearch response
    pub fn new(response: surf::Response) -> Self {
        Self(response)
    }

    /// The HTTP status code of the response
    pub fn status_code(&self) -> StatusCode {
        self.0.status()
    }

    /// The response headers
    // pub fn headers(&self) -> &HeaderMap {
    //     self.0.headers()
    // } // TODO - cannot get HeaderMap

    /// Asynchronously read the response body
    pub async fn read_body<B>(mut self) -> Result<B, Error>
    where
        B: DeserializeOwned,
    {
        let body = self.0.body_json::<B>().await?;
        Ok(body)
    }
}
