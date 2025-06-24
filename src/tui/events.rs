use crate::tui::App;
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

#[derive(Debug, PartialEq)]
pub enum AppAction {
    None,
    Quit,
    NavigateToWorkspace(String), // Return path
    DeleteWorkspace(String),     // Workspace name to delete
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
                    if !app.is_in_delete_confirmation()
                        && !app.is_in_details_view()
                        && app.get_selected_workspace().is_some()
                    {
                        app.show_delete_confirmation();
                    }
                    Ok(AppAction::None)
                }
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    if app.is_in_delete_confirmation() {
                        if let Some(workspace) = app.get_selected_workspace() {
                            let workspace_name = workspace.name.clone();
                            app.hide_delete_confirmation();
                            Ok(AppAction::DeleteWorkspace(workspace_name))
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
            AppAction::DeleteWorkspace("workspace1".to_string()),
            AppAction::DeleteWorkspace("workspace1".to_string())
        );

        assert_ne!(AppAction::None, AppAction::Quit);
        assert_ne!(
            AppAction::NavigateToWorkspace("/test/path1".to_string()),
            AppAction::NavigateToWorkspace("/test/path2".to_string())
        );
        assert_ne!(
            AppAction::DeleteWorkspace("workspace1".to_string()),
            AppAction::DeleteWorkspace("workspace2".to_string())
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
            let expected_action = AppAction::DeleteWorkspace(workspace.name.clone());
            assert_eq!(
                expected_action,
                AppAction::DeleteWorkspace("test1".to_string())
            );
        }
    }
}
