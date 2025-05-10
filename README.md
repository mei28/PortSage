# 🚀 PortSage - Process + Port Explorer

PortSage is a sleek TUI (Terminal User Interface) tool that helps you:

* 🧭 **Explore processes** on your machine
* 🔍 **Filter** by name, command, PID, or port
* 📍 **View ports** associated with each process
* 🪄 **Inspect process details** in a floating modal
* 🗑️ **Kill processes interactively** with confirmation dialog
* 📋 **Copy PID** to clipboard with one key

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
./target/release/portsage --tui
```

### 📦 Install from crates.io

```bash
cargo install portsage
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
    -f, --filter <STRING>   Filter keyword
    -p, --port <PORT>       Filter by port
        --json              Output as JSON (non-TUI)
        --kill <PID>        Kill process by PID
        --tui               Launch interactive TUI
```

---


## 🧪 Requirements

* 🦀 Rust 1.70+
* Linux/macOS (requires `lsof`)

