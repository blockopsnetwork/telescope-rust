mod exporters;
mod config;
mod helpers;

use exporters::metrics_server::MetricsServer;
use crate::config::MetricsConfig;

#[tokio::main]
async fn main() {
    let config = MetricsConfig::default();

    let metrics_server = MetricsServer::new(config);

    if let Err(e) = metrics_server.start().await {
        eprintln!("Failed to start metrics server: {}", e);
    }
}
