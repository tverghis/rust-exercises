use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct NewTicketRequest {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct PatchTicketRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}



