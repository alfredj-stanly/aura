"""OnomasticAgent - LLM-based name analysis."""

from __future__ import annotations

import time

from aura.agents.base import Agent
from aura.core.models import InferenceInput, InferenceSignal
from aura.core.types import SignalSource
from aura.services.openai_client import openai_client

from .prompt import analyze as build_prompt
from .parser import strip_markdown, parse_response


class OnomasticAgent(Agent):
    """Agent that analyzes names for gender and ethnicity inference.

    Uses GPT-4o-mini to analyze name patterns and infer:
    - Gender probability distribution
    - Ethnicity/cultural background with confidence
    """

    async def analyze(self, input: InferenceInput) -> InferenceSignal:
        """Analyze name for demographic inference."""
        start = time.perf_counter()
        signal = InferenceSignal(source=SignalSource.ONOMASTIC)

        name = self._extract_name(input)
        if not name:
            signal.latency_ms = int((time.perf_counter() - start) * 1000)
            return signal

        try:
            response = await openai_client.chat_completion(
                messages=[
                    {"role": "user", "content": build_prompt(name, input.email or "")}
                ],
            )

            if "error" in response:
                error_msg = response.get("error", {}).get("message", "Unknown error")
                signal.reasoning.append(f"API error: {error_msg}")
            else:
                usage = response.get("usage", {})
                signal.tokens_used = usage.get("total_tokens")

                choices = response.get("choices", [])
                if choices:
                    content = choices[0].get("message", {}).get("content", "")
                    parse_response(signal, strip_markdown(content))

        except Exception as e:
            signal.reasoning.append(f"Onomastic request failed: {e}")

        signal.latency_ms = int((time.perf_counter() - start) * 1000)
        return signal

    def _extract_name(self, input: InferenceInput) -> str | None:
        """Extract name from input or derive from email."""
        if input.name:
            return input.name

        if not input.email:
            return None

        username = input.email.split("@")[0]
        cleaned = username.replace(".", " ").replace("_", " ")
        cleaned = "".join(c for c in cleaned if c.isalpha() or c.isspace())
        name = cleaned.strip()

        return name if len(name) >= 2 else None
