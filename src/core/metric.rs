use super::r#type::SignalSource;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SourceMetrics {
    pub source: SignalSource,
    pub latency_ms: u64,
    pub tokens_used: Option<u32>,
    pub contributed: Vec<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct AbstentionMetrics {
    pub gender: bool,
    pub ethnicity: bool,
    pub age: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct InferenceMetrics {
    pub request_id: String,
    pub timestamp: String,
    pub inputs_provided: Vec<String>,
    pub sources_used: Vec<SourceMetrics>,
    pub sources_agreed: bool,
    pub fusion_confidence: f64,
    pub abstentions: AbstentionMetrics,
    pub edge_case: bool,
    pub total_tokens: u32,
    pub estimated_cost_usd: f64,
    pub total_latency_ms: u64,
}
