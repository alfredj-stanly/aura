"""VisionAgent - LLM-based image analysis."""

from __future__ import annotations

import time
from typing import Any

from aura.agents.base import Agent
from aura.core.models import InferenceInput, InferenceSignal
from aura.core.types import SignalSource
from aura.services.openai_client import openai_client

from .prompt import analyze_image
from .parser import strip_markdown, parse_response


class VisionAgent(Agent):
    """Agent that analyzes profile pictures for demographic inference.

    Uses GPT-4o-mini vision capabilities to analyze:
    - Gender probability distribution
    - Age group
    - Human vs non-human detection
    """

    async def analyze(self, input: InferenceInput) -> InferenceSignal:
        """Analyze profile picture for demographic inference."""
        start = time.perf_counter()
        signal = InferenceSignal(source=SignalSource.VISION)

        if not input.profile_pic_url:
            signal.latency_ms = int((time.perf_counter() - start) * 1000)
            return signal

        try:
            # Build vision request with image URL
            messages: list[dict[str, Any]] = [
                {
                    "role": "user",
                    "content": [
                        {"type": "text", "text": analyze_image()},
                        {
                            "type": "image_url",
                            "image_url": {
                                "url": input.profile_pic_url,
                                "detail": "low",
                            },
                        },
                    ],
                }
            ]

            response = await openai_client.chat_completion(messages=messages)

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
            signal.reasoning.append(f"Vision request failed: {e}")

        signal.latency_ms = int((time.perf_counter() - start) * 1000)
        return signal
