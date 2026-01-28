"""Metrics types for tracking inference performance and lineage."""

from __future__ import annotations

from pydantic import BaseModel

from .types import SignalSource


class SourceMetrics(BaseModel):
    """Metrics for a single signal source/agent."""

    source: SignalSource
    latency_ms: int
    tokens_used: int | None = None
    contributed: list[str]
    confidence: float


class InferenceMetrics(BaseModel):
    """Complete metrics for an inference request."""

    request_id: str
    timestamp: str
    inputs_provided: list[str]
    sources_used: list[SourceMetrics]
    sources_agreed: bool
    fusion_confidence: float
    edge_case: bool
    total_tokens: int
    estimated_cost_usd: float
    total_latency_ms: int
