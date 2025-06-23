mod cli;
mod config;
mod error;
mod tui;
mod utils;
mod workspace;

use clap::Parser;
use cli::{Cli, Commands};
use config::load_config_from_path;
use error::GworkError;
use tracing::{debug, error, info};
use workspace::WorkspaceManager;

fn main() {
    // ログの初期化
    init_logging();

    info!("gwork アプリケーションを開始します");

    let cli = Cli::parse();
    debug!("コマンドライン引数を解析しました");

    let workspace_manager = match WorkspaceManager::new() {
        Ok(manager) => {
            info!("WorkspaceManagerを正常に初期化しました");
            manager
        }
        Err(e) => {
            error!("WorkspaceManagerの初期化に失敗しました: {}", e);
            eprintln!("エラー: {}", e);
            std::process::exit(1);
        }
    };

    let result = match cli.command {
        Commands::Start { task_name, config } => {
            info!("ワークスペース作成を開始します: {}", task_name);
            debug!("使用する設定ファイル: {}", config);

            let config = load_config_from_path(&config);
            println!("start コマンドが実行されました: {}", task_name);

            match workspace_manager.create_workspace_with_config(
                &task_name,
                &config.workspace.base_dir,
                &config.workspace.branch_prefix,
                &config.workspace.copy_files,
                &config.workspace.pre_commands,
            ) {
                Ok(info) => {
                    info!("ワークスペース作成が完了しました: {}", info.name);
                    println!("✅ ワークスペース準備完了: {:?}", info);
                    Ok(())
                }
                Err(e) => {
                    error!("ワークスペース作成に失敗しました: {}", e);
                    eprintln!("❌ エラー: {}", e);
                    Err(e)
                }
            }
        }
        Commands::List {
            config,
            print_path_only,
        } => {
            info!("ワークスペース一覧表示を開始します");
            debug!("使用する設定ファイル: {}", config);

            let _config = load_config_from_path(&config);

            if print_path_only {
                debug!("--print-path-onlyモードを実行します");
                // --print-path-onlyモード: 全ワークスペースのパス一覧を出力
                match workspace_manager.list_workspaces() {
                    Ok(workspaces) => {
                        info!("ワークスペース一覧を取得しました: {} 件", workspaces.len());
                        for workspace in workspaces {
                            println!("{}", workspace.path);
                        }
                        Ok(())
                    }
                    Err(e) => {
                        error!("ワークスペース一覧の取得に失敗しました: {}", e);
                        Err(e)
                    }
                }
            } else {
                // 通常のTUIモード
                println!("TUIモードを開始します...");
                debug!("TUIを初期化します");

                match run_tui() {
                    Ok(Some(selected_path)) => {
                        info!("TUIで選択されたパス: {}", selected_path);
                        // Enterキーで選択されたワークスペースのパスを出力
                        // シェル関数がこのパスを受け取ってcdを実行する
                        println!("{}", selected_path);
                        Ok(())
                    }
                    Ok(None) => {
                        debug!("TUIが正常終了しました（パス選択なし）");
                        // 何も選択せずに終了
                        Ok(())
                    }
                    Err(e) => {
                        error!("TUIエラーが発生しました: {}", e);
                        eprintln!("TUIエラー: {}", e);
                        Err(GworkError::tui(format!("TUIエラー: {}", e)))
                    }
                }
            }
        }
    };

    // 最終的なエラーハンドリング
    if let Err(e) = result {
        error!("アプリケーションでエラーが発生しました: {}", e);
        eprintln!("❌ {}", e);
        std::process::exit(1);
    }

    info!("gwork アプリケーションを正常終了します");
}

/// ログの初期化
fn init_logging() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    // 環境変数 RUST_LOG でログレベルを設定可能にする
    // デフォルトは info レベル
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("gwork=info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false) // ターゲット（モジュール名）を表示しない
                .with_thread_ids(false) // スレッドIDを表示しない
                .with_file(false) // ファイル名を表示しない
                .with_line_number(false), // 行番号を表示しない
        )
        .init();
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
        terminal.draw(|f| tui::ui::draw(f, &app, &workspace_manager))?;

        match tui::events::handle_events(&mut app)? {
            tui::events::AppAction::Quit => break None,
            tui::events::AppAction::NavigateToWorkspace(path) => break Some(path),
            tui::events::AppAction::DeleteWorkspace(workspace_name) => {
                // ワークスペースを削除
                match workspace_manager.remove_workspace(&workspace_name) {
                    Ok(()) => {
                        // アプリの状態からも削除
                        app.remove_workspace(&workspace_name);
                    }
                    Err(e) => {
                        eprintln!("削除エラー: {}", e);
                        // エラーが発生してもアプリ状態は更新（表示と実際の状態の同期）
                        app.remove_workspace(&workspace_name);
                    }
                }
            }
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
