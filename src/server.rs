use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;
use prometheus::{Encoder, TextEncoder};
use opentelemetry_prometheus::PrometheusExporter;

pub async fn run_server(metrics: Arc<Mutex<super::metrics::NodeMetrics>>, port: u16) {
    let prometheus_exporter = opentelemetry_prometheus::exporter()
        .build()
        .expect("Failed to build Prometheus exporter");

    let metrics_route = warp::path!("metrics").map(move || {
        let encoder = TextEncoder::new();
        let metric_families = prometheus_exporter.gather(); // Use gather directly
        let mut buffer = vec![];
        encoder.encode(&metric_families, &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    });

    println!("Serving metrics on port {}", port);
    warp::serve(metrics_route).run(([0, 0, 0, 0], port)).await;
}
