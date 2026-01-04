-- =============================================================================
-- GeoTruth Database Initialization
-- This script runs on first database creation
-- =============================================================================

-- Enable PostGIS extension
CREATE EXTENSION IF NOT EXISTS postgis;
CREATE EXTENSION IF NOT EXISTS postgis_topology;
CREATE EXTENSION IF NOT EXISTS pg_trgm;  -- For text search

-- Create UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- =============================================================================
-- Core Tables
-- =============================================================================

-- Projects table
CREATE TABLE IF NOT EXISTS projects (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Videos table
CREATE TABLE IF NOT EXISTS videos (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id UUID REFERENCES projects(id) ON DELETE CASCADE,
    filename TEXT NOT NULL,
    duration_seconds FLOAT,
    fps FLOAT,
    width INTEGER,
    height INTEGER,
    codec TEXT,
    file_size_bytes BIGINT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- GPS Tracks table
CREATE TABLE IF NOT EXISTS gps_tracks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    video_id UUID REFERENCES videos(id) ON DELETE CASCADE,
    track_type TEXT NOT NULL,  -- 'gpx', 'nmea', 'gopro', 'embedded'
    point_count INTEGER,
    start_time TIMESTAMPTZ,
    end_time TIMESTAMPTZ,
    bounds GEOMETRY(Polygon, 4326),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- GPS Points table (optimized for bulk inserts)
CREATE TABLE IF NOT EXISTS gps_points (
    id BIGSERIAL PRIMARY KEY,
    track_id UUID REFERENCES gps_tracks(id) ON DELETE CASCADE,
    timestamp TIMESTAMPTZ NOT NULL,
    geom GEOMETRY(Point, 4326) NOT NULL,
    elevation_m FLOAT,
    speed_kmh FLOAT,
    heading_deg FLOAT,
    accuracy_m FLOAT
);

-- POIs table
CREATE TABLE IF NOT EXISTS pois (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    name_local TEXT,
    category TEXT NOT NULL,
    subcategory TEXT,
    geom GEOMETRY(Point, 4326) NOT NULL,
    tags JSONB DEFAULT '{}',
    facts JSONB DEFAULT '{}',
    source TEXT NOT NULL,  -- 'osm', 'manual', 'verified'
    confidence FLOAT DEFAULT 0.8,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Events table (Truth Bundle events)
CREATE TABLE IF NOT EXISTS events (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id UUID REFERENCES projects(id) ON DELETE CASCADE,
    video_id UUID REFERENCES videos(id) ON DELETE CASCADE,
    event_type TEXT NOT NULL,  -- 'poi_visible', 'stop', 'turn', 'landmark'
    start_time_seconds FLOAT NOT NULL,
    end_time_seconds FLOAT,
    geom GEOMETRY(Point, 4326),
    heading_deg FLOAT,
    verified BOOLEAN DEFAULT FALSE,
    verification_mode TEXT,  -- 'online', 'offline'
    truth_bundle JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- =============================================================================
-- Indexes
-- =============================================================================

-- Spatial indexes
CREATE INDEX IF NOT EXISTS idx_gps_points_geom ON gps_points USING GIST (geom);
CREATE INDEX IF NOT EXISTS idx_pois_geom ON pois USING GIST (geom);
CREATE INDEX IF NOT EXISTS idx_events_geom ON events USING GIST (geom);
CREATE INDEX IF NOT EXISTS idx_gps_tracks_bounds ON gps_tracks USING GIST (bounds);

-- Time-based indexes
CREATE INDEX IF NOT EXISTS idx_gps_points_timestamp ON gps_points (timestamp);
CREATE INDEX IF NOT EXISTS idx_events_start_time ON events (start_time_seconds);

-- Foreign key indexes
CREATE INDEX IF NOT EXISTS idx_videos_project ON videos (project_id);
CREATE INDEX IF NOT EXISTS idx_gps_tracks_video ON gps_tracks (video_id);
CREATE INDEX IF NOT EXISTS idx_gps_points_track ON gps_points (track_id);
CREATE INDEX IF NOT EXISTS idx_events_project ON events (project_id);
CREATE INDEX IF NOT EXISTS idx_events_video ON events (video_id);

-- Category indexes
CREATE INDEX IF NOT EXISTS idx_pois_category ON pois (category);
CREATE INDEX IF NOT EXISTS idx_events_type ON events (event_type);

-- =============================================================================
-- Functions
-- =============================================================================

-- Auto-update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply to relevant tables
DROP TRIGGER IF EXISTS update_projects_updated_at ON projects;
CREATE TRIGGER update_projects_updated_at
    BEFORE UPDATE ON projects
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_pois_updated_at ON pois;
CREATE TRIGGER update_pois_updated_at
    BEFORE UPDATE ON pois
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- =============================================================================
-- Initial Data
-- =============================================================================

-- Insert some test POIs (Grand Canyon area)
INSERT INTO pois (name, category, subcategory, geom, source, facts) VALUES
    ('Grand Canyon South Rim', 'natural_landmark', 'canyon', ST_SetSRID(ST_MakePoint(-112.1129, 36.0544), 4326), 'osm', '{"established": "1919", "depth_m": 1857, "unesco_site": true}'),
    ('Bright Angel Trail', 'trail', 'hiking', ST_SetSRID(ST_MakePoint(-112.1438, 36.0576), 4326), 'osm', '{"length_km": 15.3, "difficulty": "strenuous"}'),
    ('Desert View Watchtower', 'landmark', 'historic', ST_SetSRID(ST_MakePoint(-111.8261, 36.0408), 4326), 'osm', '{"built": "1932", "architect": "Mary Colter"}')
ON CONFLICT DO NOTHING;

-- Log initialization
DO $$
BEGIN
    RAISE NOTICE 'GeoTruth database initialized successfully';
END $$;
