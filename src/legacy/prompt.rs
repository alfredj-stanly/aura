pub fn infer_p(name: &str, email: &str) -> String {
    let pmt = r#"You are a demographic infer system. Given a name email, estimate gender and age probabilities.
    
        Input: 
        - Name: {NAME}
        - Email: {EMAIL}

        Respond ONLY with valid JSON in this exact format, no other text: 
        {
          "gender": { "male": 0.0, "female": "0.0", "others": 0.0 },
          "age_bucket": { "18-24": 0.0, "25-34": 0.0, "35-44": 0.0, "45+": 0.0 },
          "region_hint": "string or null",
          "confidence": 0.0,
        }

        Rules:
        - All probabilities must sum to 1.0 within their category
        - Confidence is 0.0-1.0 based on how certain you are
        - Use cultural and linguistic patterns from the name 
        - Use email domain hints (TLD, organization type)
        - If uncertain, distribute probabilities more evenly"#;

    pmt.replace("{NAME}", name).replace("{EMAIL}", email)
}

pub fn gaze_p(inputs: &str, has_profile_pic: bool) -> String {
    let pmt = r#"You are demographic infer system. Analyze ONLY the available signals. DO NOT guess.

    Input:
    {INPUTS}

    Respond ONLY with valid JSON in the exact format, no other text:
    {
      "gender": "male" | "female" | "undetermined",
      "gender_confidence": "low" | "medium" | "strong",
      "ethnicity": "string describing likely ethnicity",
      "ethnicity_confidence": "low" | "medium" | "strong",
      "age_group": "under_18" | "18-24" | "25-34" | "35-44" | "45-54" | "55-64" | "65+" | null,
      "age_group_confidence": "low" | "medium" | "strong" | null,
      "birth_year": number | null,
      "birth_year_source": "email_pattern" | "profile_image" | "browsing_history" | null,
      "reasoning": ["reason1", "reason2", ...],
      "edge_case": true | false,
    }

    SRICT RULES:
    1. birth_year: ONLY exact if a 4-digit year (1940-2012) appears in email username. Otherwise MUST be null. 
    2. age_group:
       - MUST be one of EXACTLY these values: "under_18", "18-24", "25-34", "35-44", "45-54", "55-64", "65+", or null
       - If birth_year was extracted: MUST calculate age_group from birth_year w.r.t current year and pick the matching age_bucket
       - If NO birth_year and NO profile picture: MUST be null
       - If profile picture provided: may infer from visual appearence
    3. gender:
       - Extract name from email username (e.g. "priya.sharma@" -> "Priya.Sharma")
       - If name is clearly gendered (Priya, Robert, Sarah): infer gender
       - If name is ambiguous (Alex, Jordan, Kim) or just initials (j.smith): return "undetermined"
       - If profile picture provided: use visual appearence as strongest signal
    4. ethnicity: infer from name patterns (surename, cultural origin ). Be specific: "south_asian", "east_asian", "western_european", "african", "latin_american", "middle_eastern", "unknown"
    5. confidence: "strong" ONLY when signal is unambiguous
    6. reasoning: explain what signals you used and what you could NOT determine
    7. edge_case: true if signals are weak or conflicting

    Profile picture provided: {HAS_PIC}"#;

    pmt.replace("{INPUTS}", inputs).replace("{HAS_PIC}", &has_profile_pic.to_string())
}
