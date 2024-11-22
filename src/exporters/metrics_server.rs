

use prometheus::{Registry, Encoder, TextEncoder};
use crate::MetricsConfig;
use crate::helpers::{cpu::CpuMetrics, mem::MemoryMetrics, disk::DiskMetrics};
use sysinfo::{System, Disks};
use std::sync::{Arc, Mutex};
use warp::Filter;

pub struct MetricsServer {
    config: MetricsConfig,
    registry: Arc<Registry>,
    system: Arc<Mutex<System>>,
    disks: Arc<Mutex<Disks>>,
}

impl MetricsServer {
    pub fn new(config: MetricsConfig) -> Self {
        Self {
            config,
            registry: Arc::new(Registry::new()),
            system: Arc::new(Mutex::new(System::new_all())),
            disks: Arc::new(Mutex::new(Disks::new_with_refreshed_list())),
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
    let registry = Arc::clone(&self.registry);
    let system = Arc::clone(&self.system);
    let disks = Arc::clone(&self.disks);

    let cpu_metrics = CpuMetrics::new(&registry)?;
    let memory_metrics = MemoryMetrics::new(&registry)?;
    let mut disk_metrics = DiskMetrics::new(&registry)?;

    let collection_interval = self.config.collection_interval;

    tokio::spawn(async move {
        loop {
            {
                let mut system = system.lock().unwrap();
                let mut disks = disks.lock().unwrap();

                system.refresh_all();

                cpu_metrics.collect(&system);
                memory_metrics.collect(&system);
                disk_metrics.collect(&mut disks);
            }

            tokio::time::sleep(collection_interval).await;
        }
    });

    let metrics_route = warp::path!("metrics").map(move || {
        let metric_families = registry.gather();
        let mut buffer = Vec::new();
        let encoder = TextEncoder::new();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    });

    warp::serve(metrics_route).run(self.config.address).await; // Use address directly

    Ok(())
}
}
