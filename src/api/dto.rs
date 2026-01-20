use serde::{Deserialize, Serialize};

use crate::core::{
    AgeGroup, Confidence, Gender, InferenceMetrics, InferenceSignal, OrganizationIntelligence,
};

#[derive(Debug, Deserialize)]
pub struct AnalyzeRequest {
    pub email: Option<String>,
    pub name: Option<String>,
    pub profile_pic_url: Option<String>,
    pub browsing_history: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    Raw,
    #[default]
    Fuzzy,
}

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub minimal: Option<bool>,
    #[serde(default)]
    pub format: Format,
}

#[derive(Debug, Serialize)]
pub struct InferResponse {
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
    pub organization: Option<OrganizationIntelligence>,

    pub reasoning: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<InferenceMetrics>,
}

impl InferResponse {
    pub fn with_metrics_if<F>(mut self, condition: bool, f: F) -> Self
    where
        F: FnOnce() -> InferenceMetrics,
    {
        if condition {
            self.metrics = Some(f());
        }
        self
    }
}

impl From<InferenceSignal> for InferResponse {
    fn from(s: InferenceSignal) -> Self {
        Self {
            gender_male: s.gender_male,
            gender_female: s.gender_female,
            gender_other: s.gender_other,

            ethnicity: s.ethnicity,
            ethnicity_confidence: s.ethnicity_confidence,

            age_group_under_18: s.age_group_under_18,
            age_group_18_24: s.age_group_18_24,
            age_group_25_34: s.age_group_25_34,
            age_group_35_44: s.age_group_35_44,
            age_group_45_54: s.age_group_45_54,
            age_group_55_64: s.age_group_55_64,
            age_group_65_plus: s.age_group_65_plus,

            birth_year: s.birth_year,
            organization: s.organization,

            reasoning: s.reasoning,
            metrics: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FuzzyResponse {
    pub gender: Gender,
    pub gender_confidence: Confidence,

    pub ethnicity: Option<String>,
    pub ethnicity_confidence: Confidence,

    pub age_group: Option<AgeGroup>,
    pub age_group_confidence: Confidence,

    pub organization: Option<OrganizationIntelligence>,

    pub reasoning: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<InferenceMetrics>,
}

impl From<InferenceSignal> for FuzzyResponse {
    fn from(s: InferenceSignal) -> Self {
        let (gender, gender_confidence) = resolve_gender(&s);
        let (age_group, age_group_confidence) = resolve_age_group(&s);

        Self {
            gender,
            gender_confidence,
            ethnicity: s.ethnicity,
            ethnicity_confidence: Confidence::from_probability(s.ethnicity_confidence),
            age_group,
            age_group_confidence,
            organization: s.organization,
            reasoning: s.reasoning,
            metrics: None,
        }
    }
}

impl FuzzyResponse {
    pub fn with_metrics_if<F>(mut self, condition: bool, f: F) -> Self
    where
        F: FnOnce() -> InferenceMetrics,
    {
        if condition {
            self.metrics = Some(f());
        }

        self
    }
}

fn resolve_gender(s: &InferenceSignal) -> (Gender, Confidence) {
    let (gender, prob) = [
        (Gender::Male, s.gender_male),
        (Gender::Female, s.gender_female),
        (Gender::Undetermined, s.gender_other),
    ]
    .into_iter()
    .max_by(|a, b| a.1.total_cmp(&b.1))
    .unwrap();

    if prob == 0.0 {
        return (Gender::Undetermined, Confidence::None);
    }

    (gender, Confidence::from_probability(prob))
}

fn resolve_age_group(s: &InferenceSignal) -> (Option<AgeGroup>, Confidence) {
    let probs = [
        (AgeGroup::Under18, s.age_group_under_18),
        (AgeGroup::Age18_24, s.age_group_18_24),
        (AgeGroup::Age25_34, s.age_group_25_34),
        (AgeGroup::Age35_44, s.age_group_35_44),
        (AgeGroup::Age45_54, s.age_group_45_54),
        (AgeGroup::Age55_64, s.age_group_55_64),
        (AgeGroup::Age65Plus, s.age_group_65_plus),
    ];

    let (group, prob) = probs
        .into_iter()
        .max_by(|a, b| a.1.total_cmp(&b.1))
        .unwrap();

    if prob == 0.0 {
        return (None, Confidence::None);
    }

    (Some(group), Confidence::from_probability(prob))
}
