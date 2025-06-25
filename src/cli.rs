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
        #[arg(
            short = 'p',
            long = "path-only",
            help = "Print selected workspace path only"
        )]
        print_path_only: bool,
    },
    Init {
        #[arg(
            short = 'o',
            long = "output",
            default_value = DEFAULT_CONFIG_FILE,
            help = "Output path for the configuration file"
        )]
        output: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_cli_start_command() {
        // Parse start command
        let args = vec!["ai-workspace", "start", "test-task"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Commands::Start { task_name, config } => {
                assert_eq!(task_name, "test-task");
                assert_eq!(config, DEFAULT_CONFIG_FILE); // Default value
            }
            _ => panic!("Expected Start command"),
        }
    }

    #[test]
    fn test_cli_start_command_with_config() {
        // Specify config argument for start command
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
        // Specify short form config argument for start command
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
        // Parse list command
        let args = vec!["ai-workspace", "list"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Commands::List {
                config,
                print_path_only,
            } => {
                assert_eq!(config, DEFAULT_CONFIG_FILE); // Default value
                assert!(!print_path_only); // Default is false
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_cli_list_command_with_config() {
        // Specify config argument for list command
        let args = vec!["ai-workspace", "list", "--config", "custom.yml"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Commands::List {
                config,
                print_path_only,
            } => {
                assert_eq!(config, "custom.yml");
                assert!(!print_path_only); // Default is false
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_cli_list_command_with_path_only() {
        // Specify --path-only flag for list command
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
        // Specify -p flag for list command
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
        // Specify both options for list command
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
        // Specify both options (short form) for list command
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
        // Verify that invalid command causes error
        let args = vec!["ai-workspace", "invalid"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_start_without_task_name() {
        // Verify that start command without task name causes error
        let args = vec!["ai-workspace", "start"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_task_names_with_special_characters() {
        // Test task names with special characters
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

    #[test]
    fn test_cli_init_command() {
        // Parse init command with default output
        let args = vec!["ai-workspace", "init"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Commands::Init { output } => {
                assert_eq!(output, DEFAULT_CONFIG_FILE); // Default value
            }
            _ => panic!("Expected Init command"),
        }
    }

    #[test]
    fn test_cli_init_command_with_output() {
        // Specify output argument for init command
        let args = vec!["ai-workspace", "init", "--output", "custom.yml"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Commands::Init { output } => {
                assert_eq!(output, "custom.yml");
            }
            _ => panic!("Expected Init command"),
        }
    }

    #[test]
    fn test_cli_init_command_with_short_output() {
        // Specify short form output argument for init command
        let args = vec!["ai-workspace", "init", "-o", "short.yml"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Commands::Init { output } => {
                assert_eq!(output, "short.yml");
            }
            _ => panic!("Expected Init command"),
        }
    }
}
