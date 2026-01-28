"""Signal fusion - combines multiple agent signals into a unified result."""

from .models import InferenceSignal
from .types import SignalSource, AgeGroup


def fuse(signals: list[InferenceSignal]) -> InferenceSignal:
    """Fuse multiple inference signals into a single unified signal.

    Args:
        signals: List of signals from different agents.

    Returns:
        A single fused InferenceSignal.
    """
    if not signals:
        return InferenceSignal(source=SignalSource.LOCAL)
    if len(signals) == 1:
        return signals[0]
    return _blend(signals)


def _blend(signals: list[InferenceSignal]) -> InferenceSignal:
    """Blend multiple signals using weighted averaging and selection rules.

    Fusion strategy:
    - Gender: Average probabilities from signals with gender data
    - Age: Blend distributions from signals with age data
    - Birth year: First non-None value (deterministic)
    - Organization: Prefer one with name field populated
    - Ethnicity: Highest confidence wins
    - Reasoning: Concatenate all
    - Latency: Max (parallel execution)
    - Tokens: Sum
    """
    result = InferenceSignal(source=SignalSource.LOCAL)

    # Gender: average only signals with data
    gender_signals = [s for s in signals if s.has_gender_signal()]
    if gender_signals:
        count = len(gender_signals)
        result.gender_male = sum(s.gender_male for s in gender_signals) / count
        result.gender_female = sum(s.gender_female for s in gender_signals) / count
        result.gender_other = sum(s.gender_other for s in gender_signals) / count

    # Age: blend only signals with data
    age_signals = [s for s in signals if s.has_age_signal()]
    if age_signals:
        age_dists = [s.get_age_probs() for s in age_signals]
        weights = [1.0] * len(age_signals)
        result.set_age_probs(AgeGroup.blend(age_dists, weights))

    # Birth year: first non-None (local is deterministic)
    result.birth_year = next((s.birth_year for s in signals if s.birth_year), None)

    # Organization: prefer one with name populated
    orgs = [s.organization for s in signals if s.organization]
    if orgs:
        # Sort by whether name is present, take the one with most data
        result.organization = max(orgs, key=lambda o: o.name is not None)

    # Ethnicity: highest confidence wins
    eth_signals = [s for s in signals if s.ethnicity]
    if eth_signals:
        best = max(eth_signals, key=lambda s: s.ethnicity_confidence)
        result.ethnicity = best.ethnicity
        result.ethnicity_confidence = best.ethnicity_confidence

    # Reasoning: combine all
    result.reasoning = [r for s in signals for r in s.reasoning]

    # Latency: max (parallel execution)
    result.latency_ms = max(s.latency_ms for s in signals) if signals else 0

    # Tokens: sum
    total_tokens = sum(s.tokens_used or 0 for s in signals)
    result.tokens_used = total_tokens if total_tokens > 0 else None

    return result
