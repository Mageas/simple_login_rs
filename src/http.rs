use std::collections::HashMap;
use std::fmt;

use async_trait::async_trait;
use serde_json::Value;

use crate::SimpleLoginResult;

pub type Headers = HashMap<String, String>;
pub type Query<'a> = HashMap<&'a str, &'a str>;
pub type Payload<'a> = (Option<&'a Query<'a>>, Option<&'a Value>);

#[async_trait]
pub trait BaseHttpClient: Send + Default + Clone + fmt::Debug {
    async fn get(
        &self,
        token: Option<&str>,
        url: &str,
        payload: &Payload,
    ) -> SimpleLoginResult<String>;

    async fn post(
        &self,
        token: Option<&str>,
        url: &str,
        payload: &Payload,
    ) -> SimpleLoginResult<String>;

    async fn post_public(&self, url: &str, payload: &Payload) -> SimpleLoginResult<String>;

    async fn put(
        &self,
        token: Option<&str>,
        url: &str,
        payload: &Payload,
    ) -> SimpleLoginResult<String>;

    async fn patch(
        &self,
        token: Option<&str>,
        url: &str,
        payload: &Payload,
    ) -> SimpleLoginResult<String>;

    async fn delete(
        &self,
        token: Option<&str>,
        url: &str,
        payload: &Payload,
    ) -> SimpleLoginResult<String>;
}
