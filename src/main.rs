// src/main.rs

mod analyzer;
mod github;
mod contract;

use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use std::env;
use eyre::Result;
use crate::github::GithubClient;
use crate::contract::StellarContractClient;

#[derive(Parser)]
#[command(name = "wave-ready")]
#[command(about = "Audit GitHub issues for Stellar SCF readiness", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan a repository for Stellar Community Fund (SCF) points
    Scan {
        #[arg(short, long)]
        owner: String,
        #[arg(short, long)]
        repo: String,
    },
    /// Sync suggested points to a Soroban rewards contract
    StellarSync {
        #[arg(short, long)]
        owner: String,
        #[arg(short, long)]
        repo: String,
        #[arg(short, long)]
        registry_address: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let cli = Cli::parse();

    let github_token = env::var("GITHUB_TOKEN")
        .expect("GITHUB_TOKEN must be set in .env file");
    
    let gh_client = GithubClient::new(github_token)?;

    match cli.command {
        Commands::Scan { owner, repo } => {
            println!("Scanning {}/{} for Stellar SCF readiness...", owner, repo);
            let issues = gh_client.fetch_issues(&owner, &repo).await?;
            
            println!("\n--- Suggested Points for Stellar Wave ---");
            for issue in issues {
                let points = analyzer::estimate_points(&issue);
                println!("[#{}] {}...", issue.number, &issue.title[..std::cmp::min(50, issue.title.len())]);
                println!("    Suggested Points: {}\n", points);
            }
        }
        Commands::StellarSync { owner, repo, registry_address: _ } => {
            let rpc_url = env::var("SOROBAN_RPC_URL").unwrap_or_else(|_| "https://soroban-testnet.stellar.org".to_string());
            let contract_client = StellarContractClient::new(&rpc_url)?;
            
            println!("Fetching issues to sync to Stellar...");
            let issues = gh_client.fetch_issues(&owner, &repo).await?;
            
            let mut total_points = 0;
            for issue in issues {
                total_points += analyzer::estimate_points(&issue);
            }

            println!("Calculated total Stellar Wave points: {}", total_points);
            contract_client.sync_to_stellar(&format!("{}/{}", owner, repo), total_points).await?;
            println!("Stellar Sync complete!");
        }
    }

    Ok(())
}
