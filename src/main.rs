use anyhow::{Context, Result};
use clap::Parser;
use git_tool::cli::{Cli, Commands};
use git_tool::git_cmd::GitCommand;
use git_tool::git_repository::{BranchInfo, list_branches, validate_git_repository};
use git_tool::renderer::render_branch_info;
use std::collections::HashMap;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::StaleBranches {
            origin_branch,
            threshold,
        } => {
            let git_cmd = GitCommand;
            validate_git_repository(&git_cmd).context("Not a valid git repository")?;
            println!(
                "🔍 Recherche des branches obsolètes à partir de la branch {} (>{} commits)...\n",
                origin_branch, threshold
            );

            let branches = list_branches(&git_cmd, &origin_branch, threshold)
                .context(format!("Failed list branches from {}", origin_branch))?;

            let mut branch_by_author: HashMap<String, Vec<&BranchInfo>> = HashMap::new();

            for branch in &branches {
                branch_by_author
                    .entry(branch.author.clone())
                    .or_default()
                    .push(branch);
            }

            for branches in branch_by_author.values_mut() {
                branches.sort_by(|a, b| b.commits_ahead.cmp(&a.commits_ahead));
            }

            let mut authors: Vec<&String> = branch_by_author.keys().collect();
            authors.sort();

            for author in authors {
                println!("\nAuteur: {}", author);
                let branches = &branch_by_author[author];
                render_branch_info(author, branches, threshold);
            }

            println!("Total branch: {}\n", branches.len());
            Ok(())
        }
    }
}
