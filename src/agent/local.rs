use chrono::Datelike;
use std::time::Instant;

use super::Agent;

use crate::{
    core::{AgeGroup, InferenceInput, InferenceSignal, SignalSource},
    data::PERSONAL_EMAIL_DOMAINS,
};

pub struct LocalAgent;

impl LocalAgent {
    pub fn new() -> Self {
        Self
    }

    fn current_year() -> i32 {
        chrono::Utc::now().year()
    }

    fn extract_organization(&self, email: &str) -> Option<String> {
        let domain = email.split('@').nth(1)?;

        if PERSONAL_EMAIL_DOMAINS.contains(&domain.to_lowercase().as_str()) {
            return None;
        }

        Some(domain.to_string())
    }

    fn extract_birth_year(&self, email: &str) -> Option<u16> {
        let haystack = email.split('@').next()?;
        let current_year = Self::current_year();

        let min_year = (current_year - 80) as u16;
        let max_year = (current_year - 13) as u16;

        let re = regex::Regex::new(r"\d{4}").ok()?;

        for capture in re.find_iter(haystack) {
            if let Ok(year) = capture.as_str().parse::<u16>() {
                if year >= min_year && year <= max_year {
                    return Some(year);
                }
            }
        }
        None
    }

    fn birth_year_to_age_probs(&self, birth_year: u16) -> [f64; 7] {
        let age = Self::current_year() - birth_year as i32;
        AgeGroup::from_age(age).to_one_hot()
    }
}

impl Agent for LocalAgent {
    async fn analyze(&self, input: &InferenceInput) -> InferenceSignal {
        let start = Instant::now();
        let mut signal = InferenceSignal::new(SignalSource::Local);

        if let Some(email) = &input.email {
            signal.organization = self.extract_organization(email);
            if signal.organization.is_some() {
                signal.reasoning.push(format!(
                    "Organization {} extracted from email domain.",
                    signal.organization.as_ref().unwrap()
                ));
            }

            if let Some(birth_year) = self.extract_birth_year(email) {
                signal.birth_year = Some(birth_year);
                signal.set_age_probs(self.birth_year_to_age_probs(birth_year));

                signal.reasoning.push(format!(
                    "Birth year {birth_year} extracted from email pattern."
                ));
            }
        }

        signal.latency_ms = start.elapsed().as_millis() as u64;
        signal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ntex::test]
    async fn extracts_organization() {
        let agent = LocalAgent::new();
        let input = InferenceInput {
            email: Some("trinity@vogue.com".to_string()),
            name: None,
            profile_pic_url: None,
            browsing_history: None,
        };

        let signal = agent.analyze(&input).await;
        assert_eq!(signal.organization, Some("vogue.com".to_string()));
    }

    #[ntex::test]
    async fn extracts_birth_year() {
        let agent = LocalAgent::new();
        let input = InferenceInput {
            email: Some("laura1992@gmail.com".to_string()),
            name: None,
            profile_pic_url: None,
            browsing_history: None,
        };

        let signal = agent.analyze(&input).await;
        assert_eq!(signal.birth_year, Some(1992));
    }

    #[ntex::test]
    async fn ignores_invalid_year() {
        let agent = LocalAgent::new();
        let input = InferenceInput {
            email: Some("test9162@gmail.com".to_string()),
            name: None,
            profile_pic_url: None,
            browsing_history: None,
        };

        let signal = agent.analyze(&input).await;
        assert_eq!(signal.birth_year, None);
    }

    #[ntex::test]
    async fn no_birth_year_no_age_probs() {
        let agent = LocalAgent::new();
        let input = InferenceInput {
            email: Some("aparna@gmail.com ".to_string()),
            name: None,
            profile_pic_url: None,
            browsing_history: None,
        };

        let signal = agent.analyze(&input).await;
        assert_eq!(signal.birth_year, None);
    }
}
