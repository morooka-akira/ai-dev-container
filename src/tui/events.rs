use crate::tui::App;
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

#[derive(Debug, PartialEq)]
pub enum AppAction {
    None,
    Quit,
    NavigateToWorkspace(String), // パスを返す
}

pub fn handle_events(app: &mut App) -> std::io::Result<AppAction> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    app.quit();
                    Ok(AppAction::Quit)
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    app.next();
                    Ok(AppAction::None)
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    app.previous();
                    Ok(AppAction::None)
                }
                KeyCode::Enter => {
                    if let Some(workspace) = app.get_selected_workspace() {
                        Ok(AppAction::NavigateToWorkspace(workspace.path.clone()))
                    } else {
                        Ok(AppAction::None)
                    }
                }
                _ => Ok(AppAction::None),
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

    // AppActionのテスト用ヘルパー関数
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

        assert_ne!(AppAction::None, AppAction::Quit);
        assert_ne!(
            AppAction::NavigateToWorkspace("/test/path1".to_string()),
            AppAction::NavigateToWorkspace("/test/path2".to_string())
        );
    }

    #[test]
    fn test_app_action_debug() {
        let action = AppAction::NavigateToWorkspace("/test/path".to_string());
        let debug_str = format!("{:?}", action);
        assert!(debug_str.contains("NavigateToWorkspace"));
        assert!(debug_str.contains("/test/path"));
    }

    // 模擬的なKeyCode処理テスト（実際のキーイベントは発生させずにロジックをテスト）
    #[test]
    fn test_enter_key_action_with_selected_workspace() {
        let app = create_test_app_with_workspaces();

        // Enterキーが押された時の想定される動作をテスト
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
        let app = App::new(); // 空のワークスペース

        // Enterキーが押された時、選択されたワークスペースがない場合
        assert!(app.get_selected_workspace().is_none());
        // この場合はAppAction::Noneが返されるべき
    }

    #[test]
    fn test_quit_action() {
        let mut app = create_test_app_with_workspaces();

        // qキーによる終了処理の検証
        app.quit();
        assert!(app.should_quit);
    }

    #[test]
    fn test_navigation_actions() {
        let mut app = create_test_app_with_workspaces();

        // 初期状態は0番目が選択されている
        assert_eq!(app.selected_index, 0);

        // 下方向キーのテスト
        app.next();
        assert_eq!(app.selected_index, 1);

        // 上方向キーのテスト
        app.previous();
        assert_eq!(app.selected_index, 0);
    }
}
