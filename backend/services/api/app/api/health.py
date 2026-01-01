"""
GeoTruth API - Health Check Endpoint

Provides health status for the API and its dependencies.
"""

import logging
from fastapi import APIRouter
from pydantic import BaseModel

from ..config import settings
from ..services import check_database_connection, check_redis_connection

router = APIRouter()
logger = logging.getLogger(__name__)


class ServiceStatus(BaseModel):
    """Status of individual services."""
    database: str = "disconnected"
    redis: str = "disconnected"
    valhalla: str = "pending"


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
    # Check actual service connections
    db_connected = await check_database_connection()
    redis_connected = await check_redis_connection()
    
    services = ServiceStatus(
        database="connected" if db_connected else "disconnected",
        redis="connected" if redis_connected else "disconnected",
        valhalla="pending"  # Will be implemented in Week 7
    )
    
    # Determine overall health
    is_healthy = db_connected and redis_connected
    
    logger.info(
        "Health check performed",
        extra={"context": {"services": services.model_dump(), "healthy": is_healthy}}
    )
    
    return HealthResponse(
        status="healthy" if is_healthy else "degraded",
        version=settings.VERSION,
        environment=settings.ENVIRONMENT,
        services=services
    )
