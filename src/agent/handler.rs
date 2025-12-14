use chrono::Utc;
use ntex::web::{self, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::agent::{
    AbstentionMetrics, Agent, InferenceInput, InferenceMetrics, InferenceSignal, SignalSource,
    SourceMetrics, local::LocalAgent,
};

#[derive(Debug, Deserialize)]
pub struct AnalyzeRequest {
    pub email: String,
    pub name: Option<String>,
    pub profile_pic_url: Option<String>,
    pub browsing_history: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct AnalyzeQuery {
    pub minimal: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct AnalyzeResponse {
    pub gender_male: f64,
    pub gender_female: f64,
    pub gender_other: f64,

    pub ethnicity: Option<String>,
    pub ethnicity_confidence: f64,

    pub age_group_under_18: f64,
    pub age_group_18_24: f64,
    pub age_group_25_34: f64,
    pub age_group_35_44: f64,
    pub age_group_45_54: f64,
    pub age_group_55_64: f64,
    pub age_group_65_plus: f64,

    pub birth_year: Option<u16>,
    pub organization: Option<String>,

    pub reasoning: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<InferenceMetrics>,
}

impl AnalyzeResponse {
    pub fn with_metrics_if<F>(mut self, condition: bool, f: F) -> Self
    where
        F: FnOnce() -> InferenceMetrics,
    {
        if condition {
            self.metrics = Some(f())
        }

        self
    }
}

impl From<InferenceSignal> for AnalyzeResponse {
    fn from(value: InferenceSignal) -> Self {
        Self {
            gender_male: value.gender_male,
            gender_female: value.gender_female,
            gender_other: value.gender_other,

            ethnicity: value.ethnicity,
            ethnicity_confidence: value.ethnicity_confidence,

            age_group_under_18: value.age_group_under_18,
            age_group_18_24: value.age_group_18_24,
            age_group_25_34: value.age_group_25_34,
            age_group_35_44: value.age_group_35_44,
            age_group_45_54: value.age_group_45_54,
            age_group_55_64: value.age_group_55_64,
            age_group_65_plus: value.age_group_65_plus,

            birth_year: value.birth_year,
            organization: value.organization,

            reasoning: value.reasoning,
            metrics: None,
        }
    }
}

pub async fn analyze(
    body: web::types::Json<AnalyzeRequest>,
    query: web::types::Query<AnalyzeQuery>,
) -> HttpResponse {
    let input = InferenceInput {
        email: body.email.clone(),
        name: body.name.clone(),
        profile_pic_url: body.profile_pic_url.clone(),
        browsing_history: body.browsing_history.clone(),
    };

    let local = LocalAgent::new();
    let signal = local.analyze(&input).await;
    let include_metrics = !query.minimal.unwrap_or(false);

    let response = AnalyzeResponse::from(signal.clone())
        .with_metrics_if(include_metrics,|| build_metrics(&signal, &input));

    HttpResponse::Ok().json(&response)
}

fn build_metrics(signal: &InferenceSignal, input: &InferenceInput) -> InferenceMetrics {
    let mut inputs_provided = vec!["email"];
    if input.name.is_some() {
        inputs_provided.push("name")
    }
    if input.profile_pic_url.is_some() {
        inputs_provided.push("profile_pic_url")
    }
    if input.browsing_history.is_some() {
        inputs_provided.push("browsing_history")
    }

    let mut contributed = Vec::new();
    if signal.organization.is_some() {
        contributed.push("organization")
    }
    if signal.birth_year.is_some() {
        contributed.push("birth_year")
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
        total_latency_ms: signal.latency_ms,
    }
}
