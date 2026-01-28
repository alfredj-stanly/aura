"""FastAPI application entry point."""

from __future__ import annotations

from contextlib import asynccontextmanager
from typing import AsyncGenerator

from fastapi import FastAPI

from aura.api.router import router
from aura.config import settings
from aura.services.openai_client import openai_client


@asynccontextmanager
async def lifespan(app: FastAPI) -> AsyncGenerator[None, None]:
    """Application lifespan handler for startup/shutdown."""
    # Startup: validate API key exists
    if not settings.openai_api_key:
        raise ValueError("OPENAI_API_KEY environment variable must be set")
    print(f"AURA running on http://{settings.host}:{settings.port}")
    yield
    # Shutdown: close HTTP client
    await openai_client.close()


app = FastAPI(
    title="AURA",
    description="Adaptive User Resonance Architecture - Probabilistic Identity Inference API",
    version="0.1.0",
    lifespan=lifespan,
)

app.include_router(router)


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(
        "aura.main:app",
        host=settings.host,
        port=settings.port,
        reload=settings.debug,
    )
