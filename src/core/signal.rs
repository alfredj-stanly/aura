use crate::core::OrganizationIntelligence;

use super::r#type::SignalSource;

#[derive(Debug, Clone)]
pub struct InferenceInput {
    pub email: Option<String>,
    pub name: Option<String>,
    pub profile_pic_url: Option<String>,
    pub browsing_history: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct InferenceSignal {
    pub source: SignalSource,

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

    pub latency_ms: u64,
    pub tokens_used: Option<u32>,
}

impl InferenceSignal {
    pub fn new(source: SignalSource) -> Self {
        Self {
            source,

            gender_male: 0.0,
            gender_female: 0.0,
            gender_other: 0.0,

            ethnicity: None,
            ethnicity_confidence: 0.0,

            age_group_under_18: 0.0,
            age_group_18_24: 0.0,
            age_group_25_34: 0.0,
            age_group_35_44: 0.0,
            age_group_45_54: 0.0,
            age_group_55_64: 0.0,
            age_group_65_plus: 0.0,

            birth_year: None,
            organization: None,

            reasoning: Vec::new(),

            latency_ms: 0,
            tokens_used: None,
        }
    }

    pub fn set_age_probs(&mut self, probs: [f64; 7]) {
        self.age_group_under_18 = probs[0];
        self.age_group_18_24 = probs[1];
        self.age_group_25_34 = probs[2];
        self.age_group_35_44 = probs[3];
        self.age_group_45_54 = probs[4];
        self.age_group_55_64 = probs[5];
        self.age_group_65_plus = probs[6];
    }

    pub fn has_gender_signal(&self) -> bool {
        self.gender_male > 0.0 || self.gender_female > 0.0 || self.gender_other > 0.0
    }

    pub fn has_age_signal(&self) -> bool {
        self.age_group_under_18 > 0.0
            || self.age_group_18_24 > 0.0
            || self.age_group_25_34 > 0.0
            || self.age_group_35_44 > 0.0
            || self.age_group_45_54 > 0.0
            || self.age_group_55_64 > 0.0
            || self.age_group_65_plus > 0.0
    }
}
