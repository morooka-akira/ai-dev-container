# Task 6-1: TUI操作機能の実装

## 作業概要

**日付**: 2025-06-23  
**タスク**: Task 6-1: TUI操作機能の実装（Enterキーでディレクトリ移動）  
**目的**: TUIでEnterキーを押すことで選択されたワークスペースのディレクトリに移動できる機能を実装

## 設計詳細

### 機能要件

1. **Enterキー処理**
   - TUIでEnterキーが押された時の処理を追加
   - 選択されたワークスペースのパスを取得
   - TUIを終了して標準出力にパスを出力

2. **--print-path-onlyオプション**
   - `list --print-path-only` オプションの実装
   - TUI表示をスキップして直接パス出力のモード

3. **シェル統合サポート**
   - シェル関数での利用を想定した出力形式
   - エラー時は何も出力しない（空文字列）

### 実装方針

#### 1. CLIオプションの追加
```rust
// src/cli.rs
#[derive(Subcommand)]
pub enum Commands {
    List {
        #[arg(short, long, default_value = "workspace.yml")]
        config: String,
        #[arg(long, help = "Print selected workspace path only")]
        print_path_only: bool,
    },
}
```

#### 2. TUIイベント処理の拡張
```rust
// src/tui/events.rs
pub enum AppAction {
    None,
    Quit,
    NavigateToWorkspace(String), // パスを返す
}

pub fn handle_events(app: &mut App) -> std::io::Result<AppAction> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => Ok(AppAction::Quit),
                KeyCode::Down | KeyCode::Char('j') => {
                    app.next();
                    Ok(AppAction::None)
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    app.previous();
                    Ok(AppAction::None)
                }
                KeyCode::Enter => {
                    if let Some(workspace) = app.get_selected_workspace() {
                        Ok(AppAction::NavigateToWorkspace(workspace.path.clone()))
                    } else {
                        Ok(AppAction::None)
                    }
                }
                _ => Ok(AppAction::None),
            }
        } else {
            Ok(AppAction::None)
        }
    } else {
        Ok(AppAction::None)
    }
}
```

#### 3. メイン処理の実装
```rust
// src/main.rs
fn run_tui(print_path_only: bool) -> std::io::Result<Option<String>> {
    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    // App with real data
    let mut app = tui::App::new();
    
    // Load workspace data
    let workspace_manager = match WorkspaceManager::new() {
        Ok(manager) => manager,
        Err(_) => {
            cleanup_terminal(&mut terminal)?;
            return Ok(None);
        }
    };
    
    if let Err(_) = app.load_workspaces(&workspace_manager) {
        cleanup_terminal(&mut terminal)?;
        return Ok(None);
    }
    
    // print-path-onlyモードの場合、TUIを表示せずに直接選択
    if print_path_only {
        cleanup_terminal(&mut terminal)?;
        // 最初のワークスペースのパスを返す（実際にはより複雑な選択ロジック）
        return Ok(app.workspaces.first().map(|ws| ws.path.clone()));
    }
    
    // Main loop
    let selected_path = loop {
        terminal.draw(|f| tui::ui::draw(f, &app))?;
        
        match tui::events::handle_events(&mut app)? {
            AppAction::Quit => break None,
            AppAction::NavigateToWorkspace(path) => break Some(path),
            AppAction::None => {}
        }
    };
    
    cleanup_terminal(&mut terminal)?;
    Ok(selected_path)
}
```

### テスト計画

1. **Enterキーのイベント処理テスト**
   - Enterキーで正しいパスが返されることを確認
   - ワークスペースが空の場合の処理

2. **--print-path-onlyオプションのテスト**
   - オプションが正しく解析されることを確認
   - 期待されるパスが出力されることを確認

3. **統合テスト**
   - 実際のTUI操作のシミュレーション
   - シェル統合での動作確認

## 実装手順

1. [x] 作業ドキュメントの作成
2. [ ] ブランチの作成
3. [ ] CLIオプションの追加
4. [ ] TUIイベント処理の拡張
5. [ ] メイン処理の実装
6. [ ] テストの作成
7. [ ] 動作確認
8. [ ] 品質チェック
9. [ ] PRの作成

## 期待される結果

- `gwork list` でTUIが表示され、Enterキーでワークスペースに移動できる
- `gwork list --print-path-only` で最初のワークスペースのパスが出力される
- シェル関数での利用が可能になる

## 注意事項

- TUIの終了処理を適切に行い、ターミナル状態を復元する
- エラーハンドリングを適切に行い、空の場合は何も出力しない
- 既存のTUI機能を壊さないよう注意する