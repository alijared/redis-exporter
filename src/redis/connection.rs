use redis::aio::MultiplexedConnection;
use redis::{Client, RedisResult};
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Config {
    pub name: Option<String>,
    pub url: String,
    pub response_timeout_ms: u64,
    pub connection_timeout_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: None,
            url: "redis://127.0.0.1:6379".to_string(),
            response_timeout_ms: 5000,
            connection_timeout_ms: 5000,
        }
    }
}

pub async fn get_connection(config: Config) -> RedisResult<MultiplexedConnection> {
    let client = Client::open(config.url.clone())?;
    client
        .get_multiplexed_tokio_connection_with_response_timeouts(
            Duration::from_millis(config.response_timeout_ms),
            Duration::from_millis(config.connection_timeout_ms),
        )
        .await
}
