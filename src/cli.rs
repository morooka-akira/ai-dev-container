use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ai-workspace")]
#[command(about = "AI workspace management tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Start {
        task_name: String,
        #[arg(short, long, default_value = "workspace.yml")]
        config: String,
    },
    List {
        #[arg(short, long, default_value = "workspace.yml")]
        config: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_cli_start_command() {
        // start コマンドのパース
        let args = vec!["ai-workspace", "start", "test-task"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::Start { task_name, config } => {
                assert_eq!(task_name, "test-task");
                assert_eq!(config, "workspace.yml"); // デフォルト値
            }
            _ => panic!("Expected Start command"),
        }
    }

    #[test]
    fn test_cli_start_command_with_config() {
        // start コマンドにconfig引数を指定
        let args = vec!["ai-workspace", "start", "test-task", "--config", "custom.yml"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::Start { task_name, config } => {
                assert_eq!(task_name, "test-task");
                assert_eq!(config, "custom.yml");
            }
            _ => panic!("Expected Start command"),
        }
    }

    #[test]
    fn test_cli_start_command_with_short_config() {
        // start コマンドに短縮形のconfig引数を指定
        let args = vec!["ai-workspace", "start", "test-task", "-c", "short.yml"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::Start { task_name, config } => {
                assert_eq!(task_name, "test-task");
                assert_eq!(config, "short.yml");
            }
            _ => panic!("Expected Start command"),
        }
    }

    #[test]
    fn test_cli_list_command() {
        // list コマンドのパース
        let args = vec!["ai-workspace", "list"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::List { config } => {
                assert_eq!(config, "workspace.yml"); // デフォルト値
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_cli_list_command_with_config() {
        // list コマンドにconfig引数を指定
        let args = vec!["ai-workspace", "list", "--config", "custom.yml"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::List { config } => {
                assert_eq!(config, "custom.yml");
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_cli_invalid_command() {
        // 無効なコマンドでエラーになることを確認
        let args = vec!["ai-workspace", "invalid"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_start_without_task_name() {
        // start コマンドでタスク名なしでエラーになることを確認
        let args = vec!["ai-workspace", "start"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_task_names_with_special_characters() {
        // 特殊文字を含むタスク名のテスト
        let test_cases = vec![
            "task-with-hyphens",
            "task_with_underscores",
            "task123",
            "UPPERCASE_TASK",
        ];
        
        for task_name in test_cases {
            let args = vec!["ai-workspace", "start", task_name];
            let cli = Cli::try_parse_from(args).unwrap();
            
            match cli.command {
                Commands::Start { task_name: parsed_name, .. } => {
                    assert_eq!(parsed_name, task_name);
                }
                _ => panic!("Expected Start command"),
            }
        }
    }
}
