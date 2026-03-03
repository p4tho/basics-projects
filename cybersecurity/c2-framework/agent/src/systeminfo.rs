use serde::{
    Serialize
};
use sysinfo::{
    System, 
    RefreshKind, 
    CpuRefreshKind
};

#[derive(Debug, Serialize)]
pub struct SystemInfo {
    pub system_name: String,
    pub hostname: String,
    pub os: String,
    pub os_version: String,
    pub kernel_version: String,
    pub cpu: String,
}

pub fn get_system_info() -> SystemInfo {
    let sys = System::new_with_specifics(
        RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
    );

    SystemInfo {
        system_name: System::name().unwrap(),
        hostname: System::host_name().unwrap(),
        os: std::env::consts::OS.to_string(),
        os_version: System::os_version().unwrap(),
        kernel_version: System::kernel_version().unwrap(),
        cpu: sys.cpus()[0].brand().to_string(),
    }
}