"""Request handlers for the inference API."""

from __future__ import annotations

import asyncio

from aura.agents import LocalAgent, OnomasticAgent, VisionAgent, DomainAgent
from aura.core.models import InferenceInput, InferenceSignal
from aura.core.fusion import fuse

from .dto import AnalyzeRequest, InferResponse, FuzzyResponse, Format
from .metrics_builder import build_metrics


async def infer(
    request: AnalyzeRequest,
    format: Format = Format.FUZZY,
    minimal: bool = False,
) -> InferResponse | FuzzyResponse:
    """Handle inference request.

    Args:
        request: The analysis request with user data.
        format: Response format (raw or fuzzy).
        minimal: If True, exclude metrics from response.

    Returns:
        InferResponse (raw) or FuzzyResponse (fuzzy) based on format.
    """
    input = InferenceInput(
        email=request.email,
        name=request.name,
        profile_pic_url=request.profile_pic_url,
        browsing_history=request.browsing_history,
    )

    # Create agents
    local = LocalAgent()
    onomastic = OnomasticAgent()
    domain = DomainAgent()

    # Build task list
    tasks: list[asyncio.Task[InferenceSignal]] = [
        asyncio.create_task(local.analyze(input)),
        asyncio.create_task(onomastic.analyze(input)),
        asyncio.create_task(domain.analyze(input)),
    ]

    # Conditionally add vision agent
    if input.profile_pic_url:
        vision = VisionAgent()
        tasks.append(asyncio.create_task(vision.analyze(input)))

    # Execute all agents in parallel
    signals = await asyncio.gather(*tasks)
    signals_list = list(signals)

    # Fuse signals
    fused = fuse(signals_list)

    # Build response based on format
    if format == Format.RAW:
        response = InferResponse.from_signal(fused)
    else:
        response = FuzzyResponse.from_signal(fused)

    # Add metrics if not minimal
    if not minimal:
        response.metrics = build_metrics(signals_list, input)

    return response
