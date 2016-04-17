// Copyright 2016 Adam Perry. Dual-licensed MIT and Apache 2.0 (see LICENSE files for details).

use std::collections::BTreeMap;
use std::convert::Into;
use std::i32;

use chrono::{DateTime, UTC};

use domain::github::{Issue, IssueComment, IssueLabel, Milestone, GitHubUser};

#[derive(Debug, Deserialize)]
pub struct MilestoneFromJson {
    pub id: i32,
    pub number: i32,
    pub state: String,
    pub title: String,
    pub description: Option<String>,
    pub creator: GitHubUser,
    pub open_issues: i32,
    pub closed_issues: i32,
    pub created_at: DateTime<UTC>,
    pub updated_at: DateTime<UTC>,
    pub closed_at: Option<DateTime<UTC>>,
    pub due_on: Option<DateTime<UTC>>,
}

impl Into<Milestone> for MilestoneFromJson {
    fn into(self) -> Milestone {
        Milestone {
            id: self.id,
            number: self.number,
            open: match &self.state as &str {
                "open" => true,
                _ => false,
            },
            title: self.title,
            description: self.description,
            fk_creator: self.creator.id,
            open_issues: self.open_issues,
            closed_issues: self.closed_issues,
            created_at: self.created_at.naive_utc(),
            updated_at: self.updated_at.naive_utc(),
            closed_at: self.closed_at.map(|t| t.naive_utc()),
            due_on: self.due_on.map(|t| t.naive_utc()),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LabelFromJson {
    name: String,
    color: String,
}

pub type PullRequestUrls = BTreeMap<String, String>;

#[derive(Debug, Deserialize)]
pub struct IssueFromJson {
    pub number: i32,
    pub user: GitHubUser,
    pub assignee: Option<GitHubUser>,
    pub state: String,
    pub title: String,
    pub body: Option<String>,
    pub labels: Option<Vec<LabelFromJson>>,
    pub milestone: Option<MilestoneFromJson>,
    pub locked: bool,
    pub comments: i32,
    pub pull_request: Option<PullRequestUrls>,
    pub closed_at: Option<DateTime<UTC>>,
    pub created_at: DateTime<UTC>,
    pub updated_at: DateTime<UTC>,
    pub comments_url: String,
}

impl Into<(Issue, Option<Milestone>, Vec<IssueLabel>)> for IssueFromJson {
    fn into(self) -> (Issue, Option<Milestone>, Vec<IssueLabel>) {

        let mut labels = vec![];

        if let Some(ref labels_from_json) = self.labels {
            for l in labels_from_json {
                labels.push(IssueLabel {
                    fk_issue: self.number,
                    label: l.name.clone(),
                    color: l.color.clone(),
                });
            }
        }

        let milestone_id = match self.milestone {
            Some(ref m) => Some(m.number),
            None => None,
        };

        let issue = Issue {
            number: self.number,
            fk_milestone: milestone_id,
            fk_user: self.user.id,
            fk_assignee: self.assignee.map(|a| a.id),
            open: match &*self.state {
                "open" => true,
                _ => false,
            },
            is_pull_request: self.pull_request.is_some(),
            title: self.title,
            body: self.body.unwrap_or("".to_string()),
            locked: self.locked,
            closed_at: self.closed_at.map(|t| t.naive_utc()),
            created_at: self.created_at.naive_utc(),
            updated_at: self.updated_at.naive_utc(),
        };

        (issue, self.milestone.map(|m| m.into()), labels)
    }
}

#[derive(Debug, Deserialize)]
pub struct CommentFromJson {
    pub id: i32,
    pub html_url: String,
    pub body: String,
    pub user: GitHubUser,
    pub created_at: DateTime<UTC>,
    pub updated_at: DateTime<UTC>,
}

impl Into<IssueComment> for CommentFromJson {
    fn into(self) -> IssueComment {
        let issue_id = self.html_url
                           .split('#')
                           .next()
                           .map(|r| r.split('/').last().map(|n| n.parse::<i32>()));

        let issue_id = match issue_id {
            Some(Some(Ok(n))) => n,
            _ => {
                // TODO log failed parsing
                i32::MAX
            }
        };

        IssueComment {
            id: self.id,
            fk_issue: issue_id,
            fk_user: self.user.id,
            body: self.body,
            created_at: self.created_at.naive_utc(),
            updated_at: self.updated_at.naive_utc(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PullRequestFromJson {
    number: i32,
    review_comments_url: String,
    state: String,
    title: String,
    body: Option<String>,
    assignee: Option<GitHubUser>,
    milestone: Option<MilestoneFromJson>,
    locked: bool,
    created_at: DateTime<UTC>,
    updated_at: DateTime<UTC>,
    closed_at: Option<DateTime<UTC>>,
    merged_at: Option<DateTime<UTC>>,
    commits: i32,
    additions: i32,
    deletions: i32,
    changed_files: i32,
}