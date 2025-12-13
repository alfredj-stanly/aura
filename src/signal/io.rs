use super::r#type::{SignalSource, Gender, AgeGroup};

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
    
    pub gender: Option<Gender>,
    pub gender_confidence: f64,
    
    pub ethnicity: Option<String>,
    pub ethnicity_confidence: f64,
    
    pub age_group: Option<AgeGroup>,
    pub age_group_confidence: f64,
    
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
            gender: None,
            gender_confidence: 0.0,
            ethnicity: None,
            ethnicity_confidence: 0.0,
            age_group: None,
            age_group_confidence: 0.0,
            birth_year: None,
            organization: None,
            reasoning: Vec::new(),
            latency_ms: 0,
            tokens_used: None,
        }
    }
}

