use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RedditAuth {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub scope: String,
}
