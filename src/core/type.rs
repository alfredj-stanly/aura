use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SignalSource {
    Local,
    Vision,
    Onomastic,
    Domain,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
    Undetermined,
}

#[derive(Debug, Clone, Serialize, PartialEq, Copy)]
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

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Confidence {
    Strong,
    Medium,
    Low,
    None,
}

impl Confidence {
    pub fn from_probability(p: f64) -> Self {
        match p {
            p if p >= 0.8 => Self::Strong,
            p if p >= 0.5 => Self::Medium,
            p if p > 0.0 => Self::Low,
            _ => Self::None,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct OrganizationIntelligence {
    pub domain: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_count: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_count_source: Option<String>
}

