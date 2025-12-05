use ac_library::ModInt998244353;
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
    Inv { val: u32 },
}

impl Cli {
    pub fn run(&self) -> String {
        match self.command {
            Commands::Ping => "pong".to_string(),
            Commands::Inv { val } => ModInt998244353::new(val).inv().to_string(),
        }
    }
}
