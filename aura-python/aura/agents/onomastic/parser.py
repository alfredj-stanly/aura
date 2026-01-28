"""Response parsing for onomastic analysis."""

from __future__ import annotations

import json
from typing import Any

from aura.core.models import InferenceSignal


def strip_markdown(content: str) -> str:
    """Strip markdown code block markers from content."""
    return (
        content.strip()
        .removeprefix("```json")
        .removeprefix("```")
        .removesuffix("```")
        .strip()
    )


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
    gender_male = result.get("gender_male", 0.0)
    gender_female = result.get("gender_female", 0.0)

    if gender_male > 0.0 or gender_female > 0.0:
        signal.gender_male = gender_male
        signal.gender_female = gender_female
        signal.gender_other = max(0.0, 1.0 - gender_male - gender_female)

    ethnicity = result.get("ethnicity")
    ethnicity_confidence = result.get("ethnicity_confidence", 0.0)

    if ethnicity and ethnicity_confidence > 0.0:
        signal.ethnicity = ethnicity
        signal.ethnicity_confidence = ethnicity_confidence

    reasoning = result.get("reasoning", "")
    if reasoning:
        signal.reasoning.append(reasoning)
