use super::{AgeGroup, InferenceSignal, SignalSource};

pub fn fuse(signals: Vec<InferenceSignal>) -> InferenceSignal {
    match signals.len() {
        0 => InferenceSignal::new(SignalSource::Local),
        1 => signals.into_iter().next().unwrap(),
        _ => blend(signals),
    }
}

fn blend(signals: Vec<InferenceSignal>) -> InferenceSignal {
    let mut result = InferenceSignal::new(SignalSource::Local);

    // Gender: average
    // result.gender_male = signals.iter().map(|s| s.gender_male).sum::<f64>() / count;
    // result.gender_female = signals.iter().map(|s| s.gender_female).sum::<f64>() / count;
    // result.gender_other = signals.iter().map(|s| s.gender_other).sum::<f64>() / count;

    // Gender: average only signals with data
    let gender_signals: Vec<_> = signals.iter().filter(|s| s.has_gender_signal()).collect();
    if !gender_signals.is_empty() {
        let count = gender_signals.len() as f64;
        result.gender_male = gender_signals.iter().map(|s| s.gender_male).sum::<f64>() / count;
        result.gender_female = gender_signals.iter().map(|s| s.gender_female).sum::<f64>() / count;
        result.gender_other = gender_signals.iter().map(|s| s.gender_other).sum::<f64>() / count;
    }

    // // Age: blend distributions
    // let age_dists: Vec<[f64; 7]> = signals
    //     .iter()
    //     .map(|s| {
    //         [
    //             s.age_group_under_18,
    //             s.age_group_18_24,
    //             s.age_group_25_34,
    //             s.age_group_35_44,
    //             s.age_group_45_54,
    //             s.age_group_55_64,
    //             s.age_group_65_plus,
    //         ]
    //     })
    //     .collect();
    // let weights = vec![1.0; signals.len()];
    // result.set_age_probs(AgeGroup::blend(&age_dists, &weights));

    // Age: blend only signals with data
    let age_signals: Vec<_> = signals.iter().filter(|s| s.has_age_signal()).collect();
    if !age_signals.is_empty() {
        let age_dists: Vec<[f64; 7]> = age_signals
            .iter()
            .map(|s| {
                [
                    s.age_group_under_18,
                    s.age_group_18_24,
                    s.age_group_25_34,
                    s.age_group_35_44,
                    s.age_group_45_54,
                    s.age_group_55_64,
                    s.age_group_65_plus,
                ]
            })
            .collect();
        let weights = vec![1.0; age_signals.len()];
        result.set_age_probs(AgeGroup::blend(&age_dists, &weights));
    }

    // Birth year: first non-None (local is deterministic)
    result.birth_year = signals.iter().find_map(|s| s.birth_year);

    // Organization: first non-None
    // result.organization = signals.iter().find_map(|s| s.organization.clone());

    result.organization = signals
        .iter()
        .filter_map(|s| s.organization.clone())
        .max_by_key(|org| org.name.is_some() as u8);

    // Ethnicity: highest confidence wins
    if let Some(best) = signals
        .iter()
        .filter(|s| s.ethnicity.is_some())
        .max_by(|a, b| {
            a.ethnicity_confidence
                .partial_cmp(&b.ethnicity_confidence)
                .unwrap()
        })
    {
        result.ethnicity = best.ethnicity.clone();
        result.ethnicity_confidence = best.ethnicity_confidence;
    }

    // Reasoning: combine all
    result.reasoning = signals.iter().flat_map(|s| s.reasoning.clone()).collect();

    // Latency: max (parallel execution)
    result.latency_ms = signals.iter().map(|s| s.latency_ms).max().unwrap_or(0);

    // Tokens: sum
    let total: u32 = signals.iter().filter_map(|s| s.tokens_used).sum();
    result.tokens_used = if total > 0 { Some(total) } else { None };

    result
}
