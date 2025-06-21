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
アクティブなworktreeを一覧表示し、簡単に移動できるようにする。

```bash
# 一覧表示
ai-workspace list

# 特定のworkspaceへ移動するコマンドを出力
ai-workspace go <workspace-id>
```

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
│   └── utils.rs          # ユーティリティ関数
├── Cargo.toml
└── workspace.yml         # デフォルト設定ファイル
```

### 主要な依存クレート
- `clap`: CLIパーサー
- `serde` + `serde_yaml`: 設定ファイル読み込み
- `git2`: Git操作
- `colored`: 出力の色付け

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

### work list の処理フロー

1. **worktree一覧取得**
   - `git worktree list` コマンドを実行
   - 結果をパース

2. **表示形式**
   ```
   ID  | Branch           | Path                                    | Status
   ----+------------------+-----------------------------------------+--------
   1   | work/feature-x   | ../workspaces/20250619-143022-feature-x | Clean
   2   | work/bugfix-y    | ../workspaces/20250619-140512-bugfix-y  | Modified
   ```

3. **go サブコマンド**
   ```bash
   $ ai-workspace go 1
   cd ../workspaces/20250619-143022-feature-x
   ```
   
   注: 実際のディレクトリ移動はシェルで評価する必要がある
   ```bash
   # ~/.bashrcなどに追加
   function awg() {
     cd $(ai-workspace go $1 --print-path)
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