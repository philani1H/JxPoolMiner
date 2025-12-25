use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningJob {
    pub id: String,
    pub algorithm: super::Algorithm,
    pub header: Vec<u8>,
    pub target: Vec<u8>,
    pub difficulty: f64,
    pub timestamp: DateTime<Utc>,
}

impl MiningJob {
    pub fn new(id: String, algorithm: super::Algorithm, difficulty: f64) -> Self {
        Self {
            id,
            algorithm,
            header: vec![],
            target: vec![0xFF; 32],
            difficulty,
            timestamp: Utc::now(),
        }
    }
}
