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
