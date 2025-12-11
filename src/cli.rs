use std::collections::BTreeMap;

use ac_library::ModInt998244353;
use clap::{Parser, Subcommand};
use itertools::Itertools;
use tokio::time::{Duration, timeout};

#[derive(Parser)]
#[command(name = "@BOT_hayatroid", no_binary_name = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone)]
enum Commands {
    /// Returns the multiplicative inverse of `val`.
    Inv { val: i128 },
    /// Returns `lhs` plus `rhs`.
    Add { lhs: i128, rhs: i128 },
    /// Returns `lhs` minus `rhs`.
    Sub { lhs: i128, rhs: i128 },
    /// Returns `lhs` multiplied by `rhs`.
    Mul { lhs: i128, rhs: i128 },
    /// Returns `lhs` divided by `rhs`.
    Div { lhs: i128, rhs: i128 },
    /// Returns `base` to the power of `exp`.
    Pow { base: i128, exp: u64 },
    /// Returns the prime factors of `val`.
    Factorize { val: u64 },
}

impl Cli {
    pub async fn run(&self) -> String {
        let cmd = self.command.clone();
        let mut handle = tokio::spawn(async { Self::execute(cmd).await });
        match timeout(Duration::from_secs(2), &mut handle).await {
            Ok(join_res) => match join_res {
                Ok(res) => res,
                Err(e) => {
                    if e.is_panic() {
                        let e = e.into_panic();
                        if let Some(msg) = e.downcast_ref::<&str>() {
                            return format!("```txt\n{}\n```", msg.trim());
                        }
                        if let Some(msg) = e.downcast_ref::<String>() {
                            return format!("```txt\n{}\n```", msg.trim());
                        }
                    }
                    ":internal_error:".to_string()
                }
            },
            Err(_) => {
                handle.abort();
                ":time_limit_exceeded:".to_string()
            }
        }
    }
    async fn execute(cmd: Commands) -> String {
        match cmd {
            Commands::Inv { val } => ModInt998244353::new(val).inv().to_string(),
            Commands::Add { lhs, rhs } => (ModInt998244353::new(lhs) + rhs).to_string(),
            Commands::Sub { lhs, rhs } => (ModInt998244353::new(lhs) - rhs).to_string(),
            Commands::Mul { lhs, rhs } => (ModInt998244353::new(lhs) * rhs).to_string(),
            Commands::Div { lhs, rhs } => (ModInt998244353::new(lhs) / rhs).to_string(),
            Commands::Pow { base, exp } => ModInt998244353::new(base).pow(exp).to_string(),
            Commands::Factorize { val } => {
                let mut res = BTreeMap::new();
                let mut tmp = val;
                for i in (2..).take_while(|i| i * i <= val) {
                    tokio::task::yield_now().await;
                    while tmp % i == 0 {
                        *res.entry(i).or_insert(0) += 1;
                        tmp /= i;
                    }
                }
                if tmp > 1 {
                    *res.entry(tmp).or_insert(0) += 1;
                }
                let res = res
                    .iter()
                    .map(|(base, exp)| format!("{}^{{{}}}", base, exp))
                    .join(" \\times ");
                format!("${}$", res)
            }
        }
    }
}
