"""
GeoTruth API - Enrichment Endpoints

Geospatial enrichment for GPS coordinates.
"""

import logging
from typing import List

from fastapi import APIRouter, HTTPException

from ..models import (
    EnrichRequest,
    EnrichResponse,
    EnrichBatchRequest,
    EnrichBatchResponse,
    LocationResult,
    LocationContext,
    POI,
)
from ..services import cache

router = APIRouter()
logger = logging.getLogger(__name__)


async def enrich_point(request: EnrichRequest) -> EnrichResponse:
    """
    Enrich a single GPS point with location context and POIs.
    
    This is a simplified implementation for Week 2.
    Full implementation with PostGIS queries comes in Week 7.
    """
    # Check cache first
    cache_key = f"enrich:{request.lat:.4f}:{request.lon:.4f}"
    cached = await cache.get(cache_key)
    if cached:
        logger.debug(f"Cache hit for {cache_key}")
        return EnrichResponse(**cached)
    
    # Basic location result (map matching will be added in Week 7)
    location = LocationResult(
        lat=request.lat,
        lon=request.lon,
        matched=None  # Will be populated after Valhalla integration
    )
    
    # Basic context (reverse geocoding will be enhanced)
    context = LocationContext(
        country="United States",  # Placeholder
        timezone="America/Los_Angeles",
        elevation_m=None
    )
    
    # POIs will be queried from PostGIS in Week 7
    pois: List[POI] = []
    
    response = EnrichResponse(
        location=location,
        context=context,
        pois=pois
    )
    
    # Cache the result
    await cache.set(cache_key, response.model_dump(), ttl=3600)
    
    return response


@router.post("/enrich", response_model=EnrichResponse)
async def enrich(request: EnrichRequest) -> EnrichResponse:
    """
    Enrich a GPS coordinate with geospatial context.
    
    Returns location context and nearby points of interest.
    """
    logger.info(
        "Enrichment requested",
        extra={"context": {"lat": request.lat, "lon": request.lon}}
    )
    
    try:
        result = await enrich_point(request)
        logger.info(
            "Enrichment completed",
            extra={"context": {"pois_found": len(result.pois)}}
        )
        return result
    except Exception as e:
        logger.exception("Enrichment failed")
        raise HTTPException(status_code=500, detail=str(e))


@router.post("/enrich_batch", response_model=EnrichBatchResponse)
async def enrich_batch(request: EnrichBatchRequest) -> EnrichBatchResponse:
    """
    Enrich multiple GPS coordinates in a single request.
    
    Maximum 100 points per request.
    """
    if len(request.points) > 100:
        raise HTTPException(
            status_code=400,
            detail="Maximum 100 points per batch request"
        )
    
    logger.info(
        "Batch enrichment requested",
        extra={"context": {"point_count": len(request.points)}}
    )
    
    results = []
    cache_hits = 0
    
    for point in request.points:
        # Check if result was cached
        cache_key = f"enrich:{point.lat:.4f}:{point.lon:.4f}"
        if await cache.exists(cache_key):
            cache_hits += 1
        
        result = await enrich_point(point)
        results.append(result)
    
    logger.info(
        "Batch enrichment completed",
        extra={"context": {
            "point_count": len(request.points),
            "cache_hits": cache_hits
        }}
    )
    
    return EnrichBatchResponse(
        results=results,
        meta={
            "total_points": len(request.points),
            "cache_hits": cache_hits
        }
    )
