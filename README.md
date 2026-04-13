# Wave-Ready Repository Linter 🌊 (Rust Edition)

Automate your Drips Network Wave "Scoping" phase with Rust. This tool scans your GitHub backlog, suggests Point values, and syncs them to smart contracts.

## Features
- **Fast & Safe**: Built with Rust for high performance.
- **GitHub Integration**: Uses `octocrab` to audit issues.
- **Contract Ready**: Uses `alloy` to sync scores with Drips protocol contracts.

## Setup
1. Ensure you have [Rust](https://rustup.rs/) installed.
2. Clone the repo.
3. Create a `.env` file:
   ```bash
   cp .env.example .env
   ```
4. Add your `GITHUB_TOKEN` and `RPC_URL` to `.env`.

## Usage
### Scan a repository
```bash
cargo run -- scan --owner drips-network --repo org-monorepo
```

### Sync to a contract
```bash
cargo run -- sync --owner drips-network --repo org-monorepo --contract-address 0x...
```

## How it helps
- **Consistency:** Ensures similar tasks across projects have similar point rewards.
- **Transparency:** Push scores to on-chain registries for trustless distributions.
- **Speed:** Prepare 50+ issues for a Wave in seconds.
