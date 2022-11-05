pub use account::*;

mod account;
mod utils;

pub trait SimpleLogin {
    fn get_http(&self) -> &reqwest::Client;
    fn get_token(&self) -> Option<&str>;
    fn get_url<S: AsRef<str> + std::fmt::Display>(&self, endpoint: S) -> String;
}

pub struct SimpleLoginClient<'a> {
    pub http: reqwest::Client,
    pub token: Option<&'a str>,
    pub hostname: &'a str,
}

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
}
