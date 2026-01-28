"""Core type definitions - enums and data structures."""

from __future__ import annotations

from enum import Enum
from pydantic import BaseModel


class SignalSource(str, Enum):
    """Source of an inference signal."""

    LOCAL = "local"
    VISION = "vision"
    ONOMASTIC = "onomastic"
    DOMAIN = "domain"


class Gender(str, Enum):
    """Inferred gender category."""

    MALE = "male"
    FEMALE = "female"
    UNDETERMINED = "undetermined"


class AgeGroup(str, Enum):
    """Age group buckets."""

    UNDER_18 = "under_18"
    AGE_18_24 = "18-24"
    AGE_25_34 = "25-34"
    AGE_35_44 = "35-44"
    AGE_45_54 = "45-54"
    AGE_55_64 = "55-64"
    AGE_65_PLUS = "65+"

    @classmethod
    def from_age(cls, age: int) -> "AgeGroup":
        """Convert age to age group."""
        if age < 18:
            return cls.UNDER_18
        elif age < 25:
            return cls.AGE_18_24
        elif age < 35:
            return cls.AGE_25_34
        elif age < 45:
            return cls.AGE_35_44
        elif age < 55:
            return cls.AGE_45_54
        elif age < 65:
            return cls.AGE_55_64
        else:
            return cls.AGE_65_PLUS

    def to_one_hot(self) -> list[float]:
        """Convert age group to one-hot encoded probability distribution."""
        groups = list(AgeGroup)
        probs = [0.0] * 7
        probs[groups.index(self)] = 1.0
        return probs

    @staticmethod
    def blend(distributions: list[list[float]], weights: list[float]) -> list[float]:
        """Blend multiple age distributions using weighted average."""
        result = [0.0] * 7
        total_weight = sum(weights)

        if total_weight == 0:
            return result

        for dist, w in zip(distributions, weights):
            for i in range(7):
                result[i] += dist[i] * w / total_weight

        return result


class Confidence(str, Enum):
    """Confidence level for inferences."""

    STRONG = "strong"
    MEDIUM = "medium"
    LOW = "low"
    NONE = "none"

    @classmethod
    def from_probability(cls, p: float) -> "Confidence":
        """Convert probability to confidence level."""
        if p >= 0.8:
            return cls.STRONG
        elif p >= 0.5:
            return cls.MEDIUM
        elif p > 0.0:
            return cls.LOW
        else:
            return cls.NONE


class OrganizationIntelligence(BaseModel):
    """Organization information extracted from email domain."""

    domain: str
    name: str | None = None
    category: str | None = None
    employee_count: str | None = None
    employee_count_source: str | None = None

    model_config = {"extra": "ignore"}
