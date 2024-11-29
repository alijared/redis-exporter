use crate::redis;
use serde::Deserialize;
use std::fs::File;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_prometheus_port")]
    pub prometheus_port: u32,
    pub collect_interval: u64,

    #[serde(default)]
    pub targets: Vec<redis::connection::Config>,
}

fn default_prometheus_port() -> u32 {
    9090
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unable to read config file: {0}")]
    IO(std::io::Error),

    #[error("Unable to deserialize config file: {0}")]
    Parse(serde_yml::Error),
}

pub fn load(filepath: &str) -> Result<Config, Error> {
    let file = File::open(filepath).map_err(Error::IO)?;
    serde_yml::from_reader(file).map_err(Error::Parse)
}
