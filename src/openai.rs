use crate::{model::GazeResponse, prompt};

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f64,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: MessageResponse,
}

#[derive(Debug, Deserialize)]
struct MessageResponse {
    content: String,
}

#[derive(Debug, Deserialize)]
pub struct InferResult {
    pub gender: GenderResult,
    pub age_bucket: AgeBucketResult,
    pub region_hint: Option<String>,
    pub confidence: f64,
}

#[derive(Debug, Deserialize)]
pub struct GenderResult {
    pub male: f64,
    pub female: f64,
    pub others: f64,
}

#[derive(Debug, Deserialize)]
pub struct AgeBucketResult {
    #[serde(rename = "18-24")]
    pub age_18_24: f64,

    #[serde(rename = "25-34")]
    pub age_25_34: f64,

    #[serde(rename = "35-44")]
    pub age_35_44: f64,

    #[serde(rename = "45+")]
    pub age_45_plus: f64,
}

#[derive(Clone)]
pub struct OpenAIClient {
    client: Client,
    api_key: String,
}

impl OpenAIClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    /// Internal: make API call with given prompt
    async fn call(&self, prompt: String) -> Result<String, String> {
        let request = ChatRequest {
            model: "gpt-4o-mini".to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
            temperature: 0.1,
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Request failed: {e}"))?;

        // Debug: print raw response
        let raw_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {e}"))?;
        println!("Raw API response: {raw_text}");

        let chat_response: ChatResponse = serde_json::from_str(&raw_text)
            .map_err(|e| format!("Failed to parse response: {e}"))?;

        Ok(chat_response.choices[0].message.content.clone())

        // let chat_response: ChatResponse = response
        //     .json()
        //     .await
        //     .map_err(|e| format!("Failed to parse response: {e}"))?;

        // Ok(chat_response.choices[0].message.content.clone())
    }

    /// V0 inference
    pub async fn infer(&self, name: &str, email: &str) -> Result<InferResult, String> {
        let pmt = prompt::infer_p(name, email);
        let content = self.call(pmt).await?;

        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse inference result: {e} - Raw: {content}"))
    }

    pub async fn gaze(
        &self,
        email: &str,
        name: Option<&str>,
        profile_pic_url: Option<&str>,
        browsing_history: Option<&[String]>,
    ) -> Result<GazeResponse, String> {
        let mut inputs = vec![format!("- Email: {email}")];

        if let Some(n) = name {
            inputs.push(format!("- Name: {n}"));
        }

        if let Some(history) = browsing_history {
            inputs.push(format!("- Browsing History: {}", history.join(", ")));
        }

        let has_profile_pic = profile_pic_url.is_some();
        let pmt = prompt::gaze_p(&inputs.join("\n"), has_profile_pic);
        let content = self.call(pmt).await?;

        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to gaze result: {e} - Raw: {content}"))
    }
}
