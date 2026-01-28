"""Response parsing for vision analysis."""

from __future__ import annotations

import json
from typing import Any

from aura.core.models import InferenceSignal
from aura.core.types import AgeGroup


def strip_markdown(content: str) -> str:
    """Strip markdown code block markers from content."""
    return (
        content.strip()
        .removeprefix("```json")
        .removeprefix("```")
        .removesuffix("```")
        .strip()
    )


def parse_age_group(s: str) -> AgeGroup | None:
    """Parse age group string to enum."""
    mapping = {
        "under_18": AgeGroup.UNDER_18,
        "18-24": AgeGroup.AGE_18_24,
        "25-34": AgeGroup.AGE_25_34,
        "35-44": AgeGroup.AGE_35_44,
        "45-54": AgeGroup.AGE_45_54,
        "55-64": AgeGroup.AGE_55_64,
        "65+": AgeGroup.AGE_65_PLUS,
    }
    return mapping.get(s)


def parse_response(signal: InferenceSignal, content: str) -> None:
    """Parse the LLM response and update the signal.

    Args:
        signal: The signal to update.
        content: The raw content from the LLM response.
    """
    try:
        result: dict[str, Any] = json.loads(content)
    except json.JSONDecodeError as e:
        signal.reasoning.append(f"Parse error: {e} - Raw: {content}")
        return

    apply_result(signal, result)


def apply_result(signal: InferenceSignal, result: dict[str, Any]) -> None:
    """Apply parsed result to the signal.

    Args:
        signal: The signal to update.
        result: The parsed JSON result.
    """
    is_human = result.get("is_human", False)

    if not is_human:
        signal.reasoning.append("Profile picture is not human.")
        return

    gender_male = result.get("gender_male", 0.0)
    gender_female = result.get("gender_female", 0.0)

    if gender_male > 0.0 or gender_female > 0.0:
        signal.gender_male = gender_male
        signal.gender_female = gender_female
        signal.gender_other = max(0.0, 1.0 - gender_male - gender_female)

    age_group_str = result.get("age_group")
    if age_group_str:
        age_group = parse_age_group(age_group_str)
        if age_group:
            signal.set_age_probs(age_group.to_one_hot())

    reasoning = result.get("reasoning", "")
    if reasoning:
        signal.reasoning.append(reasoning)
