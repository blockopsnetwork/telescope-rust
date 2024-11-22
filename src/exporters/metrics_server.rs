use prometheus::Registry;
use crate::MetricsConfig;
use crate::helpers::{cpu::CpuMetrics, mem::MemoryMetrics, disk::DiskMetrics};
use sysinfo::{System, Disks};

pub struct MetricsServer {
    config: MetricsConfig,
    registry: Registry,
    system: System,
    disks: Disks,
}

impl MetricsServer {
    pub fn new(config: MetricsConfig) -> Self {
        Self {
            config,
            registry: Registry::new(),
            system: System::new_all(),
            disks: Disks::new_with_refreshed_list(),
        }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let binding = format!("{}:{}", self.config.host, self.config.port).parse()?;
        let exporter = prometheus_exporter::start(binding)?;

        // Initialize metrics collectors
        let cpu_metrics = CpuMetrics::new(&self.registry)?;
        let memory_metrics = MemoryMetrics::new(&self.registry)?;
        let mut disk_metrics = DiskMetrics::new(&self.registry)?;

        loop {
            let guard = exporter.wait_duration(self.config.collection_interval);

            // Refresh system information
            self.system.refresh_all();

            // Collect metrics
            cpu_metrics.collect(&self.system);
            memory_metrics.collect(&self.system);
            disk_metrics.collect(&mut self.disks);

            drop(guard);
        }
    }
}