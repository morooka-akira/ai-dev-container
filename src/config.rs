use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

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

pub fn load_config_from_path(path: &str) -> WorkspaceConfig {
    if Path::new(path).exists() {
        match fs::read_to_string(path) {
            Ok(content) => match serde_yaml::from_str::<WorkspaceConfig>(&content) {
                Ok(config) => {
                    println!("設定ファイルを読み込みました: {}", path);
                    config
                }
                Err(e) => {
                    println!(
                        "設定ファイルの解析エラー: {}. デフォルト設定を使用します",
                        e
                    );
                    WorkspaceConfig::default()
                }
            },
            Err(e) => {
                println!(
                    "設定ファイルの読み込みエラー: {}. デフォルト設定を使用します",
                    e
                );
                WorkspaceConfig::default()
            }
        }
    } else {
        println!(
            "設定ファイル {} が見つかりません. デフォルト設定を使用します",
            path
        );
        WorkspaceConfig::default()
    }
}

pub fn _test_serialize() {
    let config = WorkspaceConfig::default();
    let yaml = serde_yaml::to_string(&config).unwrap();
    println!("Default config YAML:\n{}", yaml);
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
        // 存在しないファイルの場合はデフォルト設定が返される
        assert_eq!(config.workspace.base_dir, "../workspaces");
        assert_eq!(config.workspace.branch_prefix, "work/");
    }

    #[test]
    fn test_load_config_from_path_valid_file() {
        // テスト用の一時ファイルを作成
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

        // テストファイルを削除
        fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_load_config_from_path_invalid_yaml() {
        // 不正なYAMLのテスト用ファイルを作成
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
        // 不正なYAMLの場合はデフォルト設定が返される
        assert_eq!(config.workspace.base_dir, "../workspaces");
        assert_eq!(config.workspace.branch_prefix, "work/");

        // テストファイルを削除
        fs::remove_file(test_file).unwrap();
    }
}
