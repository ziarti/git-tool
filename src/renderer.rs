use crate::git_repository;

use comfy_table::{Cell, Color, Table};
use git_repository::BranchInfo;

pub fn render_branch_info(author: &str, branches: &[&BranchInfo], threshold: u32) {
    let table = build_branch_table(branches, threshold);
    println!("{table}");
    println!("{} branches analysées pour {}", branches.len(), author);
}

fn build_branch_table(branches: &[&BranchInfo], threshold: u32) -> Table {
    let mut table = Table::new();
    table.set_header(vec![
        "Branch",
        "Date",
        "Auteur",
        "Nombre de commits en plus",
    ]);

    for branch in branches {
        let line_color = get_branch_color(branch.commits_ahead, threshold);

        table.add_row(vec![
            Cell::new(&branch.name).fg(line_color),
            Cell::new(&branch.date).fg(line_color),
            Cell::new(&branch.author).fg(line_color),
            Cell::new(branch.commits_ahead.to_string()).fg(line_color),
        ]);
    }

    table
}

fn get_branch_color(commits_ahead: u32, threshold: u32) -> Color {
    if commits_ahead > (threshold * 2) {
        Color::Red
    } else if commits_ahead > (threshold * 3 / 2) {
        Color::Yellow
    } else {
        Color::White
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_branch_color_default() {
        assert_eq!(get_branch_color(0, 10), Color::White);
        assert_eq!(get_branch_color(14, 10), Color::White);

        assert_ne!(get_branch_color(16, 10), Color::White);
    }

    #[test]
    fn test_branch_color_warning() {
        assert_eq!(get_branch_color(16, 10), Color::Yellow);
        assert_eq!(get_branch_color(17, 10), Color::Yellow);
        assert_eq!(get_branch_color(20, 10), Color::Yellow);

        assert_ne!(get_branch_color(21, 10), Color::Yellow);
        assert_ne!(get_branch_color(14, 10), Color::Yellow);
    }

    #[test]
    fn test_branch_color_critical() {
        assert_eq!(get_branch_color(21, 10), Color::Red);

        assert_ne!(get_branch_color(19, 10), Color::Red);
        assert_ne!(get_branch_color(1, 10), Color::Red);
    }

    #[test]
    fn test_build_table_snapshot() {
        let branch1 = BranchInfo {
            name: String::from("origin/feature-old"),
            date: String::from("2025-01-15"),
            author: String::from("John Doe"),
            commits_ahead: 16,
        };

        let branch2 = BranchInfo {
            name: String::from("origin/bugfix-legacy"),
            date: String::from("2025-02-10"),
            author: String::from("Jane Smith"),
            commits_ahead: 21,
        };

        let branches = vec![&branch1, &branch2];
        let table = build_branch_table(&branches, 10);

        insta::assert_snapshot!(table.to_string());
    }
}
