"""Pytest configuration and fixtures."""

import pytest
from unittest.mock import AsyncMock, patch


@pytest.fixture
def mock_openai():
    """Mock OpenAI API responses."""
    with patch("aura.services.openai_client.openai_client.chat_completion") as mock:
        mock.return_value = {
            "choices": [
                {
                    "message": {
                        "content": '{"gender_male": 0.85, "gender_female": 0.15, "ethnicity": "european", "ethnicity_confidence": 0.7, "reasoning": "Name analysis suggests male, European origin."}'
                    }
                }
            ],
            "usage": {"total_tokens": 50},
        }
        yield mock


@pytest.fixture
def mock_openai_vision():
    """Mock OpenAI Vision API responses."""
    with patch("aura.services.openai_client.openai_client.chat_completion") as mock:
        mock.return_value = {
            "choices": [
                {
                    "message": {
                        "content": '{"gender_male": 0.9, "gender_female": 0.1, "age_group": "25-34", "age_confidence": 0.8, "is_human": true, "reasoning": "Adult male in mid-twenties to early thirties."}'
                    }
                }
            ],
            "usage": {"total_tokens": 100},
        }
        yield mock
