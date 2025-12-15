use chrono::Utc;
use uuid::Uuid;

use crate::core::{
    AbstentionMetrics, InferenceInput, InferenceMetrics, InferenceSignal, SignalSource,
    SourceMetrics,
};

pub fn build_metrics(signal: &InferenceSignal, input: &InferenceInput) -> InferenceMetrics {
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

    let mut contributed = Vec::new();
    if signal.organization.is_some() {
        contributed.push("organization");
    }
    if signal.birth_year.is_some() {
        contributed.push("birth_year");
    }

    InferenceMetrics {
        request_id: Uuid::new_v4().to_string(),
        timestamp: Utc::now().to_rfc3339(),
        inputs_provided: inputs_provided.into_iter().map(String::from).collect(),
        sources_used: vec![SourceMetrics {
            source: SignalSource::Local,
            latency_ms: signal.latency_ms,
            tokens_used: signal.tokens_used,
            contributed: contributed.into_iter().map(String::from).collect(),
            confidence: 1.0,
        }],
        sources_agreed: true,
        fusion_confidence: 1.0,
        abstentions: AbstentionMetrics {
            gender: !signal.has_gender_signal(),
            ethnicity: signal.ethnicity.is_none(),
            age: signal.birth_year.is_none(),
        },
        edge_case: false,
        total_tokens: signal.tokens_used.unwrap_or(0),
        estimated_cost_usd: 0.0,
        total_latency_ms: signal.latency_ms 
    }
}
