"""
GeoTruth API - Main Application

FastAPI application entry point with middleware and route configuration.
"""

from contextlib import asynccontextmanager
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from .config import settings
from .logging_config import setup_logging
from .api import health
from .api import enrich
from .middleware.logging import LoggingMiddleware
from .services import init_database, close_database, close_redis


@asynccontextmanager
async def lifespan(app: FastAPI):
    """Application lifespan events."""
    # Startup
    logger = setup_logging(
        log_level=settings.LOG_LEVEL,
        log_format=settings.LOG_FORMAT
    )
    logger.info(
        "Starting GeoTruth API",
        extra={"context": {"version": settings.VERSION, "environment": settings.ENVIRONMENT}}
    )
    
    # Initialize services
    await init_database()
    
    yield
    
    # Shutdown
    await close_database()
    await close_redis()
    logger.info("GeoTruth API shutdown complete")


# Create FastAPI application
app = FastAPI(
    title="GeoTruth API",
    description="Geospatial intelligence API for fact-checked travel narration",
    version=settings.VERSION,
    docs_url="/docs",
    redoc_url="/redoc",
    lifespan=lifespan,
)

# Add CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"] if settings.ENVIRONMENT == "development" else settings.ALLOWED_ORIGINS,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Add logging middleware
app.add_middleware(LoggingMiddleware)

# Include routers
app.include_router(health.router, prefix="/v1", tags=["Health"])
app.include_router(enrich.router, prefix="/v1", tags=["Enrichment"])


@app.get("/")
async def root():
    """Root endpoint with API information."""
    return {
        "name": "GeoTruth API",
        "version": settings.VERSION,
        "docs": "/docs",
        "health": "/v1/health"
    }
