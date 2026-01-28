"""Tests for core types."""

import pytest

from aura.core.types import Gender, AgeGroup, Confidence


class TestConfidence:
    def test_strong_confidence(self):
        assert Confidence.from_probability(0.8) == Confidence.STRONG
        assert Confidence.from_probability(0.9) == Confidence.STRONG
        assert Confidence.from_probability(1.0) == Confidence.STRONG

    def test_medium_confidence(self):
        assert Confidence.from_probability(0.5) == Confidence.MEDIUM
        assert Confidence.from_probability(0.7) == Confidence.MEDIUM
        assert Confidence.from_probability(0.79) == Confidence.MEDIUM

    def test_low_confidence(self):
        assert Confidence.from_probability(0.1) == Confidence.LOW
        assert Confidence.from_probability(0.49) == Confidence.LOW

    def test_none_confidence(self):
        assert Confidence.from_probability(0.0) == Confidence.NONE


class TestAgeGroup:
    def test_from_age(self):
        assert AgeGroup.from_age(10) == AgeGroup.UNDER_18
        assert AgeGroup.from_age(17) == AgeGroup.UNDER_18
        assert AgeGroup.from_age(18) == AgeGroup.AGE_18_24
        assert AgeGroup.from_age(24) == AgeGroup.AGE_18_24
        assert AgeGroup.from_age(25) == AgeGroup.AGE_25_34
        assert AgeGroup.from_age(34) == AgeGroup.AGE_25_34
        assert AgeGroup.from_age(35) == AgeGroup.AGE_35_44
        assert AgeGroup.from_age(65) == AgeGroup.AGE_65_PLUS
        assert AgeGroup.from_age(80) == AgeGroup.AGE_65_PLUS

    def test_to_one_hot(self):
        probs = AgeGroup.AGE_25_34.to_one_hot()
        assert len(probs) == 7
        assert probs[2] == 1.0  # 25-34 is index 2
        assert sum(probs) == 1.0

    def test_blend(self):
        dist1 = [0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0]  # 18-24
        dist2 = [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0]  # 25-34
        weights = [1.0, 1.0]

        result = AgeGroup.blend([dist1, dist2], weights)

        assert result[1] == 0.5  # 18-24
        assert result[2] == 0.5  # 25-34
        assert sum(result) == pytest.approx(1.0)
