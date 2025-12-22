use chrono::Utc;
use uuid::Uuid;

use crate::core::{InferenceInput, InferenceMetrics, InferenceSignal, SourceMetrics};

pub fn build_metrics(signals: &[InferenceSignal], input: &InferenceInput) -> InferenceMetrics {
    let mut inputs_provided = vec!["email"];
    if input.name.is_some() {
        inputs_provided.push("name");
    }
    if input.profile_pic_url.is_some() {
        inputs_provided.push("profile_pic_url");
    }
    if input.browsing_history.is_some() {
        inputs_provided.push("browsing_history");
    }

    let sources_used: Vec<SourceMetrics> = signals
        .iter()
        .map(|s| {
            let mut contributed = Vec::new();
            if s.organization.is_some() {
                contributed.push("organization".to_string());
            }
            if s.birth_year.is_some() {
                contributed.push("birth_year".to_string());
            }
            if s.has_gender_signal() {
                contributed.push("gender".to_string());
            }
            if s.has_age_signal() {
                contributed.push("age".to_string());
            }
            if s.ethnicity.is_some() {
                contributed.push("ethnicity".to_string());
            }

            SourceMetrics {
                source: s.source.clone(),
                latency_ms: s.latency_ms,
                tokens_used: s.tokens_used,
                contributed,
                confidence: 1.0,
            }
        })
        .collect();

    InferenceMetrics {
        request_id: Uuid::new_v4().to_string(),
        timestamp: Utc::now().to_rfc3339(),
        inputs_provided: inputs_provided.into_iter().map(String::from).collect(),
        sources_used,
        sources_agreed: true,
        fusion_confidence: 1.0,
        edge_case: false,
        total_tokens: signals.iter().filter_map(|s| s.tokens_used).sum(),
        estimated_cost_usd: 0.0,
        total_latency_ms: signals.iter().map(|s| s.latency_ms).max().unwrap_or(0),
    }
}
