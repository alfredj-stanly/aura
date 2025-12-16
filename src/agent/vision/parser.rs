use crate::core::{AgeGroup, InferenceSignal};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct VisionResult {
    pub gender_male: f64,
    pub gender_female: f64,
    pub age_group: Option<String>,
    pub age_confidence: f64,
    pub is_human: bool,
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

pub fn parse_age_group(s: &str) -> Option<AgeGroup> {
    match s {
        "under_18" => Some(AgeGroup::Under18),
        "18-24" => Some(AgeGroup::Age18_24),
        "25-34" => Some(AgeGroup::Age25_34),
        "35-44" => Some(AgeGroup::Age35_44),
        "45-54" => Some(AgeGroup::Age45_54),
        "55-64" => Some(AgeGroup::Age55_64),
        "65+" => Some(AgeGroup::Age65Plus),
        _ => None,
    }
}

pub fn apply_result(signal: &mut InferenceSignal, result: VisionResult) {
    if result.is_human {
        signal.gender_male = result.gender_male;
        signal.gender_female = result.gender_female;
        signal.gender_other = 1.0 - result.gender_male - result.gender_female;

        if let Some(age_str) = &result.age_group {
            if let Some(age_group) = parse_age_group(age_str) {
                signal.set_age_probs(age_group.to_one_hot());
            }
        }

        return signal.reasoning.push(result.reasoning);
    }

    signal
        .reasoning
        .push("Profile picture is not human.".to_string())
}
