use std::io;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitError {
    #[error("Une erreur lors de l'exécution est survenue: {error}")]
    ExecutionError { error: String },
    #[error("La commande git {command} a échoué: {stderr}")]
    CommandFailed { command: String, stderr: String },
    #[error("Erreur de parsing du nombre de commits: {error}")]
    ParsingError { error: String },
    #[error("Not a git repository: {path}")]
    NotARepository { path: String },
}

impl From<ParseIntError> for GitError {
    fn from(error: ParseIntError) -> Self {
        GitError::ParsingError {
            error: error.to_string(),
        }
    }
}

impl From<io::Error> for GitError {
    fn from(error: io::Error) -> Self {
        GitError::ExecutionError {
            error: error.to_string(),
        }
    }
}
