"""Tests for LocalAgent."""

import pytest

from aura.agents.local import LocalAgent
from aura.core.models import InferenceInput


@pytest.mark.asyncio
async def test_extracts_organization():
    """Test organization extraction from corporate email."""
    agent = LocalAgent()
    input = InferenceInput(email="trinity@vogue.com")

    signal = await agent.analyze(input)

    assert signal.organization is not None
    assert signal.organization.domain == "vogue.com"


@pytest.mark.asyncio
async def test_ignores_personal_email():
    """Test that personal email domains are ignored."""
    agent = LocalAgent()
    input = InferenceInput(email="john.doe@gmail.com")

    signal = await agent.analyze(input)

    assert signal.organization is None


@pytest.mark.asyncio
async def test_extracts_birth_year():
    """Test birth year extraction from email pattern."""
    agent = LocalAgent()
    input = InferenceInput(email="laura1992@gmail.com")

    signal = await agent.analyze(input)

    assert signal.birth_year == 1992
    assert signal.has_age_signal()


@pytest.mark.asyncio
async def test_ignores_invalid_year():
    """Test that invalid years are ignored."""
    agent = LocalAgent()
    input = InferenceInput(email="test9162@gmail.com")

    signal = await agent.analyze(input)

    assert signal.birth_year is None


@pytest.mark.asyncio
async def test_no_birth_year_no_age_probs():
    """Test that no birth year means no age probabilities."""
    agent = LocalAgent()
    input = InferenceInput(email="aparna@gmail.com")

    signal = await agent.analyze(input)

    assert signal.birth_year is None
    assert not signal.has_age_signal()
