#![allow(unused)]
use crate::types::TruthBundle;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};

/// In-memory state shared across the application
pub struct AppState {
    /// Caching for truth bundles or temporary processing results
    pub truth_cache: DashMap<String, TruthBundle>,
    /// Active processing jobs
    pub active_jobs: DashMap<String, JobStatus>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            truth_cache: DashMap::new(),
            active_jobs: DashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobStatus {
    Pending,
    Processing { progress: f32 },
    Completed,
    Failed { error: String },
}
