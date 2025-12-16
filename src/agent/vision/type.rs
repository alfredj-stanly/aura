use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f64,
}

#[derive(Serialize)]
pub struct Message {
    pub role: String,
    pub content: Vec<Content>,
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Content {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrl },
}

#[derive(Serialize)]
pub struct ImageUrl {
    pub url: String,
    pub detail: String,
}

#[derive(Deserialize)]
pub struct ChatResponse {
    pub choices: Option<Vec<Choice>>,
    pub usage: Option<Usage>,
    pub error: Option<ApiError>,
}

#[derive(Deserialize)]
pub struct Choice {
    pub message: ResponseMessage,
}

#[derive(Deserialize)]
pub struct ResponseMessage {
    pub content: String,
}

#[derive(Deserialize)]
pub struct Usage {
    pub total_tokens: u32,
}

#[derive(Deserialize)]
pub struct ApiError {
    pub message: String,
}
