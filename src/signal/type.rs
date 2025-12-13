use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SignalSource {
    Local,
    OpenAI,
    Vision,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
    Undetermined,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
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
