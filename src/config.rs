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
