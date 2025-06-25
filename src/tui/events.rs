use crate::tui::App;
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

#[derive(Debug, PartialEq)]
pub enum AppAction {
    None,
    Quit,
    NavigateToWorkspace(String),   // Return path
    DeleteWorkspaces(Vec<String>), // Workspace names to delete (supports bulk delete)
}

pub fn handle_events(app: &mut App) -> std::io::Result<AppAction> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    if app.is_in_delete_confirmation() {
                        app.hide_delete_confirmation();
                        Ok(AppAction::None)
                    } else if app.is_in_details_view() {
                        app.hide_details();
                        Ok(AppAction::None)
                    } else {
                        app.quit();
                        Ok(AppAction::Quit)
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if !app.is_in_delete_confirmation() && !app.is_in_details_view() {
                        app.next();
                    }
                    Ok(AppAction::None)
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if !app.is_in_delete_confirmation() && !app.is_in_details_view() {
                        app.previous();
                    }
                    Ok(AppAction::None)
                }
                KeyCode::Enter => {
                    if app.is_in_delete_confirmation() {
                        // Enter key handling in delete confirmation dialog
                        Ok(AppAction::None)
                    } else if app.is_in_details_view() {
                        // Enter key handling in details dialog (close)
                        app.hide_details();
                        Ok(AppAction::None)
                    } else if let Some(workspace) = app.get_selected_workspace() {
                        Ok(AppAction::NavigateToWorkspace(workspace.path.clone()))
                    } else {
                        Ok(AppAction::None)
                    }
                }
                KeyCode::Char('d') => {
                    if !app.is_in_delete_confirmation() && !app.is_in_details_view() {
                        // Check if any workspaces are selected, or use current workspace
                        let selected_count = app.get_selected_count();
                        if selected_count > 0 || app.get_selected_workspace().is_some() {
                            app.show_delete_confirmation();
                        }
                    }
                    Ok(AppAction::None)
                }
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    if app.is_in_delete_confirmation() {
                        let selected_count = app.get_selected_count();
                        let workspace_names = if selected_count > 0 {
                            // Use selected workspaces
                            app.get_selected_workspaces()
                                .into_iter()
                                .map(|w| w.name.clone())
                                .collect()
                        } else if let Some(workspace) = app.get_selected_workspace() {
                            // Use current workspace if none are selected
                            vec![workspace.name.clone()]
                        } else {
                            vec![]
                        };

                        if !workspace_names.is_empty() {
                            app.hide_delete_confirmation();
                            Ok(AppAction::DeleteWorkspaces(workspace_names))
                        } else {
                            Ok(AppAction::None)
                        }
                    } else {
                        Ok(AppAction::None)
                    }
                }
                KeyCode::Char('n') | KeyCode::Char('N') => {
                    if app.is_in_delete_confirmation() {
                        app.hide_delete_confirmation();
                    }
                    Ok(AppAction::None)
                }
                KeyCode::Char('i') => {
                    if !app.is_in_delete_confirmation()
                        && !app.is_in_details_view()
                        && app.get_selected_workspace().is_some()
                    {
                        app.show_details();
                    }
                    Ok(AppAction::None)
                }
                KeyCode::Char(' ') => {
                    // Space key: toggle selection of current workspace
                    if !app.is_in_delete_confirmation() && !app.is_in_details_view() {
                        app.toggle_current_selection();
                    }
                    Ok(AppAction::None)
                }
                KeyCode::Char('a') => {
                    // 'a' key: toggle all workspaces selection
                    if !app.is_in_delete_confirmation() && !app.is_in_details_view() {
                        app.toggle_all_selection();
                    }
                    Ok(AppAction::None)
                }
                _ => {
                    // Close details dialog with any key
                    if app.is_in_details_view() {
                        app.hide_details();
                    }
                    Ok(AppAction::None)
                }
            }
        } else {
            Ok(AppAction::None)
        }
    } else {
        Ok(AppAction::None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace::WorkspaceInfo;

    // Helper function for AppAction testing
    fn create_test_app_with_workspaces() -> App {
        let mut app = App::new();
        app.workspaces = vec![
            WorkspaceInfo {
                name: "test1".to_string(),
                path: "/path/to/workspace1".to_string(),
                branch: "work/test1".to_string(),
            },
            WorkspaceInfo {
                name: "test2".to_string(),
                path: "/path/to/workspace2".to_string(),
                branch: "work/test2".to_string(),
            },
        ];
        app.selected_workspaces = vec![false, false];
        app.selected_index = 0;
        app
    }

    #[test]
    fn test_app_action_equality() {
        assert_eq!(AppAction::None, AppAction::None);
        assert_eq!(AppAction::Quit, AppAction::Quit);
        assert_eq!(
            AppAction::NavigateToWorkspace("/test/path".to_string()),
            AppAction::NavigateToWorkspace("/test/path".to_string())
        );
        assert_eq!(
            AppAction::DeleteWorkspaces(vec!["workspace1".to_string()]),
            AppAction::DeleteWorkspaces(vec!["workspace1".to_string()])
        );

        assert_ne!(AppAction::None, AppAction::Quit);
        assert_ne!(
            AppAction::NavigateToWorkspace("/test/path1".to_string()),
            AppAction::NavigateToWorkspace("/test/path2".to_string())
        );
        assert_ne!(
            AppAction::DeleteWorkspaces(vec!["workspace1".to_string()]),
            AppAction::DeleteWorkspaces(vec!["workspace2".to_string()])
        );
    }

    #[test]
    fn test_app_action_debug() {
        let action = AppAction::NavigateToWorkspace("/test/path".to_string());
        let debug_str = format!("{action:?}");
        assert!(debug_str.contains("NavigateToWorkspace"));
        assert!(debug_str.contains("/test/path"));
    }

    // Mock KeyCode processing test (test logic without generating actual key events)
    #[test]
    fn test_enter_key_action_with_selected_workspace() {
        let app = create_test_app_with_workspaces();

        // Test expected behavior when Enter key is pressed
        if let Some(workspace) = app.get_selected_workspace() {
            let expected_action = AppAction::NavigateToWorkspace(workspace.path.clone());
            assert_eq!(
                expected_action,
                AppAction::NavigateToWorkspace("/path/to/workspace1".to_string())
            );
        }
    }

    #[test]
    fn test_enter_key_action_with_empty_workspaces() {
        let app = App::new(); // Empty workspace

        // When Enter key is pressed with no selected workspace
        assert!(app.get_selected_workspace().is_none());
        // AppAction::None should be returned in this case
    }

    #[test]
    fn test_quit_action() {
        let mut app = create_test_app_with_workspaces();

        // Verify quit processing with q key
        app.quit();
        assert!(app.should_quit);
    }

    #[test]
    fn test_navigation_actions() {
        let mut app = create_test_app_with_workspaces();

        // Initially, item 0 is selected
        assert_eq!(app.selected_index, 0);

        // Test down key
        app.next();
        assert_eq!(app.selected_index, 1);

        // Test up key
        app.previous();
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn test_delete_confirmation_flow() {
        let mut app = create_test_app_with_workspaces();

        // Display delete confirmation dialog
        assert!(!app.is_in_delete_confirmation());
        app.show_delete_confirmation();
        assert!(app.is_in_delete_confirmation());

        // Verify navigation control during delete confirmation with app logic
        // (Actual key event disabling is implemented in handle_events in events.rs)
        assert!(app.is_in_delete_confirmation());

        // Cancel
        app.hide_delete_confirmation();
        assert!(!app.is_in_delete_confirmation());
    }

    #[test]
    fn test_delete_workspace_action() {
        let app = create_test_app_with_workspaces();

        // Selected workspace is returned as delete action
        if let Some(workspace) = app.get_selected_workspace() {
            let expected_action = AppAction::DeleteWorkspaces(vec![workspace.name.clone()]);
            assert_eq!(
                expected_action,
                AppAction::DeleteWorkspaces(vec!["test1".to_string()])
            );
        }
    }
}
