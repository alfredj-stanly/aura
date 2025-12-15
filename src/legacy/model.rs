use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct InferRequest {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct GenderDistribution {
    pub male: f64,
    pub female: f64,
    pub others: f64,
}

#[derive(Debug, Serialize)]
pub struct AgeBucketDistribution {
    #[serde(rename = "18-24")]
    pub age_18_24: f64,

    #[serde(rename = "25-34")]
    pub age_25_34: f64,

    #[serde(rename = "35-44")]
    pub age_35_34: f64,

    #[serde(rename = "45+")]
    pub age_45_plus: f64,
}

#[derive(Debug, Serialize)]
pub struct InferResponse {
    pub gender: GenderDistribution,
    pub age_bucket: AgeBucketDistribution,
    pub organization: Option<String>,
    pub region_hint: Option<String>,
    pub confidence: f64,
    pub edge_case: bool,
}

//===================
// GAZE models (v1)
// ==================

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Confidence {
    Low,
    Medium,
    Strong,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
    Undetermined,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BirthYearSource {
    #[serde(rename = "email_pattern")]
    EmailPattern,
    #[serde(rename = "profile_image")]
    ProfileImage,
    #[serde(rename = "browsing_history")]
    BrowsingHistory,
}

#[derive(Debug, Deserialize)]
pub struct GazeRequest {
    pub email: String,
    pub name: Option<String>,
    pub profile_pic_url: Option<String>,
    pub browsing_history: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GazeResponse {
    pub gender: Gender,
    pub gender_confidence: Confidence,

    pub ethnicity: String,
    pub ethnicity_confidence: Confidence,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_group: Option<AgeGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_group_confidence: Option<Confidence>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_year_source: Option<BirthYearSource>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,

    pub reasoning: Vec<String>,
    pub edge_case: bool,
}
