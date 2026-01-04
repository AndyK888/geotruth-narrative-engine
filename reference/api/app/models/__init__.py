"""
GeoTruth API - Pydantic Models

Data models for API requests and responses.
"""

from datetime import datetime
from typing import Optional, List
from uuid import UUID

from pydantic import BaseModel, Field


# =============================================================================
# Common Models
# =============================================================================

class Coordinates(BaseModel):
    """GPS coordinates."""
    lat: float = Field(..., ge=-90, le=90, description="Latitude")
    lon: float = Field(..., ge=-180, le=180, description="Longitude")


class TimestampedCoordinates(Coordinates):
    """GPS coordinates with timestamp."""
    timestamp: Optional[datetime] = None
    accuracy_m: Optional[float] = Field(None, ge=0, description="GPS accuracy in meters")
    heading_deg: Optional[float] = Field(None, ge=0, lt=360, description="Heading in degrees")
    speed_kmh: Optional[float] = Field(None, ge=0, description="Speed in km/h")


# =============================================================================
# POI Models
# =============================================================================

class POIFacts(BaseModel):
    """Facts about a POI."""
    established: Optional[str] = None
    depth_m: Optional[float] = None
    unesco_site: Optional[bool] = None
    # Extensible for other fact types
    
    class Config:
        extra = "allow"


class POI(BaseModel):
    """Point of Interest."""
    id: str
    name: str
    name_local: Optional[str] = None
    category: str
    subcategory: Optional[str] = None
    lat: float
    lon: float
    distance_m: float
    bearing_deg: float
    in_fov: bool = True
    confidence: float = Field(default=0.8, ge=0, le=1)
    facts: Optional[POIFacts] = None
    tags: Optional[dict] = None


# =============================================================================
# Location Context
# =============================================================================

class MatchedLocation(BaseModel):
    """Map-matched location on road network."""
    lat: float
    lon: float
    road_name: Optional[str] = None
    road_class: Optional[str] = None
    direction: Optional[str] = None  # 'northbound', 'southbound', etc.


class LocationContext(BaseModel):
    """Geographic context for a location."""
    country: Optional[str] = None
    state: Optional[str] = None
    county: Optional[str] = None
    city: Optional[str] = None
    timezone: Optional[str] = None
    elevation_m: Optional[float] = None


class LocationResult(BaseModel):
    """Complete location information."""
    lat: float
    lon: float
    matched: Optional[MatchedLocation] = None


# =============================================================================
# Enrichment
# =============================================================================

class EnrichRequest(BaseModel):
    """Request to enrich a GPS coordinate."""
    lat: float = Field(..., ge=-90, le=90)
    lon: float = Field(..., ge=-180, le=180)
    timestamp: Optional[datetime] = None
    heading_deg: Optional[float] = Field(None, ge=0, lt=360)
    fov_deg: float = Field(default=120, ge=1, le=360)


class EnrichResponse(BaseModel):
    """Enriched location response."""
    location: LocationResult
    context: LocationContext
    pois: List[POI] = []


class EnrichBatchRequest(BaseModel):
    """Batch enrichment request."""
    points: List[EnrichRequest] = Field(..., max_length=100)
    options: Optional[dict] = None


class EnrichBatchResponse(BaseModel):
    """Batch enrichment response."""
    results: List[EnrichResponse]
    meta: dict = {}


# =============================================================================
# Map Matching
# =============================================================================

class MapMatchRequest(BaseModel):
    """Request to match GPS to road network."""
    coordinates: List[TimestampedCoordinates] = Field(..., max_length=1000)
    costing: str = Field(default="auto", pattern="^(auto|pedestrian|bicycle|bus)$")
    shape_match: str = Field(default="walk_or_snap")


class MatchedPoint(BaseModel):
    """A GPS point matched to road network."""
    lat: float
    lon: float
    edge_id: int
    distance_from_input_m: float


class RoadEdge(BaseModel):
    """Road segment information."""
    id: int
    osm_way_id: Optional[int] = None
    road_name: Optional[str] = None
    road_class: Optional[str] = None
    length_m: float
    speed_limit_kmh: Optional[float] = None
    begin_heading: float
    end_heading: float


class RouteGeometry(BaseModel):
    """Route geometry as GeoJSON."""
    type: str = "LineString"
    coordinates: List[List[float]]


class MapMatchResponse(BaseModel):
    """Map matching response."""
    matched_points: List[MatchedPoint]
    edges: List[RoadEdge]
    route: dict = {}


# =============================================================================
# Projects & Videos
# =============================================================================

class ProjectCreate(BaseModel):
    """Create a new project."""
    name: str = Field(..., min_length=1, max_length=200)
    description: Optional[str] = None


class Project(BaseModel):
    """Project model."""
    id: UUID
    name: str
    description: Optional[str]
    created_at: datetime
    updated_at: datetime


class VideoMetadata(BaseModel):
    """Video file metadata."""
    filename: str
    duration_seconds: Optional[float] = None
    fps: Optional[float] = None
    width: Optional[int] = None
    height: Optional[int] = None
    codec: Optional[str] = None
    file_size_bytes: Optional[int] = None


class Video(VideoMetadata):
    """Video model with ID."""
    id: UUID
    project_id: UUID
    created_at: datetime


# =============================================================================
# Truth Bundle
# =============================================================================

class TruthEvent(BaseModel):
    """An event in the Truth Bundle."""
    id: str
    timestamp: datetime
    duration_seconds: Optional[float] = None
    location: LocationResult
    pois: List[POI] = []
    detected_objects: List[dict] = []
    metadata: dict = {}


class TruthBundle(BaseModel):
    """Complete Truth Bundle for AI narration."""
    project_id: Optional[UUID] = None
    video_id: Optional[UUID] = None
    events: List[TruthEvent] = []
    verification_mode: str = "online"  # 'online' or 'offline'
    generated_at: datetime = Field(default_factory=datetime.utcnow)


# =============================================================================
# AI Narration
# =============================================================================

class NarrateRequest(BaseModel):
    """Request for AI narration."""
    truth_bundle: TruthBundle
    transcript: Optional[str] = None
    options: dict = {}


class Chapter(BaseModel):
    """Video chapter for YouTube."""
    time_code: str
    title: str
    description: Optional[str] = None
    formatted: Optional[str] = None


class ScriptSegment(BaseModel):
    """Narration script segment."""
    start_time: str
    end_time: str
    text: str


class NarrateResponse(BaseModel):
    """AI narration response."""
    chapters: List[Chapter] = []
    script: Optional[dict] = None
    meta: dict = {}
