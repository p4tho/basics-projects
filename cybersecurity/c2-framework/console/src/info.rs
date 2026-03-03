use common::schema::{
    Agent, Command
};
use crate::constants::{
    SERVER_URL
};
use reqwest::{
    Client
};
use tabled::{
    Table
};

pub async fn get_agent_info() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let getagents_url = format!("{}/get/allagents", SERVER_URL);

    let agents: Vec<Agent> = client
        .get(&getagents_url)
        .send()
        .await?
        .json()
        .await?;

    let table = Table::new(agents);
    println!("{table}");

    Ok(())
}

pub async fn get_commands() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let getcommands_url = format!("{}/get/allcommands", SERVER_URL);

    let commands: Vec<Command> = client
        .get(&getcommands_url)
        .send()
        .await?
        .json()
        .await?;

    let table = Table::new(commands);
    println!("{table}");

    Ok(())
}