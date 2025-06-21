# Task 1-1: 基本CLIフレームワークの実装

## 作業概要
- **目的**: コマンド構造の基盤を作成
- **対象**: Phase 1のTask 1-1
- **日時**: 2025年6月21日

## 実装内容

### 1. 依存関係の追加
- `Cargo.toml`に`clap = { version = "4.0", features = ["derive"] }`を追加

### 2. CLIモジュールの作成
- `src/cli.rs`を作成
- コマンド構造体の定義（Start, List）
- clap deriveを使用したコマンドライン解析

### 3. メイン関数の更新
- `src/main.rs`を更新
- コマンド分岐処理の実装

## 実装手順

### Step 1: Cargo.tomlの更新
```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
```

### Step 2: src/cli.rsの作成
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

### Step 3: src/main.rsの更新
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

## テスト内容
1. `cargo run -- start test-task` でテスト実行
2. `cargo run -- list` でテスト実行  
3. `cargo run -- --help` でヘルプ表示確認

## 完了条件
- [ ] 両コマンドがエラーなく実行でき、適切なログが出力される
- [ ] ヘルプメッセージが適切に表示される

## 実装メモ
- 設計ドキュメントに従い、`ai-workspace`という名前でCLIツールを作成
- 今後の拡張を考慮し、シンプルな構造から開始