# Task 4: TUI基本機能の実装

## 作業概要

Phase 4としてTUI（Terminal User Interface）の基本機能を実装します。

### 対象タスク
- Task 4-1: TUI基本構造の実装
- Task 4-2: ワークスペース一覧表示機能

## 設計確認

@ai-docs/designs/design.md の内容に従い、以下を実装：

### TUI画面レイアウト
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
└─────────────────────────────────────────────────────┘
```

### キーバインディング
- **↑/↓ または j/k**: ワークスペース選択移動
- **q または Esc**: TUI終了

## 実装詳細

### Task 4-1: TUI基本構造の実装

#### 依存関係の追加
```toml
ratatui = "0.24"
crossterm = "0.27"
```

#### モジュール構造
```
src/tui/
├── mod.rs        # TUIモジュール
├── app.rs        # アプリケーション状態管理
├── ui.rs         # UI描画ロジック
└── events.rs     # キーイベント処理
```

#### 実装手順
1. Cargo.tomlに依存関係追加
2. tui/mod.rsの作成
3. app.rs: アプリケーション状態管理
4. ui.rs: UI描画ロジック  
5. events.rs: キーイベント処理
6. main.rsにTUI統合

### Task 4-2: ワークスペース一覧表示機能

#### 機能
- 実際のworktreeデータをTUIに表示
- 空の状態での適切なメッセージ表示
- 選択状態の視覚的表示

#### 実装手順
1. app.rsを実データ対応に更新
2. ui.rsのリスト表示改善
3. main.rsのrun_tui関数を実データ対応に更新

## テスト計画

1. TUI画面の正常表示
2. 上下キーでの選択移動
3. qキーでの終了
4. 実際のworktreeデータ表示
5. 空の状態でのメッセージ表示

## 完了条件

- TUI画面が正常に表示される
- キーボード操作が正常に動作する
- 実際のワークスペースデータが表示される
- 全テストが通過する
- フォーマットとClippyチェックが通る

## 注意事項

- 既存のテスト用ワークツリークリーンアップ機能は維持
- エラーハンドリングを適切に実装
- ユーザーフレンドリーなメッセージ表示