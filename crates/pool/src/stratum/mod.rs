use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use anyhow::{Result, Context};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StratumRequest {
    pub id: u64,
    pub method: String,
    pub params: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StratumResponse {
    pub id: Option<u64>,
    pub result: Option<Value>,
    pub error: Option<Value>,
    pub method: Option<String>,
    pub params: Option<Vec<Value>>,
}

pub struct StratumClient {
    stream: Arc<RwLock<Option<TcpStream>>>,
    request_id: Arc<RwLock<u64>>,
    tx: mpsc::UnboundedSender<StratumResponse>,
    rx: Arc<RwLock<mpsc::UnboundedReceiver<StratumResponse>>>,
}

impl StratumClient {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self {
            stream: Arc::new(RwLock::new(None)),
            request_id: Arc::new(RwLock::new(1)),
            tx,
            rx: Arc::new(RwLock::new(rx)),
        }
    }

    pub async fn connect(&self, url: &str) -> Result<()> {
        let addr = url
            .strip_prefix("stratum+tcp://")
            .or_else(|| url.strip_prefix("tcp://"))
            .unwrap_or(url);

        tracing::info!("Connecting to Stratum server: {}", addr);
        let stream = TcpStream::connect(addr).await
            .context("Failed to connect to pool")?;
        
        *self.stream.write().await = Some(stream);
        
        self.start_reader().await;
        
        Ok(())
    }

    async fn start_reader(&self) {
        let stream_clone = self.stream.clone();
        let tx = self.tx.clone();

        tokio::spawn(async move {
            let mut stream_guard = stream_clone.write().await;
            if let Some(stream) = stream_guard.as_mut() {
                let (reader, _) = stream.split();
                let mut reader = BufReader::new(reader);
                let mut line = String::new();

                loop {
                    line.clear();
                    match reader.read_line(&mut line).await {
                        Ok(0) => {
                            tracing::warn!("Connection closed by server");
                            break;
                        }
                        Ok(_) => {
                            if let Ok(response) = serde_json::from_str::<StratumResponse>(&line) {
                                let _ = tx.send(response);
                            }
                        }
                        Err(e) => {
                            tracing::error!("Error reading from stream: {}", e);
                            break;
                        }
                    }
                }
            }
        });
    }

    pub async fn send_request(&self, method: &str, params: Vec<Value>) -> Result<u64> {
        let mut id_guard = self.request_id.write().await;
        let id = *id_guard;
        *id_guard += 1;
        drop(id_guard);

        let request = StratumRequest {
            id,
            method: method.to_string(),
            params,
        };

        let json = serde_json::to_string(&request)? + "\n";
        
        let mut stream_guard = self.stream.write().await;
        if let Some(stream) = stream_guard.as_mut() {
            stream.write_all(json.as_bytes()).await?;
            stream.flush().await?;
        }

        Ok(id)
    }

    pub async fn subscribe(&self, user_agent: &str) -> Result<u64> {
        self.send_request("mining.subscribe", vec![
            Value::String(user_agent.to_string()),
        ]).await
    }

    pub async fn authorize(&self, username: &str, password: &str) -> Result<u64> {
        self.send_request("mining.authorize", vec![
            Value::String(username.to_string()),
            Value::String(password.to_string()),
        ]).await
    }

    pub async fn submit(&self, worker: &str, job_id: &str, nonce: &str, result: &str) -> Result<u64> {
        self.send_request("mining.submit", vec![
            Value::String(worker.to_string()),
            Value::String(job_id.to_string()),
            Value::String(nonce.to_string()),
            Value::String(result.to_string()),
        ]).await
    }

    pub async fn receive(&self) -> Option<StratumResponse> {
        let mut rx = self.rx.write().await;
        rx.recv().await
    }
}
