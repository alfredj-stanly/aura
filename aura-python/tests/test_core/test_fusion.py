"""Tests for signal fusion."""

import pytest

from aura.core.fusion import fuse
from aura.core.models import InferenceSignal
from aura.core.types import SignalSource, OrganizationIntelligence


def test_fuse_empty_returns_default():
    """Test that fusing empty list returns default signal."""
    result = fuse([])

    assert result.source == SignalSource.LOCAL
    assert not result.has_gender_signal()


def test_fuse_single_returns_same():
    """Test that fusing single signal returns that signal."""
    signal = InferenceSignal(source=SignalSource.ONOMASTIC, gender_male=0.8, gender_female=0.2)

    result = fuse([signal])

    assert result.gender_male == 0.8
    assert result.gender_female == 0.2


def test_fuse_averages_gender():
    """Test that gender probabilities are averaged."""
    s1 = InferenceSignal(source=SignalSource.ONOMASTIC, gender_male=0.8, gender_female=0.2)
    s2 = InferenceSignal(source=SignalSource.VISION, gender_male=0.6, gender_female=0.4)

    result = fuse([s1, s2])

    assert result.gender_male == 0.7
    assert result.gender_female == 0.3


def test_fuse_ignores_empty_gender_signals():
    """Test that signals without gender data don't affect average."""
    s1 = InferenceSignal(source=SignalSource.ONOMASTIC, gender_male=0.8, gender_female=0.2)
    s2 = InferenceSignal(source=SignalSource.LOCAL)  # No gender signal

    result = fuse([s1, s2])

    assert result.gender_male == 0.8
    assert result.gender_female == 0.2


def test_fuse_prefers_org_with_name():
    """Test that organization with name is preferred."""
    org_basic = OrganizationIntelligence(domain="company.com")
    org_enriched = OrganizationIntelligence(domain="company.com", name="Company Inc", category="Tech")

    s1 = InferenceSignal(source=SignalSource.LOCAL, organization=org_basic)
    s2 = InferenceSignal(source=SignalSource.DOMAIN, organization=org_enriched)

    result = fuse([s1, s2])

    assert result.organization is not None
    assert result.organization.name == "Company Inc"


def test_fuse_highest_ethnicity_confidence_wins():
    """Test that ethnicity with highest confidence wins."""
    s1 = InferenceSignal(
        source=SignalSource.ONOMASTIC,
        ethnicity="south_asian",
        ethnicity_confidence=0.6,
    )
    s2 = InferenceSignal(
        source=SignalSource.VISION,
        ethnicity="european",
        ethnicity_confidence=0.8,
    )

    result = fuse([s1, s2])

    assert result.ethnicity == "european"
    assert result.ethnicity_confidence == 0.8


def test_fuse_combines_reasoning():
    """Test that all reasoning is combined."""
    s1 = InferenceSignal(source=SignalSource.LOCAL, reasoning=["Reason 1"])
    s2 = InferenceSignal(source=SignalSource.ONOMASTIC, reasoning=["Reason 2", "Reason 3"])

    result = fuse([s1, s2])

    assert len(result.reasoning) == 3
    assert "Reason 1" in result.reasoning
    assert "Reason 2" in result.reasoning
