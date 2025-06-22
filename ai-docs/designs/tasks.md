# 実装タスクの分割計画

## 概要

design.mdに基づいて、メンテナンス性と独立性を重視したステップバイステップでの実装タスクを定義する。

## 実装戦略

- **小さく独立した機能単位で実装**
- **各ステップでテスト可能な状態を維持**
- **依存関係を最小化**
- **段階的な機能追加**

## 作業ルール

⚠️ **重要**: 作業内容は実装前にもう一度整理し、詳細化できるなら詳細化してドキュメントを更新してから作業を開始すること
⚠️ **重要**: 作業前に @ai-docs/designs/design.md を参照し、実装設計を確認すること


## 実装タスク一覧

### Phase 1: 基本CLI構造の構築

#### Task 1-1: 基本CLIフレームワークの実装
- [x] **目的**: コマンド構造の基盤を作成
- **詳細実装手順**:
  1. [x] `Cargo.toml`に`clap = { version = "4.0", features = ["derive"] }`を追加
  2. [x] `src/cli.rs`を作成し、以下を実装:
     ```rust
     use clap::{Parser, Subcommand};
     
     #[derive(Parser)]
     #[command(name = "ai-workspace")]
     #[command(about = "AI workspace management tool")]
     pub struct Cli {
         #[command(subcommand)]
         pub command: Commands,
     }
     
     #[derive(Subcommand)]
     pub enum Commands {
         Start { task_name: String },
         List,
     }
     ```
  3. [x] `src/main.rs`を更新:
     ```rust
     mod cli;
     use clap::Parser;
     use cli::{Cli, Commands};
     
     fn main() {
         let cli = Cli::parse();
         match cli.command {
             Commands::Start { task_name } => {
                 println!("start コマンドが実行されました: {}", task_name);
             }
             Commands::List => {
                 println!("list コマンドが実行されました");
             }
         }
     }
     ```
  4. [x] `cargo run -- start test-task` でテスト実行
  5. [x] `cargo run -- list` でテスト実行
  6. [x] `cargo run -- --help` でヘルプ表示確認
- **完了条件**: 
  - [x] 両コマンドがエラーなく実行でき、適切なログが出力される
  - [x] ヘルプメッセージが適切に表示される

#### Task 1-2: プロジェクト構造の整備
- [x] **目的**: 必要なモジュールファイルの作成
- **詳細実装手順**:
  1. [x] `src/workspace.rs`を作成:
     ```rust
     pub struct WorkspaceManager;
     
     impl WorkspaceManager {
         pub fn new() -> Self {
             Self
         }
         
         pub fn create_workspace(&self, task_name: &str) -> Result<(), String> {
             println!("Creating workspace for: {}", task_name);
             Ok(())
         }
         
         pub fn list_workspaces(&self) -> Result<Vec<String>, String> {
             println!("Listing workspaces");
             Ok(vec![])
         }
     }
     ```
  2. [x] `src/config.rs`を作成:
     ```rust
     pub struct WorkspaceConfig {
         pub base_dir: String,
         pub branch_prefix: String,
     }
     
     impl Default for WorkspaceConfig {
         fn default() -> Self {
             Self {
                 base_dir: "../workspaces".to_string(),
                 branch_prefix: "work/".to_string(),
             }
         }
     }
     
     pub fn load_config() -> WorkspaceConfig {
         println!("Loading config (using defaults for now)");
         WorkspaceConfig::default()
     }
     ```
  3. [x] `src/utils.rs`を作成:
     ```rust
     use chrono::{DateTime, Local};
     
     pub fn generate_timestamp() -> String {
         let now: DateTime<Local> = Local::now();
         now.format("%Y%m%d-%H%M%S").to_string()
     }
     ```
  4. [x] `Cargo.toml`に`chrono = { version = "0.4", features = ["serde"] }`を追加
  5. [x] `src/main.rs`を更新してモジュールを使用:
     ```rust
     mod cli;
     mod workspace;
     mod config;
     mod utils;
     
     use clap::Parser;
     use cli::{Cli, Commands};
     use workspace::WorkspaceManager;
     use config::load_config;
     
     fn main() {
         let cli = Cli::parse();
         let _config = load_config();
         let workspace_manager = WorkspaceManager::new();
         
         match cli.command {
             Commands::Start { task_name } => {
                 println!("start コマンドが実行されました: {}", task_name);
                 let _ = workspace_manager.create_workspace(&task_name);
             }
             Commands::List => {
                 println!("list コマンドが実行されました");
                 let _ = workspace_manager.list_workspaces();
             }
         }
     }
     ```
  6. [x] `cargo check`でコンパイル確認
  7. [x] `cargo run -- start test-task`で動作確認
- **完了条件**: 
  - [x] 全モジュールが正常にコンパイルできる
  - [x] 各モジュールの関数が呼び出され、ログが出力される

### Phase 2: 設定ファイル機能の実装

#### Task 2-1: 設定ファイル構造体の定義
- [x] **目的**: 設定ファイルのデータ構造を定義
- **詳細実装手順**:
  1. [x] `Cargo.toml`に以下を追加:
     ```toml
     serde = { version = "1.0", features = ["derive"] }
     serde_yaml = "0.9"
     ```
  2. [x] `src/config.rs`を以下に更新:
     ```rust
     use serde::{Deserialize, Serialize};
     
     #[derive(Debug, Serialize, Deserialize, Clone)]
     pub struct WorkspaceConfig {
         pub workspace: WorkspaceSettings,
     }
     
     #[derive(Debug, Serialize, Deserialize, Clone)]
     pub struct WorkspaceSettings {
         pub base_dir: String,
         pub branch_prefix: String,
         pub copy_files: Vec<String>,
         pub pre_commands: Vec<String>,
     }
     
     impl Default for WorkspaceConfig {
         fn default() -> Self {
             Self {
                 workspace: WorkspaceSettings {
                     base_dir: "../workspaces".to_string(),
                     branch_prefix: "work/".to_string(),
                     copy_files: vec![],
                     pre_commands: vec![],
                 },
             }
         }
     }
     
     pub fn load_config() -> WorkspaceConfig {
         println!("Loading config (using defaults for now)");
         WorkspaceConfig::default()
     }
     ```
  3. [ ] デフォルト設定でのYAML出力テスト用に一時的な関数を追加:
     ```rust
     pub fn _test_serialize() {
         let config = WorkspaceConfig::default();
         let yaml = serde_yaml::to_string(&config).unwrap();
         println!("Default config YAML:\n{}", yaml);
     }
     ```
  4. [x] `cargo check`でコンパイル確認
  5. [x] テスト実行で構造体の動作確認
- **完了条件**: 
  - [x] 設定構造体が正常に定義されserialize/deserializeできる
  - [x] デフォルト値で初期化できる

#### Task 2-2: 設定ファイル読み込み機能
- [x] **目的**: YAMLファイルから設定を読み込む
- **詳細実装手順**:
  1. [x] `src/config.rs`の`load_config`関数を実装:
     ```rust
     use std::fs;
     use std::path::Path;
     
     pub fn load_config() -> WorkspaceConfig {
         load_config_from_path("workspace.yml")
     }
     
     pub fn load_config_from_path(path: &str) -> WorkspaceConfig {
         if Path::new(path).exists() {
             match fs::read_to_string(path) {
                 Ok(content) => {
                     match serde_yaml::from_str::<WorkspaceConfig>(&content) {
                         Ok(config) => {
                             println!("設定ファイルを読み込みました: {}", path);
                             config
                         }
                         Err(e) => {
                             println!("設定ファイルの解析エラー: {}. デフォルト設定を使用します", e);
                             WorkspaceConfig::default()
                         }
                     }
                 }
                 Err(e) => {
                     println!("設定ファイルの読み込みエラー: {}. デフォルト設定を使用します", e);
                     WorkspaceConfig::default()
                 }
             }
         } else {
             println!("設定ファイル {} が見つかりません. デフォルト設定を使用します", path);
             WorkspaceConfig::default()
         }
     }
     ```
  2. [x] テスト用の設定ファイル`test-workspace.yml`を作成:
     ```yaml
     workspace:
       base_dir: "../test-workspaces"
       branch_prefix: "test/"
       copy_files:
         - ".env"
         - ".env.local"
       pre_commands:
         - "echo 'setup complete'"
     ```
  3. [x] CLI引数に設定ファイルパスオプションを追加:
     ```rust
     // src/cli.rs
     #[derive(Subcommand)]
     pub enum Commands {
         Start { 
             task_name: String,
             #[arg(short, long, default_value = "workspace.yml")]
             config: String,
         },
         List {
             #[arg(short, long, default_value = "workspace.yml")]
             config: String,
         },
     }
     ```
  4. [x] `src/main.rs`を更新して設定ファイルパスを使用:
     ```rust
     match cli.command {
         Commands::Start { task_name, config } => {
             let config = config::load_config_from_path(&config);
             println!("start コマンドが実行されました: {}", task_name);
             println!("設定: {:?}", config);
             let _ = workspace_manager.create_workspace(&task_name);
         }
         Commands::List { config } => {
             let config = config::load_config_from_path(&config);
             println!("list コマンドが実行されました");
             println!("設定: {:?}", config);
             let _ = workspace_manager.list_workspaces();
         }
     }
     ```
  5. [x] テスト実行:
     - `cargo run -- start test --config test-workspace.yml`
     - `cargo run -- start test` (デフォルト設定)
     - `cargo run -- start test --config nonexistent.yml` (エラーケース)
- **完了条件**: 
  - [x] 設定ファイルが正常に読み込める
  - [x] 存在しないファイルでもエラーにならずデフォルト設定で動作する
  - [x] YAML解析エラー時もデフォルト設定で動作する

### Phase 3: Git Worktree機能の実装

#### Task 3-1: Git操作の基本実装
- [x] **目的**: Git worktreeの基本操作を実装
- **詳細実装手順**:
  1. [x] `Cargo.toml`に`git2 = "0.18"`を追加
  2. [x] `src/workspace.rs`を以下に更新:
     ```rust
     use git2::{Repository, WorktreeAddOptions};
     use std::path::Path;
     
     pub struct WorkspaceManager {
         repo: Repository,
     }
     
     #[derive(Debug)]
     pub struct WorkspaceInfo {
         pub name: String,
         pub path: String,
         pub branch: String,
     }
     
     impl WorkspaceManager {
         pub fn new() -> Result<Self, String> {
             let repo = Repository::open(".").map_err(|e| format!("Gitリポジトリが見つかりません: {}", e))?;
             Ok(Self { repo })
         }
         
         pub fn create_workspace(&self, task_name: &str, base_dir: &str, branch_prefix: &str) -> Result<WorkspaceInfo, String> {
             let timestamp = crate::utils::generate_timestamp();
             let workspace_name = format!("{}-{}", timestamp, task_name);
             let branch_name = format!("{}{}", branch_prefix, task_name);
             let workspace_path = format!("{}/{}", base_dir, workspace_name);
             
             println!("Creating workspace:");
             println!("  Name: {}", workspace_name);
             println!("  Path: {}", workspace_path);
             println!("  Branch: {}", branch_name);
             
             // 実際のworktree作成は次のタスクで実装
             println!("  Status: 準備完了（実際の作成は次の段階で実装）");
             
             Ok(WorkspaceInfo {
                 name: workspace_name,
                 path: workspace_path,
                 branch: branch_name,
             })
         }
         
         pub fn list_workspaces(&self) -> Result<Vec<WorkspaceInfo>, String> {
             println!("Listing workspaces (mock data):");
             
             // テスト用のモックデータ
             let mock_workspaces = vec![
                 WorkspaceInfo {
                     name: "20250621-140000-example".to_string(),
                     path: "../workspaces/20250621-140000-example".to_string(),
                     branch: "work/example".to_string(),
                 },
             ];
             
             for workspace in &mock_workspaces {
                 println!("  - {} -> {}", workspace.branch, workspace.path);
             }
             
             Ok(mock_workspaces)
         }
     }
     ```
  3. [x] `src/main.rs`でWorkspaceManagerの初期化をエラーハンドリング付きに更新:
     ```rust
     fn main() {
         let cli = Cli::parse();
         
         let workspace_manager = match WorkspaceManager::new() {
             Ok(manager) => manager,
             Err(e) => {
                 eprintln!("エラー: {}", e);
                 std::process::exit(1);
             }
         };
         
         match cli.command {
             Commands::Start { task_name, config } => {
                 let config = config::load_config_from_path(&config);
                 println!("start コマンドが実行されました: {}", task_name);
                 
                 match workspace_manager.create_workspace(&task_name, &config.workspace.base_dir, &config.workspace.branch_prefix) {
                     Ok(info) => println!("✅ ワークスペース準備完了: {:?}", info),
                     Err(e) => eprintln!("❌ エラー: {}", e),
                 }
             }
             Commands::List { config } => {
                 let _config = config::load_config_from_path(&config);
                 println!("list コマンドが実行されました");
                 
                 match workspace_manager.list_workspaces() {
                     Ok(workspaces) => println!("📋 ワークスペース一覧: {} 件", workspaces.len()),
                     Err(e) => eprintln!("❌ エラー: {}", e),
                 }
             }
         }
     }
     ```
  4. [x] `cargo check`でコンパイル確認
  5. [x] Gitリポジトリ内で`cargo run -- start test-basic`をテスト実行
  6. [x] 非Gitディレクトリでのエラーハンドリング確認
- **完了条件**: 
  - [x] Gitリポジトリ内で正常に動作する
  - [x] 非Gitディレクトリで適切なエラーメッセージが表示される
  - [x] ワークスペース情報の構造化ができている

#### Task 3-2: workspace startコマンドの実装
- [x] **目的**: 実際にworktreeを作成する機能を実装
- **詳細実装手順**:
  1. [x] `src/workspace.rs`の`create_workspace`関数で実際のworktree作成を実装:
     ```rust
     pub fn create_workspace(&self, task_name: &str, base_dir: &str, branch_prefix: &str) -> Result<WorkspaceInfo, String> {
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
             std::fs::create_dir_all(parent).map_err(|e| format!("ディレクトリ作成エラー: {}", e))?;
         }
         
         // Worktreeの作成
         let mut opts = WorktreeAddOptions::new();
         self.repo.worktree(&workspace_name, Path::new(&workspace_path), Some(&mut opts))
             .map_err(|e| format!("Worktree作成エラー: {}", e))?;
         
         // ブランチの作成と切り替え
         let worktree_repo = Repository::open(&workspace_path)
             .map_err(|e| format!("作成されたワークスペースのオープンエラー: {}", e))?;
         
         let head = worktree_repo.head()
             .map_err(|e| format!("HEADの取得エラー: {}", e))?;
         let target_commit = head.target().ok_or("HEADのコミットIDが取得できません")?;
         let commit = worktree_repo.find_commit(target_commit)
             .map_err(|e| format!("コミットの取得エラー: {}", e))?;
         
         let branch = worktree_repo.branch(&branch_name, &commit, false)
             .map_err(|e| format!("ブランチ作成エラー: {}", e))?;
         
         worktree_repo.set_head(&format!("refs/heads/{}", branch_name))
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
     ```
  2. [x] 実際のworktree一覧取得機能を実装:
     ```rust
     pub fn list_workspaces(&self) -> Result<Vec<WorkspaceInfo>, String> {
         let worktrees = self.repo.worktrees()
             .map_err(|e| format!("Worktree一覧取得エラー: {}", e))?;
         
         let mut workspace_list = Vec::new();
         
         for worktree_name in worktrees.iter().flatten() {
             if let Ok(worktree) = self.repo.find_worktree(worktree_name) {
                 if let Some(path) = worktree.path().to_str() {
                     // ワークスペース名からブランチ名を推測（実装を簡素化）
                     let branch = format!("work/{}", worktree_name);
                     
                     workspace_list.push(WorkspaceInfo {
                         name: worktree_name.to_string(),
                         path: path.to_string(),
                         branch,
                     });
                 }
             }
         }
         
         println!("📋 発見されたワークスペース: {} 件", workspace_list.len());
         for workspace in &workspace_list {
             println!("  - {} -> {}", workspace.branch, workspace.path);
         }
         
         Ok(workspace_list)
     }
     ```
  3. [x] エラーハンドリングの改善とuser-friendlyなメッセージ
  4. [x] テスト実行:
     - `cargo run -- start test-real`で実際のworktree作成確認
     - `cargo run -- list`で作成されたworktreeが表示される確認
     - 作成されたディレクトリに実際に移動してGit状態確認
  5. [x] 作成されたワークスペースの手動削除: `git worktree remove <path>`
- **完了条件**: 
  - [x] `ai-workspace start test-task`で実際にworktreeが作成される
  - [x] 作成されたワークスペースが正しいブランチを持つ
  - [x] `ai-workspace list`で作成されたワークスペースが表示される
  - [x] 適切なパスとブランチ名が使用される

### Phase 4: 基本TUI機能の実装

#### Task 4-1: TUI基本構造の実装
- [x] **目的**: Ratatuiベースの基本TUI画面を作成
- **詳細実装手順**:
  1. [x] `Cargo.toml`に以下を追加:
     ```toml
     ratatui = "0.24"
     crossterm = "0.27"
     ```
  2. [x] `src/tui/mod.rs`を作成:
     ```rust
     pub mod app;
     pub mod ui;
     pub mod events;
     
     pub use app::App;
     ```
  3. [x] `src/tui/app.rs`を作成（基本的なアプリケーション状態管理）:
     ```rust
     use crate::workspace::WorkspaceInfo;
     
     pub struct App {
         pub should_quit: bool,
         pub workspaces: Vec<WorkspaceInfo>,
         pub selected_index: usize,
     }
     
     impl App {
         pub fn new() -> Self {
             // テスト用の固定データ
             let mock_workspaces = vec![
                 WorkspaceInfo {
                     name: "20250621-140000-example".to_string(),
                     path: "../workspaces/20250621-140000-example".to_string(),
                     branch: "work/example".to_string(),
                 },
                 WorkspaceInfo {
                     name: "20250621-141500-feature".to_string(),
                     path: "../workspaces/20250621-141500-feature".to_string(),
                     branch: "work/feature".to_string(),
                 },
             ];
             
             Self {
                 should_quit: false,
                 workspaces: mock_workspaces,
                 selected_index: 0,
             }
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
     ```
  4. [x] `src/tui/ui.rs`を作成（UI描画ロジック）:
     ```rust
     use ratatui::{
         backend::Backend,
         layout::{Constraint, Direction, Layout, Margin},
         style::{Color, Modifier, Style},
         text::{Span, Spans},
         widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
         Frame,
     };
     use crate::tui::App;
     
     pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
         let chunks = Layout::default()
             .direction(Direction::Vertical)
             .margin(1)
             .constraints([
                 Constraint::Length(3),  // Header
                 Constraint::Min(0),     // Content
             ])
             .split(f.size());
         
         // Header
         let header = Paragraph::new("AI Workspace Manager")
             .style(Style::default().fg(Color::Cyan))
             .block(
                 Block::default()
                     .borders(Borders::ALL)
                     .title("ワークスペース管理")
             );
         f.render_widget(header, chunks[0]);
         
         // Help text
         let help_text = Paragraph::new("↑/↓: 選択  q: 終了")
             .style(Style::default().fg(Color::Gray))
             .block(Block::default().borders(Borders::ALL));
         
         let content_layout = Layout::default()
             .direction(Direction::Vertical)
             .constraints([
                 Constraint::Length(3),  // Help
                 Constraint::Min(0),     // List
             ])
             .split(chunks[1]);
         
         f.render_widget(help_text, content_layout[0]);
         
         // Workspace list
         let items: Vec<ListItem> = app
             .workspaces
             .iter()
             .enumerate()
             .map(|(i, workspace)| {
                 let style = if i == app.selected_index {
                     Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                 } else {
                     Style::default()
                 };
                 
                 let content = vec![
                     Spans::from(vec![
                         Span::styled(format!("● {}", workspace.branch), style),
                     ]),
                     Spans::from(vec![
                         Span::styled(format!("  └─ {}", workspace.path), Style::default().fg(Color::Gray)),
                     ]),
                     Spans::from(vec![
                         Span::styled("  └─ Status: Clean  Files: 42  Size: 1.2MB", Style::default().fg(Color::Green)),
                     ]),
                 ];
                 
                 ListItem::new(content).style(style)
             })
             .collect();
         
         let list = List::new(items)
             .block(Block::default().borders(Borders::ALL).title("ワークスペース一覧"))
             .highlight_style(Style::default().add_modifier(Modifier::BOLD))
             .highlight_symbol("→ ");
         
         let mut list_state = ListState::default();
         list_state.select(Some(app.selected_index));
         
         f.render_stateful_widget(list, content_layout[1], &mut list_state);
     }
     ```
  5. [x] `src/tui/events.rs`を作成（キーイベント処理）:
     ```rust
     use crossterm::event::{self, Event, KeyCode};
     use std::time::Duration;
     use crate::tui::App;
     
     pub fn handle_events(app: &mut App) -> std::io::Result<()> {
         if event::poll(Duration::from_millis(100))? {
             if let Event::Key(key) = event::read()? {
                 match key.code {
                     KeyCode::Char('q') | KeyCode::Esc => app.quit(),
                     KeyCode::Down | KeyCode::Char('j') => app.next(),
                     KeyCode::Up | KeyCode::Char('k') => app.previous(),
                     _ => {}
                 }
             }
         }
         Ok(())
     }
     ```
  6. [x] `src/main.rs`にTUIモジュールを追加し、listコマンドでTUIを起動:
     ```rust
     mod tui;
     
     // ... 既存のコード ...
     
     Commands::List { config } => {
         let _config = config::load_config_from_path(&config);
         println!("TUIモードを開始します...");
         
         if let Err(e) = run_tui() {
             eprintln!("TUIエラー: {}", e);
         }
     }
     
     fn run_tui() -> std::io::Result<()> {
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
         
         // App
         let mut app = tui::App::new();
         
         // Main loop
         loop {
             terminal.draw(|f| tui::ui::draw(f, &app))?;
             tui::events::handle_events(&mut app)?;
             
             if app.should_quit {
                 break;
             }
         }
         
         // Cleanup
         disable_raw_mode()?;
         execute!(
             terminal.backend_mut(),
             LeaveAlternateScreen,
             DisableMouseCapture
         )?;
         terminal.show_cursor()?;
         
         Ok(())
     }
     ```
  7. [x] `cargo check`でコンパイル確認
  8. [x] `cargo run -- list`でTUI画面表示確認
  9. [x] 上下キーとqキーでの操作確認
- **完了条件**: 
  - [x] TUI画面が正常に表示される
  - [x] 上下キーで選択移動ができる
  - [x] qキーで終了できる
  - [x] 固定データが適切に表示される

#### Task 4-2: ワークスペース一覧表示機能
- [x] **目的**: 実際のworktreeデータをTUIに表示
- **詳細実装手順**:
  1. [x] `src/tui/app.rs`を実データ対応に更新:
     ```rust
     use crate::workspace::{WorkspaceInfo, WorkspaceManager};
     
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
         
         pub fn get_selected_workspace(&self) -> Option<&WorkspaceInfo> {
             self.workspaces.get(self.selected_index)
         }
     }
     ```
  2. [x] `src/tui/ui.rs`のリスト表示を改善:
     ```rust
     // Workspace list
     if app.workspaces.is_empty() {
         let empty_msg = Paragraph::new("ワークスペースが見つかりません。\n\n'ai-workspace start <task-name>' でワークスペースを作成してください。")
             .style(Style::default().fg(Color::Yellow))
             .block(Block::default().borders(Borders::ALL).title("ワークスペース一覧"));
         f.render_widget(empty_msg, content_layout[1]);
     } else {
         let items: Vec<ListItem> = app
             .workspaces
             .iter()
             .enumerate()
             .map(|(i, workspace)| {
                 let style = if i == app.selected_index {
                     Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                 } else {
                     Style::default()
                 };
                 
                 let content = vec![
                     Spans::from(vec![
                         Span::styled(format!("● {}", workspace.branch), style),
                     ]),
                     Spans::from(vec![
                         Span::styled(format!("  └─ {}", workspace.path), Style::default().fg(Color::Gray)),
                     ]),
                     Spans::from(vec![
                         Span::styled("  └─ Status: Clean  Files: --  Size: --", Style::default().fg(Color::Green)),
                     ]),
                 ];
                 
                 ListItem::new(content).style(style)
             })
             .collect();
         
         let list = List::new(items)
             .block(Block::default().borders(Borders::ALL).title(format!("ワークスペース一覧 ({} 件)", app.workspaces.len())))
             .highlight_style(Style::default().add_modifier(Modifier::BOLD))
             .highlight_symbol("→ ");
         
         let mut list_state = ListState::default();
         list_state.select(Some(app.selected_index));
         
         f.render_stateful_widget(list, content_layout[1], &mut list_state);
     }
     ```
  3. [x] `src/main.rs`のrun_tui関数を実データ対応に更新:
     ```rust
     fn run_tui() -> std::io::Result<()> {
         // ... Terminal setup code ...
         
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
                 return Ok(());
             }
         };
         
         if let Err(e) = app.load_workspaces(&workspace_manager) {
             // Show error but continue with empty list
             eprintln!("ワークスペース読み込み警告: {}", e);
         }
         
         // Main loop
         loop {
             terminal.draw(|f| tui::ui::draw(f, &app))?;
             tui::events::handle_events(&mut app)?;
             
             if app.should_quit {
                 break;
             }
         }
         
         // ... Cleanup code ...
         
         Ok(())
     }
     ```
  4. [x] リフレッシュ機能（'r'キー）を追加:
     ```rust
     // src/tui/events.rs
     KeyCode::Char('r') => {
         // リフレッシュ処理のフラグを設定
         // 実装は次のタスクで詳細化
     },
     ```
  5. [x] テスト実行:
     - 事前に`cargo run -- start test-data`でワークスペース作成
     - `cargo run -- list`で作成されたワークスペースがTUIに表示される確認
     - 空の状態でのメッセージ表示確認
- **完了条件**: 
  - [x] 実際のworktreeデータがTUI画面に表示される
  - [x] 選択状態が視覚的に確認できる
  - [x] ワークスペースが0件の場合の適切なメッセージ表示
  - [x] 複数のワークスペースでの選択移動が正常に動作する

### Phase 5: ファイル操作機能の実装

#### Task 5-1: ファイルコピー機能の実装
- [x] **目的**: 設定ファイルに基づくファイルコピー機能
- **実装内容**:
  - [x] workspace startコマンドでのファイルコピー処理
  - [x] 存在しないファイルのスキップ処理
  - [x] エラーハンドリングと警告表示
- **テスト方法**: 
  - [x] 設定ファイルに指定したファイルがコピーされる
  - [x] 存在しないファイルがスキップされる
- **完了条件**: 
  - [x] 指定ファイルが正常にコピーされる
  - [x] エラー時も処理が継続される

#### Task 5-2: 事前コマンド実行機能
- [x] **目的**: workspace作成後の事前コマンド実行
- **実装内容**:
  - [x] 設定ファイルで指定されたコマンドの実行
  - [x] 新しいworktreeディレクトリでの実行
  - [x] エラー時の継続処理
- **テスト方法**: 
  - [x] 設定コマンドが正しいディレクトリで実行される
  - [x] エラー時もアプリケーションが継続する
- **完了条件**: 
  - [x] 事前コマンドが正常に実行される
  - [x] 実行結果が適切に表示される

### Phase 6: 高度なTUI機能の実装

#### Task 6-1: TUI操作機能の実装
- [x] **目的**: Enterキーでのディレクトリ移動機能
- **実装内容**:
  - [x] 選択されたワークスペースパスの標準出力
  - [x] --print-path-onlyオプションの実装
  - [x] シェル統合のための出力形式調整
  - [x] cdコマンド方式での実装（プロセス内移動ではなく）
- **テスト方法**: 
  - [x] Enterキーで正しいパスが出力される
  - [x] シェル関数での統合動作確認
  - [x] --print-path-onlyオプションでパス一覧表示
- **完了条件**: 
  - [x] 選択ワークスペースへの移動が正常に動作する
  - [x] README.mdでシェル統合方法が説明されている

#### Task 6-2: 削除機能の実装
- **目的**: TUIからのワークスペース削除機能
- **実装内容**:
  - 'd'キーでの削除確認ダイアログ
  - Y/Nでの確認処理
  - worktreeの実際の削除処理
- **テスト方法**: 
  - 削除確認ダイアログが正常に表示される
  - 削除処理が正常に実行される
- **完了条件**: 
  - 安全な削除確認フローが実装されている
  - 削除後のリスト更新が正常に動作する

#### Task 6-3: 詳細情報表示機能
- **目的**: 'i'キーでの詳細情報モーダル
- **実装内容**:
  - ワークスペースの詳細情報取得
  - モーダルダイアログの表示
  - Git情報（コミット履歴等）の表示
- **テスト方法**: 
  - 詳細情報が正確に表示される
  - モーダルの開閉が正常に動作する
- **完了条件**: 
  - 詳細情報モーダルが実装されている
  - Git情報が正確に表示される

### Phase 7: 最終仕上げとテスト

#### Task 7-1: エラーハンドリングの強化
- **目的**: 包括的なエラーハンドリング
- **実装内容**:
  - 全機能のエラーケース対応
  - ユーザーフレンドリーなエラーメッセージ
  - ログ出力の改善
- **テスト方法**: 
  - 異常ケースでの動作確認
  - エラーメッセージの品質確認
- **完了条件**: 
  - 予期しないエラーでもアプリケーションが安定している

#### Task 7-2: 統合テストとドキュメント
- **目的**: 最終的な品質確保
- **実装内容**:
  - 統合テストの実装
  - README.mdの更新
  - 使用方法の文書化
- **テスト方法**: 
  - 全機能の動作確認
  - ドキュメントの正確性確認
- **完了条件**: 
  - 全機能が統合されて正常に動作する
  - ドキュメントが整備されている

## 実装順序とマイルストーン

### Milestone 1: CLI基盤 (Task 1-1, 1-2)
基本的なコマンド構造が動作する状態

### Milestone 2: 設定機能 (Task 2-1, 2-2)
設定ファイルの読み込みが可能な状態

### Milestone 3: 基本機能 (Task 3-1, 3-2)
worktreeの作成が可能な状態

### Milestone 4: TUI基盤 (Task 4-1, 4-2)
基本的なTUI表示と選択が可能な状態

### Milestone 5: 完全機能 (Task 5-1, 5-2, 6-1, 6-2, 6-3)
全ての主要機能が実装された状態

### Milestone 6: 製品品質 (Task 7-1, 7-2)
製品として使用可能な品質の状態

## 全タスク一覧（チェックリスト）

### Phase 1: 基本CLI構造の構築
- [x] Task 1-1: 基本CLIフレームワークの実装
- [x] Task 1-2: プロジェクト構造の整備

### Phase 2: 設定ファイル機能の実装  
- [x] Task 2-1: 設定ファイル構造体の定義
- [x] Task 2-2: 設定ファイル読み込み機能

### Phase 3: Git Worktree機能の実装
- [x] Task 3-1: Git操作の基本実装
- [x] Task 3-2: workspace startコマンドの実装

### Phase 4: 基本TUI機能の実装
- [x] Task 4-1: TUI基本構造の実装
- [x] Task 4-2: ワークスペース一覧表示機能

### Phase 5: ファイル操作機能の実装
- [x] Task 5-1: ファイルコピー機能の実装
- [x] Task 5-2: 事前コマンド実行機能

### Phase 6: 高度なTUI機能の実装
- [x] Task 6-1: TUI操作機能の実装
- [ ] Task 6-2: 削除機能の実装
- [ ] Task 6-3: 詳細情報表示機能

### Phase 7: 最終仕上げとテスト
- [ ] Task 7-1: エラーハンドリングの強化
- [ ] Task 7-2: 統合テストとドキュメント

## 次のステップ

1. Task 1-1から順次実装を開始
2. 各タスク完了時にテストを実行
3. 問題が発生した場合は設計の見直しを検討
4. 各マイルストーン完了時にコミット&プッシュ
5. 実装開始前に、そのタスクの詳細をもう一度確認し、可能なら更に詳細化してドキュメントを更新する
