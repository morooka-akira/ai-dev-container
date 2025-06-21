# シンプルなワークスペース管理ツール設計

## 概要

git worktreeを活用して、複数の作業環境を簡単に管理するシンプルなCLIツール。

## 主要機能

### 1. work start コマンド
作業用のworktreeを作成し、環境をセットアップする。

```bash
# 基本的な使い方
ai-workspace start <task-name>

# オプション付き
ai-workspace start feature-x --branch main --config workspace.yml
```

**動作:**
- git worktreeで新しいブランチとディレクトリを作成
- 設定ファイルに基づいてファイルをコピー
- 事前実行コマンドを実行
- 作成したディレクトリパスを表示

### 2. work list コマンド
インタラクティブなTUIでワークスペースを管理する。

```bash
# インタラクティブなTUI画面を起動
ai-workspace list
```

**TUI操作:**
- ↑/↓ または j/k: ワークスペース選択
- Enter: 選択したワークスペースに移動
- d: 選択したワークスペースを削除
- i: 詳細情報を表示
- q: 終了

## 設定ファイル形式

```yaml
# workspace.yml
workspace:
  # worktreeのベースディレクトリ
  base_dir: "../workspaces"
  
  # ブランチ名のプレフィックス
  branch_prefix: "work/"
  
  # コピーするファイル一覧
  copy_files:
    - .env
    - .env.local
    - config/secrets.json
    
  # 事前実行コマンド
  pre_commands:
    - "npm install"
    - "cargo build"
```

## 実装アーキテクチャ

### ディレクトリ構造
```
ai-dev-container/
├── src/
│   ├── main.rs           # CLIエントリポイント
│   ├── cli.rs            # コマンドライン引数処理
│   ├── workspace.rs      # worktree操作
│   ├── config.rs         # 設定ファイル読み込み
│   ├── tui/
│   │   ├── mod.rs        # TUIモジュール
│   │   ├── app.rs        # アプリケーション状態管理
│   │   ├── ui.rs         # UI描画ロジック
│   │   └── events.rs     # キーイベント処理
│   └── utils.rs          # ユーティリティ関数
├── Cargo.toml
└── workspace.yml         # デフォルト設定ファイル
```

### 主要な依存クレート
- `clap`: CLIパーサー
- `serde` + `serde_yaml`: 設定ファイル読み込み
- `git2`: Git操作
- `ratatui`: TUIフレームワーク
- `crossterm`: ターミナル操作
- `tokio`: 非同期実行時

## 詳細仕様

### work start の処理フロー

1. **引数とオプションの解析**
   - タスク名（必須）
   - ベースブランチ（オプション、デフォルト: 現在のブランチ）
   - 設定ファイルパス（オプション、デフォルト: ./workspace.yml）

2. **worktree作成**
   ```rust
   // 擬似コード
   let workspace_name = format!("{}-{}", timestamp(), task_name);
   let branch_name = format!("{}{}", config.branch_prefix, task_name);
   let workspace_path = format!("{}/{}", config.base_dir, workspace_name);
   
   git_worktree_add(&workspace_path, &branch_name);
   ```

3. **ファイルコピー**
   - 設定ファイルに指定されたファイルを新しいworktreeにコピー
   - 存在しないファイルはスキップ（警告表示）

4. **事前コマンド実行**
   - 新しいworktreeディレクトリで指定コマンドを実行
   - エラーが発生しても処理は継続（警告表示）

5. **結果表示**
   ```
   ✅ Workspace created: ../workspaces/20250619-143022-feature-x
   📁 Branch: work/feature-x
   
   To enter the workspace:
     cd ../workspaces/20250619-143022-feature-x
   ```

### work list TUIの詳細設計

#### 画面レイアウト
```
┌─ AI Workspace Manager ─────────────────────────────┐
│ ↑/↓ Select  Enter: Open  d: Delete  i: Info  q: Quit │
├─────────────────────────────────────────────────────┤
│ ● work/feature-x                                    │
│   └─ ../workspaces/20250619-143022-feature-x       │
│   └─ Status: Clean  Files: 42  Size: 1.2MB         │
│                                                     │
│   work/bugfix-y                                     │
│   └─ ../workspaces/20250619-140512-bugfix-y        │
│   └─ Status: Modified (3 files)  Size: 890KB       │
│                                                     │
│   work/refactor-z                                   │
│   └─ ../workspaces/20250619-120345-refactor-z      │
│   └─ Status: Clean  Files: 28  Size: 756KB         │
└─────────────────────────────────────────────────────┘
```

#### キーバインディング
- **↑/↓ または j/k**: ワークスペース選択移動
- **Enter**: 選択ワークスペースに移動（シェルスクリプト出力）
- **d**: 選択ワークスペースの削除確認ダイアログ
- **i**: 詳細情報モーダル表示
- **r**: ワークスペース一覧リフレッシュ
- **q または Esc**: TUI終了

#### 詳細情報モーダル
```
┌─ Workspace Details ─────────────────┐
│ Branch: work/feature-x              │
│ Path: ../workspaces/20250619-...    │
│ Created: 2025-06-19 14:30:22        │
│ Last Modified: 2025-06-19 16:45:12  │
│ Status: Clean                       │
│ Files: 42 tracked, 3 untracked     │
│ Size: 1.2MB                         │
│                                     │
│ Recent Commits:                     │
│ - Fix input validation (2h ago)     │
│ - Add error handling (5h ago)       │
│                                     │
│ Press any key to close              │
└─────────────────────────────────────┘
```

#### 削除確認ダイアログ
```
┌─ Delete Workspace ──────────┐
│ Are you sure you want to    │
│ delete this workspace?      │
│                             │
│ work/feature-x              │
│ ../workspaces/20250619-...  │
│                             │
│ [Y]es  [N]o                 │
└─────────────────────────────┘
```

#### ディレクトリ移動の実装
TUIツール終了時に、選択されたパスを標準出力に出力し、シェル関数で`cd`を実行：

```bash
# ~/.bashrcまたは~/.zshrcに追加
awl() {
    local target_path
    target_path=$(ai-workspace list --print-path-only)
    if [ -n "$target_path" ]; then
        cd "$target_path"
    fi
}
```

## エラーハンドリング

- Git worktree作成エラー: 明確なエラーメッセージを表示
- ファイルコピーエラー: 警告を表示して継続
- 事前コマンドエラー: 警告を表示して継続
- 設定ファイル不在: デフォルト設定で動作

## 将来の拡張案

1. **workspace clean** コマンド
   - 不要になったworktreeを削除

2. **workspace sync** コマンド
   - メインブランチの変更を取り込む

3. **テンプレート機能**
   - タスクタイプ別の設定テンプレート

4. **履歴機能**
   - 過去に作成したworkspaceの履歴管理