# Task 3-1: Git操作の基本実装

## 作業概要
Task 3-1: Git操作の基本実装を実行します。

## 実装目標
- Git worktreeの基本操作を実装
- WorkspaceManagerをGit対応に更新
- ワークスペース情報の構造化

## 設計確認
@ai-docs/designs/design.md と @ai-docs/designs/tasks.md を参照し、Git worktree機能の実装について確認済み。

## 作業リスト

### 準備作業
- [x] 作業ドキュメント作成（本ファイル）
- [x] mainブランチからfeature/task-3-1-git-operationsブランチを作成
- [x] 設計ドキュメント確認

### 実装作業
- [x] Cargo.tomlにgit2 = "0.18"を追加
- [ ] src/workspace.rsをGit対応に更新
  - [ ] WorkspaceManagerにRepositoryフィールドを追加
  - [ ] WorkspaceInfo構造体を定義
  - [ ] new()メソッドをResult型に変更
  - [ ] create_workspace()メソッドを詳細実装
  - [ ] list_workspaces()メソッドを詳細実装
- [ ] src/main.rsでエラーハンドリング付き初期化に更新
- [ ] テストの実装・更新

### 品質確認
- [ ] cargo checkでコンパイル確認
- [ ] Gitリポジトリ内でのテスト実行
- [ ] 非Gitディレクトリでのエラーハンドリング確認
- [ ] cargo test実行
- [ ] cargo fmt実行
- [ ] cargo clippy実行

### 完了作業
- [ ] PRの作成
- [ ] タスクリストの更新

## 完了条件
- Gitリポジトリ内で正常に動作する
- 非Gitディレクトリで適切なエラーメッセージが表示される
- ワークスペース情報の構造化ができている

## 注意事項
- 実際のworktree作成は次のタスク（Task 3-2）で実装予定
- 今回は基本的なGit操作とデータ構造の準備に集中