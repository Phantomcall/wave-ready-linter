# Wave-Ready Contribution Plan 🌊

This document outlines the types of work and milestones we prioritize for the **Wave-Ready Repository Linter**. These categories help determine the suggested "Points" for the **Stellar Community Fund (SCF)** and other rewards programs.

---

## 🏗️ Types of Work

### 1. 🐞 Bug Fixes
*   **Description**: Identifying and resolving unintended behaviors in the linter or contract sync logic.
*   **Priority**: High.
*   **Examples**:
    *   Fixing rate-limiting issues with the GitHub API.
    *   Correcting type mismatches in XDR serialization.
*   **Suggested Points**: 10 - 25 pts (depending on severity).

### 2. ✨ New Features
*   **Description**: Adding new capabilities to enhance the "Scoping" and "Audit" process.
*   **Priority**: Medium - High.
*   **Examples**:
    *   Adding support for GitLab or Bitbucket.
    *   Implementing a web-based dashboard for point visualization.
    *   New scoring strategies (e.g., historical PR velocity weighting).
*   **Suggested Points**: 25 - 50 pts.

### 3. 📄 Documentation
*   **Description**: Improving the onboarding experience and technical clarity.
*   **Priority**: Medium.
*   **Examples**:
    *   Writing detailed tutorials for syncing with Soroban contracts.
    *   Creating "Best Practices" guides for repo maintainers.
    *   Translating the README into multiple languages.
*   **Suggested Points**: 5 - 15 pts.

### 4. 🧪 Testing & QA
*   **Description**: Increasing the robustness and reliability of the codebase.
*   **Priority**: High.
*   **Examples**:
    *   Implementing property-based testing for the scoring engine.
    *   Setting up mock Soroban RPC environments for integration tests.
    *   Increasing unit test coverage to >90%.
*   **Suggested Points**: 10 - 30 pts.

### 5. 🌌 Stellar Ecosystem Integration
*   **Description**: Aligning the tool with new SEPs (Stellar Ecosystem Proposals) or Soroban upgrades.
*   **Priority**: Critical.
*   **Examples**:
    *   Updating `stellar-xdr` to the latest protocol version.
    *   Integrating with popular Stellar wallets for transaction signing.
*   **Suggested Points**: 30 - 60 pts.

---

## 📊 Suggested Points Table

| Work Type | Difficulty | Suggested Points |
| :--- | :--- | :--- |
| Documentation Tweak | Trivial | 5 pts |
| Minor Bug Fix | Easy | 10 pts |
| Unit Test Addition | Easy | 15 pts |
| New Scan Strategy | Moderate | 25 pts |
| Soroban Contract Sync | Complex | 40 pts |
| Major Feature / SEP | Advanced | 50+ pts |

---

## 🤝 How to Propose Work
If you have a task that doesn't fit these categories, please open an **Issue** with the `proposal` label. We will review it and assign a point value based on its impact on the **Wave-Ready** ecosystem.
