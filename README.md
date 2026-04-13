# Wave-Ready Repository Linter 🌊 (Stellar Edition)

Automate your **Stellar Community Fund (SCF)** and **Soroban** milestone readiness. This tool scans your GitHub backlog, suggests Point values based on Stellar ecosystem standards, and syncs them to Soroban-based registries.

## Features
- **Stellar Native**: Built for developers working on Soroban and Stellar Ecosystem Proposals (SEPs).
- **SCF Integration**: Specifically tuned for Stellar Community Fund tranches and milestones.
- **Fast & Safe**: Rust-powered linter utilizing `stellar-rpc-client` and `octocrab`.

## Setup
1. Ensure you have [Rust](https://rustup.rs/) installed.
2. Clone the repo.
3. Create a `.env` file:
   ```bash
   cp .env.example .env
   ```
4. Add your `GITHUB_TOKEN` and `SOROBAN_RPC_URL` to `.env`.

## Usage
### Scan for Stellar Points
```bash
cargo run -- scan --owner stellar --repo soroban-sdk
```

### Sync to Stellar/Soroban
```bash
cargo run -- stellar-sync --owner stellar --repo soroban-sdk --registry-address [CONTRACT_ID]
```

## Scoring Criteria
- **Soroban Integration**: 30 points per issue.
- **SEP Implementation**: 40 points (Ecosystem priority).
- **SCF Milestones**: +20 points for critical delivery artifacts.
- **Bug/Enhancement**: Standard refinement scores.
