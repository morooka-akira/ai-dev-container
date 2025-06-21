mod cli;
mod config;
mod utils;
mod workspace;

use clap::Parser;
use cli::{Cli, Commands};
use config::load_config;
use workspace::WorkspaceManager;

fn main() {
    let cli = Cli::parse();
    let _config = load_config();
    let workspace_manager = WorkspaceManager::new();

    match cli.command {
        Commands::Start { task_name } => {
            println!("start コマンドが実行されました: {}", task_name);
            let _ = workspace_manager.create_workspace(&task_name);
        }
        Commands::List => {
            println!("list コマンドが実行されました");
            let _ = workspace_manager.list_workspaces();
        }
    }
}
