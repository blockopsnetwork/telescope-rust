use std::net::SocketAddr;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct MetricsConfig {
    pub address: SocketAddr,
    pub collection_interval: Duration,
}
//edit this to run on different prot
//TODO: move configs to env
impl MetricsConfig {
    pub fn default() -> Self {
        MetricsConfig {
            address: "127.0.0.1:9090".parse().unwrap(),
            collection_interval: Duration::from_secs(5),
        }
    }
}
