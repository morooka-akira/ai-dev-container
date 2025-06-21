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
cargo clippy
```

## Project Structure

The project follows standard Rust/Cargo conventions:
- `Cargo.toml`: Project manifest and dependencies
- `src/main.rs`: Application entry point

## Current State

This is a newly initialized Rust project with a basic "Hello, world!" program. The project uses Rust edition 2024 and has no dependencies configured yet.
