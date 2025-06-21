# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

** 回答は日本語で答えてください **

## Project Overview

This is a Rust binary application project named `ai-dev-container`.

## Common Commands

```bash
# Build the project
cargo build

# Run the application
cargo run

# Build and run in release mode
cargo run --release

# Run tests
cargo test

# Run a specific test
cargo test test_name

# Check code without building
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy --all-targets -- -D warnings
```

## Project Structure

The project follows standard Rust/Cargo conventions:
- `Cargo.toml`: Project manifest and dependencies
- `src/main.rs`: Application entry point

## Current State

This is a newly initialized Rust project with a basic "Hello, world!" program. The project uses Rust edition 2024 and has no dependencies configured yet.

## Working Rules
- 作業を行う前に、設計、検討内容、作業リストを ai-docs/works/ 配下に必ず書いて作業を開始してください
    - format: yyyymmdd-<workname>.md
- 作業にファイル更新が入る場合は、mainブランチからブランチを切って作業してください
- 実装については、@ai-docs/designs/design.md を参照して進めてください。
- 実装方針に変更があった場合は必ず、@ai-docs/designs/design.md, @ai-docs/designs/tasks.md も更新してください。
- 実装は @ai-docs/designs/tasks.md を見て進め、リストの更新(チェック)も必ず行って同期してください
- 実装が完了したら、test, fmt, clippyは必ず通してください(コマンド参照)
- 完了したら、gh コマンドでPRを作成して報告してください
