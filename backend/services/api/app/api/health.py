"""
GeoTruth API - Health Check Endpoint

Provides health status for the API and its dependencies.
"""

import logging
from fastapi import APIRouter
from pydantic import BaseModel

from ..config import settings

router = APIRouter()
logger = logging.getLogger(__name__)


class ServiceStatus(BaseModel):
    """Status of individual services."""
    database: str = "not_configured"
    redis: str = "not_configured"
    valhalla: str = "not_configured"


class HealthResponse(BaseModel):
    """Health check response."""
    status: str
    version: str
    environment: str
    services: ServiceStatus


@router.get("/health", response_model=HealthResponse)
async def health_check() -> HealthResponse:
    """
    Check the health status of the API and its dependencies.
    
    Returns:
        HealthResponse: Current health status of all services.
    """
    # For Week 1, we return a basic healthy status
    # In Week 2, we'll add actual dependency checks
    
    services = ServiceStatus(
        database="pending",  # Will be implemented in Week 2
        redis="pending",     # Will be implemented in Week 2
        valhalla="pending"   # Will be implemented in Week 7
    )
    
    logger.info(
        "Health check performed",
        extra={"context": {"services": services.model_dump()}}
    )
    
    return HealthResponse(
        status="healthy",
        version=settings.VERSION,
        environment=settings.ENVIRONMENT,
        services=services
    )
