use clap::{Parser, Subcommand};

const DEFAULT_CONFIG_FILE: &str = "workspace.yml";

#[derive(Parser, Debug)]
#[command(name = "ai-workspace")]
#[command(about = "AI workspace management tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Start {
        task_name: String,
        #[arg(short, long, default_value = DEFAULT_CONFIG_FILE)]
        config: String,
    },
    List {
        #[arg(short, long, default_value = DEFAULT_CONFIG_FILE)]
        config: String,
        #[arg(short = 'p', long = "path-only", help = "Print selected workspace path only")]
        print_path_only: bool,
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
                assert_eq!(config, DEFAULT_CONFIG_FILE); // デフォルト値
            }
            _ => panic!("Expected Start command"),
        }
    }

    #[test]
    fn test_cli_start_command_with_config() {
        // start コマンドにconfig引数を指定
        let args = vec![
            "ai-workspace",
            "start",
            "test-task",
            "--config",
            "custom.yml",
        ];
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
            Commands::List {
                config,
                print_path_only,
            } => {
                assert_eq!(config, DEFAULT_CONFIG_FILE); // デフォルト値
                assert!(!print_path_only); // デフォルトはfalse
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
            Commands::List {
                config,
                print_path_only,
            } => {
                assert_eq!(config, "custom.yml");
                assert!(!print_path_only); // デフォルトはfalse
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_cli_list_command_with_path_only() {
        // list コマンドに--path-onlyフラグを指定
        let args = vec!["ai-workspace", "list", "--path-only"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Commands::List {
                config,
                print_path_only,
            } => {
                assert_eq!(config, DEFAULT_CONFIG_FILE);
                assert!(print_path_only);
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_cli_list_command_with_short_print_path_only() {
        // list コマンドに-pフラグを指定
        let args = vec!["ai-workspace", "list", "-p"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Commands::List {
                config,
                print_path_only,
            } => {
                assert_eq!(config, DEFAULT_CONFIG_FILE);
                assert!(print_path_only);
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_cli_list_command_with_both_options() {
        // list コマンドに両方のオプションを指定
        let args = vec![
            "ai-workspace",
            "list",
            "--config",
            "test.yml",
            "--path-only",
        ];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Commands::List {
                config,
                print_path_only,
            } => {
                assert_eq!(config, "test.yml");
                assert!(print_path_only);
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_cli_list_command_with_both_options_short() {
        // list コマンドに両方のオプション（短縮形）を指定
        let args = vec!["ai-workspace", "list", "-c", "test.yml", "-p"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Commands::List {
                config,
                print_path_only,
            } => {
                assert_eq!(config, "test.yml");
                assert!(print_path_only);
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
                Commands::Start {
                    task_name: parsed_name,
                    ..
                } => {
                    assert_eq!(parsed_name, task_name);
                }
                _ => panic!("Expected Start command"),
            }
        }
    }
}
