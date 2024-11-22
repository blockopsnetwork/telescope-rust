use prometheus::Registry;
use sysinfo::Disks;

pub struct DiskMetrics {
    disk_total_bytes: prometheus::GaugeVec,
    disk_free_bytes: prometheus::GaugeVec,
    disk_read_bytes: prometheus::CounterVec,
    disk_write_bytes: prometheus::CounterVec,
}

impl DiskMetrics {
    pub fn new(registry: &Registry) -> Result<Self, prometheus::Error> {
        let disk_total_bytes = prometheus::GaugeVec::new(
            prometheus::Opts::new("node_disk_total_bytes", "Total disk space in bytes"),
            &["device"],
        )?;
        let disk_free_bytes = prometheus::GaugeVec::new(
            prometheus::Opts::new("node_disk_free_bytes", "Free disk space in bytes"),
            &["device"],
        )?;
        let disk_read_bytes = prometheus::CounterVec::new(
            prometheus::Opts::new("node_disk_read_bytes_total", "Total bytes read from disk"),
            &["device"],
        )?;
        let disk_write_bytes = prometheus::CounterVec::new(
            prometheus::Opts::new("node_disk_write_bytes_total", "Total bytes written to disk"),
            &["device"],
        )?;

        registry.register(Box::new(disk_total_bytes.clone()))?;
        registry.register(Box::new(disk_free_bytes.clone()))?;
        registry.register(Box::new(disk_read_bytes.clone()))?;
        registry.register(Box::new(disk_write_bytes.clone()))?;

        Ok(Self {
            disk_total_bytes,
            disk_free_bytes,
            disk_read_bytes,
            disk_write_bytes,
        })
    }

    pub fn collect(&mut self, disks: &mut Disks) {
        for disk in disks.list_mut() {
            
            disk.refresh();
            let device_name = disk.name().to_string_lossy();
            
            self.disk_total_bytes
                .with_label_values(&[&device_name])
                .set(disk.total_space() as f64);
            self.disk_free_bytes
                .with_label_values(&[&device_name])
                .set(disk.available_space() as f64);
            self.disk_read_bytes
                .with_label_values(&[&device_name])
                .inc_by(disk.total_space() as f64);
            self.disk_write_bytes
                .with_label_values(&[&device_name])
                .inc_by(disk.available_space() as f64);
        }
    }

    pub fn initialize_metrics(registry: &Registry) -> Result<Self, prometheus::Error> {
        Self::new(registry)
    }
}