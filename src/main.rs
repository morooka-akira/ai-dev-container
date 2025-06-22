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

    let workspace_manager = match WorkspaceManager::new() {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("エラー: {}", e);
            std::process::exit(1);
        }
    };

    match cli.command {
        Commands::Start { task_name, config } => {
            let config = load_config_from_path(&config);
            println!("start コマンドが実行されました: {}", task_name);

            match workspace_manager.create_workspace(
                &task_name,
                &config.workspace.base_dir,
                &config.workspace.branch_prefix,
            ) {
                Ok(info) => println!("✅ ワークスペース準備完了: {:?}", info),
                Err(e) => eprintln!("❌ エラー: {}", e),
            }
        }
        Commands::List { config } => {
            let _config = load_config_from_path(&config);
            println!("list コマンドが実行されました");

            match workspace_manager.list_workspaces() {
                Ok(workspaces) => println!("📋 ワークスペース一覧: {} 件", workspaces.len()),
                Err(e) => eprintln!("❌ エラー: {}", e),
            }
        }
    }
}
