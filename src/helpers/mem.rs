use prometheus::Registry;
use sysinfo::System;

pub struct MemoryMetrics {
    memory_total_gauge: prometheus::Gauge,
    memory_used_gauge: prometheus::Gauge,
    memory_free_gauge: prometheus::Gauge,
    swap_total_gauge: prometheus::Gauge,
    swap_used_gauge: prometheus::Gauge,
}

impl MemoryMetrics {
    pub fn new(registry: &Registry) -> Result<Self, prometheus::Error> {
        let memory_total_gauge = prometheus::Gauge::new(
            "node_memory_total_bytes",
            "Total memory in bytes",
        )?;
        let memory_used_gauge = prometheus::Gauge::new(
            "node_memory_used_bytes",
            "Used memory in bytes",
        )?;
        let memory_free_gauge = prometheus::Gauge::new(
            "node_memory_free_bytes",
            "Free memory in bytes",
        )?;
        let swap_total_gauge = prometheus::Gauge::new(
            "node_swap_total_bytes",
            "Total swap in bytes",
        )?;
        let swap_used_gauge = prometheus::Gauge::new(
            "node_swap_used_bytes",
            "Used swap in bytes",
        )?;

        registry.register(Box::new(memory_total_gauge.clone()))?;
        registry.register(Box::new(memory_used_gauge.clone()))?;
        registry.register(Box::new(memory_free_gauge.clone()))?;
        registry.register(Box::new(swap_total_gauge.clone()))?;
        registry.register(Box::new(swap_used_gauge.clone()))?;

        Ok(Self {
            memory_total_gauge,
            memory_used_gauge,
            memory_free_gauge,
            swap_total_gauge,
            swap_used_gauge,
        })
    }

    pub fn collect(&self, system: &System) {
        self.memory_total_gauge.set(system.total_memory() as f64);
        self.memory_used_gauge.set(system.used_memory() as f64);
        self.memory_free_gauge.set(system.free_memory() as f64);
        self.swap_total_gauge.set(system.total_swap() as f64);
        self.swap_used_gauge.set(system.used_swap() as f64);
    }
}