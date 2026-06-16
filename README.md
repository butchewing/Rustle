# 🦀 Rustle

```
    ____             __  __   
   / __ \__  _______/ /_/ /__ 
  / /_/ / / / / ___/ __/ / _ \
 / _, _/ /_/ (__  ) /_/ /  __/
/_/ |_|\__,_/____/\__/_/\___/ 
```

**The fast, delightful, Rust-powered CLI task manager that actually gets sh*t done.**

No bloat. No distractions. Just pure speed and reliability.

---

## ✨ Features

- ✅ Add tasks with natural language due dates (`--due "tomorrow"`, `--due "2 weeks"`, `--due "next monday"`)
- 📋 Smart listing with filters: `--pending`, `--completed`, `--overdue`
- 🎨 Beautiful colored terminal output with overdue highlighting
- 🗄️ Zero-config SQLite database (`tasks.db`)
- ⚡ Blazing fast and memory-safe (thanks to Rust)
- 🛠️ Easy to extend and hack on

## 🚀 Quick Start

```bash
# 1. Install Rust (if you don't have it)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Install Rustle
cargo install --git https://github.com/butchewing/rustle.git

# 3. Start rustling!
rustle add "Finish the Rustle README" --due "today"
rustle list
rustle list --overdue
rustle complete 1
```

## 📖 Available Commands

| Command                    | Description                              | Example |
|---------------------------|------------------------------------------|--------|
| `add <description>`       | Create a new task                        | `rustle add "Buy milk" --due "2 days"` |
| `list`                    | Show all tasks (default)                 | `rustle list` |
| `list --pending`          | Show only pending tasks                  | `rustle list --pending` |
| `list --completed`        | Show only completed tasks                | `rustle list --completed` |
| `list --overdue`          | Show overdue tasks                       | `rustle list --overdue` |
| `complete <id>`           | Mark a task as done                      | `rustle complete 3` |
| `delete <id>`             | Delete a task                            | `rustle delete 5` |

## 🛠️ Building from Source

```bash
git clone https://github.com/butchewing/rustle.git
cd rustle
cargo build --release

# Run it directly
./target/release/rustle list
```

## Contributing

Found a bug? Want to add priorities, search, or recurring tasks?  
PRs are very welcome! Fork it and start rustling some code.

## License

[MIT License](LICENSE) — Free to use, modify, and ship.

---

**Made with ❤️ for fellow Rustaceans and productivity nerds.**

*Built with Grok + Rust*
