use crate::error::GworkResult;
use crate::workspace::{WorkspaceInfo, WorkspaceManager};
use tracing::debug;

pub struct App {
    pub should_quit: bool,
    pub workspaces: Vec<WorkspaceInfo>,
    pub selected_index: usize,
    pub show_delete_dialog: bool,
    pub show_details_dialog: bool,
    pub selected_workspaces: Vec<bool>, // Multi-selection state for each workspace
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            workspaces: Vec::new(),
            selected_index: 0,
            show_delete_dialog: false,
            show_details_dialog: false,
            selected_workspaces: Vec::new(),
        }
    }

    pub fn load_workspaces(&mut self, workspace_manager: &WorkspaceManager) -> GworkResult<()> {
        debug!("Loading workspace list into TUI app");
        self.workspaces = workspace_manager.list_workspaces()?;

        // Initialize selection state for each workspace
        self.selected_workspaces = vec![false; self.workspaces.len()];

        if self.workspaces.is_empty() {
            debug!("No workspaces found");
            self.selected_index = 0;
        } else {
            debug!("Loaded {} workspaces", self.workspaces.len());
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

    pub fn show_delete_confirmation(&mut self) {
        self.show_delete_dialog = true;
    }

    pub fn hide_delete_confirmation(&mut self) {
        self.show_delete_dialog = false;
    }

    pub fn is_in_delete_confirmation(&self) -> bool {
        self.show_delete_dialog
    }

    pub fn remove_workspace(&mut self, workspace_name: &str) {
        if let Some(index) = self
            .workspaces
            .iter()
            .position(|w| w.name == workspace_name)
        {
            self.workspaces.remove(index);
            // Only remove from selected_workspaces if it has the same length
            if index < self.selected_workspaces.len() {
                self.selected_workspaces.remove(index);
            }
        }

        // Adjust selection index
        if self.selected_index >= self.workspaces.len() && !self.workspaces.is_empty() {
            self.selected_index = self.workspaces.len() - 1;
        } else if self.workspaces.is_empty() {
            self.selected_index = 0;
        }
    }

    pub fn show_details(&mut self) {
        self.show_details_dialog = true;
    }

    pub fn hide_details(&mut self) {
        self.show_details_dialog = false;
    }

    pub fn is_in_details_view(&self) -> bool {
        self.show_details_dialog
    }

    /// Toggle selection state of current workspace
    pub fn toggle_current_selection(&mut self) {
        if self.selected_index < self.selected_workspaces.len() {
            self.selected_workspaces[self.selected_index] =
                !self.selected_workspaces[self.selected_index];
        }
    }

    /// Toggle all workspaces selection state
    pub fn toggle_all_selection(&mut self) {
        let all_selected = self.selected_workspaces.iter().all(|&selected| selected);
        let new_state = !all_selected;

        for selected in &mut self.selected_workspaces {
            *selected = new_state;
        }
    }

    /// Get number of selected workspaces
    pub fn get_selected_count(&self) -> usize {
        self.selected_workspaces
            .iter()
            .filter(|&&selected| selected)
            .count()
    }

    /// Get indices of selected workspaces
    pub fn get_selected_indices(&self) -> Vec<usize> {
        self.selected_workspaces
            .iter()
            .enumerate()
            .filter_map(|(i, &selected)| if selected { Some(i) } else { None })
            .collect()
    }

    /// Get selected workspaces
    pub fn get_selected_workspaces(&self) -> Vec<&WorkspaceInfo> {
        self.get_selected_indices()
            .into_iter()
            .filter_map(|i| self.workspaces.get(i))
            .collect()
    }

    /// Clear all selections
    pub fn clear_all_selections(&mut self) {
        for selected in &mut self.selected_workspaces {
            *selected = false;
        }
    }

    /// Check if current workspace is selected
    #[allow(dead_code)]
    pub fn is_current_workspace_selected(&self) -> bool {
        self.selected_workspaces
            .get(self.selected_index)
            .copied()
            .unwrap_or(false)
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
        assert!(!app.show_delete_dialog);
        assert!(!app.show_details_dialog);
        assert!(app.selected_workspaces.is_empty());
    }

    #[test]
    fn test_app_navigation_empty() {
        let mut app = App::new();

        // No change in empty state
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

        // Move down
        app.next();
        assert_eq!(app.selected_index, 1);

        // From last to first
        app.next();
        assert_eq!(app.selected_index, 0);

        // Move up
        app.previous();
        assert_eq!(app.selected_index, 1);

        // From first to last
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

    #[test]
    fn test_delete_confirmation_dialog() {
        let mut app = App::new();
        assert!(!app.is_in_delete_confirmation());

        app.show_delete_confirmation();
        assert!(app.is_in_delete_confirmation());

        app.hide_delete_confirmation();
        assert!(!app.is_in_delete_confirmation());
    }

    #[test]
    fn test_remove_workspace() {
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
        app.selected_workspaces = vec![false, false];

        // Remove first workspace
        app.remove_workspace("workspace1");
        assert_eq!(app.workspaces.len(), 1);
        assert_eq!(app.workspaces[0].name, "workspace2");
        assert_eq!(app.selected_index, 0);

        // Remove remaining workspace
        app.remove_workspace("workspace2");
        assert_eq!(app.workspaces.len(), 0);
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn test_remove_workspace_adjust_selection() {
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
        app.selected_workspaces = vec![false, false];

        // Select second item then remove first
        app.selected_index = 1;
        app.remove_workspace("workspace1");
        assert_eq!(app.workspaces.len(), 1);
        assert_eq!(app.selected_index, 0); // Index is adjusted
    }

    #[test]
    fn test_details_dialog() {
        let mut app = App::new();
        assert!(!app.is_in_details_view());

        app.show_details();
        assert!(app.is_in_details_view());

        app.hide_details();
        assert!(!app.is_in_details_view());
    }

    #[test]
    fn test_multi_selection() {
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
        app.selected_workspaces = vec![false, false];

        // Initially no selections
        assert_eq!(app.get_selected_count(), 0);
        assert!(!app.is_current_workspace_selected());

        // Toggle current selection (index 0)
        app.toggle_current_selection();
        assert_eq!(app.get_selected_count(), 1);
        assert!(app.is_current_workspace_selected());
        assert_eq!(app.get_selected_indices(), vec![0]);

        // Move to next and toggle
        app.next();
        app.toggle_current_selection();
        assert_eq!(app.get_selected_count(), 2);
        assert!(app.is_current_workspace_selected());
        assert_eq!(app.get_selected_indices(), vec![0, 1]);

        // Get selected workspaces
        let selected = app.get_selected_workspaces();
        assert_eq!(selected.len(), 2);
        assert_eq!(selected[0].name, "workspace1");
        assert_eq!(selected[1].name, "workspace2");

        // Clear all selections
        app.clear_all_selections();
        assert_eq!(app.get_selected_count(), 0);
        assert!(!app.is_current_workspace_selected());
    }

    #[test]
    fn test_toggle_all_selection() {
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
        app.selected_workspaces = vec![false, false];

        // Initially no selections, toggle all should select all
        assert_eq!(app.get_selected_count(), 0);
        app.toggle_all_selection();
        assert_eq!(app.get_selected_count(), 2);

        // All selected, toggle all should deselect all
        app.toggle_all_selection();
        assert_eq!(app.get_selected_count(), 0);

        // Partial selection, toggle all should select all
        app.selected_workspaces[0] = true;
        assert_eq!(app.get_selected_count(), 1); // Confirm partial selection
        app.toggle_all_selection();
        assert_eq!(app.get_selected_count(), 2);
    }
}
