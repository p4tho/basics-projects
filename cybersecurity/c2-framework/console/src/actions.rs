use crate::constants::{
    SERVER_URL
};
use reqwest::{
    Client
};
use serde_json::json;

pub async fn send_shell_command(agent_id: u32, cmd: String) -> () {
    let client = Client::new();
    let addcommand_url = format!("{}/addcommand", SERVER_URL);

    client
        .post(&addcommand_url)
        .json(&json!({ 
            "agent_id": agent_id,
            "command_type": 0,
            "cmd": cmd,
        }))
        .send()
        .await;
}