use sysinfo::System;
use prometheus::{Registry, GaugeVec};

pub struct CpuMetrics {
    cpu_usage_gauge: GaugeVec,
    cpu_frequency_gauge: GaugeVec,
}

impl CpuMetrics {
    pub fn new(registry: &Registry) -> Result<Self, prometheus::Error> {
        let cpu_usage_gauge = GaugeVec::new(
            prometheus::Opts::new("node_cpu_usage", "CPU usage percentage per core"),
            &["cpu"],
        )?;
        let cpu_frequency_gauge = GaugeVec::new(
            prometheus::Opts::new("node_cpu_frequency_mhz", "CPU frequency in MHz per core"),
            &["cpu"],
        )?;

        registry.register(Box::new(cpu_usage_gauge.clone()))?;
        registry.register(Box::new(cpu_frequency_gauge.clone()))?;

        Ok(Self {
            cpu_usage_gauge,
            cpu_frequency_gauge,
        })
    }

    pub fn collect(&self, system: &System) {
        for (i, cpu) in system.cpus().iter().enumerate() {
            let cpu_id = i.to_string();
            self.cpu_usage_gauge
                .with_label_values(&[&cpu_id])
                .set(cpu.cpu_usage() as f64);
            self.cpu_frequency_gauge
                .with_label_values(&[&cpu_id])
                .set(cpu.frequency() as f64);
        }
    }

}
