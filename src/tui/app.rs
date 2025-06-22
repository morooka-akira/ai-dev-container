use crate::workspace::{WorkspaceInfo, WorkspaceManager};

pub struct App {
    pub should_quit: bool,
    pub workspaces: Vec<WorkspaceInfo>,
    pub selected_index: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            workspaces: Vec::new(),
            selected_index: 0,
        }
    }

    pub fn load_workspaces(&mut self, workspace_manager: &WorkspaceManager) -> Result<(), String> {
        self.workspaces = workspace_manager.list_workspaces()?;
        if self.workspaces.is_empty() {
            self.selected_index = 0;
        } else {
            self.selected_index = self.selected_index.min(self.workspaces.len() - 1);
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_selected_workspace(&self) -> Option<&WorkspaceInfo> {
        self.workspaces.get(self.selected_index)
    }

    pub fn next(&mut self) {
        if !self.workspaces.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.workspaces.len();
        }
    }

    pub fn previous(&mut self) {
        if !self.workspaces.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                self.workspaces.len() - 1
            } else {
                self.selected_index - 1
            };
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace::WorkspaceInfo;

    #[test]
    fn test_app_new() {
        let app = App::new();
        assert!(!app.should_quit);
        assert!(app.workspaces.is_empty());
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn test_app_navigation_empty() {
        let mut app = App::new();

        // 空の状態では何も変化しない
        app.next();
        assert_eq!(app.selected_index, 0);

        app.previous();
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn test_app_navigation_with_items() {
        let mut app = App::new();
        app.workspaces = vec![
            WorkspaceInfo {
                name: "workspace1".to_string(),
                path: "/path1".to_string(),
                branch: "branch1".to_string(),
            },
            WorkspaceInfo {
                name: "workspace2".to_string(),
                path: "/path2".to_string(),
                branch: "branch2".to_string(),
            },
        ];

        // 下移動
        app.next();
        assert_eq!(app.selected_index, 1);

        // 最後から次は最初へ
        app.next();
        assert_eq!(app.selected_index, 0);

        // 上移動
        app.previous();
        assert_eq!(app.selected_index, 1);

        // 最初から前は最後へ
        app.previous();
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn test_app_quit() {
        let mut app = App::new();
        assert!(!app.should_quit);

        app.quit();
        assert!(app.should_quit);
    }

    #[test]
    fn test_get_selected_workspace() {
        let mut app = App::new();
        app.workspaces = vec![WorkspaceInfo {
            name: "workspace1".to_string(),
            path: "/path1".to_string(),
            branch: "branch1".to_string(),
        }];

        let selected = app.get_selected_workspace();
        assert!(selected.is_some());
        assert_eq!(selected.unwrap().name, "workspace1");
    }

    #[test]
    fn test_get_selected_workspace_empty() {
        let app = App::new();
        let selected = app.get_selected_workspace();
        assert!(selected.is_none());
    }
}
