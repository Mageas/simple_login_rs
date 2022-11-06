use std::collections::HashMap;

use async_trait::async_trait;

pub use account::*;
pub use alias::*;

use crate::{SimpleLoginError, SimpleLoginResult};

mod account;
mod alias;
mod utils;

#[async_trait]
pub trait SimpleLogin {
    fn get_http(&self) -> &reqwest::Client;
    fn get_token(&self) -> Option<&str>;
    fn get_url<S: AsRef<str> + std::fmt::Display>(&self, endpoint: S) -> String;
    fn get_hostname(&self) -> &str;

    async fn request<B, Q>(
        &self,
        authentication: bool,
        method: reqwest::Method,
        endpoint: &str,
        body: Option<B>,
        query: Option<Q>,
    ) -> SimpleLoginResult<String>
    where
        B: serde::Serialize + Send,
        Q: serde::Serialize + Send;

    async fn get<B, Q>(
        &self,
        endpoint: &str,
        body: Option<B>,
        query: Option<Q>,
    ) -> SimpleLoginResult<String>
    where
        B: serde::Serialize + Send,
        Q: serde::Serialize + Send;

    async fn post<B, Q>(
        &self,
        endpoint: &str,
        body: Option<B>,
        query: Option<Q>,
    ) -> SimpleLoginResult<String>
    where
        B: serde::Serialize + Send,
        Q: serde::Serialize + Send;

    async fn put<B, Q>(
        &self,
        endpoint: &str,
        body: Option<B>,
        query: Option<Q>,
    ) -> SimpleLoginResult<String>
    where
        B: serde::Serialize + Send,
        Q: serde::Serialize + Send;

    async fn patch<B, Q>(
        &self,
        endpoint: &str,
        body: Option<B>,
        query: Option<Q>,
    ) -> SimpleLoginResult<String>
    where
        B: serde::Serialize + Send,
        Q: serde::Serialize + Send;

    async fn delete<B, Q>(
        &self,
        endpoint: &str,
        body: Option<B>,
        query: Option<Q>,
    ) -> SimpleLoginResult<String>
    where
        B: serde::Serialize + Send,
        Q: serde::Serialize + Send;
}

pub struct SimpleLoginClient<'a> {
    pub http: reqwest::Client,
    pub token: Option<&'a str>,
    pub hostname: &'a str,
}

#[async_trait]
impl SimpleLogin for SimpleLoginClient<'_> {
    fn get_http(&self) -> &reqwest::Client {
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

    /// Get request api
    async fn get<B, Q>(
        &self,
        endpoint: &str,
        body: Option<B>,
        query: Option<Q>,
    ) -> SimpleLoginResult<String>
    where
        B: serde::Serialize + Send,
        Q: serde::Serialize + Send,
    {
        self.request(true, reqwest::Method::GET, endpoint, body, query)
            .await
    }

    /// Post request api
    async fn post<B, Q>(
        &self,
        endpoint: &str,
        body: Option<B>,
        query: Option<Q>,
    ) -> SimpleLoginResult<String>
    where
        B: serde::Serialize + Send,
        Q: serde::Serialize + Send,
    {
        self.request(true, reqwest::Method::POST, endpoint, body, query)
            .await
    }

    /// Put request api
    async fn put<B, Q>(
        &self,
        endpoint: &str,
        body: Option<B>,
        query: Option<Q>,
    ) -> SimpleLoginResult<String>
    where
        B: serde::Serialize + Send,
        Q: serde::Serialize + Send,
    {
        self.request(true, reqwest::Method::PUT, endpoint, body, query)
            .await
    }

    /// Patch request api
    async fn patch<B, Q>(
        &self,
        endpoint: &str,
        body: Option<B>,
        query: Option<Q>,
    ) -> SimpleLoginResult<String>
    where
        B: serde::Serialize + Send,
        Q: serde::Serialize + Send,
    {
        self.request(true, reqwest::Method::PATCH, endpoint, body, query)
            .await
    }

    /// Delete request api
    async fn delete<B, Q>(
        &self,
        endpoint: &str,
        body: Option<B>,
        query: Option<Q>,
    ) -> SimpleLoginResult<String>
    where
        B: serde::Serialize + Send,
        Q: serde::Serialize + Send,
    {
        self.request(true, reqwest::Method::DELETE, endpoint, body, query)
            .await
    }

    /// Make the request
    async fn request<B, Q>(
        &self,
        authentication: bool,
        method: reqwest::Method,
        endpoint: &str,
        body: Option<B>,
        query: Option<Q>,
    ) -> SimpleLoginResult<String>
    where
        B: serde::Serialize + Send,
        Q: serde::Serialize + Send,
    {
        let mut request = self
            .get_http()
            .request(method.clone(), self.get_url(&endpoint));

        if let true = authentication {
            let token = self.get_token().ok_or(SimpleLoginError::TokenNotSet)?;
            request = request.header("Authentication", token);
        }

        if let Some(body) = body {
            request = request.json(&body);
        }

        if let Some(query) = query {
            request = request.query(&query);
        }

        let response = request
            .send()
            .await
            .map_err(|e| SimpleLoginError::GenericRequest(e, endpoint.into()))?;

        let status = response.status();
        let response = response
            .text()
            .await
            .map_err(|e| SimpleLoginError::GenericRequest(e, endpoint.into()))?;

        dbg!(&response);

        utils::parse_error_from_response(&response, status, endpoint).await?;

        Ok(response)
    }
}

impl<'a> SimpleLoginClient<'a> {
    pub fn new(hostname: &'a str) -> Self {
        Self {
            http: reqwest::Client::new(),
            token: None,
            hostname,
        }
    }

    pub fn account(&self) -> EndpointsAccount<'_, Self> {
        EndpointsAccount(self)
    }

    pub fn alias(&self) -> EndpointsAlias<'_, Self> {
        EndpointsAlias(self)
    }
}
