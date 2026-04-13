// src/analyzer.rs

pub struct Issue {
    pub number: u64,
    pub title: String,
    pub labels: Vec<String>,
    pub comment_count: u64,
}

pub fn estimate_points(issue: &Issue) -> u64 {
    let mut score = 10; // Base score (Stellar SCF Base)

    let labels: Vec<String> = issue.labels.iter().map(|l| l.to_lowercase()).collect();

    // Soroban and Smart Contract specific scoring
    if labels.contains(&"soroban".to_string()) || labels.contains(&"smart-contract".to_string()) {
        score = 30; // High value for Soroban integration
    }
    
    // Stellar Ecosystem Proposals (SEP) scoring
    if labels.contains(&"sep".to_string()) {
        score = 40; // Critical ecosystem alignment
    }

    // Bug & Enhancement scoring
    if labels.contains(&"bug".to_string()) {
        score += 5;
    }
    if labels.contains(&"enhancement".to_string()) || labels.contains(&"feature".to_string()) {
        score += 10;
    }

    // Security and Stellar Community Fund (SCF) Milestone readiness
    if labels.contains(&"security".to_string()) || labels.contains(&"scf-milestone".to_string()) {
        score += 20;
    }

    // Complexity modifier based on community discussion (SCF community importance)
    if issue.comment_count > 10 {
        score += 10;
    }

    score
}
