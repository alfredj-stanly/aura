"""DomainAgent - LLM-based organization enrichment."""

from __future__ import annotations

import json
import time

from aura.agents.base import Agent
from aura.core.models import InferenceInput, InferenceSignal
from aura.core.types import SignalSource, OrganizationIntelligence
from aura.services.openai_client import openai_client

from .prompt import enrich_domain


class DomainAgent(Agent):
    """Agent that enriches email domains with organization intelligence.

    Uses GPT-4o-mini to look up:
    - Organization name (with parent company relationships)
    - Industry category
    - Employee count estimates
    """

    async def analyze(self, input: InferenceInput) -> InferenceSignal:
        """Analyze email domain for organization intelligence."""
        start = time.perf_counter()
        signal = InferenceSignal(source=SignalSource.DOMAIN)

        if not input.email:
            signal.latency_ms = int((time.perf_counter() - start) * 1000)
            return signal

        parts = input.email.split("@")
        if len(parts) != 2:
            signal.latency_ms = int((time.perf_counter() - start) * 1000)
            return signal

        domain = parts[1].lower()
        org = await self._enrich_domain(domain)

        if org:
            signal.organization = org
            signal.reasoning.append(
                f"Domain {domain} enriched: {org.name or 'unknown'} ({org.category or 'unknown'})"
            )

        signal.latency_ms = int((time.perf_counter() - start) * 1000)
        return signal

    async def _enrich_domain(self, domain: str) -> OrganizationIntelligence | None:
        """Enrich a domain with organization intelligence."""
        try:
            response = await openai_client.chat_completion(
                messages=[{"role": "user", "content": enrich_domain(domain)}],
                max_tokens=200,
            )

            if "error" in response:
                return OrganizationIntelligence(domain=domain)

            choices = response.get("choices", [])
            if not choices:
                return OrganizationIntelligence(domain=domain)

            content = choices[0].get("message", {}).get("content", "")

            # Strip markdown
            clean = (
                content.strip()
                .removeprefix("```json")
                .removeprefix("```")
                .removesuffix("```")
                .strip()
            )

            parsed = json.loads(clean)

            return OrganizationIntelligence(
                domain=domain,
                name=parsed.get("name"),
                category=parsed.get("category"),
                employee_count=parsed.get("employee_count"),
                employee_count_source=parsed.get("employee_count_source"),
            )

        except Exception:
            return OrganizationIntelligence(domain=domain)
