use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "@BOT_hayatroid", no_binary_name = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Ping,
}

impl Cli {
    pub fn run(&self) -> String {
        match self.command {
            Commands::Ping => "pong".to_string(),
        }
    }
}
