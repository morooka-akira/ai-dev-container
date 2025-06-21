mod cli;
use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Start { task_name } => {
            println!("start コマンドが実行されました: {}", task_name);
        }
        Commands::List => {
            println!("list コマンドが実行されました");
        }
    }
}
