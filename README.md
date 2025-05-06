# rushstr

**rushstr** is a blazing-fast, Rust-powered interactive shell history searcher inspired by [`hstr`](https://github.com/dvorka/hstr). Navigate, search, and reuse your command-line history with speed and style.

---

## ✨ Features

- 🚀 Fast and lightweight — written in Rust
- 🔍 Fuzzy search through your shell history
- ⌨️ Keyboard-driven interactive UI
- 🧠 Learns from your usage patterns (coming soon!)
- 💻 Works with Bash, Zsh, and Fish shells

---

## 📦 Installation

### Using `cargo`

```bash
cargo install rushstr
```

> Requires [Rust](https://www.rust-lang.org/tools/install)

### From source

```bash
git clone https://github.com/donhk/rushstr.git
cd rushstr
cargo build --release
./target/release/rushstr
```

---

## 🔧 Shell Integration

Enable `rushstr` as your reverse history search (e.g., replacing `Ctrl+R`):

### Bash

Add this to your `~/.bashrc`:

```bash
bind -x '"\C-r": "rushstr"'
```

Then apply changes:

```bash
source ~/.bashrc
```

---

### Zsh

Add this to your `~/.zshrc`:

```zsh
bindkey '^R' rushstr
```

Then apply changes:

```bash
source ~/.zshrc
```

---

### Fish

Add this to your `~/.config/fish/config.fish`:

```fish
function rushstr_search
    rushstr
end
bind \cr rushstr_search
```

Then apply changes:

```fish
source ~/.config/fish/config.fish
```

---

## 📸 Screenshots

<img src="./assets/demo.gif" alt="rushstr demo" width="600"/>

---

## 🛠 Development

```bash
# Run in debug mode
cargo run

# Run tests
cargo test
```

---
