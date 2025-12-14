use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalSource {
    Local,
    OpenAI,
    Vision,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
    Undetermined,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub enum AgeGroup {
    #[serde(rename = "under_18")]
    Under18,
    #[serde(rename = "18-24")]
    Age18_24,
    #[serde(rename = "25-34")]
    Age25_34,
    #[serde(rename = "35-44")]
    Age35_44,
    #[serde(rename = "45-54")]
    Age45_54,
    #[serde(rename = "55-64")]
    Age55_64,
    #[serde(rename = "65+")]
    Age65Plus,
}

impl AgeGroup {
    pub fn from_age(age: i32) -> Self {
        match age {
            0..=17 => Self::Under18,
            18..=24 => Self::Age18_24,
            25..=34 => Self::Age25_34,
            35..=44 => Self::Age35_44,
            45..=54 => Self::Age45_54,
            55..=64 => Self::Age55_64,
            _ => Self::Age65Plus,
        }
    }

    pub fn to_one_hot(&self) -> [f64; 7] {
        let mut probs = [0.0; 7];
        probs[*self as usize] = 1.0;
        probs
    }

    pub fn blend(distributions: &[[f64; 7]], weights: &[f64]) -> [f64; 7] {
        let mut result = [0.0; 7];
        let total_weight: f64 = weights.iter().sum();

        for (dist, &w) in distributions.iter().zip(weights) {
            for i in 0..7 {
                result[i] += dist[i] * w / total_weight;
            }
        }
        result
    }
}

#[derive(Debug, Clone)]
pub struct InferenceInput {
    pub email: String,
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
    pub organization: Option<String>,

    pub reasoning: Vec<String>,

    pub latency_ms: u64,
    pub tokens_used: Option<u32>,
}

impl InferenceSignal {
    pub fn default(source: SignalSource) -> Self {
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
}
