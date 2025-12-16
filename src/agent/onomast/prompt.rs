pub fn analyze(name: &str, email: &str) -> String {
    format!(
        r#"Analyze the name and email to infer gender and ethnicity/cultural background.

Name: {}
Email: {}

Return ONLY JSON, no markdown:
{{"gender_male": 0.0, "gender_female": 0.0, "ethnicity": "south_asian"|"east_asian"|"southeast_asian"|"european"|"african"|"latin_american"|"middle_eastern"|null, "ethnicity_confidence": 0.0, "reasoning": "..."}}"#,
        name, email
    )
}
