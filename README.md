# Wave-Ready Repository Linter 🌊🌌

The **Wave-Ready Repository Linter** is a high-performance Rust tool designed to help developers and maintainers in the **Stellar** and **Soroban** ecosystems automate their "Scoping" phase. It audits GitHub issues to suggest reward points based on technical complexity and ecosystem impact, specifically aligned with the **Stellar Community Fund (SCF)** framework.

---

## 🏗️ Project Scaffold

The project is structured for high performance, utilizing async Rust to handle concurrent API requests and blockchain interactions.

```text
wave-ready-linter/
├── .github/
│   └── workflows/
│       └── wave-check.yml      # CI/CD for automatic SCF audits
├── src/
│   ├── main.rs                 # CLI entry point and Command orchestration
│   ├── analyzer.rs             # Core "Brain": Point estimation logic
│   ├── github.rs               # Octocrab implementation for GitHub API
│   └── contract.rs             # Stellar/Soroban RPC client for on-chain sync
├── Cargo.toml                  # Dependencies: clap, octocrab, stellar-rpc, alloy
├── .env.example                # Configuration for Stellar & GitHub tokens
└── README.md                   # You are here!
```

---

## 🧠 The Scoring Brain (`analyzer.rs`)

At the heart of the linter is the `estimate_points` function. It evaluates GitHub issues based on labels and community engagement to determine their value in a "Wave" (funding cycle).

### Key Indicators:
- **Soroban Integration**: Issues tagged with `soroban` or `smart-contract` are prioritized (30 points).
- **Ecosystem Alignment**: `SEP` (Stellar Ecosystem Proposals) implementation carries the highest weight (40 points).
- **SCF Milestones**: Labels like `scf-milestone` flag critical path items (+20 points).
- **Community Discussion**: High comment counts (>10) increase the score to reflect architectural complexity.

### Logic Snippet:
```rust
pub fn estimate_points(issue: &Issue) -> u64 {
    let mut score = 10; // Base score

    let labels: Vec<String> = issue.labels.iter().map(|l| l.to_lowercase()).collect();

    if labels.contains(&"soroban".to_string()) {
        score = 30; // Priority for Soroban work
    }
    
    if labels.contains(&"sep".to_string()) {
        score = 40; // Critical for ecosystem growth
    }

    if issue.comment_count > 10 {
        score += 10; // Complexity weight
    }

    score
}
```

---

## 🚀 Getting Started

### 1. Prerequisites
Ensure you have the Rust toolchain installed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Configuration
Create a `.env` file based on the example:
```bash
cp .env.example .env
```
Fill in your `GITHUB_TOKEN` and `SOROBAN_RPC_URL` (usually `https://soroban-testnet.stellar.org`).

### 3. Usage

#### **Scan for Points**
Audit a repository to see suggested points for open issues:
```bash
cargo run -- scan --owner stellar --repo soroban-sdk
```

#### **Sync to Stellar**
Push the calculated scores to a Soroban-based rewards registry:
```bash
cargo run -- stellar-sync --owner stellar --repo your-repo --registry-address [CONTRACT_ID]
```

---

## 🛠️ Built With
- **[Octocrab](https://github.com/XAMPPRocky/octocrab)**: A modern Rust library for the GitHub API.
- **[Stellar/Soroban RPC](https://github.com/stellar/rs-stellar-rpc-client)**: For trustless on-chain synchronization.
- **[Clap](https://github.com/clap-rs/clap)**: For a robust and intuitive CLI.
- **[Tokio](https://github.com/tokio-rs/tokio)**: The industry-standard async runtime for Rust.

---

## 🌊 How it helps Dapps
1. **Consistency**: Ensures similar tasks have similar rewards across different Stellar projects.
2. **Transparency**: Syncing scores directly to Soroban contracts ensures contributors can trust the payout logic.
3. **Speed**: Prepare 50+ issues for an SCF tranche in seconds.
