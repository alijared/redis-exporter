use axum::routing::get;
use axum::Router;
use log::info;
use prometheus::{Encoder, TextEncoder};
use thiserror::Error;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unable to listen on specified address: {0}")]
    Listen(std::io::Error),

    #[error("Serve error: {0}")]
    Serve(std::io::Error),
}

pub async fn start(port: u32, cancellation_token: CancellationToken) -> Result<(), Error> {
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(addr.clone())
        .await
        .map_err(Error::Listen)?;

    let router = Router::new().route("/metrics", get(serve_metrics));

    info!("Prometheus server started at http://{}", addr);
    axum::serve(listener, router)
        .with_graceful_shutdown(async move {
            cancellation_token.cancelled().await;
        })
        .await
        .map_err(Error::Serve)
}

async fn serve_metrics() -> String {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    String::from_utf8(buffer).unwrap()
}
