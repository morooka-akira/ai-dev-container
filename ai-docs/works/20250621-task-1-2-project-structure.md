# Task 1-2: プロジェクト構造の整備

## 作業概要
- **目的**: 必要なモジュールファイルの作成
- **対象**: Phase 1のTask 1-2
- **日時**: 2025年6月21日

## 実装内容

### 1. 基本モジュールの作成
- `src/workspace.rs`: ワークスペース管理機能
- `src/config.rs`: 設定ファイル管理
- `src/utils.rs`: ユーティリティ関数

### 2. 依存関係の追加
- `chrono`: タイムスタンプ生成用

### 3. モジュール統合
- `src/main.rs`を更新してモジュールを統合

## 実装手順

### Step 1: src/workspace.rsの作成
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

### Step 2: src/config.rsの作成
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

### Step 3: src/utils.rsの作成
```rust
use chrono::{DateTime, Local};

pub fn generate_timestamp() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%Y%m%d-%H%M%S").to_string()
}
```

### Step 4: Cargo.tomlの更新
```toml
chrono = { version = "0.4", features = ["serde"] }
```

### Step 5: src/main.rsの更新
各モジュールを統合し、実際の関数を呼び出す。

## テスト内容
1. `cargo check`でコンパイル確認
2. `cargo run -- start test-task`で動作確認

## 完了条件
- [ ] 全モジュールが正常にコンパイルできる
- [ ] 各モジュールの関数が呼び出され、ログが出力される

## 実装メモ
- 各モジュールは今後の拡張を考慮した基本構造
- 実際のGit操作は次のフェーズで実装