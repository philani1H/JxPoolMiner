use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Share {
    pub job_id: String,
    pub nonce: u64,
    pub hash: String,
    pub timestamp: DateTime<Utc>,
    pub difficulty: f64,
}

impl Share {
    pub fn new(job_id: String, nonce: u64, hash: String, difficulty: f64) -> Self {
        Self {
            job_id,
            nonce,
            hash,
            timestamp: Utc::now(),
            difficulty,
        }
    }
}
