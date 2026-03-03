mod actions;
mod constants;
mod info;

use crate::actions::{
    send_shell_command
};
use crate::constants::{
    HELP_MENU
};
use crate::info::{
    get_agent_info, get_commands
};
use common::schema::{
    Agent
};
use std::io;
use std::io::{
    Write
};
use std::process::{
    exit
};

// Split operater input arguments
pub fn split_input(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for c in input.trim().chars() {
        match c {
            '"' => {
                in_quotes = !in_quotes;
            }
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    args.push(current.clone());
                    current.clear();
                }
            }
            _ => current.push(c),
        }
    }

    if !current.is_empty() {
        args.push(current);
    }

    args
}

// Parse arguments and execute command through C2 server API
pub async fn execute_command(command: &[String]) -> () {
    if let Some(action) = command.first() {
        match action.as_str() {
            "help" =>
                println!("{}", HELP_MENU),
            "exit" =>
                exit(1),
            "agents" => {
                if let Err(e) = get_agent_info().await {
                    eprintln!("Failed to get agents: {e}");
                }
            },
            "commands" => {
                if let Err(e) = get_commands().await {
                    eprintln!("Failed to get commands: {e}");
                }
            }
            "send" => {
                if command.len() != 3 {
                    println!("send usage: send <agent id> <shell command>")
                }

                let agent_id = match command[1].parse::<u32>() {
                    Ok(id) => id,
                    Err(_) => {
                        eprintln!("agent_id must be a valid u32");
                        return;
                    }
                };
                let shell_command = command[2].clone();

                send_shell_command(agent_id, shell_command).await
            },
            _ => println!("{} command does not exist", action)
        }
    }
}

#[tokio::main]
async fn main() {
    let mut input = String::new();
    let mut command: Vec<String>;

    loop {
        input.clear();
        print!("kakerou > ");
        io::stdout().flush().unwrap();

        // Get operater command
        io::stdin().read_line(&mut input).expect("Failed to read line");
        command = split_input(&input);

        // Run command
        execute_command(&command).await;
    }
}
