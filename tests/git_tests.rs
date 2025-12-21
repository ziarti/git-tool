use git_tool::errors::GitError;
use git_tool::git_repository;
use git_tool::git_cmd::GitCommandTrait;

/// Mock qui retourne des données prédéfinies au lieu d'exécuter Git
struct GitCommandMock {
    /// Données simulées pour "git for-each-ref"
    branches_output: String,
    /// Données simulées pour "git rev-list --count"
    /// Clé: "branch_name..origin_branch", Valeur: nombre de commits
    commit_counts: std::collections::HashMap<String, u32>,
}

impl GitCommandMock {
    fn new() -> Self {
        Self {
            branches_output: String::new(),
            commit_counts: std::collections::HashMap::new(),
        }
    }

    /// Configure le mock pour retourner des branches
    ///
    /// # Format attendu (comme git for-each-ref)
    /// "origin/branch-name date author\n..."
    fn with_branches(mut self, output: &str) -> Self {
        self.branches_output = output.to_string();
        self
    }

    /// Configure le mock pour retourner un nombre de commits pour une branche
    ///
    /// # Arguments
    /// * `branch` - Nom de la branche
    /// * `origin` - Branche de référence
    /// * `count` - Nombre de commits derrière
    fn with_commit_count(mut self, branch: &str, origin: &str, count: u32) -> Self {
        let key = format!("{}..{}", branch, origin);
        self.commit_counts.insert(key, count);
        self
    }
}

impl GitCommandTrait for GitCommandMock {
    fn rev_list(&self, args: &[&str]) -> Result<String, GitError> {
        // Extraire le range "branch..origin" des arguments
        // Format: ["--count", "origin/feature-a..origin/production"]
        if args.len() < 2 {
            return Err(GitError::ExecutionError {error: "Arguments insuffisants pour rev-list".to_string()});
        }

        let key_range = args[1];
        match self.commit_counts.get(key_range) {
            Some(&count) => Ok(count.to_string()),
            None => Err(GitError::ExecutionError {error: format!("Pas de données mockées pour le range: {}", key_range)}),
        }
    }

    fn for_each_ref(&self, _args: &[&str]) -> Result<String, GitError> {
        // Retourner les branches simulées
        Ok(self.branches_output.clone())
    }
    
    fn rev_parse(&self, _args: &[&str]) -> Result<String, GitError> {
        Ok(".git".to_string())
    }
}

#[test]
fn test_list_branches_with_mock() {
    // Créer un mock qui simule 2 branches
    let mock = GitCommandMock::new()
        .with_branches(
            "origin/feature-old 2025-01-15 John Doe\n\
             origin/bugfix-legacy 2025-02-10 Jane Smith",
        )
        .with_commit_count("origin/feature-old", "origin/production", 150)
        .with_commit_count("origin/bugfix-legacy", "origin/production", 75);

    // Appeler list_branches avec le mock
    let result = git_repository::list_branches(&mock, "origin/production", 100);

    // Vérifier le résultat
    assert!(result.is_ok(), "list_branches devrait réussir");

    let branches = result.unwrap();

    // Seulement 1 branche devrait être retournée (>= 100 commits)
    assert_eq!(branches.len(), 1, "Devrait retourner 1 branche");

    // Vérifier que c'est la bonne branche
    assert_eq!(branches[0].name, "origin/feature-old");
    assert_eq!(branches[0].commits_ahead, 150);
    assert_eq!(branches[0].author, "John Doe");
}

#[test]
fn test_list_branches_empty() {
    // Mock sans branches
    let mock = GitCommandMock::new().with_branches("");

    let result = git_repository::list_branches(&mock, "origin/production", 100);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0, "Devrait retourner 0 branches");
}

#[test]
fn test_list_branches_below_threshold() {
    // Toutes les branches sont en dessous du seuil
    let mock = GitCommandMock::new()
        .with_branches("origin/feature-a 2025-01-15 Alice")
        .with_commit_count("origin/feature-a", "origin/main", 50);

    let result = git_repository::list_branches(&mock, "origin/main", 100);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0, "Aucune branche ne dépasse le seuil");
}
