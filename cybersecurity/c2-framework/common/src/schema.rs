use serde::{
    Deserialize, Serialize
};
use tabled::{
    Tabled
};

#[derive(Debug, Deserialize, Tabled)]
pub struct Agent {
    pub id: u32,
    pub name: String,
    pub system_name: String,
    pub hostname: String,
    pub os: String,
    pub os_version: String,
    pub kernel_version: String,
    pub cpu: String
}

#[derive(Debug, Deserialize, Tabled)]
pub struct Command {
    pub id: u32,
    pub agent_id: u32,
    pub command_type: i32,
    pub cmd: String,
    pub status: String,
    pub result: String,
}

#[derive(Debug, Serialize)]
pub struct CommandResult {
    pub command_id: u32,
    pub result: String,
}
