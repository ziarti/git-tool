use crate::errors::GitError;
use crate::git_cmd::GitCommandTrait;
use std::env;
use std::path::PathBuf;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BranchInfo {
    pub name: String,
    pub date: String,
    pub author: String,
    pub commits_ahead: u32,
}

pub fn list_branches(
    cmd: &impl GitCommandTrait,
    origin_branch: &str,
    threshold: u32,
) -> Result<Vec<BranchInfo>, GitError> {
    let result = cmd.for_each_ref(&[
        "--format",
        "%(refname:short) %(committerdate:short) %(authorname)",
        "refs/remotes/origin/",
    ])?;

    let mut stale_branches: Vec<BranchInfo> = Vec::new();

    for line in result.lines() {
        let parts: Vec<&str> = line.splitn(3, ' ').collect();

        if parts.len() < 3 {
            continue;
        }

        let branch_name = parts[0];
        if branch_name == origin_branch {
            continue;
        }

        let behind_count = get_commit_behind(cmd, origin_branch, branch_name)?;
        if behind_count < threshold {
            continue;
        }

        let commit_date = parts[1];
        let author_name = parts[2];

        stale_branches.push(BranchInfo {
            name: branch_name.to_string(),
            date: commit_date.to_string(),
            author: author_name.to_string(),
            commits_ahead: behind_count,
        });
    }

    Ok(stale_branches)
}

fn get_commit_behind(
    cmd: &impl GitCommandTrait,
    origin_branch: &str,
    branch_name: &str,
) -> Result<u32, GitError> {
    let result = cmd.rev_list(&["--count", &format!("{}..{}", branch_name, origin_branch)])?;

    let count: u32 = result.trim().parse()?;

    Ok(count)
}

pub fn validate_git_repository(cmd: &impl GitCommandTrait) -> Result<(), GitError> {
    cmd.rev_parse(&["--git-dir"])
        .map_err(|_| GitError::NotARepository {
            path: env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .display()
                .to_string(),
        })?;

    Ok(())
}
