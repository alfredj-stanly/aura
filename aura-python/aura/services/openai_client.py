"""Shared async OpenAI HTTP client."""

from __future__ import annotations

from typing import Any

import httpx

from aura.config import settings


class OpenAIClient:
    """Async HTTP client for OpenAI API calls."""

    BASE_URL = "https://api.openai.com/v1"

    def __init__(self) -> None:
        self._client: httpx.AsyncClient | None = None

    async def get_client(self) -> httpx.AsyncClient:
        """Get or create the async HTTP client."""
        if self._client is None or self._client.is_closed:
            self._client = httpx.AsyncClient(
                base_url=self.BASE_URL,
                headers={"Authorization": f"Bearer {settings.openai_api_key}"},
                timeout=settings.llm_timeout,
            )
        return self._client

    async def chat_completion(
        self,
        messages: list[dict[str, Any]],
        model: str | None = None,
        temperature: float | None = None,
        max_tokens: int = 500,
    ) -> dict[str, Any]:
        """Make a chat completion request to OpenAI.

        Args:
            messages: List of message dictionaries.
            model: Model to use (defaults to settings.llm_model).
            temperature: Temperature for sampling (defaults to settings.llm_temperature).
            max_tokens: Maximum tokens in response.

        Returns:
            The JSON response from OpenAI.
        """
        client = await self.get_client()
        response = await client.post(
            "/chat/completions",
            json={
                "model": model or settings.llm_model,
                "messages": messages,
                "temperature": temperature if temperature is not None else settings.llm_temperature,
                "max_tokens": max_tokens,
            },
        )
        return response.json()

    async def close(self) -> None:
        """Close the HTTP client."""
        if self._client:
            await self._client.aclose()
            self._client = None


# Singleton instance
openai_client = OpenAIClient()
