use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Share {
    pub job_id: String,
    pub nonce: u64,
    pub hash: Vec<u8>,
    pub device_id: String,
    pub timestamp: DateTime<Utc>,
}

impl Share {
    pub fn new(job_id: String, nonce: u64, hash: Vec<u8>, device_id: String) -> Self {
        Self {
            job_id,
            nonce,
            hash,
            device_id,
            timestamp: Utc::now(),
        }
    }
}
