//! DuckDB Local Database
//!
//! Embedded database for local project storage in the desktop app.

use std::path::PathBuf;
use std::sync::Arc;
use duckdb::{Connection, Result as DuckResult, params};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{debug, info, warn};
use tokio::sync::Mutex;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database error: {0}")]
    DuckDb(#[from] duckdb::Error),
    
    #[error("Database not initialized")]
    NotInitialized,
    
    #[error("Record not found")]
    NotFound,
    
    #[error("Serialization error: {0}")]
    Serialization(String),
}

/// Project record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub video_count: u32,
}

/// Video record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    pub id: String,
    pub project_id: String,
    pub filename: String,
    pub duration_seconds: Option<f64>,
    pub fps: Option<f64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub codec: Option<String>,
    pub file_size_bytes: Option<i64>,
    pub file_path: String,
    pub created_at: DateTime<Utc>,
}

/// GPS point record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpsPoint {
    pub id: i64,
    pub video_id: String,
    pub timestamp: DateTime<Utc>,
    pub lat: f64,
    pub lon: f64,
    pub elevation_m: Option<f64>,
    pub speed_kmh: Option<f64>,
    pub heading_deg: Option<f64>,
}

/// Event record (for Truth Bundle)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub video_id: String,
    pub event_type: String,
    pub start_time_seconds: f64,
    pub end_time_seconds: Option<f64>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub heading_deg: Option<f64>,
    pub verified: bool,
    pub verification_mode: Option<String>,
    pub truth_bundle_json: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Local DuckDB database manager
pub struct LocalDatabase {
    conn: Arc<Mutex<Connection>>,
    path: PathBuf,
}

impl LocalDatabase {
    /// Open or create database at path
    pub fn open(path: PathBuf) -> Result<Self, DatabaseError> {
        info!("Opening local database: {:?}", path);
        
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        
        let conn = Connection::open(&path)?;
        
        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
            path,
        };
        
        Ok(db)
    }
    
    /// Initialize database schema
    pub async fn init(&self) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().await;
        
        // Create tables
        conn.execute_batch(r#"
            -- Projects table
            CREATE TABLE IF NOT EXISTS projects (
                id VARCHAR PRIMARY KEY,
                name VARCHAR NOT NULL,
                description VARCHAR,
                created_at TIMESTAMP DEFAULT current_timestamp,
                updated_at TIMESTAMP DEFAULT current_timestamp
            );
            
            -- Videos table
            CREATE TABLE IF NOT EXISTS videos (
                id VARCHAR PRIMARY KEY,
                project_id VARCHAR NOT NULL REFERENCES projects(id),
                filename VARCHAR NOT NULL,
                duration_seconds DOUBLE,
                fps DOUBLE,
                width INTEGER,
                height INTEGER,
                codec VARCHAR,
                file_size_bytes BIGINT,
                file_path VARCHAR NOT NULL,
                created_at TIMESTAMP DEFAULT current_timestamp
            );
            
            -- GPS points table (optimized for bulk operations)
            CREATE TABLE IF NOT EXISTS gps_points (
                id BIGINT PRIMARY KEY,
                video_id VARCHAR NOT NULL REFERENCES videos(id),
                timestamp TIMESTAMP NOT NULL,
                lat DOUBLE NOT NULL,
                lon DOUBLE NOT NULL,
                elevation_m DOUBLE,
                speed_kmh DOUBLE,
                heading_deg DOUBLE
            );
            
            -- Create sequence for GPS points
            CREATE SEQUENCE IF NOT EXISTS gps_points_seq;
            
            -- Events table (Truth Bundle events)
            CREATE TABLE IF NOT EXISTS events (
                id VARCHAR PRIMARY KEY,
                video_id VARCHAR NOT NULL REFERENCES videos(id),
                event_type VARCHAR NOT NULL,
                start_time_seconds DOUBLE NOT NULL,
                end_time_seconds DOUBLE,
                lat DOUBLE,
                lon DOUBLE,
                heading_deg DOUBLE,
                verified BOOLEAN DEFAULT false,
                verification_mode VARCHAR,
                truth_bundle_json VARCHAR,
                created_at TIMESTAMP DEFAULT current_timestamp
            );
            
            -- Transcription segments table
            CREATE TABLE IF NOT EXISTS transcriptions (
                id VARCHAR PRIMARY KEY,
                video_id VARCHAR NOT NULL REFERENCES videos(id),
                start_ms BIGINT NOT NULL,
                end_ms BIGINT NOT NULL,
                text VARCHAR NOT NULL,
                language VARCHAR
            );
            
            -- Create indexes
            CREATE INDEX IF NOT EXISTS idx_videos_project ON videos(project_id);
            CREATE INDEX IF NOT EXISTS idx_gps_video ON gps_points(video_id);
            CREATE INDEX IF NOT EXISTS idx_gps_timestamp ON gps_points(timestamp);
            CREATE INDEX IF NOT EXISTS idx_events_video ON events(video_id);
            CREATE INDEX IF NOT EXISTS idx_events_time ON events(start_time_seconds);
            CREATE INDEX IF NOT EXISTS idx_transcriptions_video ON transcriptions(video_id);
        "#)?;
        
        info!("Database schema initialized");
        Ok(())
    }
    
    // ==========================================================================
    // Projects
    // ==========================================================================
    
    /// Create a new project
    pub async fn create_project(&self, name: &str, description: Option<&str>) -> Result<Project, DatabaseError> {
        let conn = self.conn.lock().await;
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        
        conn.execute(
            "INSERT INTO projects (id, name, description, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
            params![id, name, description, now.to_rfc3339(), now.to_rfc3339()],
        )?;
        
        debug!("Created project: {}", id);
        
        Ok(Project {
            id,
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            created_at: now,
            updated_at: now,
            video_count: 0,
        })
    }
    
    /// Get all projects
    pub async fn get_projects(&self) -> Result<Vec<Project>, DatabaseError> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT p.id, p.name, p.description, p.created_at, p.updated_at, 
                    COUNT(v.id) as video_count
             FROM projects p
             LEFT JOIN videos v ON v.project_id = p.id
             GROUP BY p.id, p.name, p.description, p.created_at, p.updated_at
             ORDER BY p.updated_at DESC"
        )?;
        
        let projects = stmt.query_map([], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                created_at: Utc::now(), // Simplified for demo
                updated_at: Utc::now(),
                video_count: row.get::<_, i64>(5)? as u32,
            })
        })?.filter_map(|r| r.ok()).collect();
        
        Ok(projects)
    }
    
    // ==========================================================================
    // Videos
    // ==========================================================================
    
    /// Add a video to a project
    pub async fn add_video(
        &self,
        project_id: &str,
        filename: &str,
        file_path: &str,
        metadata: Option<VideoMetadata>,
    ) -> Result<Video, DatabaseError> {
        let conn = self.conn.lock().await;
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        
        let (duration, fps, width, height, codec, size) = metadata
            .map(|m| (m.duration_seconds, m.fps, m.width, m.height, m.codec, m.file_size_bytes))
            .unwrap_or((None, None, None, None, None, None));
        
        conn.execute(
            "INSERT INTO videos (id, project_id, filename, file_path, duration_seconds, fps, width, height, codec, file_size_bytes, created_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![id, project_id, filename, file_path, duration, fps, width, height, codec, size, now.to_rfc3339()],
        )?;
        
        debug!("Added video: {} to project {}", id, project_id);
        
        Ok(Video {
            id,
            project_id: project_id.to_string(),
            filename: filename.to_string(),
            duration_seconds: duration,
            fps,
            width,
            height,
            codec,
            file_size_bytes: size,
            file_path: file_path.to_string(),
            created_at: now,
        })
    }
    
    /// Get videos for a project
    pub async fn get_project_videos(&self, project_id: &str) -> Result<Vec<Video>, DatabaseError> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT id, project_id, filename, file_path, duration_seconds, fps, width, height, codec, file_size_bytes, created_at
             FROM videos WHERE project_id = ? ORDER BY created_at DESC"
        )?;
        
        let videos = stmt.query_map(params![project_id], |row| {
            Ok(Video {
                id: row.get(0)?,
                project_id: row.get(1)?,
                filename: row.get(2)?,
                file_path: row.get(3)?,
                duration_seconds: row.get(4)?,
                fps: row.get(5)?,
                width: row.get(6)?,
                height: row.get(7)?,
                codec: row.get(8)?,
                file_size_bytes: row.get(9)?,
                created_at: Utc::now(),
            })
        })?.filter_map(|r| r.ok()).collect();
        
        Ok(videos)
    }
    
    /// Get database path
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

/// Video metadata for import
#[derive(Debug, Clone)]
pub struct VideoMetadata {
    pub duration_seconds: Option<f64>,
    pub fps: Option<f64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub codec: Option<String>,
    pub file_size_bytes: Option<i64>,
}
