use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct CompletionContent {
    pub r#type: String,
    pub text: String,
}

#[derive(Serialize)]
pub struct CompletionRequestMessage {
    pub role: String,
    pub content: Vec<CompletionContent>,
}

#[derive(Serialize)]
pub struct CompletionBody {
    pub model: String,
    pub messages: Vec<CompletionRequestMessage>,
    pub temperature: i32,
    pub max_tokens: i32,
    pub top_p: i32,
    pub frequency_penalty: i32,
    pub presence_penalty: i32,
}

#[derive(Deserialize)]
pub struct CompletionResponseMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct CompletionChoices {
    pub index: i32,
    pub message: CompletionResponseMessage,
}

#[derive(Deserialize)]
pub struct CompletionResponse {
    pub model: String,
    pub choices: Vec<CompletionChoices>,
}
