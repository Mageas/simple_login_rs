pub mod account;
pub mod alias;

#[derive(Debug, serde::Deserialize)]
pub struct ErrorData {
    pub error: String,
}
