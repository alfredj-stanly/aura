use std::time::Instant;

use super::Agent;
use crate::core::{InferenceInput, InferenceSignal, OrganizationIntelligence, SignalSource};

pub struct DomainAgent {
    client: reqwest::Client,
    api_key: String,
}

impl DomainAgent {
    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
        }
    }

    async fn enrich_domain(&self, domain: &str) -> Option<OrganizationIntelligence> {
        let prompt = format!(
            r#"Given the email domain "{}", provide organization intelligence.

If this is a subsidiary or owned by a parent company, include that relationship in the name field like "Company Name (subsidiary of Parent)" or "Company Name (part of Parent)".

Return JSON only, no markdown:
{{
  "name": "Full org name with parent relationship if applicable" or null,
  "category": "Industry / Sub-category" or null,
  "employee_count": "~X employees" or null,
  "employee_count_source": "Source name" or null
}}
            
If you can't confidently identify the organization, return all nulls."#,
            domain
        );

        let body = serde_json::json!({
            "model": "gpt-4o-mini",
            "messages": [
                {"role": "user", "content": prompt}
            ],
            "temperature": 0.1,
            "max_tokens": 200
        });

        let resp = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await
            .ok()?;

        let json: serde_json::Value = resp.json().await.ok()?;
        let content = json["choices"][0]["message"]["content"].as_str()?;

        let clean = content
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        let parsed: serde_json::Value = serde_json::from_str(clean).ok()?;

        Some(OrganizationIntelligence {
            domain: domain.to_string(),
            name: parsed["name"].as_str().map(String::from),
            category: parsed["category"].as_str().map(String::from),
            employee_count: parsed["employee_count"].as_str().map(String::from),
            employee_count_source: parsed["employee_count_source"].as_str().map(String::from),
        })
    }
}

impl Agent for DomainAgent {
    async fn analyze(&self, input: &InferenceInput) -> InferenceSignal {
        let start = Instant::now();
        let mut signal = InferenceSignal::new(SignalSource::Domain);

        if let Some(email) = &input.email {
            if let Some(domain) = email.split('@').nth(1) {
                if let Some(org) = self.enrich_domain(domain).await {
                    signal.reasoning.push(format!(
                        "Domain {} enriched: {} ({})",
                        domain,
                        org.name.as_deref().unwrap_or("unknown"),
                        org.category.as_deref().unwrap_or("unknown")
                    ));
                    signal.organization = Some(org);
                }
            }
        }

        signal.latency_ms = start.elapsed().as_millis() as u64;
        signal
    }
}
