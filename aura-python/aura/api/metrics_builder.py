"""Build inference metrics from signals and input."""

import uuid
from datetime import datetime, timezone

from aura.core.models import InferenceInput, InferenceSignal
from aura.core.metrics import InferenceMetrics, SourceMetrics


def build_metrics(signals: list[InferenceSignal], input: InferenceInput) -> InferenceMetrics:
    """Build metrics from a list of signals and the original input.

    Args:
        signals: List of signals from all agents.
        input: The original inference input.

    Returns:
        InferenceMetrics with full lineage tracking.
    """
    # Track which inputs were provided
    inputs_provided = []
    if input.email:
        inputs_provided.append("email")
    if input.name:
        inputs_provided.append("name")
    if input.profile_pic_url:
        inputs_provided.append("profile_pic_url")
    if input.browsing_history:
        inputs_provided.append("browsing_history")

    # Build source metrics for each signal
    sources_used = []
    for s in signals:
        contributed = []
        if s.organization:
            contributed.append("organization")
        if s.birth_year:
            contributed.append("birth_year")
        if s.has_gender_signal():
            contributed.append("gender")
        if s.has_age_signal():
            contributed.append("age")
        if s.ethnicity:
            contributed.append("ethnicity")

        sources_used.append(
            SourceMetrics(
                source=s.source,
                latency_ms=s.latency_ms,
                tokens_used=s.tokens_used,
                contributed=contributed,
                confidence=1.0,
            )
        )

    # Calculate totals
    total_tokens = sum(s.tokens_used or 0 for s in signals)
    total_latency = max(s.latency_ms for s in signals) if signals else 0

    return InferenceMetrics(
        request_id=str(uuid.uuid4()),
        timestamp=datetime.now(timezone.utc).isoformat(),
        inputs_provided=inputs_provided,
        sources_used=sources_used,
        sources_agreed=True,
        fusion_confidence=1.0,
        edge_case=False,
        total_tokens=total_tokens,
        estimated_cost_usd=0.0,
        total_latency_ms=total_latency,
    )
