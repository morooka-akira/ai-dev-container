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
