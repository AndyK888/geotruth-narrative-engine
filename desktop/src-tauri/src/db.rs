#![allow(dead_code)]
use anyhow::Result;
use duckdb::Connection;
use tauri::Manager;
use std::sync::Mutex;
use tracing::info;

pub struct DbState {
    pub conn: Mutex<Connection>,
}

impl DbState {
    pub fn new(app_handle: &tauri::AppHandle) -> Result<Self> {
        let app_dir = app_handle.path().app_data_dir()?;
        std::fs::create_dir_all(&app_dir)?;
        let db_path = app_dir.join("geotruth.duckdb");
        
        info!("Opening database at {:?}", db_path);
        let conn = Connection::open(db_path)?;

        // Initialize extensions if needed (checking if they are available)
        // For now, we will assume core functionality or handle geometry as BLOBs if extensions fail
        // attempt_load_extension(&conn, "spatial");
        // attempt_load_extension(&conn, "json");

        init_schema(&conn)?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }
}

fn attempt_load_extension(conn: &Connection, ext_name: &str) {
    if let Err(e) = conn.execute(&format!("INSTALL {}; LOAD {};", ext_name, ext_name), []) {
        info!("Extension {} could not be loaded (might be bundled or missing): {}", ext_name, e);
    } else {
        info!("Extension {} loaded successfully", ext_name);
    }
}

fn init_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE SEQUENCE IF NOT EXISTS seq_gps_points_id START 1;

        CREATE TABLE IF NOT EXISTS projects (
            id VARCHAR PRIMARY KEY DEFAULT uuid(),
            name VARCHAR NOT NULL,
            description VARCHAR,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS videos (
            id VARCHAR PRIMARY KEY DEFAULT uuid(),
            project_id VARCHAR REFERENCES projects(id),
            filename VARCHAR NOT NULL,
            duration_seconds DOUBLE,
            fps DOUBLE,
            width INTEGER,
            height INTEGER,
            codec VARCHAR,
            file_size_bytes BIGINT,
            created_at TIMESTAMPTZ DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS gps_tracks (
            id VARCHAR PRIMARY KEY DEFAULT uuid(),
            video_id VARCHAR REFERENCES videos(id),
            track_type VARCHAR NOT NULL,
            point_count INTEGER,
            start_time TIMESTAMPTZ,
            end_time TIMESTAMPTZ,
            bounds BLOB, -- WKB Geometry
            created_at TIMESTAMPTZ DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS gps_points (
            id BIGINT PRIMARY KEY DEFAULT nextval('seq_gps_points_id'),
            track_id VARCHAR REFERENCES gps_tracks(id),
            timestamp TIMESTAMPTZ NOT NULL,
            geom BLOB NOT NULL, -- WKB Geometry (Point)
            elevation_m DOUBLE,
            speed_kmh DOUBLE,
            heading_deg DOUBLE,
            accuracy_m DOUBLE
        );

        CREATE TABLE IF NOT EXISTS pois (
            id VARCHAR PRIMARY KEY DEFAULT uuid(),
            name VARCHAR NOT NULL,
            name_local VARCHAR,
            category VARCHAR NOT NULL,
            subcategory VARCHAR,
            geom BLOB NOT NULL, -- WKB Geometry (Point)
            tags JSON,
            facts JSON,
            source VARCHAR NOT NULL,
            confidence DOUBLE DEFAULT 0.8,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS events (
            id VARCHAR PRIMARY KEY DEFAULT uuid(),
            project_id VARCHAR REFERENCES projects(id),
            video_id VARCHAR REFERENCES videos(id),
            event_type VARCHAR NOT NULL,
            start_time_seconds DOUBLE NOT NULL,
            end_time_seconds DOUBLE,
            geom BLOB, -- WKB Geometry (Point)
            heading_deg DOUBLE,
            verified BOOLEAN DEFAULT FALSE,
            verification_mode VARCHAR,
            truth_bundle JSON,
            created_at TIMESTAMPTZ DEFAULT NOW()
        );
        "#,
    )?;
    
    info!("Database schema initialized.");
    Ok(())
}
