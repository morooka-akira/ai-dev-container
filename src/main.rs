mod cli;
mod config;
mod utils;
mod workspace;

use clap::Parser;
use cli::{Cli, Commands};
use config::load_config_from_path;
use workspace::WorkspaceManager;

fn main() {
    let cli = Cli::parse();
    let workspace_manager = WorkspaceManager::new();

    match cli.command {
        Commands::Start { task_name, config } => {
            let config = load_config_from_path(&config);
            println!("start コマンドが実行されました: {}", task_name);
            println!("設定: {:?}", config);
            let _ = workspace_manager.create_workspace(&task_name);
        }
        Commands::List { config } => {
            let config = load_config_from_path(&config);
            println!("list コマンドが実行されました");
            println!("設定: {:?}", config);
            let _ = workspace_manager.list_workspaces();
        }
    }
}
