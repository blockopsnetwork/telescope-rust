use opentelemetry::metrics::{Counter, Meter, ObservableGauge, Unit};
use opentelemetry::KeyValue;
use sysinfo::{Cpu, Disk, System};

pub struct NodeMetrics {
    pub sys: System,
    pub cpu_usage: ObservableGauge<f64>,
    pub memory_usage: ObservableGauge<f64>,
    pub disk_read_counter: Counter<u64>,
    pub disk_write_counter: Counter<u64>,
}

impl NodeMetrics {
    pub fn new(meter: Meter) -> Self {
        let sys = System::new_all();

        let cpu_usage = meter
            .f64_observable_gauge("node_cpu_usage")
            .with_description("CPU usage in percentage")
            .with_unit(Unit::new("%"))
            .init();

        let memory_usage = meter
            .f64_observable_gauge("node_memory_usage")
            .with_description("Memory usage in percentage")
            .with_unit(Unit::new("%"))
            .init();

        let disk_read_counter = meter
            .u64_counter("node_disk_read_bytes")
            .with_description("Total bytes read from disk")
            .with_unit(Unit::new("bytes"))
            .init();

        let disk_write_counter = meter
            .u64_counter("node_disk_write_bytes")
            .with_description("Total bytes written to disk")
            .with_unit(Unit::new("bytes"))
            .init();

        Self {
            sys,
            cpu_usage,
            memory_usage,
            disk_read_counter,
            disk_write_counter,
        }
    }

    pub fn collect_metrics(&mut self) {
        self.sys.refresh_all();

        // CPU Usage
        for (index, cpu) in self.sys.cpus().iter().enumerate() {
            self.cpu_usage.observe(cpu.cpu_usage() as f64, &[KeyValue::new("cpu", index.to_string())]);
        }

        // Memory Usage
        let total_memory = self.sys.total_memory();
        let used_memory = self.sys.used_memory();
        let memory_usage = (used_memory as f64 / total_memory as f64) * 100.0;
        self.memory_usage.observe(memory_usage, &[]);

        // Disk I/O
        for disk in self.sys.disks() {
            self.disk_read_counter.add(
                disk.total_read_bytes(),
                &[KeyValue::new("device", disk.name().to_string_lossy().into_owned())],
            );
            self.disk_write_counter.add(
                disk.total_written_bytes(),
                &[KeyValue::new("device", disk.name().to_string_lossy().into_owned())],
            );
        }
    }
}
