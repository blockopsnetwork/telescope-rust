// use prometheus_exporter::{self, prometheus};
// use prometheus::{register_counter, register_gauge};
// use sysinfo::{System, Disks};
// use std::time::Duration;

// fn main() {
//     // Start Prometheus exporter on port 9100
//     let binding = "127.0.0.1:9100".parse().unwrap();
//     let exporter = prometheus_exporter::start(binding).expect("Failed to start Prometheus exporter");

//     // Register metrics
//     let cpu_usage_gauge = register_gauge!("node_cpu_usage", "CPU usage in percentage").unwrap();
//     let memory_usage_gauge = register_gauge!("node_memory_usage", "Memory usage in percentage").unwrap();
//     let disk_read_counter = register_counter!("node_disk_read_bytes", "Total bytes read from disk").unwrap();
//     let disk_write_counter = register_counter!("node_disk_write_bytes", "Total bytes written to disk").unwrap();

//     // Create a system instance
//     let mut system = System::new_all();
//     let mut disks = Disks::new_with_refreshed_list();

//     // Periodically update metrics
//     loop {
        
//         let guard = exporter.wait_duration(Duration::from_secs(1));

//         // Refresh system information
//         system.refresh_all();

//         // Update CPU usage (average across all CPUs)
//         system.refresh_cpu_usage();
//         let total_cpu_usage: f64 = system.cpus().iter().map(|cpu| cpu.cpu_usage() as f64).sum();
//         cpu_usage_gauge.set(total_cpu_usage);

//         // Update memory usage
//         let total_memory = system.total_memory() as f64;
//         let used_memory = system.used_memory() as f64;
//         let memory_usage = (used_memory / total_memory) * 100.0;
//         memory_usage_gauge.set(memory_usage);

//         // Update disk metrics
//         for disk in disks.list_mut() {
//             disk.refresh(); // Refresh disk information
//             disk_read_counter.inc_by(disk.total_space() as f64);
//             disk_write_counter.inc_by(disk.available_space() as f64);
//         }

//         // Drop the guard to allow HTTP responses
//         drop(guard);
//     }
// }

use prometheus_exporter::{self, prometheus};
use std::time::Duration;

mod helpers;
mod exporters;
use helpers::{cpu, mem, disk, temp, gpu};
use exporters::metrics_server;

#[derive(Default)]
pub struct MetricsConfig {
    port: u16,
    host: String,
    collection_interval: Duration,
    enable_gpu: bool,
}

impl MetricsConfig {
    pub fn new(port: u16, host: String, collection_interval: Duration, enable_gpu: bool) -> Self {
        Self {
            port,
            host,
            collection_interval,
            enable_gpu,
        }
    }
}

fn main() {
    // Initialize configuration
    let config = MetricsConfig::new(
        9100,
        "127.0.0.1".to_string(),
        Duration::from_secs(1),
        true,
    );

    // Initialize metrics from helpers
    let cpu_metrics = cpu::initialize_metrics().expect("Failed to initialize CPU metrics");
    let mem_metrics = mem::initialize_metrics().expect("Failed to initialize memory metrics");
    let disk_metrics = disk::initialize_metrics().expect("Failed to initialize disk metrics");
    let temp_metrics = temp::initialize_metrics().expect("Failed to initialize temperature metrics");
    let gpu_metrics = gpu::initialize_metrics().expect("Failed to initialize GPU metrics");

    // Start the metrics server
    let mut server = metrics_server::MetricsServer::new(config);
    server.start().expect("Failed to start metrics server");
}