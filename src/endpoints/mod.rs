use async_trait::async_trait;

pub use account::*;
pub use alias::*;
pub use contact::*;
pub use custom_domain::*;
pub use mailbox::*;
pub use notification::*;
pub use setting::*;

use crate::{SimpleLoginError, SimpleLoginResult};

mod account;
mod alias;
mod contact;
mod custom_domain;
mod mailbox;
mod notification;
mod setting;
mod utils;

pub trait SimpleLogin {
    fn get_http(&self) -> &HttpClient;
    fn get_token(&self) -> Option<&str>;
    fn get_url<S: AsRef<str> + std::fmt::Display>(&self, endpoint: S) -> String;
    fn get_hostname(&self) -> &str;
}

pub struct SimpleLoginClient<'a> {
    pub http: HttpClient,
    pub hostname: &'a str,
    pub token: Option<&'a str>,
}

impl SimpleLogin for SimpleLoginClient<'_> {
    fn get_http(&self) -> &HttpClient {
        &self.http
    }

    fn get_token(&self) -> Option<&str> {
        self.token
    }

    fn get_url<S: AsRef<str> + std::fmt::Display>(&self, endpoint: S) -> String {
        format!("https://{}/{}", self.hostname, endpoint)
    }

    fn get_hostname(&self) -> &str {
        &self.hostname
    }
}

impl<'a> SimpleLoginClient<'a> {
    pub fn new(hostname: &'a str) -> Self {
        Self {
            http: HttpClient {
                client: reqwest::Client::new(),
            },
            hostname,
            token: None,
        }
    }

    pub fn account(&self) -> EndpointsAccount<'_, Self> {
        EndpointsAccount(self)
    }

    pub fn alias(&self) -> EndpointsAlias<'_, Self> {
        EndpointsAlias(self)
    }

    pub fn mailbox(&self) -> EndpointsMailbox<'_, Self> {
        EndpointsMailbox(self)
    }

    pub fn custom_domain(&self) -> EndpointsCustomDomain<'_, Self> {
        EndpointsCustomDomain(self)
    }

    pub fn contact(&self) -> EndpointsContact<'_, Self> {
        EndpointsContact(self)
    }

    pub fn notification(&self) -> EndpointsNotification<'_, Self> {
        EndpointsNotification(self)
    }

    pub fn setting(&self) -> EndpointsSetting<'_, Self> {
        EndpointsSetting(self)
    }
}

use crate::{BaseHttpClient, Payload};

use reqwest::{Method, RequestBuilder};

#[derive(Default, Debug, Clone)]
pub struct HttpClient {
    client: reqwest::Client,
}

impl HttpClient {
    async fn request<D>(
        &self,
        token: Option<&str>,
        method: Method,
        url: &str,
        add_data: D,
    ) -> SimpleLoginResult<String>
    where
        D: Fn(RequestBuilder) -> RequestBuilder,
    {
        let mut request = self.client.request(method.clone(), url);

        if let Some(token) = token {
            // let token = self.token.ok_or(SimpleLoginError::TokenNotSet)?;
            request = request.header("Authentication", token);
        }

        request = add_data(request);

        let response = request
            .send()
            .await
            .map_err(|e| SimpleLoginError::Request(e, url.into()))?;

        let status = response.status();
        let response = response
            .text()
            .await
            .map_err(|e| SimpleLoginError::Request(e, url.into()))?;

        dbg!(&response);

        utils::parse_error_from_response(&response, status, url).await?;

        Ok(response)
    }
}

#[async_trait]
impl BaseHttpClient for HttpClient {
    #[inline]
    async fn get(
        &self,
        token: Option<&str>,
        url: &str,
        payload: &Payload,
    ) -> SimpleLoginResult<String> {
        self.request(token, Method::GET, url, |req| set_payload(req, payload))
            .await
    }

    #[inline]
    async fn post(
        &self,
        token: Option<&str>,
        url: &str,
        payload: &Payload,
    ) -> SimpleLoginResult<String> {
        self.request(token, Method::POST, url, |req| set_payload(req, payload))
            .await
    }

    #[inline]
    async fn post_public(&self, url: &str, payload: &Payload) -> SimpleLoginResult<String> {
        self.request(None, Method::POST, url, |req| set_payload(req, payload))
            .await
    }

    #[inline]
    async fn put(
        &self,
        token: Option<&str>,
        url: &str,
        payload: &Payload,
    ) -> SimpleLoginResult<String> {
        self.request(token, Method::PUT, url, |req| set_payload(req, payload))
            .await
    }

    #[inline]
    async fn patch(
        &self,
        token: Option<&str>,
        url: &str,
        payload: &Payload,
    ) -> SimpleLoginResult<String> {
        self.request(token, Method::PATCH, url, |req| set_payload(req, payload))
            .await
    }

    #[inline]
    async fn delete(
        &self,
        token: Option<&str>,
        url: &str,
        payload: &Payload,
    ) -> SimpleLoginResult<String> {
        self.request(token, Method::DELETE, url, |req| set_payload(req, payload))
            .await
    }
}

#[inline]
fn set_payload(req: RequestBuilder, payload: &Payload) -> RequestBuilder {
    let req = match payload.0 {
        Some(query) => req.query(query),
        None => req,
    };
    let req = match payload.1 {
        Some(json) => req.json(json),
        None => req,
    };
    req
}
