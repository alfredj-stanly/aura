"""Abstract base class for inference agents."""

from abc import ABC, abstractmethod

from aura.core.models import InferenceInput, InferenceSignal


class Agent(ABC):
    """Abstract base class for all inference agents."""

    @abstractmethod
    async def analyze(self, input: InferenceInput) -> InferenceSignal:
        """Analyze input and return an inference signal.

        Args:
            input: The inference input containing user data.

        Returns:
            An InferenceSignal with the analysis results.
        """
        pass
