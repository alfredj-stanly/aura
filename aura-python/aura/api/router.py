"""API route definitions."""

from __future__ import annotations

from fastapi import APIRouter, Query

from .dto import AnalyzeRequest, Format
from .handlers import infer

router = APIRouter()


@router.get("/health")
async def health() -> dict[str, str]:
    """Health check endpoint."""
    return {"status": "As strong as an Ox!"}


@router.post("/v1/infer")
async def inference_endpoint(
    request: AnalyzeRequest,
    format: Format = Query(default=Format.FUZZY),
    minimal: bool = Query(default=False),
):
    """Main inference endpoint.

    Analyzes user signals (name, email, profile picture) and returns
    probabilistic demographic inferences.

    Args:
        request: User data to analyze.
        format: Response format - 'fuzzy' for labels + confidence, 'raw' for probabilities.
        minimal: If true, exclude metrics from response.

    Returns:
        FuzzyResponse or InferResponse depending on format parameter.
    """
    return await infer(request, format=format, minimal=minimal)
