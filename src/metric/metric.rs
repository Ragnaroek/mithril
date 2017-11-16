
#[derive(Clone)]
pub struct MetricConfig {
    pub enabled: bool,
    pub resolution: u64,
    pub sample_interval_seconds: u64,
    pub report_file: String
}
