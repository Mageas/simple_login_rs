use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ToggleContactData {
    pub block_forward: bool,
}
