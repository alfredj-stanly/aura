"""Application configuration using pydantic-settings."""

from pydantic_settings import BaseSettings, SettingsConfigDict


class Settings(BaseSettings):
    """Application settings loaded from environment variables."""

    openai_api_key: str = ""
    host: str = "127.0.0.1"
    port: int = 7878
    debug: bool = False

    # LLM settings
    llm_model: str = "gpt-4o-mini"
    llm_temperature: float = 0.1
    llm_timeout: float = 30.0

    model_config = SettingsConfigDict(
        env_file=".env",
        env_file_encoding="utf-8",
    )


settings = Settings()
