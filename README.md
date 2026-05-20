# 🚀 PortSage - Process + Port Explorer

PortSage is a sleek TUI (Terminal User Interface) tool that helps you:

* 🧭 **Explore processes** on your machine
* 🔍 **Filter** by name, command, PID, or port
* 📍 **View ports** associated with each process
* 🪄 **Inspect process details** in a side panel or floating modal
* 🗑️ **Kill processes interactively** with confirmation dialog
* 📋 **Copy PID** to clipboard with one key
* 🎨 **Switch themes** between Kanagawa, Tokyo Night, and Nord
* 🪟 **Adapts to terminal width** with a two-pane layout on wide screens

---

## ✨ Features

### 📊 Process Overview

* Lists all processes
* Shows PID, name, ports, and command line
* Port-bound processes are sorted to the top

### 🎯 Filter Mode

* Press `:` to enter filter mode
* Matches against name, command, and PID

### 🔎 Detail Mode

* Press `Tab` to open process detail window
* Shows full metadata including:

  * CPU usage, memory
  * Executable path
  * Current working directory
  * Listening ports

### 📋 Copy PID

* Press `Enter` to copy the selected PID to clipboard
* Confirmation message appears for 2 seconds

### ❌ Kill Process

* Press `x` to open confirmation dialog
* Press `y` to kill the process (SIGKILL)
* Press `n` or `Esc` to cancel
* ✅ Success message shown after killing

### 🪟 Responsive Layout

* Terminals **≥ 100 columns** wide split into a list pane (left) and a live detail panel (right)
* The right panel updates as you navigate; `Tab` still opens a full-screen expanded view
* Narrower terminals fall back to a single-pane list with the floating Tab modal
* The whole UI is capped at **120 columns** and centered, so ultra-wide terminals stay readable

### 🎨 Themes

* Three built-in palettes: **Kanagawa** (default), **Tokyo Night**, **Nord**
* Pick one with `--theme <name>` or set `PORTSAGE_THEME=<name>`
* The CLI flag wins over the env var; unknown names exit with a clear error

```bash
portsage                          # Kanagawa (default)
portsage --theme tokyonight       # Tokyo Night via flag
PORTSAGE_THEME=nord portsage      # Nord via env var
```

---

## 🎮 Key Bindings

| Key          | Action                 |
| ------------ | ---------------------- |
| `j` / `Down` | Move down              |
| `k` / `Up`   | Move up                |
| `:`          | Enter filter mode      |
| `Tab`        | Show detail            |
| `Enter`      | Copy PID to clipboard  |
| `x`          | Kill process (confirm) |
| `q` / `Esc`  | Quit                   |

---

## 🛠️ Install & Run

```bash
cargo build --release
./target/release/portsage
```

### 📦 Install from crates.io

```bash
cargo install portsage
portsage
```

### ❄️ Install via Nix Flakes


```bash
nix run github:mei28/PortSage
```
---

## 📦 CLI Options

```bash
USAGE:
    portsage [OPTIONS]

OPTIONS:
        --cli               Use CLI mode (non-interactive). TUI is the default.
    -f, --filter <STRING>   Filter keyword
    -p, --port <PORT>       Filter by port
        --json              Output as JSON (CLI mode only)
        --kill <PID>        Kill process by PID
        --theme <NAME>      Color theme: kanagawa, tokyonight, or nord
    -h, --help              Print help
    -V, --version           Print version
```

Theme can also be set via the `PORTSAGE_THEME` environment variable. The CLI flag takes precedence.

---


## 🧪 Requirements

* 🦀 Rust 1.70+
* Linux/macOS (requires `lsof`)

---

## 🎯 Examples

### TUI Mode (Default)
```bash
# Launch interactive TUI
portsage

# Use a specific theme
portsage --theme tokyonight

# Or set the theme via env var
PORTSAGE_THEME=nord portsage
```

### CLI Mode
```bash
# List all processes with ports
portsage --cli

# Filter by process name
portsage --cli --filter nginx

# Filter by port
portsage --cli --port 8080

# Output as JSON
portsage --cli --json

# Kill a process
portsage --kill 1234
```

