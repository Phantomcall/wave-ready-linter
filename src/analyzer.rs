// src/analyzer.rs

pub struct Issue {
    pub number: u64,
    pub title: String,
    pub labels: Vec<String>,
    pub comment_count: u64,
}

pub fn estimate_points(issue: &Issue) -> u64 {
    let mut score = 10; // Base score

    let labels: Vec<String> = issue.labels.iter().map(|l| l.to_lowercase()).collect();

    if labels.contains(&"good first issue".to_string()) {
        score = 5;
    }
    if labels.contains(&"enhancement".to_string()) || labels.contains(&"feature".to_string()) {
        score = 20;
    }
    if labels.contains(&"critical".to_string()) || labels.contains(&"security".to_string()) {
        score = 50;
    }

    // Complexity modifier based on discussion length
    if issue.comment_count > 10 {
        score += 5;
    }

    score
}
