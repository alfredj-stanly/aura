use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct ImageUrl {
    url: String,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
enum ContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrl },
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: Vec<ContentPart>,
}

#[derive(Debug, Serialize)]
struct VisionRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
    temperature: f64,
}

#[derive(Debug, Deserialize)]
struct VisionResponse {
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

/// What we get back from vision inference
#[derive(Debug, Deserialize)]
pub struct VisionResult {
    pub gender: GenderResult,
    pub age_bucket: AgeBucketResult,
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
pub struct VisionClient {
    client: Client,
    api_key: String,
}

impl VisionClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn infer(&self, image_url: &str) -> Result<VisionResult, String> {
        let prompt = r#"Analyze this profile picture to estimate demographic information.

Respond ONLY with valid JSON in this exact format, no other text:
{
  "gender": { "male": 0.0, "female": 0.0, "others": 0.0 },
  "age_bucket": { "18-24": 0.0, "25-34": 0.0, "35-44": 0.0, "45+": 0.0 },
  "confidence": 0.0
}

Rules:
- All probabilities must sum to 1.0 within their category
- Confidence is 0.0-1.0 based on image clarity and face visibility
- If no face is visible, set confidence to 0.0
- Base estimates on apparent age and presentation only"#;

        let request = VisionRequest {
            model: "gpt-4o-mini".to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: vec![
                    ContentPart::Text { text: prompt.to_string() },
                    ContentPart::ImageUrl {
                        image_url: ImageUrl { url: image_url.to_string() },
                    },
                ],
            }],
            max_tokens: 200,
            temperature: 0.1,
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Vision request failed: {}", e))?;

        let vision_response: VisionResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse vision response: {}", e))?;

        let content = &vision_response.choices[0].message.content;

        serde_json::from_str(content)
            .map_err(|e| format!("Failed to parse vision result: {} - Raw: {}", e, content))
    }
}
