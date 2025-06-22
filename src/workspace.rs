use git2::Repository;

pub struct WorkspaceManager {
    #[allow(dead_code)]
    repo: Repository,
}

#[derive(Debug)]
pub struct WorkspaceInfo {
    #[allow(dead_code)]
    pub name: String,
    pub path: String,
    pub branch: String,
}

impl WorkspaceManager {
    pub fn new() -> Result<Self, String> {
        let repo =
            Repository::open(".").map_err(|e| format!("Gitリポジトリが見つかりません: {}", e))?;
        Ok(Self { repo })
    }

    pub fn create_workspace(
        &self,
        task_name: &str,
        base_dir: &str,
        branch_prefix: &str,
    ) -> Result<WorkspaceInfo, String> {
        let timestamp = crate::utils::generate_timestamp();
        let workspace_name = format!("{}-{}", timestamp, task_name);
        let branch_name = format!("{}{}", branch_prefix, task_name);
        let workspace_path = format!("{}/{}", base_dir, workspace_name);

        println!("Creating workspace:");
        println!("  Name: {}", workspace_name);
        println!("  Path: {}", workspace_path);
        println!("  Branch: {}", branch_name);

        println!("  Status: 準備完了（実際の作成は次の段階で実装）");

        Ok(WorkspaceInfo {
            name: workspace_name,
            path: workspace_path,
            branch: branch_name,
        })
    }

    pub fn list_workspaces(&self) -> Result<Vec<WorkspaceInfo>, String> {
        println!("Listing workspaces (mock data):");

        let mock_workspaces = vec![WorkspaceInfo {
            name: "20250621-140000-example".to_string(),
            path: "../workspaces/20250621-140000-example".to_string(),
            branch: "work/example".to_string(),
        }];

        for workspace in &mock_workspaces {
            println!("  - {} -> {}", workspace.branch, workspace.path);
        }

        Ok(mock_workspaces)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_manager_new_in_git_repo() {
        // Gitリポジトリ内でのテスト（このプロジェクト自体がGitリポジトリなので）
        match WorkspaceManager::new() {
            Ok(_manager) => {
                // Gitリポジトリ内では正常に作成できる
            }
            Err(_) => {
                // 非Gitディレクトリの場合はエラーになる
                panic!("Should be able to create WorkspaceManager in git repo");
            }
        }
    }

    #[test]
    fn test_workspace_info_structure() {
        let info = WorkspaceInfo {
            name: "test-workspace".to_string(),
            path: "/path/to/workspace".to_string(),
            branch: "work/test".to_string(),
        };

        assert_eq!(info.name, "test-workspace");
        assert_eq!(info.path, "/path/to/workspace");
        assert_eq!(info.branch, "work/test");
    }

    #[test]
    fn test_create_workspace_parameters() {
        if let Ok(manager) = WorkspaceManager::new() {
            let result = manager.create_workspace("test-task", "../test-workspaces", "test/");

            assert!(result.is_ok());
            let workspace = result.unwrap();
            assert!(workspace.name.contains("test-task"));
            assert!(workspace.path.contains("../test-workspaces"));
            assert!(workspace.branch.starts_with("test/"));
        }
    }

    #[test]
    fn test_create_workspace_with_various_names() {
        if let Ok(manager) = WorkspaceManager::new() {
            let test_cases = vec![
                "simple-task",
                "task_with_underscores",
                "task-with-numbers-123",
                "UPPERCASE-TASK",
                "mixed_Case-Task_123",
            ];

            for task_name in test_cases {
                let result = manager.create_workspace(task_name, "../test-workspaces", "test/");
                assert!(result.is_ok(), "Failed for task name: {}", task_name);
                let workspace = result.unwrap();
                assert!(workspace.name.contains(task_name));
            }
        }
    }

    #[test]
    fn test_create_workspace_empty_name() {
        if let Ok(manager) = WorkspaceManager::new() {
            let result = manager.create_workspace("", "../test-workspaces", "test/");
            // 現在の実装では空文字でも成功する（将来的にはバリデーションが必要）
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_list_workspaces_mock_data() {
        if let Ok(manager) = WorkspaceManager::new() {
            let result = manager.list_workspaces();
            assert!(result.is_ok());
            let workspaces = result.unwrap();
            // モックデータが1件返される
            assert_eq!(workspaces.len(), 1);
            assert_eq!(workspaces[0].name, "20250621-140000-example");
            assert_eq!(workspaces[0].branch, "work/example");
        }
    }

    #[test]
    fn test_workspace_manager_multiple_operations() {
        if let Ok(manager) = WorkspaceManager::new() {
            // 複数の操作を連続して実行してもエラーにならないことを確認
            assert!(
                manager
                    .create_workspace("task1", "../test", "test/")
                    .is_ok()
            );
            assert!(
                manager
                    .create_workspace("task2", "../test", "test/")
                    .is_ok()
            );
            assert!(manager.list_workspaces().is_ok());
            assert!(
                manager
                    .create_workspace("task3", "../test", "test/")
                    .is_ok()
            );
        }
    }

    #[test]
    fn test_workspace_info_debug_format() {
        let info = WorkspaceInfo {
            name: "test".to_string(),
            path: "/path".to_string(),
            branch: "branch".to_string(),
        };
        let debug_str = format!("{:?}", info);
        assert!(debug_str.contains("WorkspaceInfo"));
        assert!(debug_str.contains("test"));
        assert!(debug_str.contains("/path"));
        assert!(debug_str.contains("branch"));
    }
}
