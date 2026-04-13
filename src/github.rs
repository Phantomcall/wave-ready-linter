// src/github.rs

use octocrab::Octocrab;
use crate::analyzer::Issue;
use eyre::Result;

pub struct GithubClient {
    octocrab: Octocrab,
}

impl GithubClient {
    pub fn new(token: String) -> Result<Self> {
        let octocrab = Octocrab::builder()
            .personal_token(token)
            .build()?;
        Ok(Self { octocrab })
    }

    pub async fn fetch_issues(&self, owner: &str, repo: &str) -> Result<Vec<Issue>> {
        let issues_page = self.octocrab
            .issues(owner, repo)
            .list()
            .state(octocrab::params::State::Open)
            .send()
            .await?;

        let mut results = Vec::new();

        for issue in issues_page {
            // Filter out Pull Requests (GitHub API treats PRs as issues)
            if issue.pull_request.is_none() {
                results.push(Issue {
                    number: issue.number,
                    title: issue.title.clone(),
                    labels: issue.labels.iter().map(|l| l.name.clone()).collect(),
                    comment_count: issue.comments as u64,
                });
            }
        }

        Ok(results)
    }
}
