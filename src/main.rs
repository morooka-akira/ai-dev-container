mod cli;
mod config;
mod tui;
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

            match workspace_manager.create_workspace_with_config(
                &task_name,
                &config.workspace.base_dir,
                &config.workspace.branch_prefix,
                &config.workspace.copy_files,
                &config.workspace.pre_commands,
            ) {
                Ok(info) => println!("✅ ワークスペース準備完了: {:?}", info),
                Err(e) => eprintln!("❌ エラー: {}", e),
            }
        }
        Commands::List {
            config,
            print_path_only,
        } => {
            let _config = load_config_from_path(&config);

            if print_path_only {
                // --print-path-onlyモード: 最初のワークスペースのパスを出力
                if let Ok(workspace_manager) = WorkspaceManager::new() {
                    if let Ok(workspaces) = workspace_manager.list_workspaces() {
                        if let Some(first_workspace) = workspaces.first() {
                            println!("{}", first_workspace.path);
                        }
                    }
                }
            } else {
                // 通常のTUIモード
                println!("TUIモードを開始します...");

                match run_tui() {
                    Ok(Some(selected_path)) => {
                        // Enterキーで選択されたパスを出力
                        println!("{}", selected_path);
                    }
                    Ok(None) => {
                        // 何も選択せずに終了
                    }
                    Err(e) => {
                        eprintln!("TUIエラー: {}", e);
                    }
                }
            }
        }
    }
}

fn run_tui() -> std::io::Result<Option<String>> {
    use crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    };
    use ratatui::{Terminal, backend::CrosstermBackend};
    use std::io;

    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // App with real data
    let mut app = tui::App::new();

    // Load workspace data
    let workspace_manager = match WorkspaceManager::new() {
        Ok(manager) => manager,
        Err(e) => {
            // Cleanup and return error
            disable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )?;
            eprintln!("ワークスペース管理の初期化エラー: {}", e);
            return Ok(None);
        }
    };

    if let Err(e) = app.load_workspaces(&workspace_manager) {
        // Show error but continue with empty list
        eprintln!("ワークスペース読み込み警告: {}", e);
    }

    // Main loop
    let selected_path = loop {
        terminal.draw(|f| tui::ui::draw(f, &app))?;

        match tui::events::handle_events(&mut app)? {
            tui::events::AppAction::Quit => break None,
            tui::events::AppAction::NavigateToWorkspace(path) => break Some(path),
            tui::events::AppAction::None => {}
        }
    };

    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(selected_path)
}
