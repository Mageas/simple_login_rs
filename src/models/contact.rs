use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ToggleContactData {
    pub block_forward: bool,
}
