use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct DeCustomDomainData {
    pub custom_domain: CustomDomainData,
}

#[derive(Debug, Deserialize)]
pub(crate) struct VecCustomDomainData {
    pub custom_domains: Vec<CustomDomainData>,
}

#[derive(Debug, Deserialize)]
pub struct CustomDomainData {
    pub catch_all: bool,
    pub creation_date: String,
    pub creation_timestamp: usize,
    pub domain_name: String,
    pub id: usize,
    pub is_verified: bool,
    pub mailboxes: Vec<CustomDomainMailboxData>,
    pub name: Option<String>,
    pub nb_alias: usize,
    pub random_prefix_generation: bool,
}

#[derive(Debug, Deserialize)]
pub struct CustomDomainMailboxData {
    pub email: String,
    pub id: usize,
}

#[derive(Debug, Deserialize)]
pub(crate) struct VecDeletedAliasData {
    pub aliases: Vec<DeletedAliasData>,
}

#[derive(Debug, Deserialize)]
pub struct DeletedAliasData {
    pub alias: String,
    pub deletion_timestamp: usize,
}
