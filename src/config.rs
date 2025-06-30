use crate::error::{GitwsError, GitwsResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use tracing::{debug, error, warn};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkspaceConfig {
    pub workspace: WorkspaceSettings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkspaceSettings {
    pub base_dir: String,
    pub branch_prefix: String,
    pub copy_files: Vec<String>,
    pub pre_commands: Vec<String>,
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            workspace: WorkspaceSettings {
                base_dir: "../workspaces".to_string(),
                branch_prefix: "work/".to_string(),
                copy_files: vec![],
                pre_commands: vec![],
            },
        }
    }
}

/// Load configuration file and return GitwsError on error
#[allow(dead_code)]
pub fn load_config_from_path_safe(path: &str) -> GitwsResult<WorkspaceConfig> {
    debug!(
        "Starting configuration file loading (error handling version): {}",
        path
    );

    if Path::new(path).exists() {
        let content = fs::read_to_string(path).map_err(|e| {
            error!("Failed to read configuration file: {} - {}", path, e);
            GitwsError::config(format!("Configuration file read error: {e}"))
        })?;

        debug!("Configuration file content read: {} bytes", content.len());

        let config = serde_yaml::from_str::<WorkspaceConfig>(&content).map_err(|e| {
            error!("Failed to parse configuration file: {} - {}", path, e);
            GitwsError::config(format!("YAML parsing error: {e}"))
        })?;

        debug!("Configuration file loaded successfully: {}", path);
        Ok(config)
    } else {
        debug!("Configuration file does not exist: {}", path);
        Err(GitwsError::config(format!(
            "Configuration file not found: {path}"
        )))
    }
}

/// Load configuration file and return default settings on error (kept for backward compatibility)
pub fn load_config_from_path(path: &str) -> WorkspaceConfig {
    debug!("Starting configuration file loading: {}", path);

    if Path::new(path).exists() {
        match fs::read_to_string(path) {
            Ok(content) => {
                debug!("Configuration file content read: {} bytes", content.len());
                match serde_yaml::from_str::<WorkspaceConfig>(&content) {
                    Ok(config) => {
                        debug!("Configuration file loaded successfully: {}", path);
                        config
                    }
                    Err(e) => {
                        error!("Failed to parse configuration file: {} - {}", path, e);
                        warn!("Using default settings");
                        WorkspaceConfig::default()
                    }
                }
            }
            Err(e) => {
                error!("Failed to read configuration file: {} - {}", path, e);
                warn!("Using default settings");
                WorkspaceConfig::default()
            }
        }
    } else {
        debug!("Configuration file does not exist: {}", path);
        debug!("Using default settings");
        WorkspaceConfig::default()
    }
}

pub fn _test_serialize() {
    let config = WorkspaceConfig::default();
    let yaml = serde_yaml::to_string(&config).unwrap();
    println!("Default config YAML:\n{yaml}");
}

/// Generate a template configuration file
pub fn generate_template_config(output_path: &str) -> GitwsResult<()> {
    debug!("Generating template configuration file: {}", output_path);

    // Check if file already exists
    if Path::new(output_path).exists() {
        debug!("Configuration file already exists: {}", output_path);

        // Ask for confirmation to overwrite
        print!(
            "Configuration file '{}' already exists. Overwrite? (y/N): ",
            output_path
        );
        io::stdout().flush().map_err(|e| {
            error!("Failed to flush stdout: {}", e);
            GitwsError::io(format!("IO error: {e}"))
        })?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| {
            error!("Failed to read user input: {}", e);
            GitwsError::io(format!("Input error: {e}"))
        })?;

        let input = input.trim().to_lowercase();
        if input != "y" && input != "yes" {
            debug!("User cancelled overwrite operation");
            println!("Operation cancelled.");
            return Ok(());
        }
    }

    let template_content = create_template_content();

    fs::write(output_path, template_content).map_err(|e| {
        error!("Failed to write template file: {} - {}", output_path, e);
        GitwsError::io(format!("Failed to write configuration file: {e}"))
    })?;

    println!("✅ Configuration template created: {}", output_path);
    println!("📝 Edit the file to customize your workspace settings");

    debug!(
        "Template configuration file generated successfully: {}",
        output_path
    );
    Ok(())
}

/// Create template configuration content with comments
fn create_template_content() -> String {
    r#"# Gitws Configuration Template
# Workspace management settings for git worktree automation

workspace:
  # Base directory for creating workspaces (relative to current directory)
  base_dir: "../workspaces"
  
  # Branch name prefix for new branches
  branch_prefix: "work/"
  
  # Files to copy from main workspace to new workspace
  copy_files:
    - ".env"
    - ".env.local"
    # - "config/database.yml"
    # - "docker-compose.override.yml"
  
  # Commands to execute after workspace creation
  pre_commands:
    - "npm install"
    # - "cargo build"
    # - "bundle install"
    # - "docker-compose up -d"
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_workspace_config_default() {
        let config = WorkspaceConfig::default();
        assert_eq!(config.workspace.base_dir, "../workspaces");
        assert_eq!(config.workspace.branch_prefix, "work/");
        assert!(config.workspace.copy_files.is_empty());
        assert!(config.workspace.pre_commands.is_empty());
    }

    #[test]
    fn test_workspace_config_serialization() {
        let config = WorkspaceConfig::default();
        let yaml = serde_yaml::to_string(&config).unwrap();
        assert!(yaml.contains("base_dir"));
        assert!(yaml.contains("branch_prefix"));
        assert!(yaml.contains("copy_files"));
        assert!(yaml.contains("pre_commands"));
    }

    #[test]
    fn test_workspace_config_deserialization() {
        let yaml = r#"
workspace:
  base_dir: "../test-workspaces"
  branch_prefix: "test/"
  copy_files:
    - ".env"
    - ".env.local"
  pre_commands:
    - "echo 'setup complete'"
"#;
        let config: WorkspaceConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.workspace.base_dir, "../test-workspaces");
        assert_eq!(config.workspace.branch_prefix, "test/");
        assert_eq!(config.workspace.copy_files, vec![".env", ".env.local"]);
        assert_eq!(config.workspace.pre_commands, vec!["echo 'setup complete'"]);
    }

    #[test]
    fn test_load_config_from_path_nonexistent_file() {
        let config = load_config_from_path("nonexistent.yml");
        // Default settings are returned for non-existent files
        assert_eq!(config.workspace.base_dir, "../workspaces");
        assert_eq!(config.workspace.branch_prefix, "work/");
    }

    #[test]
    fn test_load_config_from_path_valid_file() {
        // Create temporary file for testing
        let test_content = r#"
workspace:
  base_dir: "../test-workspaces"
  branch_prefix: "test/"
  copy_files: []
  pre_commands: []
"#;
        let test_file = "test_config.yml";
        fs::write(test_file, test_content).unwrap();

        let config = load_config_from_path(test_file);
        assert_eq!(config.workspace.base_dir, "../test-workspaces");
        assert_eq!(config.workspace.branch_prefix, "test/");

        // Delete test file
        fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_load_config_from_path_invalid_yaml() {
        // Create test file with invalid YAML
        let invalid_yaml = r#"
workspace:
  base_dir: "../test-workspaces"
  branch_prefix: "test/"
  copy_files
    - ".env"
"#;
        let test_file = "invalid_config.yml";
        fs::write(test_file, invalid_yaml).unwrap();

        let config = load_config_from_path(test_file);
        // Default settings are returned for invalid YAML
        assert_eq!(config.workspace.base_dir, "../workspaces");
        assert_eq!(config.workspace.branch_prefix, "work/");

        // Delete test file
        fs::remove_file(test_file).unwrap();
    }
}
