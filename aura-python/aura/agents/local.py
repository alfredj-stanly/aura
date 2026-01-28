"""LocalAgent - deterministic extraction from email patterns."""

from __future__ import annotations

import re
import time
from datetime import datetime

from .base import Agent
from aura.core.models import InferenceInput, InferenceSignal
from aura.core.types import SignalSource, AgeGroup, OrganizationIntelligence
from aura.data.domains import PERSONAL_EMAIL_DOMAINS


class LocalAgent(Agent):
    """Deterministic agent that extracts information from email patterns.

    Extracts:
    - Organization domain from email (filtering personal domains)
    - Birth year from email username patterns (e.g., laura1992@gmail.com)
    """

    async def analyze(self, input: InferenceInput) -> InferenceSignal:
        """Analyze input and extract deterministic signals."""
        start = time.perf_counter()
        signal = InferenceSignal(source=SignalSource.LOCAL)

        if input.email:
            # Extract organization
            org = self._extract_organization(input.email)
            if org:
                signal.organization = org
                signal.reasoning.append(
                    f"Organization {org.domain} extracted from email domain."
                )

            # Extract birth year
            birth_year = self._extract_birth_year(input.email)
            if birth_year:
                signal.birth_year = birth_year
                signal.set_age_probs(self._birth_year_to_age_probs(birth_year))
                signal.reasoning.append(
                    f"Birth year {birth_year} extracted from email pattern."
                )

        signal.latency_ms = int((time.perf_counter() - start) * 1000)
        return signal

    def _extract_organization(self, email: str) -> OrganizationIntelligence | None:
        """Extract organization from email domain."""
        parts = email.split("@")
        if len(parts) != 2:
            return None

        domain = parts[1].lower()
        if domain in PERSONAL_EMAIL_DOMAINS:
            return None

        return OrganizationIntelligence(domain=domain)

    def _extract_birth_year(self, email: str) -> int | None:
        """Extract birth year from email username pattern."""
        username = email.split("@")[0]
        current_year = datetime.now().year

        min_year = current_year - 80
        max_year = current_year - 13

        for match in re.finditer(r"\d{4}", username):
            year = int(match.group())
            if min_year <= year <= max_year:
                return year

        return None

    def _birth_year_to_age_probs(self, birth_year: int) -> list[float]:
        """Convert birth year to age group probability distribution."""
        age = datetime.now().year - birth_year
        return AgeGroup.from_age(age).to_one_hot()
