use chrono::Datelike;
use std::time::Instant;

use crate::agent::{AgeGroup, Agent, InferenceSignal};
pub struct LocalAgent;

impl LocalAgent {
    pub fn new() -> Self {
        Self
    }

    fn current_year() -> i32 {
        chrono::Utc::now().year()
    }

    fn extract_organization(&self, email: &str) -> Option<String> {
        email.split('@').nth(1).map(|s| s.to_string())
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

    fn calculate_age_group(&self, birth_year: u16) -> AgeGroup {
        let age = Self::current_year() - birth_year as i32;

        match age {
            0..=17 => AgeGroup::Under18,
            18..=24 => AgeGroup::Age18_24,
            25..=34 => AgeGroup::Age25_34,
            35..=44 => AgeGroup::Age35_44,
            45..=54 => AgeGroup::Age45_54,
            55..=64 => AgeGroup::Age55_64,
            _ => AgeGroup::Age65Plus,
        }
    }
}

impl Agent for LocalAgent {
    fn analyze(&self, input: &super::InferenceInput) -> InferenceSignal {
        let start = Instant::now();
        let mut signal = InferenceSignal::default(super::SignalSource::Local);

        signal.organization = self.extract_organization(&input.email);
        if signal.organization.is_some() {
            signal
                .reasoning
                .push("Organization extracted from email domain.".to_string());
        }

        if let Some(year) = self.extract_birth_year(&input.email) {
            signal.birth_year = Some(year);
            signal.age_group = Some(self.calculate_age_group(year));

            signal.age_group_confidence = 1.0;
            signal
                .reasoning
                .push(format!("Birth year {year} extracted from email pattern."));
        }

        signal.latency_ms = start.elapsed().as_millis() as u64;
        signal
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::InferenceInput;

    #[test]
    fn extracts_organization() {
        let agent = LocalAgent::new();
        let input = InferenceInput {
            email: "priya@vogue.com".to_string(),
            name: None,
            profile_pic_url: None,
            browsing_history: None,
        };

        let signal = agent.analyze(&input);
        assert_eq!(signal.organization, Some("vogue.com".to_string()));
    }

    #[test]
    fn extracts_birth_year() {
        let agent = LocalAgent::new();
        let input = InferenceInput {
            email: "priya1992@gmail.com".to_string(),
            name: None,
            profile_pic_url: None,
            browsing_history: None,
        };

        let signal = agent.analyze(&input);
        assert_eq!(signal.birth_year, Some(1992));
        assert!(signal.age_group.is_some());
    }

    #[test]
    fn ignores_invalid_year() {
        let agent = LocalAgent::new();
        let input = InferenceInput {
            email: "test2099@gmail.com".to_string(),
            name: None,
            profile_pic_url: None,
            browsing_history: None,
        };

        let signal = agent.analyze(&input);
        assert_eq!(signal.birth_year, None);
    }

    #[test]
    fn no_birth_year_no_age() {
        let agent = LocalAgent::new();
        let input = InferenceInput {
            email: "priya@gmail.com".to_string(),
            name: None,
            profile_pic_url: None,
            browsing_history: None,
        };

        let signal = agent.analyze(&input);
        assert_eq!(signal.birth_year, None);
        assert_eq!(signal.age_group, None);
    }
}
