mod cli;
mod config;
mod error;
mod tui;
mod utils;
mod workspace;

use clap::Parser;
use cli::{Cli, Commands};
use config::{generate_template_config, load_config_from_path};
use error::GitwsError;
use tracing::{debug, error};
use workspace::WorkspaceManager;

fn main() {
    let cli = Cli::parse();

    // Lower log level for TUI mode
    let is_tui_mode = matches!(
        cli.command,
        Commands::List {
            print_path_only: false,
            ..
        }
    );
    init_logging(is_tui_mode);

    debug!("Starting gitws application");
    debug!("Command line arguments parsed");

    let result = match cli.command {
        Commands::Init { output } => {
            debug!("Starting configuration file initialization");
            debug!("Output path: {}", output);

            // Init command doesn't require git repository or workspace manager
            match generate_template_config(&output) {
                Ok(()) => {
                    debug!("Configuration template generated successfully");
                    Ok(())
                }
                Err(e) => {
                    error!("Failed to generate configuration template: {}", e);
                    eprintln!("❌ Error: {e}");
                    Err(e)
                }
            }
        }
        _ => {
            // For other commands, initialize WorkspaceManager
            let workspace_manager = match WorkspaceManager::new() {
                Ok(manager) => {
                    debug!("WorkspaceManager initialized successfully");
                    manager
                }
                Err(e) => {
                    error!("Failed to initialize WorkspaceManager: {}", e);
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            };

            match cli.command {
                Commands::Start { task_name, config } => {
                    debug!("Starting workspace creation: {}", task_name);
                    debug!("Using configuration file: {}", config);

                    let config = load_config_from_path(&config);

                    match workspace_manager.create_workspace_with_config(
                        &task_name,
                        &config.workspace.base_dir,
                        &config.workspace.branch_prefix,
                        &config.workspace.copy_files,
                        &config.workspace.pre_commands,
                    ) {
                        Ok(info) => {
                            debug!("Workspace creation completed: {}", info.name);
                            Ok(())
                        }
                        Err(e) => {
                            error!("Failed to create workspace: {}", e);
                            eprintln!("❌ Error: {e}");
                            Err(e)
                        }
                    }
                }
                Commands::List {
                    config,
                    print_path_only,
                } => {
                    debug!("Starting workspace list display");
                    debug!("Using configuration file: {}", config);

                    let _config = load_config_from_path(&config);

                    if print_path_only {
                        debug!("Executing --path-only mode");
                        // --path-only mode: output list of all workspace paths
                        match workspace_manager.list_workspaces() {
                            Ok(workspaces) => {
                                debug!("Retrieved workspace list: {} items", workspaces.len());
                                for workspace in workspaces {
                                    println!("{}", workspace.path);
                                }
                                Ok(())
                            }
                            Err(e) => {
                                error!("Failed to retrieve workspace list: {}", e);
                                Err(e)
                            }
                        }
                    } else {
                        // Normal TUI mode
                        debug!("Starting TUI mode");
                        debug!("Initializing TUI");

                        match run_tui() {
                            Ok(Some(selected_path)) => {
                                debug!("Path selected in TUI: {}", selected_path);
                                // Output path of workspace selected with Enter key
                                // Shell function receives this path and executes cd
                                println!("{selected_path}");
                                Ok(())
                            }
                            Ok(None) => {
                                debug!("TUI exited normally (no path selected)");
                                // Exit without selecting anything
                                Ok(())
                            }
                            Err(e) => {
                                error!("TUI error occurred: {}", e);
                                eprintln!("TUI error: {e}");
                                Err(GitwsError::tui(format!("TUI error: {e}")))
                            }
                        }
                    }
                }
                Commands::Init { .. } => {
                    // This case is already handled above
                    unreachable!()
                }
            }
        }
    };

    // Final error handling
    if let Err(e) = result {
        error!("Application error occurred: {}", e);
        eprintln!("❌ {e}");
        std::process::exit(1);
    }

    debug!("Exiting gitws application normally");
}

/// Initialize logging
fn init_logging(is_tui_mode: bool) {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    // Allow setting log level via RUST_LOG environment variable
    // Different default levels for debug and release builds
    let default_level = if cfg!(debug_assertions) {
        // Debug build
        if is_tui_mode {
            "gitws=warn"
        } else {
            "gitws=info"
        }
    } else {
        // Release build: show errors only
        "gitws=error"
    };
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(default_level));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(std::io::stderr) // Output logs to stderr
                .with_target(false) // Don't show target (module name)
                .with_thread_ids(false) // Don't show thread IDs
                .with_file(false) // Don't show file names
                .with_line_number(false), // Don't show line numbers
        )
        .init();
}

fn run_tui() -> std::io::Result<Option<String>> {
    use crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use ratatui::{backend::CrosstermBackend, Terminal};
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
            eprintln!("Workspace management initialization error: {e}");
            return Ok(None);
        }
    };

    if let Err(e) = app.load_workspaces(&workspace_manager) {
        // Show error but continue with empty list
        eprintln!("Workspace loading warning: {e}");
    }

    // Main loop
    let selected_path = loop {
        terminal.draw(|f| tui::ui::draw(f, &app, &workspace_manager))?;

        match tui::events::handle_events(&mut app)? {
            tui::events::AppAction::Quit => break None,
            tui::events::AppAction::NavigateToWorkspace(path) => break Some(path),
            tui::events::AppAction::DeleteWorkspaces(workspace_names) => {
                // Delete workspaces (supports bulk delete)
                if workspace_names.len() > 1 {
                    // Use bulk delete method for multiple workspaces
                    match workspace_manager.remove_multiple_workspaces(&workspace_names) {
                        Ok(()) => {
                            // Remove all from app state
                            for workspace_name in &workspace_names {
                                app.remove_workspace(workspace_name);
                            }
                        }
                        Err(e) => {
                            eprintln!("Bulk deletion error: {e}");
                            // Update app state for all workspaces (sync display with actual state)
                            for workspace_name in &workspace_names {
                                app.remove_workspace(workspace_name);
                            }
                        }
                    }
                } else {
                    // Single workspace deletion
                    for workspace_name in workspace_names {
                        match workspace_manager.remove_workspace(&workspace_name) {
                            Ok(()) => {
                                app.remove_workspace(&workspace_name);
                            }
                            Err(e) => {
                                eprintln!("Deletion error for {workspace_name}: {e}");
                                app.remove_workspace(&workspace_name);
                            }
                        }
                    }
                }
                // Clear selections after any delete operation
                app.clear_all_selections();
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
