use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitError {
    #[error("Une erreur lors de l'exécution est survenue: {error}")]
    ExecutionError {
        error: String,
    },
    #[error("La commande git {command} a échoué: {stderr}")]
    CommandFailed {
        command: String,
        stderr: String
    },
    #[error("Erreur de parsing du nombre de commits: {error}")]
    ParsingError {
        error: String
    }
}