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
    /// Retruns the multiplicative inverse of `self`.
    Inv { val: u32 },
    /// Returns `self` to the power of `n`.
    Pow { val: u32, n: u64 },
}

impl Cli {
    pub fn run(&self) -> String {
        match self.command {
            Commands::Inv { val } => ModInt998244353::new(val).inv().to_string(),
            Commands::Pow { val, n } => ModInt998244353::new(val).pow(n).to_string(),
        }
    }
}
