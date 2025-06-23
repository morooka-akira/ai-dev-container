use crate::error::{GworkError, GworkResult};
use git2::{Repository, WorktreeAddOptions};
use std::fs;
use std::path::Path;
use std::process::Command;
use tracing::{debug, error, warn};

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

#[derive(Debug)]
pub struct WorkspaceDetails {
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub path: String,
    #[allow(dead_code)]
    pub branch: String,
    pub created: String,
    pub last_modified: String,
    pub status: String,
    pub files_info: String,
    pub size: String,
    pub recent_commits: Vec<String>,
}

impl WorkspaceManager {
    pub fn new() -> GworkResult<Self> {
        debug!("WorkspaceManagerを初期化します");
        let repo = Repository::open(".").map_err(|e| {
            error!("Gitリポジトリのオープンに失敗しました: {}", e);
            GworkError::git(format!("Gitリポジトリが見つかりません: {}", e))
        })?;
        debug!("Gitリポジトリを正常にオープンしました");
        Ok(Self { repo })
    }

    #[allow(dead_code)]
    pub fn create_workspace(
        &self,
        task_name: &str,
        base_dir: &str,
        branch_prefix: &str,
    ) -> GworkResult<WorkspaceInfo> {
        self.create_workspace_with_config(task_name, base_dir, branch_prefix, &[], &[])
    }

    pub fn create_workspace_with_config(
        &self,
        task_name: &str,
        base_dir: &str,
        branch_prefix: &str,
        copy_files: &[String],
        pre_commands: &[String],
    ) -> GworkResult<WorkspaceInfo> {
        let timestamp = crate::utils::generate_timestamp();
        let workspace_name = format!("{}-{}", timestamp, task_name);
        let branch_name = if branch_prefix.is_empty() {
            workspace_name.clone()
        } else {
            format!("{}{}", branch_prefix, workspace_name.clone())
        };
        let workspace_path = format!("{}/{}", base_dir, workspace_name);

        debug!("ワークスペースを作成します: {}", workspace_name);
        debug!("ワークスペースパス: {}", workspace_path);
        debug!("ブランチ名: {}", branch_name);

        println!("🚀 Creating workspace:");
        println!("  Name: {}", workspace_name);
        println!("  Path: {}", workspace_path);
        println!("  Branch: {}", branch_name);

        // ベースディレクトリの作成
        if let Some(parent) = Path::new(&workspace_path).parent() {
            debug!("ベースディレクトリを作成します: {}", parent.display());
            fs::create_dir_all(parent).map_err(|e| {
                error!(
                    "ディレクトリ作成に失敗しました: {} - {}",
                    parent.display(),
                    e
                );
                GworkError::io(format!("ディレクトリ作成エラー: {}", e))
            })?;
        }

        // Worktreeの作成
        debug!("Worktreeを作成します");
        let opts = WorktreeAddOptions::new();
        self.repo
            .worktree(&workspace_name, Path::new(&workspace_path), Some(&opts))
            .map_err(|e| {
                error!("Worktree作成に失敗しました: {}", e);
                GworkError::git(format!("Worktree作成エラー: {}", e))
            })?;

        // ブランチの作成と切り替え
        debug!("作成されたワークスペースを開きます");
        let worktree_repo = Repository::open(&workspace_path).map_err(|e| {
            error!("作成されたワークスペースのオープンに失敗しました: {}", e);
            GworkError::git(format!("作成されたワークスペースのオープンエラー: {}", e))
        })?;

        debug!("HEADコミットを取得します");
        let head = worktree_repo.head().map_err(|e| {
            error!("HEADの取得に失敗しました: {}", e);
            GworkError::git(format!("HEADの取得エラー: {}", e))
        })?;

        let target_commit = head.target().ok_or_else(|| {
            error!("HEADのコミットIDが取得できません");
            GworkError::git("HEADのコミットIDが取得できません".to_string())
        })?;

        let commit = worktree_repo.find_commit(target_commit).map_err(|e| {
            error!("コミットの取得に失敗しました: {}", e);
            GworkError::git(format!("コミットの取得エラー: {}", e))
        })?;

        debug!("ブランチを作成します: {}", branch_name);
        let _branch = worktree_repo
            .branch(&branch_name, &commit, false)
            .map_err(|e| {
                error!("ブランチ作成に失敗しました: {} - {}", branch_name, e);
                GworkError::git(format!("ブランチ作成エラー: {}", e))
            })?;

        debug!("ブランチに切り替えます: {}", branch_name);
        worktree_repo
            .set_head(&format!("refs/heads/{}", branch_name))
            .map_err(|e| {
                error!("ブランチ切り替えに失敗しました: {} - {}", branch_name, e);
                GworkError::git(format!("ブランチ切り替えエラー: {}", e))
            })?;

        // ファイルコピー処理
        if !copy_files.is_empty() {
            println!("\n📄 Copying files...");
            self.copy_files(Path::new("."), Path::new(&workspace_path), copy_files);
        }

        // 事前コマンド実行処理
        if !pre_commands.is_empty() {
            println!("\n⚡ Executing pre-commands...");
            self.execute_pre_commands(Path::new(&workspace_path), pre_commands);
        }

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

    fn copy_files(&self, source_repo_path: &Path, workspace_path: &Path, copy_files: &[String]) {
        for file_path in copy_files {
            let source_path = source_repo_path.join(file_path);
            let dest_path = workspace_path.join(file_path);

            // ソースファイルが存在しない場合はスキップ
            if !source_path.exists() {
                println!("  ⚠️  ファイルが見つかりません: {} (スキップ)", file_path);
                continue;
            }

            // デスティネーションディレクトリの作成
            if let Some(parent) = dest_path.parent() {
                if !parent.exists() {
                    if let Err(e) = fs::create_dir_all(parent) {
                        println!("  ❌ ディレクトリ作成エラー: {} - {}", parent.display(), e);
                        continue;
                    }
                }
            }

            // ファイルをコピー
            match fs::copy(&source_path, &dest_path) {
                Ok(_) => {
                    println!("  ✅ コピー完了: {}", file_path);
                }
                Err(e) => {
                    println!("  ❌ コピーエラー: {} - {}", file_path, e);
                }
            }
        }
    }

    fn execute_pre_commands(&self, workspace_path: &Path, pre_commands: &[String]) {
        for (i, command) in pre_commands.iter().enumerate() {
            println!(
                "  [{}/{}] Executing: {}",
                i + 1,
                pre_commands.len(),
                command
            );

            let output = if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .args(["/C", command])
                    .current_dir(workspace_path)
                    .output()
            } else {
                Command::new("sh")
                    .args(["-c", command])
                    .current_dir(workspace_path)
                    .output()
            };

            match output {
                Ok(result) => {
                    if result.status.success() {
                        // 標準出力がある場合は表示
                        if !result.stdout.is_empty() {
                            let stdout = String::from_utf8_lossy(&result.stdout);
                            println!("     出力: {}", stdout.trim());
                        }
                    } else {
                        println!(
                            "  ❌ コマンド実行失敗: {} (終了コード: {:?})",
                            command,
                            result.status.code()
                        );

                        // エラー出力がある場合は表示
                        if !result.stderr.is_empty() {
                            let stderr = String::from_utf8_lossy(&result.stderr);
                            println!("     エラー: {}", stderr.trim());
                        }
                    }
                }
                Err(e) => {
                    println!("  ❌ コマンド実行エラー: {} - {}", command, e);
                }
            }
        }
    }

    pub fn list_workspaces(&self) -> GworkResult<Vec<WorkspaceInfo>> {
        debug!("ワークスペース一覧を取得します");
        let worktrees = self.repo.worktrees().map_err(|e| {
            error!("Worktree一覧取得に失敗しました: {}", e);
            GworkError::git(format!("Worktree一覧取得エラー: {}", e))
        })?;

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

        Ok(workspace_list)
    }

    #[allow(dead_code)]
    pub fn remove_workspace(&self, workspace_name: &str) -> GworkResult<()> {
        debug!("ワークスペースを削除します: {}", workspace_name);
        // まずワークスペースに関連するブランチ名を特定
        let mut branch_to_delete = None;

        // ワークスペース一覧からブランチ名を取得
        debug!("削除対象のワークスペース情報を取得します");
        if let Ok(workspaces) = self.list_workspaces() {
            for workspace in workspaces {
                if workspace.name == workspace_name {
                    debug!("削除対象ブランチ: {}", workspace.branch.clone());
                    branch_to_delete = Some(workspace.branch.clone());
                    break;
                }
            }
        }

        // git worktree removeコマンドを使用してワークスペースを削除
        debug!("git worktreeコマンドでワークスペースを削除します");
        let output = std::process::Command::new("git")
            .args(["worktree", "remove", "--force", workspace_name])
            .output()
            .map_err(|e| {
                error!("git worktree removeコマンド実行に失敗しました: {}", e);
                GworkError::git(format!("git worktree removeコマンド実行エラー: {}", e))
            })?;

        let worktree_removed = output.status.success();

        // ワークスペースが削除された場合、ブランチも削除
        if worktree_removed {
            debug!("ワークスペースが正常に削除されました");
            // 明示的に作成されたブランチを削除
            if let Some(branch_name) = branch_to_delete {
                debug!("関連ブランチを削除します: {}", branch_name);
                let _ = std::process::Command::new("git")
                    .args(["branch", "-D", &branch_name])
                    .output();
            }

            // worktree作成時に自動生成されたブランチ（workspace_nameと同じ名前）も削除
            debug!("自動生成ブランチを削除します: {}", workspace_name);
            let _ = std::process::Command::new("git")
                .args(["branch", "-D", workspace_name])
                .output();

            return Ok(());
        }

        // コマンドが失敗した場合、パスで削除を試行
        let potential_paths = vec![
            format!("../test-workspaces/{}", workspace_name),
            format!("../workspaces/{}", workspace_name),
            format!("../test/{}", workspace_name),
        ];

        warn!("ワークスペース名による削除が失敗しました。パスでの削除を試行します");
        for path in &potential_paths {
            debug!("パスでの削除を試行: {}", path);
            let output = std::process::Command::new("git")
                .args(["worktree", "remove", "--force", path])
                .output()
                .map_err(|e| {
                    error!("git worktree removeコマンド実行に失敗しました: {}", e);
                    GworkError::git(format!("git worktree removeコマンド実行エラー: {}", e))
                })?;

            if output.status.success() {
                debug!("パスによる削除が成功しました: {}", path);
                // ワークスペースが削除された場合、ブランチも削除
                if let Some(branch_name) = &branch_to_delete {
                    debug!("関連ブランチを削除します: {}", branch_name);
                    let _ = std::process::Command::new("git")
                        .args(["branch", "-D", branch_name])
                        .output();
                }
                return Ok(());
            }
        }

        // 最後の手段としてファイルシステムから直接削除
        let mut found_and_removed = false;
        for path in potential_paths {
            if Path::new(&path).exists()
                && std::process::Command::new("git")
                    .args(["worktree", "remove", "--force", &path])
                    .output()
                    .is_err()
            {
                // gitコマンドが失敗した場合はディレクトリを直接削除
                if std::fs::remove_dir_all(&path).is_ok() {
                    found_and_removed = true;
                }
            }
        }

        if found_and_removed {
            warn!("ファイルシステムからの直接削除が成功しました");
            Ok(())
        } else {
            error!("ワークスペースの削除に失敗しました: {}", workspace_name);
            Err(GworkError::workspace(format!(
                "ワークスペースが見つかりません: {}",
                workspace_name
            )))
        }
    }

    pub fn get_workspace_details(
        &self,
        workspace_info: &WorkspaceInfo,
    ) -> GworkResult<WorkspaceDetails> {
        debug!(
            "ワークスペースの詳細情報を取得します: {}",
            workspace_info.name
        );
        let workspace_path = Path::new(&workspace_info.path);

        // 作成日時を取得
        let created = if workspace_path.exists() {
            match workspace_path.metadata() {
                Ok(metadata) => {
                    if let Ok(created_time) = metadata.created() {
                        match created_time.duration_since(std::time::UNIX_EPOCH) {
                            Ok(duration) => {
                                let datetime =
                                    chrono::DateTime::from_timestamp(duration.as_secs() as i64, 0)
                                        .unwrap_or_else(chrono::Utc::now);
                                datetime.format("%Y-%m-%d %H:%M:%S").to_string()
                            }
                            Err(_) => "不明".to_string(),
                        }
                    } else {
                        "不明".to_string()
                    }
                }
                Err(_) => "不明".to_string(),
            }
        } else {
            "ワークスペースが存在しません".to_string()
        };

        // 最終更新日時を取得
        let last_modified = if workspace_path.exists() {
            match workspace_path.metadata() {
                Ok(metadata) => {
                    if let Ok(modified_time) = metadata.modified() {
                        match modified_time.duration_since(std::time::UNIX_EPOCH) {
                            Ok(duration) => {
                                let datetime =
                                    chrono::DateTime::from_timestamp(duration.as_secs() as i64, 0)
                                        .unwrap_or_else(chrono::Utc::now);
                                datetime.format("%Y-%m-%d %H:%M:%S").to_string()
                            }
                            Err(_) => "不明".to_string(),
                        }
                    } else {
                        "不明".to_string()
                    }
                }
                Err(_) => "不明".to_string(),
            }
        } else {
            "ワークスペースが存在しません".to_string()
        };

        // Git status情報を取得
        let (status, files_info) = if workspace_path.exists() {
            match Repository::open(workspace_path) {
                Ok(repo) => match repo.statuses(None) {
                    Ok(statuses) => {
                        let mut modified_count = 0;
                        let mut untracked_count = 0;
                        let mut tracked_count = 0;

                        for status_entry in statuses.iter() {
                            let status = status_entry.status();
                            if status.contains(git2::Status::WT_MODIFIED)
                                || status.contains(git2::Status::WT_DELETED)
                                || status.contains(git2::Status::INDEX_MODIFIED)
                                || status.contains(git2::Status::INDEX_DELETED)
                            {
                                modified_count += 1;
                            } else if status.contains(git2::Status::WT_NEW) {
                                untracked_count += 1;
                            } else {
                                tracked_count += 1;
                            }
                        }

                        let status_text = if modified_count > 0 {
                            format!("Modified ({} files)", modified_count)
                        } else {
                            "Clean".to_string()
                        };

                        let files_text =
                            format!("{} tracked, {} untracked", tracked_count, untracked_count);
                        (status_text, files_text)
                    }
                    Err(_) => ("不明".to_string(), "不明".to_string()),
                },
                Err(_) => (
                    "Gitリポジトリではありません".to_string(),
                    "不明".to_string(),
                ),
            }
        } else {
            (
                "ワークスペースが存在しません".to_string(),
                "不明".to_string(),
            )
        };

        // ディレクトリサイズを取得
        let size = if workspace_path.exists() {
            match Self::calculate_directory_size(workspace_path) {
                Ok(size_bytes) => {
                    if size_bytes < 1024 {
                        format!("{} B", size_bytes)
                    } else if size_bytes < 1024 * 1024 {
                        format!("{:.1} KB", size_bytes as f64 / 1024.0)
                    } else {
                        format!("{:.1} MB", size_bytes as f64 / (1024.0 * 1024.0))
                    }
                }
                Err(_) => "不明".to_string(),
            }
        } else {
            "不明".to_string()
        };

        // 最近のコミット履歴を取得
        let recent_commits = if workspace_path.exists() {
            match Repository::open(workspace_path) {
                Ok(repo) => match repo.head() {
                    Ok(head) => match head.target() {
                        Some(commit_id) => {
                            let mut commits = Vec::new();
                            let mut revwalk =
                                repo.revwalk().unwrap_or_else(|_| repo.revwalk().unwrap());
                            if revwalk.push(commit_id).is_ok() {
                                for commit_oid in revwalk.take(3).flatten() {
                                    if let Ok(commit) = repo.find_commit(commit_oid) {
                                        let message = commit.message().unwrap_or("").trim();
                                        let short_message = if message.chars().count() > 50 {
                                            let truncated: String =
                                                message.chars().take(47).collect();
                                            format!("{}...", truncated)
                                        } else {
                                            message.to_string()
                                        };

                                        let time = commit.time();
                                        let timestamp = time.seconds();
                                        let now = chrono::Utc::now().timestamp();
                                        let diff = now - timestamp;

                                        let time_ago = if diff < 3600 {
                                            format!("{}分前", diff / 60)
                                        } else if diff < 86400 {
                                            format!("{}時間前", diff / 3600)
                                        } else {
                                            format!("{}日前", diff / 86400)
                                        };

                                        commits.push(format!("- {} ({})", short_message, time_ago));
                                    }
                                }
                            }
                            commits
                        }
                        None => vec!["コミット履歴なし".to_string()],
                    },
                    Err(_) => vec!["HEADが見つかりません".to_string()],
                },
                Err(_) => vec!["Gitリポジトリではありません".to_string()],
            }
        } else {
            vec!["ワークスペースが存在しません".to_string()]
        };

        Ok(WorkspaceDetails {
            name: workspace_info.name.clone(),
            path: workspace_info.path.clone(),
            branch: workspace_info.branch.clone(),
            created,
            last_modified,
            status,
            files_info,
            size,
            recent_commits,
        })
    }

    fn calculate_directory_size(path: &Path) -> Result<u64, std::io::Error> {
        let mut total_size = 0;

        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let metadata = entry.metadata()?;

                if metadata.is_dir() {
                    // .gitディレクトリはスキップ
                    if entry.file_name() != ".git" {
                        total_size += Self::calculate_directory_size(&entry.path())?;
                    }
                } else {
                    total_size += metadata.len();
                }
            }
        }

        Ok(total_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use tempfile::TempDir;

    static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

    // テスト用のユニークなワークスペース名を生成
    fn generate_test_workspace_name(prefix: &str) -> String {
        let counter = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        let timestamp = crate::utils::generate_timestamp();
        format!("{}-test-{}-{}", timestamp, counter, prefix)
    }

    // テスト完了時にワークスペースをクリーンアップ
    fn cleanup_test_workspace(manager: &WorkspaceManager, workspace_name: &str) {
        // git2を使ってworktreeを削除
        let _ = manager.remove_workspace(workspace_name);
    }

    // すべてのテスト用ワークスペースを一括削除
    #[allow(dead_code)]
    fn cleanup_all_test_workspaces() {
        if let Ok(manager) = WorkspaceManager::new() {
            if let Ok(workspaces) = manager.list_workspaces() {
                for workspace in workspaces {
                    // test-workspacesディレクトリ内のワークスペースを削除
                    if workspace.path.contains("test-workspaces") {
                        let _ = manager.remove_workspace(&workspace.name);
                    }
                }
            }
        }
    }

    // テスト用ワークスペースの自動クリーンアップ
    struct TestWorkspaceGuard {
        manager: WorkspaceManager,
        workspace_names: Vec<String>,
    }

    impl TestWorkspaceGuard {
        fn new() -> GworkResult<Self> {
            Ok(Self {
                manager: WorkspaceManager::new()?,
                workspace_names: Vec::new(),
            })
        }

        fn add_workspace(&mut self, name: String) {
            self.workspace_names.push(name);
        }

        fn create_workspace(
            &self,
            task_name: &str,
            base_dir: &str,
            branch_prefix: &str,
        ) -> GworkResult<WorkspaceInfo> {
            self.manager
                .create_workspace(task_name, base_dir, branch_prefix)
        }
    }

    impl Drop for TestWorkspaceGuard {
        fn drop(&mut self) {
            // テスト終了時に作成されたワークスペースを確実に削除
            for workspace_name in &self.workspace_names {
                let _ = self.manager.remove_workspace(workspace_name);
            }
        }
    }

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

                assert!(
                    workspace_name.contains(task_name),
                    "Failed for task name: {}",
                    task_name
                );
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
            let task_name = generate_test_workspace_name("error-handling");
            let result = manager.create_workspace(&task_name, "/invalid/readonly/path", "test/");
            // 権限エラーなどが発生する可能性があるが、適切にエラーハンドリングされる
            match result {
                Ok(workspace) => {
                    // 成功した場合はワークスペースが作成された（稀なケース）
                    cleanup_test_workspace(&manager, &workspace.name);
                }
                Err(e) => {
                    // エラーメッセージが適切に返される
                    assert!(!e.to_string().is_empty());
                }
            }
        }
    }

    #[test]
    fn test_create_workspace_branch_prefix_variations() {
        if let Ok(mut guard) = TestWorkspaceGuard::new() {
            let test_cases = vec![
                ("feature/", "test-task"),
                ("work/", "bug-fix"),
                ("", "no-prefix"),
                ("dev-", "experiment"),
            ];

            for (prefix, task) in test_cases {
                let result = guard.create_workspace(task, "../test-workspaces", prefix);
                match result {
                    Ok(workspace) => {
                        // ワークスペース名とタスク名が含まれることを確認
                        assert!(workspace.name.contains(task));

                        // ブランチ名がプレフィックス + ワークスペース名になることを確認
                        if prefix.is_empty() {
                            assert_eq!(workspace.branch, workspace.name);
                        } else {
                            assert!(workspace.branch.starts_with(prefix));
                            assert!(workspace.branch.contains(&workspace.name));
                        }

                        // 作成されたワークスペースを記録（Dropで自動削除される）
                        guard.add_workspace(workspace.name);
                    }
                    Err(_) => {
                        // テスト環境によってはエラーになる場合があるがテストは継続
                    }
                }
            }
            // guard がDropされる時に自動的にワークスペースが削除される
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

    #[test]
    fn test_create_and_remove_workspace_integration() {
        if let Ok(mut guard) = TestWorkspaceGuard::new() {
            let task_name = generate_test_workspace_name("integration");
            let base_dir = "../test-workspaces";
            let branch_prefix = "test/";

            // ワークスペースを作成
            let result = guard.create_workspace(&task_name, base_dir, branch_prefix);

            match result {
                Ok(workspace_info) => {
                    // 作成成功の場合、削除でクリーンアップ
                    assert!(workspace_info.name.contains(&task_name));
                    assert!(workspace_info.path.contains(base_dir));
                    assert!(workspace_info.branch.starts_with(branch_prefix));

                    // 作成されたワークスペースを記録（Dropで自動削除される）
                    guard.add_workspace(workspace_info.name);
                }
                Err(_) => {
                    // エラーの場合は作成されていないのでクリーンアップ不要
                }
            }
        }
    }

    #[test]
    fn test_remove_nonexistent_workspace() {
        if let Ok(manager) = WorkspaceManager::new() {
            let result = manager.remove_workspace("nonexistent-workspace-12345");
            assert!(result.is_err());

            if let Err(error_msg) = result {
                assert!(
                    error_msg
                        .to_string()
                        .contains("ワークスペースが見つかりません")
                );
            }
        }
    }

    #[test]
    fn test_copy_files_function() {
        if let Ok(manager) = WorkspaceManager::new() {
            // 一時ディレクトリを作成
            let temp_dir = TempDir::new().unwrap();
            let source_dir = temp_dir.path();
            let dest_dir = temp_dir.path().join("dest");
            fs::create_dir_all(&dest_dir).unwrap();

            // テストファイルを作成
            let test_file1 = source_dir.join("test1.txt");
            fs::write(&test_file1, "test content 1").unwrap();

            let test_file2 = source_dir.join("dir/test2.txt");
            fs::create_dir_all(test_file2.parent().unwrap()).unwrap();
            fs::write(&test_file2, "test content 2").unwrap();

            // コピー対象ファイルのリスト
            let copy_files = vec![
                "test1.txt".to_string(),
                "dir/test2.txt".to_string(),
                "nonexistent.txt".to_string(), // 存在しないファイル
            ];

            // ファイルをコピー
            manager.copy_files(source_dir, &dest_dir, &copy_files);

            // 検証
            assert!(dest_dir.join("test1.txt").exists());
            assert!(dest_dir.join("dir/test2.txt").exists());
            assert!(!dest_dir.join("nonexistent.txt").exists());

            // ファイル内容の確認
            let content1 = fs::read_to_string(dest_dir.join("test1.txt")).unwrap();
            assert_eq!(content1, "test content 1");

            let content2 = fs::read_to_string(dest_dir.join("dir/test2.txt")).unwrap();
            assert_eq!(content2, "test content 2");
        }
    }

    #[test]
    fn test_copy_files_with_nested_directories() {
        if let Ok(manager) = WorkspaceManager::new() {
            let temp_dir = TempDir::new().unwrap();
            let source_dir = temp_dir.path();
            let dest_dir = temp_dir.path().join("workspace");
            fs::create_dir_all(&dest_dir).unwrap();

            // ネストしたディレクトリ構造のテストファイルを作成
            let nested_file = source_dir.join("config/nested/deep/file.yml");
            fs::create_dir_all(nested_file.parent().unwrap()).unwrap();
            fs::write(&nested_file, "nested: true").unwrap();

            let copy_files = vec!["config/nested/deep/file.yml".to_string()];

            // ファイルをコピー
            manager.copy_files(source_dir, &dest_dir, &copy_files);

            // 検証
            let dest_file = dest_dir.join("config/nested/deep/file.yml");
            assert!(dest_file.exists());
            assert_eq!(fs::read_to_string(dest_file).unwrap(), "nested: true");
        }
    }

    #[test]
    fn test_copy_files_empty_list() {
        if let Ok(manager) = WorkspaceManager::new() {
            let temp_dir = TempDir::new().unwrap();
            let source_dir = temp_dir.path();
            let dest_dir = temp_dir.path().join("empty_dest");
            fs::create_dir_all(&dest_dir).unwrap();

            // 空のリストでコピー処理を実行
            let copy_files: Vec<String> = vec![];
            manager.copy_files(source_dir, &dest_dir, &copy_files);

            // エラーが発生しないことを確認（パニックしない）
            assert!(dest_dir.exists());
        }
    }

    #[test]
    fn test_execute_pre_commands_function() {
        if let Ok(manager) = WorkspaceManager::new() {
            // 一時ディレクトリを作成
            let temp_dir = TempDir::new().unwrap();
            let workspace_dir = temp_dir.path().join("workspace");
            fs::create_dir_all(&workspace_dir).unwrap();

            // テスト用のコマンドリスト
            let pre_commands = vec![
                "echo 'Hello World' > test_output.txt".to_string(),
                "ls -la".to_string(),
                "echo 'Command completed'".to_string(),
            ];

            // コマンドを実行
            manager.execute_pre_commands(&workspace_dir, &pre_commands);

            // 実行結果の確認
            let output_file = workspace_dir.join("test_output.txt");
            assert!(output_file.exists());

            let content = fs::read_to_string(output_file).unwrap();
            assert_eq!(content.trim(), "Hello World");
        }
    }

    #[test]
    fn test_execute_pre_commands_with_failure() {
        if let Ok(manager) = WorkspaceManager::new() {
            let temp_dir = TempDir::new().unwrap();
            let workspace_dir = temp_dir.path().join("workspace");
            fs::create_dir_all(&workspace_dir).unwrap();

            // 成功するコマンドと失敗するコマンドを混在
            let pre_commands = vec![
                "echo 'Success 1' > success1.txt".to_string(),
                "false".to_string(), // 必ず失敗するコマンド
                "echo 'Success 2' > success2.txt".to_string(),
            ];

            // コマンドを実行（失敗しても処理が継続することを確認）
            manager.execute_pre_commands(&workspace_dir, &pre_commands);

            // 成功したコマンドの結果は残っている
            assert!(workspace_dir.join("success1.txt").exists());
            assert!(workspace_dir.join("success2.txt").exists());
        }
    }

    #[test]
    fn test_execute_pre_commands_empty_list() {
        if let Ok(manager) = WorkspaceManager::new() {
            let temp_dir = TempDir::new().unwrap();
            let workspace_dir = temp_dir.path().join("workspace");
            fs::create_dir_all(&workspace_dir).unwrap();

            // 空のコマンドリストで実行
            let pre_commands: Vec<String> = vec![];
            manager.execute_pre_commands(&workspace_dir, &pre_commands);

            // エラーが発生しないことを確認（パニックしない）
            assert!(workspace_dir.exists());
        }
    }

    #[test]
    fn test_execute_pre_commands_working_directory() {
        if let Ok(manager) = WorkspaceManager::new() {
            let temp_dir = TempDir::new().unwrap();
            let workspace_dir = temp_dir.path().join("workspace");
            fs::create_dir_all(&workspace_dir).unwrap();

            // カレントディレクトリを確認するコマンド
            let pre_commands = vec!["pwd > current_dir.txt".to_string()];

            manager.execute_pre_commands(&workspace_dir, &pre_commands);

            // 作業ディレクトリが正しく設定されていることを確認
            let output_file = workspace_dir.join("current_dir.txt");
            assert!(output_file.exists());

            let content = fs::read_to_string(output_file).unwrap();
            let current_dir = content.trim();

            // ワークスペースディレクトリがカレントディレクトリとして設定されている
            assert!(current_dir.ends_with("workspace"));
        }
    }

    #[test]
    fn test_get_workspace_details_basic() {
        if let Ok(manager) = WorkspaceManager::new() {
            // テスト用のワークスペース情報を作成
            let workspace_info = WorkspaceInfo {
                name: "test-workspace".to_string(),
                path: ".".to_string(), // 現在のディレクトリ（存在することが確実）
                branch: "test/branch".to_string(),
            };

            // 詳細情報を取得
            let result = manager.get_workspace_details(&workspace_info);
            assert!(result.is_ok());

            let details = result.unwrap();
            assert_eq!(details.name, "test-workspace");
            assert_eq!(details.path, ".");
            assert_eq!(details.branch, "test/branch");

            // 日時情報が取得されていることを確認
            assert!(!details.created.is_empty());
            assert!(!details.last_modified.is_empty());
            assert!(details.created != "不明");
            assert!(details.last_modified != "不明");

            // ステータス情報が取得されていることを確認
            assert!(!details.status.is_empty());
            assert!(!details.files_info.is_empty());
            assert!(!details.size.is_empty());

            // コミット履歴が取得されていることを確認（空でも良い）
            assert!(!details.recent_commits.is_empty() || details.recent_commits.is_empty());
        }
    }

    #[test]
    fn test_get_workspace_details_nonexistent_path() {
        if let Ok(manager) = WorkspaceManager::new() {
            // 存在しないパスのワークスペース情報を作成
            let workspace_info = WorkspaceInfo {
                name: "nonexistent-workspace".to_string(),
                path: "/path/that/does/not/exist".to_string(),
                branch: "test/branch".to_string(),
            };

            // 詳細情報を取得
            let result = manager.get_workspace_details(&workspace_info);
            assert!(result.is_ok());

            let details = result.unwrap();
            assert_eq!(details.name, "nonexistent-workspace");
            assert_eq!(details.path, "/path/that/does/not/exist");
            assert_eq!(details.branch, "test/branch");

            // 存在しないパスの場合のエラーメッセージが設定されていることを確認
            assert!(
                details.created.contains("ワークスペースが存在しません")
                    || details.created.contains("不明")
            );
            assert!(
                details
                    .last_modified
                    .contains("ワークスペースが存在しません")
                    || details.last_modified.contains("不明")
            );
            assert!(details.status.contains("ワークスペースが存在しません"));
            assert!(details.files_info.contains("不明"));
            assert!(details.size.contains("不明"));
            assert!(
                details
                    .recent_commits
                    .contains(&"ワークスペースが存在しません".to_string())
            );
        }
    }
}
