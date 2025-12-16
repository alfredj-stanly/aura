mod parser;
mod prompt;
mod r#type;

use reqwest::Client;
use std::time::Instant;

use super::Agent;
use crate::core::{InferenceInput, InferenceSignal, SignalSource};
use r#type::*;

pub struct VisionAgent {
    client: Client,
    api_key: String,
}

impl VisionAgent {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }
}

impl Agent for VisionAgent {
    async fn analyze(&self, input: &InferenceInput) -> InferenceSignal {
        let start = Instant::now();
        let mut signal = InferenceSignal::new(SignalSource::Vision);

        let image_url = match &input.profile_pic_url {
            Some(url) => url,
            None => {
                signal.latency_ms = start.elapsed().as_millis() as u64;
                return signal;
            }
        };

        let request = ChatRequest {
            model: "gpt-4o-mini".to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: vec![
                    Content::Text {
                        text: prompt::analyze_image(),
                    },
                    Content::ImageUrl {
                        image_url: ImageUrl {
                            url: image_url.clone(),
                            detail: "low".to_string(),
                        },
                    },
                ],
            }],
            temperature: 0.1,
        };

        let response = match self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => {
                signal
                    .reasoning
                    .push(format!("Vision request failed: {}", e));
                signal.latency_ms = start.elapsed().as_millis() as u64;
                return signal;
            }
        };

        let chat_response: ChatResponse = match response.json().await {
            Ok(r) => r,
            Err(e) => {
                signal
                    .reasoning
                    .push(format!("Failed to parse response: {}", e));
                signal.latency_ms = start.elapsed().as_millis() as u64;
                return signal;
            }
        };

        if let Some(error) = chat_response.error {
            signal
                .reasoning
                .push(format!("API error: {}", error.message));
            signal.latency_ms = start.elapsed().as_millis() as u64;
            return signal;
        }

        signal.tokens_used = chat_response.usage.map(|u| u.total_tokens);

        if let Some(choices) = chat_response.choices {
            if let Some(choice) = choices.first() {
                let content = parser::strip_markdown(&choice.message.content);

                match serde_json::from_str::<parser::VisionResult>(content) {
                    Ok(result) => parser::apply_result(&mut signal, result),
                    Err(e) => {
                        signal
                            .reasoning
                            .push(format!("Parse error: {} - Raw: {}", e, content));
                    }
                }
            }
        }

        signal.latency_ms = start.elapsed().as_millis() as u64;
        signal
    }
}
