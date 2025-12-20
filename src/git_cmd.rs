use std::process::Command;
use crate::errors::GitError;

pub trait GitCommandTrait {
    fn rev_list(&self, args: &[&str]) -> Result<String, GitError>;
    fn for_each_ref(&self, args: &[&str]) -> Result<String, GitError>;
}

pub struct GitCommand;

impl GitCommand {
    fn execute_git_command(&self, sub_command: &str, args: &[&str]) -> Result<String, GitError> {
        let output = Command::new("git")
            .arg(sub_command)
            .args(args)
            .output()
            .map_err(|error| GitError::ExecutionError {error: error.to_string() })?;

        if !output.status.success() {
            return Err(GitError::CommandFailed {
                command: format!("{} {}", sub_command.to_string(), args.join(" ")),
                stderr: String::from_utf8_lossy(&output.stderr).to_string()}
            );
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

impl GitCommandTrait for GitCommand {
    fn rev_list(&self, args: &[&str]) -> Result<String, GitError> {
        self.execute_git_command("rev-list", args)
    }

    fn for_each_ref(&self, args: &[&str]) -> Result<String, GitError> {
        self.execute_git_command("for-each-ref", args)
    }
}