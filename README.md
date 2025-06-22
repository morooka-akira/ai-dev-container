# AI Workspace Manager

[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A simple yet powerful CLI tool for managing multiple development workspaces using Git worktrees. Create isolated work environments in seconds with automated setup and an intuitive TUI for workspace management.

## ✨ Features

- **🚀 Quick Workspace Creation**: Create new Git worktrees with dedicated branches in one command
- **📁 File Management**: Automatically copy configuration files to new workspaces
- **⚡ Pre-command Execution**: Run setup commands (npm install, cargo build, etc.) automatically
- **🎯 Interactive TUI**: Browse, navigate, and manage workspaces with an intuitive terminal interface
- **⚙️ Configurable**: YAML-based configuration for customizing behavior
- **🔗 Shell Integration**: Seamlessly navigate to workspaces from the terminal

## 🛠️ Installation

### From Source

```bash
git clone https://github.com/morooka-akira/gwork.git
cd gwork
cargo build --release
```

### Using Cargo

```bash
cargo install --path .
```

## 🚀 Quick Start

### 1. Create Your First Workspace

```bash
ai-workspace start feature-authentication
```

This creates:
- A new Git worktree in `../workspaces/20250621-143022-feature-authentication`
- A new branch `work/feature-authentication`
- Copies configured files to the new workspace
- Runs pre-configured setup commands

### 2. List and Manage Workspaces

```bash
ai-workspace list
```

Opens an interactive TUI where you can:
- Navigate through workspaces with ↑/↓ or j/k
- Press Enter to switch to a workspace
- Press 'd' to delete a workspace
- Press 'i' for detailed information
- Press 'q' to quit

## ⚙️ Configuration

Create a `workspace.yml` file in your project root:

```yaml
workspace:
  # Base directory for workspaces
  base_dir: "../workspaces"
  
  # Branch name prefix
  branch_prefix: "work/"
  
  # Files to copy to new workspaces
  copy_files:
    - .env
    - .env.local
    - config/secrets.json
    
  # Commands to run after workspace creation
  pre_commands:
    - "npm install"
    - "cargo build"
```

## 📖 Usage

### Commands

#### `start <task-name>`
Creates a new workspace for the given task.

```bash
ai-workspace start feature-user-auth
ai-workspace start bugfix-login --config custom.yml
```

Options:
- `--config <file>`: Use custom configuration file (default: `workspace.yml`)

#### `list`
Opens the interactive TUI for workspace management.

```bash
ai-workspace list
ai-workspace list --config custom.yml
```

### TUI Controls

| Key | Action |
|-----|--------|
| ↑/↓ or j/k | Navigate workspaces |
| Enter | Open selected workspace |
| d | Delete workspace (with confirmation) |
| i | Show workspace details |
| r | Refresh workspace list |
| q/Esc | Quit |

### Shell Integration

Add this function to your `.bashrc` or `.zshrc` for seamless navigation:

```bash
awl() {
    local target_path
    target_path=$(ai-workspace list --print-path-only)
    if [ -n "$target_path" ]; then
        cd "$target_path"
    fi
}
```

## 🏗️ Development

### Prerequisites

- Rust 2024 edition or later
- Git (for worktree operations)

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Check code style
cargo fmt
cargo clippy --all-targets -- -D warnings
```

### Project Structure

```
src/
├── main.rs          # CLI entry point
├── cli.rs           # Command-line argument parsing
├── workspace.rs     # Git worktree operations
├── config.rs        # Configuration file handling
├── utils.rs         # Utility functions
└── tui/             # Terminal UI components (planned)
    ├── mod.rs
    ├── app.rs       # Application state
    ├── ui.rs        # UI rendering
    └── events.rs    # Event handling
```

## 🎯 Roadmap

- [x] Basic CLI framework
- [x] Git worktree integration
- [x] Configuration file support
- [x] File copying functionality
- [x] Pre-command execution
- [x] Interactive TUI (basic)
- [ ] Advanced TUI features (deletion, details)
- [ ] Shell integration helpers
- [ ] Workspace templates
- [ ] Synchronization commands

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by Git worktree functionality
- Built with [clap](https://github.com/clap-rs/clap) for CLI parsing
- TUI powered by [ratatui](https://github.com/ratatui-org/ratatui)
- Git operations using [git2](https://github.com/rust-lang/git2-rs)

---

**[日本語のドキュメント](README.ja.md) | [English Documentation](README.md)**