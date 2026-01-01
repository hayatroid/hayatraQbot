use std::{collections::BTreeMap, sync::Arc};

use ac_library::ModInt998244353;
use clap::{Parser, Subcommand};
use itertools::Itertools;
use rand::{Rng, seq::IndexedRandom};
use serde::Deserialize;
use tokio::time::{Duration, timeout};
use traq_ws_bot::events::common::Message;

use crate::Resource;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone)]
enum Commands {
    /// Returns a random item from `items`.
    Choose {
        #[arg(required = true)]
        items: Vec<String>,
    },
    /// Returns the multiplicative inverse of `val`.
    Inv { val: i128 },
    /// Returns the sum of `items`.
    Add { items: Vec<i128> },
    /// Returns the product of `items`.
    Mul { items: Vec<i128> },
    /// Returns `base` to the power of `exp`.
    Pow { base: i128, exp: u64 },
    /// Returns the prime factors of `val`.
    Factorize { val: u64 },
    /// Returns the rating of AtCoder algo.
    Rating,
    /// Returns 𝑩𝑰𝑮 𝑮𝑼𝑬𝑼𝑬
    Gueue,
}

impl Cli {
    pub async fn run(&self, message: Arc<Message>, resource: Arc<Resource>) -> String {
        let cmd = self.command.clone();
        let mut handle = tokio::spawn(async { Self::execute(cmd, message, resource).await });
        match timeout(Duration::from_secs(10), &mut handle).await {
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
    async fn execute(cmd: Commands, message: Arc<Message>, resource: Arc<Resource>) -> String {
        match cmd {
            Commands::Choose { items } => {
                let mut rng = rand::rng();
                items.choose(&mut rng).unwrap().to_string()
            }
            Commands::Inv { val } => ModInt998244353::new(val).inv().to_string(),
            Commands::Add { items } => items
                .iter()
                .fold(ModInt998244353::new(0), |acc, &item| acc + item)
                .to_string(),
            Commands::Mul { items } => items
                .iter()
                .fold(ModInt998244353::new(1), |acc, &item| acc * item)
                .to_string(),
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
            Commands::Rating => {
                let url = format!("https://portfolio.trap.jp/api/v1/users/{}", message.user.id);
                let portfolio = reqwest::get(url)
                    .await
                    .unwrap()
                    .json::<Portfolio>()
                    .await
                    .unwrap();
                if let Some(account) = portfolio.accounts.iter().find(|x| x.r#type == 8) {
                    let url = format!(
                        "https://atcoder-badges.now.sh/api/atcoder/json/{}",
                        account.display_name
                    );
                    let badge = reqwest::get(url)
                        .await
                        .unwrap()
                        .json::<Badge>()
                        .await
                        .unwrap();
                    format!("$\\color{{{}}}\\textbf{{{}}}$", badge.color, badge.message)
                } else {
                    "[traPortfolio に AtCoder ID を紐づけなさ～～～い！！！](https://portfolio-admin.trap.jp/user/accounts)".to_string()
                }
            }
            Commands::Gueue => {
                let priority = {
                    let mut rng = rand::rng();
                    rng.random::<u8>()
                };
                resource.gueue.lock().await.push((
                    priority,
                    message.user.name.clone(),
                    message.id.clone(),
                ));
                format!(
                    "優先度付きぎゅーに ({}, :@{}:) を挿入しました！",
                    priority, message.user.name
                )
            }
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Portfolio {
    accounts: Vec<PortfolioAccount>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PortfolioAccount {
    display_name: String,
    r#type: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Badge {
    message: String,
    color: String,
}
