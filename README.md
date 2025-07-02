English | [日本語](README.ja.md)

# gitws - Git Worktree Manager

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

### Using Homebrew (Recommended)

```bash
brew tap morooka-akira/gitws
brew install gitws
```

### From Source

```bash
git clone https://github.com/morooka-akira/gitws.git
cd gitws
cargo build --release
```

### Using Cargo

```bash
cargo install --path .
```

## 🚀 Quick Start

### 1. Initialize Configuration (First Time Only)

```bash
gitws init
```

This creates a `.gitws.yml` file. Edit it as needed for your project.

### 2. Create Your First Workspace

```bash
gitws start feature-authentication
```

This creates:

- A new Git worktree in `../workspaces/20250625-HHMMSS-feature-authentication`
- A new branch `work/20250625-HHMMSS-feature-authentication`
- Copies configured files to the new workspace
- Runs pre-configured setup commands

### 3. List and Manage Workspaces

**Important**: To navigate to workspaces, shell function setup is required.

#### Shell Function Setup (One-time only)

Add the following to your `.bashrc` or `.zshrc`:

```bash
# Select and navigate to workspace via TUI
awl() {
    local target_path
    target_path=$(gitws list)
    if [ -n "$target_path" ]; then
        cd "$target_path"
    fi
}
```

After setup, restart your shell or run:

```bash
source ~/.bashrc  # or source ~/.zshrc
```

#### Usage

```bash
awl  # Opens TUI to select and navigate to workspace
```

**TUI Controls**:

- Navigate through workspaces with ↑/↓ or j/k
- **Press Enter to navigate to workspace**
- Press Space to toggle selection on current workspace
- Press 'a' to select/deselect all workspaces
- Press 'd' to delete selected workspace(s) (with confirmation)
- Press 'i' to show workspace details
- Press 'q' to quit

**Direct execution won't navigate**:

```bash
# ❌ This won't change directory
gitws list
```

## ⚙️ Configuration

### Generate Configuration File

```bash
gitws init
# or generate at custom path
gitws init --output my-config.yml
```

### Configuration Example

Create a `.gitws.yml` file in your project root:

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

#### `init`

Generate a configuration file template.

```bash
gitws init
gitws init --output custom.yml
gitws init -o my-config.yml
```

Options:

- `--output <file>` or `-o <file>`: Specify output file path (default: `.gitws.yml`)

#### `start <task-name>`

Creates a new workspace for the given task.

```bash
gitws start feature-user-auth
gitws start bugfix-login --config custom.yml
```

Options:

- `--config <file>` or `-c <file>`: Use custom configuration file (default: `.gitws.yml`)

#### `list`

Opens the interactive TUI for workspace management.

```bash
gitws list
gitws list --config custom.yml
gitws list --path-only  # Output paths only (for shell scripts)
```

Options:

- `--config <file>` or `-c <file>`: Use custom configuration file (default: `.gitws.yml`)
- `--path-only` or `-p`: Output only workspace paths

### TUI Controls

| Key        | Action                                           |
| ---------- | ------------------------------------------------ |
| ↑/↓ or j/k | Navigate workspaces                              |
| Enter      | Open selected workspace                          |
| Space      | Toggle selection on current workspace            |
| a          | Toggle select/deselect all                       |
| d          | Delete selected workspace(s) (with confirmation) |
| i          | Show workspace details                           |
| q/Esc      | Quit                                             |

### Shell Integration

Add this function to your `.bashrc` or `.zshrc` for seamless navigation:

```bash
# Select and navigate to workspace via TUI
awl() {
    local target_path
    target_path=$(gitws list)
    if [ -n "$target_path" ]; then
        cd "$target_path"
    fi
}

# List all workspace paths
awl-list() {
    gitws list --path-only
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
├── error.rs         # Error handling
├── utils.rs         # Utility functions
└── tui/             # Terminal UI components
    ├── mod.rs
    ├── app.rs       # Application state
    ├── ui.rs        # UI rendering
    └── events.rs    # Event handling
```

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
