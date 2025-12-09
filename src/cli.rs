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
    /// Returns the multiplicative inverse of `val`.
    Inv { val: u64 },
    /// Returns `lhs` plus `rhs`.
    Add { lhs: u64, rhs: u64 },
    /// Returns `lhs` minus `rhs`.
    Sub { lhs: u64, rhs: u64 },
    /// Returns `lhs` multiplied by `rhs`.
    Mul { lhs: u64, rhs: u64 },
    /// Returns `lhs` divided by `rhs`.
    Div { lhs: u64, rhs: u64 },
    /// Returns `base` to the power of `exp`.
    Pow { base: u64, exp: u64 },
}

impl Cli {
    pub fn run(&self) -> String {
        match self.command {
            Commands::Inv { val } => ModInt998244353::new(val).inv().to_string(),
            Commands::Add { lhs, rhs } => (ModInt998244353::new(lhs) + rhs).to_string(),
            Commands::Sub { lhs, rhs } => (ModInt998244353::new(lhs) - rhs).to_string(),
            Commands::Mul { lhs, rhs } => (ModInt998244353::new(lhs) * rhs).to_string(),
            Commands::Div { lhs, rhs } => (ModInt998244353::new(lhs) / rhs).to_string(),
            Commands::Pow { base, exp } => ModInt998244353::new(base).pow(exp).to_string(),
        }
    }
}
