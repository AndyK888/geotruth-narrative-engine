"""
GeoTruth API - Map Matching Endpoint

Valhalla map matching integration for snapping GPS to road network.
"""

import logging
from typing import List

import httpx
from fastapi import APIRouter, HTTPException

from ..config import settings
from ..models import (
    MapMatchRequest,
    MapMatchResponse,
    MatchedPoint,
    RoadEdge,
)
from ..services import cache

router = APIRouter()
logger = logging.getLogger(__name__)


async def match_to_road_network(coordinates: List[dict]) -> MapMatchResponse:
    """
    Match GPS coordinates to road network using Valhalla.
    """
    cache_key = f"mapmatch:{hash(str(coordinates)[:100])}"
    cached = await cache.get(cache_key)
    if cached:
        return MapMatchResponse(**cached)
    
    # Build Valhalla request
    valhalla_request = {
        "shape": [{"lat": c["lat"], "lon": c["lon"]} for c in coordinates],
        "costing": "auto",
        "shape_match": "walk_or_snap",
        "filters": {
            "attributes": [
                "edge.way_id",
                "edge.road_class",
                "edge.names",
                "edge.length",
                "edge.speed_limit",
                "matched.point",
                "matched.edge_index",
                "matched.distance_from_trace_point"
            ],
            "action": "include"
        }
    }
    
    try:
        async with httpx.AsyncClient(timeout=30.0) as client:
            response = await client.post(
                f"{settings.VALHALLA_URL}/trace_attributes",
                json=valhalla_request
            )
            
            if response.status_code != 200:
                logger.warning(f"Valhalla error: {response.text}")
                # Return empty result instead of failing
                return MapMatchResponse(
                    matched_points=[],
                    edges=[],
                    route={}
                )
            
            data = response.json()
            
    except Exception as e:
        logger.error(f"Valhalla connection failed: {e}")
        return MapMatchResponse(
            matched_points=[],
            edges=[],
            route={}
        )
    
    # Parse Valhalla response
    matched_points = []
    edges = []
    
    if "matched_points" in data:
        for i, mp in enumerate(data["matched_points"]):
            if mp.get("type") == "matched":
                matched_points.append(MatchedPoint(
                    lat=mp.get("lat", coordinates[i]["lat"]),
                    lon=mp.get("lon", coordinates[i]["lon"]),
                    edge_id=mp.get("edge_index", 0),
                    distance_from_input_m=mp.get("distance_from_trace_point", 0)
                ))
    
    if "edges" in data:
        for edge in data["edges"]:
            edges.append(RoadEdge(
                id=edge.get("id", 0),
                osm_way_id=edge.get("way_id"),
                road_name=edge.get("names", [None])[0] if edge.get("names") else None,
                road_class=edge.get("road_class"),
                length_m=edge.get("length", 0) * 1000,  # km to m
                speed_limit_kmh=edge.get("speed_limit"),
                begin_heading=edge.get("begin_heading", 0),
                end_heading=edge.get("end_heading", 0)
            ))
    
    result = MapMatchResponse(
        matched_points=matched_points,
        edges=edges,
        route=data.get("shape", {})
    )
    
    # Cache for 1 hour
    await cache.set(cache_key, result.model_dump(), ttl=3600)
    
    return result


@router.post("/map_match", response_model=MapMatchResponse)
async def map_match(request: MapMatchRequest) -> MapMatchResponse:
    """
    Match GPS coordinates to road network.
    
    Uses Valhalla for map matching. Falls back gracefully if unavailable.
    """
    logger.info(
        "Map matching requested",
        extra={"context": {"point_count": len(request.coordinates)}}
    )
    
    if len(request.coordinates) < 2:
        raise HTTPException(
            status_code=400,
            detail="At least 2 coordinates required for map matching"
        )
    
    if len(request.coordinates) > 1000:
        raise HTTPException(
            status_code=400,
            detail="Maximum 1000 coordinates per request"
        )
    
    coordinates = [
        {"lat": c.lat, "lon": c.lon, "time": c.timestamp.isoformat() if c.timestamp else None}
        for c in request.coordinates
    ]
    
    result = await match_to_road_network(coordinates)
    
    logger.info(
        "Map matching completed",
        extra={"context": {
            "matched_points": len(result.matched_points),
            "edges": len(result.edges)
        }}
    )
    
    return result
