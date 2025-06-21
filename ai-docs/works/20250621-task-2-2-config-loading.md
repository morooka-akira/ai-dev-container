# Task 2-2: 設定ファイル読み込み機能の実装

## 作業概要

YAMLファイルから設定を読み込む機能を実装する。Task 2-1で定義した設定構造体を使用して、実際のファイル読み込みとエラーハンドリングを含む完全な設定システムを構築する。

## 実装予定内容

### 1. 設定ファイル読み込み機能の実装

#### src/config.rsの更新
- `load_config_from_path`関数の実装
- エラーハンドリング（ファイル不存在、YAML解析エラー）
- デフォルト設定へのフォールバック

#### CLI引数への設定ファイルパス追加
- `src/cli.rs`に`--config`オプション追加
- start/listコマンド両方に対応

#### main.rsでの設定ファイル使用
- コマンド実行時に設定ファイルを読み込み
- WorkspaceManagerに設定を渡す

### 2. テスト用設定ファイルの作成
- `test-workspace.yml`を作成してテスト用の設定を用意

### 3. エラーケースのテスト
- 存在しないファイル指定時の動作確認
- 不正なYAML形式での動作確認
- デフォルト設定での動作確認

## 前提条件の確認

### Task 2-1の完了状況
以下がTask 2-1で実装済みである必要がある：
- WorkspaceConfig構造体の定義
- serde/serde_yamlの依存関係
- 基本的なデフォルト設定

### 依存関係
```toml
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
```

## 実装手順

### 1. 設定ファイル読み込み関数の実装
```rust
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

### 2. CLI引数の更新
```rust
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

### 3. main.rsでの設定使用
```rust
match cli.command {
    Commands::Start { task_name, config } => {
        let config = config::load_config_from_path(&config);
        // configを使用してワークスペース作成
    }
    Commands::List { config } => {
        let config = config::load_config_from_path(&config);
        // configを使用してワークスペース一覧表示
    }
}
```

## テスト計画

### テストケース
1. **正常ケース**: 有効な設定ファイルが正しく読み込まれる
2. **ファイル不存在**: デフォルト設定で動作する
3. **YAML解析エラー**: デフォルト設定で動作する
4. **デフォルト設定**: workspace.ymlが指定されない場合

### テスト用設定ファイル
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

## 完了条件

- [ ] 設定ファイルが正常に読み込める
- [ ] 存在しないファイルでもエラーにならずデフォルト設定で動作する
- [ ] YAML解析エラー時もデフォルト設定で動作する
- [ ] CLI引数で設定ファイルパスを指定できる
- [ ] 設定内容がログに表示される
- [ ] 全テストケースが通る

## 依存関係・前提条件

- Task 2-1が完了している必要がある
- serde, serde_yamlがCargo.tomlに追加済み
- 基本的なCLI構造が動作している

## 注意事項

- エラー時も処理を継続させる（デフォルト設定使用）
- ユーザーフレンドリーなエラーメッセージを表示
- 設定ファイルパスは相対パスでも絶対パスでも対応

## 作業後の確認項目

- [ ] cargo fmt でフォーマット
- [ ] cargo clippy で警告チェック  
- [ ] cargo test でテスト実行
- [ ] 各種テストケースでの動作確認
- [ ] PRの作成