//! HTTP client interacting with Medea via its Control API.

use derive_more::{Display, Error, From};
use medea_control_api_mock::{
    api::Response,
    callback::CallbackItem,
    proto::{CreateResponse, Element, SingleGetResponse},
};

/// All errors which can happen while working with a Control API.
#[derive(Debug, Display, Error, From)]
pub enum Error {
    Reqwest(reqwest::Error),
}

type Result<T> = std::result::Result<T, Error>;

/// Client of a Control API.
pub struct Client {
    inner: reqwest::Client,
    control_api_address: String,
}

impl Client {
    /// Returns a new Control API [`Client`].
    #[must_use]
    pub fn new(control_api_address: &str) -> Self {
        Self {
            inner: reqwest::Client::new(),
            control_api_address: control_api_address.to_owned(),
        }
    }

    /// Creates the provided media [`Element`] in the provided `path` on a Medea
    /// media server.
    pub async fn create(
        &self,
        path: &str,
        element: Element,
    ) -> Result<CreateResponse> {
        let gg : CreateResponse = self
        .inner
        .post(&get_url(&self.control_api_address, path))
        .json(&element)
        .send()
        .await?
        .json()
        .await?;
        println!("create {:?}", gg);
        if let Some(err) = gg.error {
            panic!();
        }
        Ok(gg)
    }

    /// Deletes a media [`Element`] identified by the provided `path`.
    pub async fn delete(&self, path: &str) -> Result<Response> {
        let gg: Response = self
        .inner
        .delete(&get_url(&self.control_api_address, path))
        .send()
        .await?
        .json()
        .await?;
        println!("delete {:?}", gg);
        if let Some(err) = gg.error {
            panic!();
        }
        Ok(gg)
    }

    /// Returns a media [`Element`] identified by the provided `path`.
    pub async fn get(&self, path: &str) -> Result<SingleGetResponse> {
        let gg: SingleGetResponse = self
        .inner
        .get(&get_url(&self.control_api_address, path))
        .send()
        .await?
        .json()
        .await?;
        println!("delete {:?}", gg);
        if let Some(err) = gg.error {
            panic!();
        }
        Ok(gg)
    }

    /// Applies on a media server the provided media [`Element`] identified by
    /// the provided `path`.
    pub async fn apply(
        &self,
        path: &str,
        element: Element,
    ) -> Result<CreateResponse> {
        let gg: CreateResponse = self
        .inner
        .put(&get_url(&self.control_api_address, path))
        .json(&element)
        .send()
        .await?
        .json()
        .await?;
        println!("apply {:?}", &gg);
        if let Some(err) = gg.error {
            panic!();
        }
        Ok(gg)
    }

    // TODO: Server side filtering on GET requests or SSE/WS subscription would
    //       speed up things. We a probably wasting a lot of time on ser/deser
    //       of huge JSON's.
    /// Fetches all callbacks received by Control API mock server.
    pub async fn callbacks(&self) -> Result<Vec<CallbackItem>> {

        let gg: Vec<CallbackItem> = self
        .inner
        .get(&format!("{}/callbacks", self.control_api_address))
        .send()
        .await?
        .json()
        .await?;
        
        Ok(gg)
    }
}

/// Returns URL of a media [`Element`] identified by the provided `path`.
fn get_url(control_api_address: &str, path: &str) -> String {
    format!("{control_api_address}/control-api/{path}")
}
