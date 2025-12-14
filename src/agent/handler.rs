use ntex::web::{self, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::agent::{Agent, InferenceInput, InferenceSignal, local::LocalAgent};

#[derive(Debug, Deserialize)]
pub struct AnalyzeRequest {
    pub email: String,
    pub name: Option<String>,
    pub profile_pic_url: Option<String>,
    pub browsing_history: Option<Vec<String>>,
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
        }
    }
}

pub async fn analyze(body: web::types::Json<AnalyzeRequest>) -> HttpResponse {
    let input = InferenceInput {
        email: body.email.clone(),
        name: body.name.clone(),
        profile_pic_url: body.profile_pic_url.clone(),
        browsing_history: body.browsing_history.clone(),
    };

    let local = LocalAgent::new();
    let signal = local.analyze(&input).await;

    let response: AnalyzeResponse  = signal.into();
    HttpResponse::Ok().json(&response)
}
