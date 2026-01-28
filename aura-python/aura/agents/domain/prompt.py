"""Prompt templates for domain (organization) enrichment."""


def enrich_domain(domain: str) -> str:
    """Build the prompt for domain enrichment."""
    return f"""Given the email domain "{domain}", provide organization intelligence.

If this is a subsidiary or owned by a parent company, include that relationship in the name field like "Company Name (subsidiary of Parent)" or "Company Name (part of Parent)".

Return JSON only, no markdown:
{{
  "name": "Full org name with parent relationship if applicable" or null,
  "category": "Industry / Sub-category" or null,
  "employee_count": "~X employees" or null,
  "employee_count_source": "Source name" or null
}}

If you can't confidently identify the organization, return all nulls."""
