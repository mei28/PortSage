# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

**Build and Run:**
```bash
# Build release binary
cargo build --release

# Run TUI mode
cargo run -- --tui
# or
./target/release/portsage --tui

# Run CLI mode with options
cargo run -- --cli --filter <keyword>
cargo run -- --cli --port <port>
cargo run -- --cli --json
```

**Testing:**
```bash
# Run all tests
cargo test

# Run specific test file
cargo test --test cli_test
cargo test --test filter_test
cargo test --test port_test  
cargo test --test process_test
```

**Other Commands:**
```bash
# Check code
cargo check

# Format code
cargo fmt

# Run clippy linter
cargo clippy
```

## Architecture Overview

PortSage is a TUI application for exploring processes and their listening ports on Unix systems. The application has two main modes:

1. **CLI Mode** (`--cli` flag): Outputs process information as formatted tables or JSON
2. **TUI Mode** (default): Interactive terminal interface with filtering, detail views, and process management

### Core Modules

- **`process.rs`**: Process information collection using `sysinfo` crate, provides `ProcessInfo` struct with PID, name, command, ports, memory usage, etc.
- **`port.rs`**: Port-to-PID mapping using `lsof` command parsing 
- **`filter.rs`**: Process filtering logic by name, command, and PID
- **`tui/`**: Terminal UI implementation using `ratatui` and `crossterm`
  - State management with different modes (Normal, FilterInput, Detail, ConfirmKill)
  - Key bindings for navigation and actions
  - Clipboard integration for PID copying
  - Process termination with confirmation dialogs

### Key Data Flow

1. `get_all_processes()` collects system process information
2. `get_port_pid_map()` maps listening ports to process PIDs via `lsof`
3. Processes are enhanced with port information and sorted (port-bound processes first)
4. TUI provides interactive filtering, detail inspection, and process management
5. CLI mode applies filters and outputs formatted results

### Dependencies

- `sysinfo`: Cross-platform system information
- `ratatui` + `crossterm`: Terminal UI framework
- `clap`: Command-line argument parsing
- `nix`: Unix system calls (process termination)
- `arboard`: Clipboard access
- `tabled`: Table formatting for CLI output

The application requires `lsof` to be available on the system for port detection.