#[derive(Debug)]
pub struct AgentConfig {
    pub name: String,
    pub server_url: String,
    pub beacon_duration_sec: u64,
    pub duration_range_sec: u64
}
