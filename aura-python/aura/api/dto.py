"""Request and response DTOs (Data Transfer Objects)."""

from __future__ import annotations

from enum import Enum

from pydantic import BaseModel

from aura.core.models import InferenceSignal
from aura.core.metrics import InferenceMetrics
from aura.core.types import (
    Gender,
    AgeGroup,
    Confidence,
    OrganizationIntelligence,
)


class Format(str, Enum):
    """Response format options."""

    RAW = "raw"
    FUZZY = "fuzzy"


class AnalyzeRequest(BaseModel):
    """Request body for the inference endpoint."""

    email: str | None = None
    name: str | None = None
    profile_pic_url: str | None = None
    browsing_history: list[str] | None = None


class InferResponse(BaseModel):
    """Raw probability response format."""

    gender_male: float
    gender_female: float
    gender_other: float

    ethnicity: str | None
    ethnicity_confidence: float

    age_group_under_18: float
    age_group_18_24: float
    age_group_25_34: float
    age_group_35_44: float
    age_group_45_54: float
    age_group_55_64: float
    age_group_65_plus: float

    birth_year: int | None
    organization: OrganizationIntelligence | None

    reasoning: list[str]

    metrics: InferenceMetrics | None = None

    @classmethod
    def from_signal(cls, s: InferenceSignal) -> "InferResponse":
        """Create response from inference signal."""
        return cls(
            gender_male=s.gender_male,
            gender_female=s.gender_female,
            gender_other=s.gender_other,
            ethnicity=s.ethnicity,
            ethnicity_confidence=s.ethnicity_confidence,
            age_group_under_18=s.age_group_under_18,
            age_group_18_24=s.age_group_18_24,
            age_group_25_34=s.age_group_25_34,
            age_group_35_44=s.age_group_35_44,
            age_group_45_54=s.age_group_45_54,
            age_group_55_64=s.age_group_55_64,
            age_group_65_plus=s.age_group_65_plus,
            birth_year=s.birth_year,
            organization=s.organization,
            reasoning=s.reasoning,
        )


class FuzzyResponse(BaseModel):
    """Human-readable response with labels and confidence levels."""

    gender: Gender
    gender_confidence: Confidence

    ethnicity: str | None
    ethnicity_confidence: Confidence

    age_group: AgeGroup | None
    age_group_confidence: Confidence

    organization: OrganizationIntelligence | None

    reasoning: list[str]

    metrics: InferenceMetrics | None = None

    @classmethod
    def from_signal(cls, s: InferenceSignal) -> "FuzzyResponse":
        """Create fuzzy response from inference signal."""
        gender, gender_conf = _resolve_gender(s)
        age_group, age_conf = _resolve_age_group(s)

        return cls(
            gender=gender,
            gender_confidence=gender_conf,
            ethnicity=s.ethnicity,
            ethnicity_confidence=Confidence.from_probability(s.ethnicity_confidence),
            age_group=age_group,
            age_group_confidence=age_conf,
            organization=s.organization,
            reasoning=s.reasoning,
        )


def _resolve_gender(s: InferenceSignal) -> tuple[Gender, Confidence]:
    """Resolve gender from probabilities to label + confidence."""
    options = [
        (Gender.MALE, s.gender_male),
        (Gender.FEMALE, s.gender_female),
        (Gender.UNDETERMINED, s.gender_other),
    ]

    gender, prob = max(options, key=lambda x: x[1])

    if prob == 0.0:
        return Gender.UNDETERMINED, Confidence.NONE

    return gender, Confidence.from_probability(prob)


def _resolve_age_group(s: InferenceSignal) -> tuple[AgeGroup | None, Confidence]:
    """Resolve age group from probabilities to label + confidence."""
    options = [
        (AgeGroup.UNDER_18, s.age_group_under_18),
        (AgeGroup.AGE_18_24, s.age_group_18_24),
        (AgeGroup.AGE_25_34, s.age_group_25_34),
        (AgeGroup.AGE_35_44, s.age_group_35_44),
        (AgeGroup.AGE_45_54, s.age_group_45_54),
        (AgeGroup.AGE_55_64, s.age_group_55_64),
        (AgeGroup.AGE_65_PLUS, s.age_group_65_plus),
    ]

    group, prob = max(options, key=lambda x: x[1])

    if prob == 0.0:
        return None, Confidence.NONE

    return group, Confidence.from_probability(prob)
