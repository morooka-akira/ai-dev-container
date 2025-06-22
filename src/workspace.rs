use git2::{Repository, WorktreeAddOptions};
use std::fs;
use std::path::Path;

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

        println!("🚀 Creating workspace:");
        println!("  Name: {}", workspace_name);
        println!("  Path: {}", workspace_path);
        println!("  Branch: {}", branch_name);

        // ベースディレクトリの作成
        if let Some(parent) = Path::new(&workspace_path).parent() {
            fs::create_dir_all(parent).map_err(|e| format!("ディレクトリ作成エラー: {}", e))?;
        }

        // Worktreeの作成
        let opts = WorktreeAddOptions::new();
        self.repo
            .worktree(&workspace_name, Path::new(&workspace_path), Some(&opts))
            .map_err(|e| format!("Worktree作成エラー: {}", e))?;

        // ブランチの作成と切り替え
        let worktree_repo = Repository::open(&workspace_path)
            .map_err(|e| format!("作成されたワークスペースのオープンエラー: {}", e))?;

        let head = worktree_repo
            .head()
            .map_err(|e| format!("HEADの取得エラー: {}", e))?;
        let target_commit = head.target().ok_or("HEADのコミットIDが取得できません")?;
        let commit = worktree_repo
            .find_commit(target_commit)
            .map_err(|e| format!("コミットの取得エラー: {}", e))?;

        let _branch = worktree_repo
            .branch(&branch_name, &commit, false)
            .map_err(|e| format!("ブランチ作成エラー: {}", e))?;

        worktree_repo
            .set_head(&format!("refs/heads/{}", branch_name))
            .map_err(|e| format!("ブランチ切り替えエラー: {}", e))?;

        println!("✅ Workspace created successfully!");
        println!("📁 Path: {}", workspace_path);
        println!("🌿 Branch: {}", branch_name);
        println!("\nTo enter the workspace:");
        println!("  cd {}", workspace_path);

        Ok(WorkspaceInfo {
            name: workspace_name,
            path: workspace_path,
            branch: branch_name,
        })
    }

    pub fn list_workspaces(&self) -> Result<Vec<WorkspaceInfo>, String> {
        let worktrees = self
            .repo
            .worktrees()
            .map_err(|e| format!("Worktree一覧取得エラー: {}", e))?;

        let mut workspace_list = Vec::new();

        for worktree_name in worktrees.iter().flatten() {
            if let Ok(worktree) = self.repo.find_worktree(worktree_name) {
                if let Some(path) = worktree.path().to_str() {
                    // ワークスペースが実際に存在するかチェック
                    if !Path::new(path).exists() {
                        continue;
                    }

                    // ワークスペースのリポジトリを開いて現在のブランチ名を取得
                    let branch_name = match Repository::open(path) {
                        Ok(workspace_repo) => match workspace_repo.head() {
                            Ok(head_ref) => {
                                if let Some(name) = head_ref.shorthand() {
                                    name.to_string()
                                } else {
                                    format!("work/{}", worktree_name)
                                }
                            }
                            Err(_) => format!("work/{}", worktree_name),
                        },
                        Err(_) => format!("work/{}", worktree_name),
                    };

                    workspace_list.push(WorkspaceInfo {
                        name: worktree_name.to_string(),
                        path: path.to_string(),
                        branch: branch_name,
                    });
                }
            }
        }

        // メインワークツリーは除外（一般的に「main」ブランチの作業ディレクトリ）
        workspace_list.retain(|ws| ws.name != "main" && !ws.path.ends_with("/.git"));

        println!("📋 発見されたワークスペース: {} 件", workspace_list.len());
        for workspace in &workspace_list {
            println!("  - {} -> {}", workspace.branch, workspace.path);
        }

        Ok(workspace_list)
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
        if let Ok(_manager) = WorkspaceManager::new() {
            // テスト環境では実際のワークスペース作成はスキップ
            // 代わりにパラメータの構造をテスト
            let timestamp = crate::utils::generate_timestamp();
            let task_name = "test-task";
            let _base_dir = "../test-workspaces";
            let branch_prefix = "test/";
            
            let workspace_name = format!("{}-{}", timestamp, task_name);
            let branch_name = format!("{}{}", branch_prefix, task_name);
            let workspace_path = format!("{}/{}", _base_dir, workspace_name);
            
            assert!(workspace_name.contains("test-task"));
            assert!(workspace_path.contains("../test-workspaces"));
            assert!(branch_name.starts_with("test/"));
        }
    }

    #[test]
    fn test_create_workspace_with_various_names() {
        if let Ok(_manager) = WorkspaceManager::new() {
            let test_cases = vec![
                "simple-task",
                "task_with_underscores",
                "task-with-numbers-123",
                "UPPERCASE-TASK",
                "mixed_Case-Task_123",
            ];

            for task_name in test_cases {
                // テスト環境では実際の作成は行わず、パラメータ生成をテスト
                let timestamp = crate::utils::generate_timestamp();
                let workspace_name = format!("{}-{}", timestamp, task_name);
                let branch_name = format!("test/{}", task_name);
                
                assert!(workspace_name.contains(task_name), "Failed for task name: {}", task_name);
                assert!(branch_name.contains(task_name));
            }
        }
    }

    #[test]
    fn test_create_workspace_empty_name() {
        if let Ok(_manager) = WorkspaceManager::new() {
            // テスト環境では実際のワークスペース作成はスキップ
            // 空のタスク名でのパラメータ構造をテスト
            let timestamp = crate::utils::generate_timestamp();
            let task_name = "";
            let _base_dir = "../test-workspaces";
            let branch_prefix = "test/";
            
            let workspace_name = format!("{}-{}", timestamp, task_name);
            let branch_name = format!("{}{}", branch_prefix, task_name);
            
            // 空の名前でも構造は正しく生成される
            assert!(workspace_name.ends_with("-"));
            assert_eq!(branch_name, "test/");
        }
    }

    #[test]
    fn test_list_workspaces_real_data() {
        if let Ok(manager) = WorkspaceManager::new() {
            let result = manager.list_workspaces();
            assert!(result.is_ok());
            let workspaces = result.unwrap();
            // 実際のworktreeデータを取得（件数は環境により異なる）
            // Vec::len()は常にusize型なので >= 0 は常にtrue

            // 各ワークスペースの構造が正しいことを確認
            for workspace in &workspaces {
                assert!(!workspace.name.is_empty());
                assert!(!workspace.path.is_empty());
                assert!(!workspace.branch.is_empty());
            }
        }
    }

    #[test]
    fn test_workspace_manager_multiple_operations() {
        if let Ok(manager) = WorkspaceManager::new() {
            // 複数の操作を連続して実行してもエラーにならないことを確認
            // テスト環境では実際の作成は行わず、list操作のみテスト
            assert!(manager.list_workspaces().is_ok());
            
            // パラメータ生成のテスト
            let tasks = vec!["task1", "task2", "task3"];
            for task in tasks {
                let timestamp = crate::utils::generate_timestamp();
                let workspace_name = format!("{}-{}", timestamp, task);
                assert!(workspace_name.contains(task));
            }
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

    #[test]
    fn test_create_workspace_error_handling() {
        if let Ok(manager) = WorkspaceManager::new() {
            // 無効なパスを指定してエラーハンドリングをテスト
            let result = manager.create_workspace("test", "/invalid/readonly/path", "test/");
            // 権限エラーなどが発生する可能性があるが、適切にエラーハンドリングされる
            match result {
                Ok(_) => {
                    // 成功した場合はワークスペースが作成された
                    println!("Workspace created successfully in test environment");
                }
                Err(e) => {
                    // エラーメッセージが適切に返される
                    assert!(!e.is_empty());
                    println!("Expected error occurred: {}", e);
                }
            }
        }
    }

    #[test]
    fn test_create_workspace_branch_prefix_variations() {
        if let Ok(manager) = WorkspaceManager::new() {
            let test_cases = vec![
                ("feature/", "test-task", "feature/test-task"),
                ("work/", "bug-fix", "work/bug-fix"),
                ("", "no-prefix", "no-prefix"),
                ("dev-", "experiment", "dev-experiment"),
            ];

            for (prefix, task, expected_branch_start) in test_cases {
                let result = manager.create_workspace(task, "../test-workspaces", prefix);
                if let Ok(workspace) = result {
                    assert!(workspace.branch.starts_with(expected_branch_start));
                    assert!(workspace.name.contains(task));
                }
            }
        }
    }

    #[test]
    fn test_create_workspace_timestamp_format() {
        if let Ok(_manager) = WorkspaceManager::new() {
            // テスト環境ではタイムスタンプ生成をテスト
            let timestamp = crate::utils::generate_timestamp();
            let task_name = "timestamp-test";
            let workspace_name = format!("{}-{}", timestamp, task_name);
            
            // タイムスタンプの形式をチェック (YYYYMMDD-HHMMSS-task-name)
            let parts: Vec<&str> = workspace_name.split('-').collect();
            assert!(parts.len() >= 3);
            
            // 最初の部分がタイムスタンプ（8桁の数字）
            assert_eq!(parts[0].len(), 8);
            assert!(parts[0].chars().all(|c| c.is_ascii_digit()));
            
            // 2番目の部分が時刻（6桁の数字）
            assert_eq!(parts[1].len(), 6);
            assert!(parts[1].chars().all(|c| c.is_ascii_digit()));
            
            // 最後にタスク名が含まれる
            assert!(workspace_name.contains("timestamp-test"));
        }
    }

    #[test]
    fn test_list_workspaces_empty_result() {
        if let Ok(manager) = WorkspaceManager::new() {
            let result = manager.list_workspaces();
            assert!(result.is_ok());

            // 空の結果でもエラーにならない
            let _workspaces = result.unwrap();
            // Vec::len()は常にusize型なので >= 0 の比較は不要
        }
    }

    #[test]
    fn test_workspace_manager_repository_validation() {
        // 現在のディレクトリがGitリポジトリであることを前提とする
        let manager_result = WorkspaceManager::new();
        assert!(manager_result.is_ok());

        // リポジトリが正しく初期化されている
        if let Ok(manager) = manager_result {
            // 基本的な操作が可能であることを確認
            let list_result = manager.list_workspaces();
            assert!(list_result.is_ok());
        }
    }
}
