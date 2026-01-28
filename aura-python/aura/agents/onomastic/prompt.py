"""Prompt templates for onomastic (name) analysis."""


def analyze(name: str, email: str) -> str:
    """Build the prompt for name/email analysis."""
    return f"""Analyze the name and email to infer gender and ethnicity/cultural background.

Name: {name}
Email: {email}

Return ONLY JSON, no markdown:
{{"gender_male": 0.0, "gender_female": 0.0, "ethnicity": "south_asian"|"east_asian"|"southeast_asian"|"european"|"african"|"latin_american"|"middle_eastern"|null, "ethnicity_confidence": 0.0, "reasoning": "..."}}"""
