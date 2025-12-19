use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "git-tool",
    version = "0.1",
    author = "Arthur Farrugia",
    about = "CLI Application for Git operations"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Lister les branches potentiellement obsolètes
    StaleBranches {
        /// Seuil minimum de commits de retard
        #[arg(short, long, default_value_t = 100)]
        threshold: u32,
        /// Branche d'origine pour la comparaison
        #[arg(short, long, default_value = "origin/production")]
        origin_branch: String,
    },
}
