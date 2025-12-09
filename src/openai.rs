use reqwest::Client;
use serde::{Deserialize, Serialize};

/// OpenAI chat message
#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

/// OpenAI request body
#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f64,
}

/// OpenAI response structures
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

/// What we ask OpenAI to return
#[derive(Debug, Deserialize)]
pub struct InferenceResult {
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

    pub async fn infer(&self, name: &str, email: &str) -> Result<InferenceResult, String> {
        let prompt = format!(
            r#"You are a demographic inference system. Given a name and email, estimate gender and age probabilities.

Input:
- Name: {}
- Email: {}

Respond ONLY with valid JSON in this exact format, no other text:
{{
  "gender": {{ "male": 0.0, "female": 0.0, "others": 0.0 }},
  "age_bucket": {{ "18-24": 0.0, "25-34": 0.0, "35-44": 0.0, "45+": 0.0 }},
  "region_hint": "string or null",
  "confidence": 0.0
}}

Rules:
- All probabilities must sum to 1.0 within their category
- Confidence is 0.0-1.0 based on how certain you are
- Use cultural and linguistic patterns from the name
- Use email domain hints (TLD, organization type)
- If uncertain, distribute probabilities more evenly"#,
            name, email
        );

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
            .map_err(|e| format!("Request failed: {}", e))?;

        let chat_response: ChatResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        let content = &chat_response.choices[0].message.content;
        
        serde_json::from_str(content)
            .map_err(|e| format!("Failed to parse inference result: {} - Raw: {}", e, content))
    }
}
