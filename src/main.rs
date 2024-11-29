use clap::Parser;
use futures_util::future;
use log::{debug, error, info};
use std::time::Duration;
use tokio::signal;
use tokio_util::sync::CancellationToken;

mod config;
mod redis;
mod prometheus;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[arg(short, long, default_value = "config.yaml")]
    config_filepath: String,

    #[arg(long, default_value_t = log::LevelFilter::Info)]
    log_level: log::LevelFilter,
}

#[tokio::main]
async fn main() {
    let cli_args = CliArgs::parse();
    env_logger::builder()
        .filter_level(cli_args.log_level)
        .init();

    if let Err(e) = run(cli_args).await {
        error!("{}", e);
    }
}

async fn run(args: CliArgs) -> Result<(), anyhow::Error> {
    let config = config::load(&args.config_filepath)?;
    let cancellation_token = CancellationToken::new();

    shutdown(cancellation_token.clone());

    let mut futures = Vec::with_capacity(config.targets.len());
    for target in config.targets {
        let conn = redis::connection::get_connection(target.clone()).await?;
        let target_url = target.url.clone();
        let target_name = target.name.unwrap_or(target_url.clone());
        let collector = redis::metrics::Collector::new(conn, &target.url, &target_name)?;

        let cancel = cancellation_token.clone();
        futures.push(tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(config.collect_interval));
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        debug!("Collecting metrics for {} ({})", &target_name, &target_url);
                        if let Err(e) = collector.collect().await {
                            error!(
                                "Error collecting metrics for target {}: {}",
                                &target_name, e
                            )
                        }
                }
                _ = cancel.cancelled() => {
                        info!("Shutting down collector for target {}", &target_name);
                        break;
                    }
                }
            }
        }));
    }


    let prom_handler = tokio::spawn(async move {
        info!("Starting prometheus webserver");
        prometheus::server::start(config.prometheus_port, cancellation_token.clone()).await
    });

    future::join_all(futures).await;
    let _ = prom_handler.await?;

    Ok(())
}

fn shutdown(cancellation_token: CancellationToken) {
    tokio::spawn(async move {
        let mut terminate = signal::unix::signal(signal::unix::SignalKind::terminate()).unwrap();
        tokio::select! {
            _ = signal::ctrl_c() => {},
            _ = terminate.recv() => {},
        }
        info!("Received shutdown signal");
        cancellation_token.cancel();
    });
}
