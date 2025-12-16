use serde::Deserialize;
use crate::core::InferenceSignal;

#[derive(Deserialize)]
pub struct OnomasticResult {
    pub gender_male: f64,
    pub gender_female: f64,
    pub ethnicity: Option<String>,
    pub ethnicity_confidence: f64,
    pub reasoning: String,
}

pub fn strip_markdown(content: &str) -> &str {
    content
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim()
}

pub fn apply_result(signal: &mut InferenceSignal, result: OnomasticResult) {
    if result.gender_male > 0.0 || result.gender_female > 0.0 {
        signal.gender_male = result.gender_male;
        signal.gender_female = result.gender_female;
        signal.gender_other = (1.0 - result.gender_male - result.gender_female).max(0.0);
    }

    if result.ethnicity.is_some() && result.ethnicity_confidence > 0.0 {
        signal.ethnicity = result.ethnicity;
        signal.ethnicity_confidence = result.ethnicity_confidence;
    }

    signal.reasoning.push(result.reasoning);
}
