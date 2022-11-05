pub mod account;

#[derive(Debug, serde::Deserialize)]
pub struct ErrorData {
    pub error: String,
}
