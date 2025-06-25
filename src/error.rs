use thiserror::Error;

/// Unified error type used throughout the application
#[derive(Error, Debug)]
pub enum GitwsError {
    /// Git repository related errors
    #[error("Git repository error: {message}")]
    Git { message: String },

    /// File operation related errors
    #[error("File operation error: {message}")]
    Io { message: String },

    /// Configuration file related errors
    #[error("Configuration file error: {message}")]
    Config { message: String },

    /// Workspace related errors
    #[error("Workspace error: {message}")]
    Workspace { message: String },

    /// TUI related errors
    #[error("TUI error: {message}")]
    Tui { message: String },

    /// General errors
    #[error("Error: {message}")]
    General { message: String },
}

impl GitwsError {
    /// Create Git error
    pub fn git<S: Into<String>>(message: S) -> Self {
        Self::Git {
            message: message.into(),
        }
    }

    /// Create file operation error
    pub fn io<S: Into<String>>(message: S) -> Self {
        Self::Io {
            message: message.into(),
        }
    }

    /// Create configuration file error
    pub fn config<S: Into<String>>(message: S) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    /// Create workspace error
    pub fn workspace<S: Into<String>>(message: S) -> Self {
        Self::Workspace {
            message: message.into(),
        }
    }

    /// Create TUI error
    pub fn tui<S: Into<String>>(message: S) -> Self {
        Self::Tui {
            message: message.into(),
        }
    }

    /// Create general error
    #[allow(dead_code)]
    pub fn general<S: Into<String>>(message: S) -> Self {
        Self::General {
            message: message.into(),
        }
    }
}

impl From<git2::Error> for GitwsError {
    fn from(err: git2::Error) -> Self {
        let message = match err.code() {
            git2::ErrorCode::NotFound => "Specified resource not found".to_string(),
            git2::ErrorCode::Exists => "Resource already exists".to_string(),
            git2::ErrorCode::Ambiguous => "Resource specification is ambiguous".to_string(),
            git2::ErrorCode::BufSize => "Buffer size insufficient".to_string(),
            git2::ErrorCode::User => "User operation was cancelled".to_string(),
            git2::ErrorCode::BareRepo => "Cannot execute in bare repository".to_string(),
            git2::ErrorCode::UnbornBranch => "Branch does not exist".to_string(),
            git2::ErrorCode::Unmerged => "Unmerged changes exist".to_string(),
            git2::ErrorCode::NotFastForward => "Cannot fast-forward".to_string(),
            git2::ErrorCode::InvalidSpec => "Invalid specification".to_string(),
            git2::ErrorCode::Conflict => "Conflict occurred".to_string(),
            git2::ErrorCode::Locked => "Resource is locked".to_string(),
            git2::ErrorCode::Modified => "File has been modified".to_string(),
            git2::ErrorCode::Auth => "Authentication failed".to_string(),
            git2::ErrorCode::Certificate => "Certificate error".to_string(),
            git2::ErrorCode::Applied => "Patch already applied".to_string(),
            git2::ErrorCode::Peel => "Failed to expand object".to_string(),
            git2::ErrorCode::Eof => "Reached end of file".to_string(),
            git2::ErrorCode::Invalid => "Invalid operation".to_string(),
            git2::ErrorCode::Uncommitted => "Uncommitted changes exist".to_string(),
            git2::ErrorCode::Directory => "Directory does not exist".to_string(),
            git2::ErrorCode::MergeConflict => "Merge conflict occurred".to_string(),
            git2::ErrorCode::HashsumMismatch => "Hashsum mismatch".to_string(),
            git2::ErrorCode::IndexDirty => "Index has been modified".to_string(),
            git2::ErrorCode::ApplyFail => "Failed to apply patch".to_string(),
            _ => format!("Git error occurred: {}", err.message()),
        };
        Self::git(message)
    }
}

impl From<std::io::Error> for GitwsError {
    fn from(err: std::io::Error) -> Self {
        let message = match err.kind() {
            std::io::ErrorKind::NotFound => "File or directory not found".to_string(),
            std::io::ErrorKind::PermissionDenied => "Permission denied".to_string(),
            std::io::ErrorKind::ConnectionRefused => "Connection refused".to_string(),
            std::io::ErrorKind::ConnectionReset => "Connection reset".to_string(),
            std::io::ErrorKind::ConnectionAborted => "Connection aborted".to_string(),
            std::io::ErrorKind::NotConnected => "Not connected".to_string(),
            std::io::ErrorKind::AddrInUse => "Address in use".to_string(),
            std::io::ErrorKind::AddrNotAvailable => "Address not available".to_string(),
            std::io::ErrorKind::BrokenPipe => "Broken pipe".to_string(),
            std::io::ErrorKind::AlreadyExists => "File already exists".to_string(),
            std::io::ErrorKind::WouldBlock => "Operation would block".to_string(),
            std::io::ErrorKind::InvalidInput => "Invalid input".to_string(),
            std::io::ErrorKind::InvalidData => "Invalid data".to_string(),
            std::io::ErrorKind::TimedOut => "Timed out".to_string(),
            std::io::ErrorKind::WriteZero => "Write zero".to_string(),
            std::io::ErrorKind::Interrupted => "Interrupted".to_string(),
            std::io::ErrorKind::UnexpectedEof => "Unexpected end of file".to_string(),
            _ => format!("I/O error occurred: {err}"),
        };
        Self::io(message)
    }
}

impl From<serde_yaml::Error> for GitwsError {
    fn from(err: serde_yaml::Error) -> Self {
        Self::config(format!("Failed to parse YAML configuration file: {err}"))
    }
}

/// Unified Result type used throughout the application
pub type GitwsResult<T> = Result<T, GitwsError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let git_error = GitwsError::git("Test Git error");
        assert!(matches!(git_error, GitwsError::Git { .. }));
        assert_eq!(
            git_error.to_string(),
            "Git repository error: Test Git error"
        );

        let io_error = GitwsError::io("Test IO error");
        assert!(matches!(io_error, GitwsError::Io { .. }));
        assert_eq!(io_error.to_string(), "File operation error: Test IO error");

        let config_error = GitwsError::config("Test config error");
        assert!(matches!(config_error, GitwsError::Config { .. }));
        assert_eq!(
            config_error.to_string(),
            "Configuration file error: Test config error"
        );

        let workspace_error = GitwsError::workspace("Test workspace error");
        assert!(matches!(workspace_error, GitwsError::Workspace { .. }));
        assert_eq!(
            workspace_error.to_string(),
            "Workspace error: Test workspace error"
        );

        let tui_error = GitwsError::tui("Test TUI error");
        assert!(matches!(tui_error, GitwsError::Tui { .. }));
        assert_eq!(tui_error.to_string(), "TUI error: Test TUI error");

        let general_error = GitwsError::general("Test general error");
        assert!(matches!(general_error, GitwsError::General { .. }));
        assert_eq!(general_error.to_string(), "Error: Test general error");
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "test file not found");
        let gitws_err: GitwsError = io_err.into();
        assert!(matches!(gitws_err, GitwsError::Io { .. }));
        assert!(gitws_err
            .to_string()
            .contains("File or directory not found"));
    }

    #[test]
    fn test_yaml_error_conversion() {
        let yaml_content = "invalid: yaml: content: [";
        let yaml_err = serde_yaml::from_str::<serde_yaml::Value>(yaml_content).unwrap_err();
        let gitws_err: GitwsError = yaml_err.into();
        assert!(matches!(gitws_err, GitwsError::Config { .. }));
        assert!(gitws_err
            .to_string()
            .contains("Failed to parse YAML configuration file"));
    }

    #[test]
    fn test_error_display() {
        let error = GitwsError::workspace("Test message");
        let formatted = format!("{error}");
        assert_eq!(formatted, "Workspace error: Test message");
    }

    #[test]
    fn test_error_debug() {
        let error = GitwsError::git("Debug test");
        let debug_str = format!("{error:?}");
        assert!(debug_str.contains("Git"));
        assert!(debug_str.contains("Debug test"));
    }
}
