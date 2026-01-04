"""
GeoTruth API - Configuration

Pydantic settings for environment-based configuration.
"""

from typing import List
from pydantic_settings import BaseSettings


class Settings(BaseSettings):
    """Application settings loaded from environment."""
    
    # Application
    VERSION: str = "0.1.0"
    ENVIRONMENT: str = "development"
    
    # Logging
    LOG_LEVEL: str = "INFO"
    LOG_FORMAT: str = "json"  # "json" or "pretty"
    
    # Database
    POSTGRES_HOST: str = "localhost"
    POSTGRES_PORT: int = 5432
    POSTGRES_USER: str = "geotruth"
    POSTGRES_PASSWORD: str = ""
    POSTGRES_DB: str = "geotruth"
    
    # Redis
    REDIS_URL: str = "redis://localhost:6379/0"
    
    # Valhalla
    VALHALLA_URL: str = "http://localhost:8002"
    
    # AI
    GEMINI_API_KEY: str = ""
    
    # Security
    JWT_SECRET: str = ""
    ALLOWED_ORIGINS: List[str] = ["http://localhost:5173"]
    
    @property
    def database_url(self) -> str:
        """Get async database URL."""
        return (
            f"postgresql+asyncpg://{self.POSTGRES_USER}:{self.POSTGRES_PASSWORD}"
            f"@{self.POSTGRES_HOST}:{self.POSTGRES_PORT}/{self.POSTGRES_DB}"
        )
    
    class Config:
        env_file = ".env"
        case_sensitive = True


settings = Settings()
