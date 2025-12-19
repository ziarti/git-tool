use std::process::Command;

pub trait GitCommandTrait {
    fn rev_list(&self, args: &[&str]) -> Result<String, String>;
    fn for_each_ref(&self, args: &[&str]) -> Result<String, String>;
}

pub struct GitCommand;

impl GitCommand {
    fn execute_git_command(&self, sub_command: &str, args: &[&str]) -> Result<String, String> {
        let output = Command::new("git")
            .arg(sub_command)
            .args(args)
            .output()
            .map_err(|e| format!("Impossible d'exécuter git {}: {}", sub_command, e))?;

        if !output.status.success() {
            return Err(format!("La commande git {} a échoué: {:?}", sub_command, output.stderr));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

impl GitCommandTrait for GitCommand {
    fn rev_list(&self, args: &[&str]) -> Result<String, String> {
        self.execute_git_command("rev-list", args)
    }

    fn for_each_ref(&self, args: &[&str]) -> Result<String, String> {
        self.execute_git_command("for-each-ref", args)
    }
}