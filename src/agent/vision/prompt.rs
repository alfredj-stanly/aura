pub fn analyze_image() -> String {
    r#"Analyze this profile picture. Return ONLY JSON, no markdown:
    {
        "gender_male": 0.0, 
        "gender_female": 0.0, 
        "age_group": "18-24"|"25-34"|"35-44"|"45-54"|"55-64"|"65+"|null, 
        "age_confidence": 0.0, 
        "is_human": true|false, 
        "reasoning": "..."
    }
    "#
    .to_string()
}
