"""Core data models for inference input and signals."""

from __future__ import annotations

from pydantic import BaseModel, Field

from .types import SignalSource, OrganizationIntelligence


class InferenceInput(BaseModel):
    """Input data for inference."""

    email: str | None = None
    name: str | None = None
    profile_pic_url: str | None = None
    browsing_history: list[str] | None = None


class InferenceSignal(BaseModel):
    """Output signal from an inference agent."""

    source: SignalSource

    # Gender probabilities
    gender_male: float = 0.0
    gender_female: float = 0.0
    gender_other: float = 0.0

    # Ethnicity
    ethnicity: str | None = None
    ethnicity_confidence: float = 0.0

    # Age group probabilities (7 buckets)
    age_group_under_18: float = 0.0
    age_group_18_24: float = 0.0
    age_group_25_34: float = 0.0
    age_group_35_44: float = 0.0
    age_group_45_54: float = 0.0
    age_group_55_64: float = 0.0
    age_group_65_plus: float = 0.0

    # Additional fields
    birth_year: int | None = None
    organization: OrganizationIntelligence | None = None

    reasoning: list[str] = Field(default_factory=list)

    # Metrics
    latency_ms: int = 0
    tokens_used: int | None = None

    def set_age_probs(self, probs: list[float]) -> None:
        """Set age group probabilities from a list."""
        self.age_group_under_18 = probs[0]
        self.age_group_18_24 = probs[1]
        self.age_group_25_34 = probs[2]
        self.age_group_35_44 = probs[3]
        self.age_group_45_54 = probs[4]
        self.age_group_55_64 = probs[5]
        self.age_group_65_plus = probs[6]

    def get_age_probs(self) -> list[float]:
        """Get age group probabilities as a list."""
        return [
            self.age_group_under_18,
            self.age_group_18_24,
            self.age_group_25_34,
            self.age_group_35_44,
            self.age_group_45_54,
            self.age_group_55_64,
            self.age_group_65_plus,
        ]

    def has_gender_signal(self) -> bool:
        """Check if this signal has any gender probability data."""
        return self.gender_male > 0.0 or self.gender_female > 0.0 or self.gender_other > 0.0

    def has_age_signal(self) -> bool:
        """Check if this signal has any age probability data."""
        return any(self.get_age_probs())
