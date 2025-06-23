use thiserror::Error;

/// アプリケーション全体で使用する統一エラー型
#[derive(Error, Debug)]
pub enum GworkError {
    /// Gitリポジトリ関連のエラー
    #[error("Gitリポジトリエラー: {message}")]
    Git { message: String },

    /// ファイル操作関連のエラー
    #[error("ファイル操作エラー: {message}")]
    Io { message: String },

    /// 設定ファイル関連のエラー
    #[error("設定ファイルエラー: {message}")]
    Config { message: String },

    /// ワークスペース関連のエラー
    #[error("ワークスペースエラー: {message}")]
    Workspace { message: String },

    /// TUI関連のエラー
    #[error("TUIエラー: {message}")]
    Tui { message: String },

    /// 一般的なエラー
    #[error("エラー: {message}")]
    General { message: String },
}

impl GworkError {
    /// Gitエラーを作成
    pub fn git<S: Into<String>>(message: S) -> Self {
        Self::Git {
            message: message.into(),
        }
    }

    /// ファイル操作エラーを作成
    pub fn io<S: Into<String>>(message: S) -> Self {
        Self::Io {
            message: message.into(),
        }
    }

    /// 設定ファイルエラーを作成
    pub fn config<S: Into<String>>(message: S) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    /// ワークスペースエラーを作成
    pub fn workspace<S: Into<String>>(message: S) -> Self {
        Self::Workspace {
            message: message.into(),
        }
    }

    /// TUIエラーを作成
    pub fn tui<S: Into<String>>(message: S) -> Self {
        Self::Tui {
            message: message.into(),
        }
    }

    /// 一般的なエラーを作成
    #[allow(dead_code)]
    pub fn general<S: Into<String>>(message: S) -> Self {
        Self::General {
            message: message.into(),
        }
    }
}

impl From<git2::Error> for GworkError {
    fn from(err: git2::Error) -> Self {
        let message = match err.code() {
            git2::ErrorCode::NotFound => "指定されたリソースが見つかりません".to_string(),
            git2::ErrorCode::Exists => "リソースが既に存在します".to_string(),
            git2::ErrorCode::Ambiguous => "リソースの指定があいまいです".to_string(),
            git2::ErrorCode::BufSize => "バッファサイズが不足しています".to_string(),
            git2::ErrorCode::User => "ユーザー操作がキャンセルされました".to_string(),
            git2::ErrorCode::BareRepo => "ベアリポジトリでは実行できません".to_string(),
            git2::ErrorCode::UnbornBranch => "ブランチが存在しません".to_string(),
            git2::ErrorCode::Unmerged => "マージされていない変更があります".to_string(),
            git2::ErrorCode::NotFastForward => "Fast-forwardできません".to_string(),
            git2::ErrorCode::InvalidSpec => "無効な仕様です".to_string(),
            git2::ErrorCode::Conflict => "コンフリクトが発生しています".to_string(),
            git2::ErrorCode::Locked => "リソースがロックされています".to_string(),
            git2::ErrorCode::Modified => "ファイルが変更されています".to_string(),
            git2::ErrorCode::Auth => "認証に失敗しました".to_string(),
            git2::ErrorCode::Certificate => "証明書エラーです".to_string(),
            git2::ErrorCode::Applied => "パッチが既に適用されています".to_string(),
            git2::ErrorCode::Peel => "オブジェクトの展開に失敗しました".to_string(),
            git2::ErrorCode::Eof => "ファイルの終端に達しました".to_string(),
            git2::ErrorCode::Invalid => "無効な操作です".to_string(),
            git2::ErrorCode::Uncommitted => "コミットされていない変更があります".to_string(),
            git2::ErrorCode::Directory => "ディレクトリが存在しません".to_string(),
            git2::ErrorCode::MergeConflict => "マージコンフリクトが発生しています".to_string(),
            git2::ErrorCode::HashsumMismatch => "ハッシュサムが一致しません".to_string(),
            git2::ErrorCode::IndexDirty => "インデックスが変更されています".to_string(),
            git2::ErrorCode::ApplyFail => "パッチの適用に失敗しました".to_string(),
            _ => format!("Gitエラーが発生しました: {}", err.message()),
        };
        Self::git(message)
    }
}

impl From<std::io::Error> for GworkError {
    fn from(err: std::io::Error) -> Self {
        let message = match err.kind() {
            std::io::ErrorKind::NotFound => {
                "ファイルまたはディレクトリが見つかりません".to_string()
            }
            std::io::ErrorKind::PermissionDenied => "アクセス権限がありません".to_string(),
            std::io::ErrorKind::ConnectionRefused => "接続が拒否されました".to_string(),
            std::io::ErrorKind::ConnectionReset => "接続がリセットされました".to_string(),
            std::io::ErrorKind::ConnectionAborted => "接続が中止されました".to_string(),
            std::io::ErrorKind::NotConnected => "接続されていません".to_string(),
            std::io::ErrorKind::AddrInUse => "アドレスが使用中です".to_string(),
            std::io::ErrorKind::AddrNotAvailable => "アドレスが利用できません".to_string(),
            std::io::ErrorKind::BrokenPipe => "パイプが破損しています".to_string(),
            std::io::ErrorKind::AlreadyExists => "ファイルが既に存在します".to_string(),
            std::io::ErrorKind::WouldBlock => "操作がブロックされます".to_string(),
            std::io::ErrorKind::InvalidInput => "無効な入力です".to_string(),
            std::io::ErrorKind::InvalidData => "無効なデータです".to_string(),
            std::io::ErrorKind::TimedOut => "タイムアウトしました".to_string(),
            std::io::ErrorKind::WriteZero => "書き込みできませんでした".to_string(),
            std::io::ErrorKind::Interrupted => "操作が中断されました".to_string(),
            std::io::ErrorKind::UnexpectedEof => "予期しないファイル終端です".to_string(),
            _ => format!("I/Oエラーが発生しました: {}", err),
        };
        Self::io(message)
    }
}

impl From<serde_yaml::Error> for GworkError {
    fn from(err: serde_yaml::Error) -> Self {
        Self::config(format!("YAML設定ファイルの解析に失敗しました: {}", err))
    }
}

/// アプリケーション全体で使用する統一Result型
pub type GworkResult<T> = Result<T, GworkError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let git_error = GworkError::git("テストGitエラー");
        assert!(matches!(git_error, GworkError::Git { .. }));
        assert_eq!(
            git_error.to_string(),
            "Gitリポジトリエラー: テストGitエラー"
        );

        let io_error = GworkError::io("テストIOエラー");
        assert!(matches!(io_error, GworkError::Io { .. }));
        assert_eq!(io_error.to_string(), "ファイル操作エラー: テストIOエラー");

        let config_error = GworkError::config("テスト設定エラー");
        assert!(matches!(config_error, GworkError::Config { .. }));
        assert_eq!(
            config_error.to_string(),
            "設定ファイルエラー: テスト設定エラー"
        );

        let workspace_error = GworkError::workspace("テストワークスペースエラー");
        assert!(matches!(workspace_error, GworkError::Workspace { .. }));
        assert_eq!(
            workspace_error.to_string(),
            "ワークスペースエラー: テストワークスペースエラー"
        );

        let tui_error = GworkError::tui("テストTUIエラー");
        assert!(matches!(tui_error, GworkError::Tui { .. }));
        assert_eq!(tui_error.to_string(), "TUIエラー: テストTUIエラー");

        let general_error = GworkError::general("テスト一般エラー");
        assert!(matches!(general_error, GworkError::General { .. }));
        assert_eq!(general_error.to_string(), "エラー: テスト一般エラー");
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "test file not found");
        let gwork_err: GworkError = io_err.into();
        assert!(matches!(gwork_err, GworkError::Io { .. }));
        assert!(
            gwork_err
                .to_string()
                .contains("ファイルまたはディレクトリが見つかりません")
        );
    }

    #[test]
    fn test_yaml_error_conversion() {
        let yaml_content = "invalid: yaml: content: [";
        let yaml_err = serde_yaml::from_str::<serde_yaml::Value>(yaml_content).unwrap_err();
        let gwork_err: GworkError = yaml_err.into();
        assert!(matches!(gwork_err, GworkError::Config { .. }));
        assert!(
            gwork_err
                .to_string()
                .contains("YAML設定ファイルの解析に失敗しました")
        );
    }

    #[test]
    fn test_error_display() {
        let error = GworkError::workspace("テストメッセージ");
        let formatted = format!("{}", error);
        assert_eq!(formatted, "ワークスペースエラー: テストメッセージ");
    }

    #[test]
    fn test_error_debug() {
        let error = GworkError::git("デバッグテスト");
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("Git"));
        assert!(debug_str.contains("デバッグテスト"));
    }
}
