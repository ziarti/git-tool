use std::collections::HashMap;
use clap::Parser;
use git_tool::cli::{Cli, Commands};
use git_tool::git::{BranchInfo, list_branches};
use git_tool::git_cmd::GitCommand;
use git_tool::renderer::render_branch_info;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::StaleBranches {
            origin_branch,
            threshold,
        } => {
            println!(
                "🔍 Recherche des branches obsolètes à partir de la branch {} (>{} commits)...\n",
                origin_branch, threshold
            );

            let git_cmd = GitCommand;
            match list_branches(&git_cmd, &origin_branch, threshold) {
                Ok(branches) => {
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
                }
                Err(e) => {
                    eprintln!("Erreur: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
