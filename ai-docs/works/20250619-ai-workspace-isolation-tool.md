# AI開発ワークスペース分離ツール設計案

## 概要

特定のリポジトリから、AIが安全に作業するための隔離されたワークスペースを自動的に作成・管理するツール。

## 要件整理

### 機能要件
1. **ワークスペース分離**
   - 元のリポジトリから独立した作業領域の作成
   - git worktreeを活用した効率的なブランチ管理
   - 作業完了後の安全なマージ機能

2. **安全な実行環境**
   - DevContainerまたはDockerコンテナでの隔離環境
   - Claude Codeの自動起動と設定
   - 権限の適切な制限

3. **ファイル管理**
   - git管理外ファイルの選択的コピー（.env、秘密鍵など）
   - 設定ファイルによる柔軟な制御
   - 機密情報の安全な取り扱い

### 非機能要件
- 高速なワークスペース作成
- 最小限のディスク使用量
- 直感的なCLIインターフェース

## 実装方法論

### 1. Git Worktree活用パターン

```bash
# ワークスペース作成の基本フロー
git worktree add ../ai-workspace-<timestamp> -b ai-work/<task-name>
```

**メリット:**
- 軽量（ファイルの物理的なコピーが不要）
- 高速な切り替え
- 本体リポジトリとの同期が容易

**考慮点:**
- サブモジュールの扱い
- LFSファイルの管理

### 2. コンテナ化戦略

#### Option A: DevContainer
```json
// .devcontainer/devcontainer.json
{
  "name": "AI Workspace",
  "dockerComposeFile": "docker-compose.yml",
  "service": "workspace",
  "workspaceFolder": "/workspace",
  "features": {
    "ghcr.io/devcontainers/features/rust:1": {}
  },
  "postCreateCommand": "claude-code init"
}
```

#### Option B: Docker + docker-use
```yaml
# docker-compose.yml
version: '3.8'
services:
  workspace:
    image: rust:latest
    volumes:
      - ./:/workspace
      - ${HOME}/.ssh:/home/user/.ssh:ro
    environment:
      - ANTHROPIC_API_KEY
```

### 3. 設定ファイル設計

```yaml
# ai-workspace.config.yml
workspace:
  base_path: "./ai-workspaces"
  naming_pattern: "{timestamp}-{task_name}"
  
git:
  worktree: true
  default_branch_prefix: "ai-work/"
  
files:
  # git管理外ファイルのコピー設定
  copy_untracked:
    - pattern: ".env*"
      exclude: [".env.example"]
    - pattern: "config/secrets/*"
      decrypt: true
  
  # シンボリックリンクで参照
  symlink:
    - "node_modules"
    - ".cargo"
    
container:
  type: "devcontainer"  # or "docker"
  auto_start_claude: true
  mount_ssh_keys: true
  environment:
    inherit: ["ANTHROPIC_API_KEY", "GITHUB_TOKEN"]
```

## アーキテクチャ設計

### コンポーネント構成

```
ai-dev-container/
├── src/
│   ├── main.rs              # CLIエントリポイント
│   ├── workspace/
│   │   ├── mod.rs          # ワークスペース管理
│   │   ├── git.rs          # Git worktree操作
│   │   └── files.rs        # ファイル管理
│   ├── container/
│   │   ├── mod.rs          # コンテナ管理
│   │   ├── devcontainer.rs # DevContainer実装
│   │   └── docker.rs       # Docker直接操作
│   └── config/
│       └── mod.rs          # 設定管理
```

### 処理フロー

1. **初期化フェーズ**
   ```
   設定読み込み → リポジトリ検証 → ワークスペース名生成
   ```

2. **ワークスペース作成**
   ```
   git worktree作成 → ファイルコピー/リンク → コンテナ準備
   ```

3. **実行フェーズ**
   ```
   コンテナ起動 → Claude Code初期化 → 作業開始
   ```

4. **終了処理**
   ```
   変更の確認 → コミット支援 → worktree削除
   ```

## CLI設計

```bash
# 基本的な使用方法
ai-workspace create --task "feature-implementation"
ai-workspace list
ai-workspace enter <workspace-id>
ai-workspace cleanup <workspace-id>

# 高度な使用例
ai-workspace create \
  --repo ../main-project \
  --branch feature/new-api \
  --copy-env \
  --container devcontainer
```

## セキュリティ考慮事項

1. **機密情報の扱い**
   - .envファイルの暗号化オプション
   - APIキーの環境変数経由での受け渡し
   - SSHキーの読み取り専用マウント

2. **コンテナセキュリティ**
   - 非rootユーザーでの実行
   - 必要最小限の権限
   - ネットワーク分離オプション

3. **ファイルアクセス制御**
   - ホワイトリスト方式でのファイルコピー
   - シンボリックリンクの検証

## 実装優先順位

### Phase 1: MVP
- git worktreeベースの基本的なワークスペース作成
- 簡単な設定ファイル読み込み
- 基本的なCLIコマンド

### Phase 2: コンテナ統合
- DevContainerサポート
- 自動的なClaude Code起動

### Phase 3: 高度な機能
- 複数ワークスペースの管理
- ファイル暗号化
- カスタムフック機能

## 技術選択の根拠

- **Rust**: 高速で安全なシステムプログラミング
- **git worktree**: 効率的なブランチ管理
- **DevContainer**: VSCode/Claude Codeとの親和性
- **YAML設定**: 人間に優しい設定フォーマット

## 今後の検討事項

1. **拡張性**
   - プラグインシステムの導入
   - 他のAI開発環境への対応

2. **監査機能**
   - AIの作業履歴の記録
   - 変更内容の自動レビュー

3. **チーム機能**
   - 複数人でのワークスペース共有
   - 作業結果の共有機能