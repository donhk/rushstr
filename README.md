# 🚀 rushstr

**rushstr** is a fast, Rust-powered interactive shell history searcher — a modern, dependency-free alternative to [`hstr`](https://github.com/dvorka/hstr). Search, filter, and reuse your command-line history with fuzzy matching and a responsive TUI.

---

## ✨ Highlights

- ⚡ **Blazing fast** — implemented in pure Rust
- 🔍 **Fuzzy, regex, and exact** matchers
- 🎯 **Keyboard-first interface** — no mouse needed
- 💾 **Persistent command history** with favorites and usage stats
- 🐚 **Zsh support** — seamlessly integrates into your existing shell

---

## 📦 Installation

### Using `cargo`

```zsh
cargo install rushstr
```

> Requires [Rust](https://www.rust-lang.org/tools/install)

### From source

```zsh
git clone https://github.com/donhk/rushstr.git
cd rushstr
cargo build --release
./target/release/rushstr
```

### Pre-built binaries

_Coming soon..._

---

## 🔧 Shell Integration

To use `rushstr` as your reverse history search (e.g., replacing `Ctrl+R`):

### Zsh

Add this to your `~/.zshrc`:

```zsh
eval "$(rushstr --zsh-shell-conf)"
```

Then apply the changes:

```zsh
source ~/.zshrc
```

---

## 🎮 Keybindings

| Key Combo | Action                          |
|-----------|---------------------------------|
| `Ctrl+X`  | Mark/unmark as favorite         |
| `Ctrl+F`  | Show only favorites             |
| `Ctrl+T`  | Switch between matching modes   |
| `Enter`   | Select and print command        |
| `Ctrl+C`  | Copy selected command to clipboard and exit |

---

## 📸 Screenshot

<img src="./assets/demo.png" alt="rushstr demo" width="600"/>

---

## 🛠 Development

```bash
# Run in debug mode
cargo run

# Run tests
cargo test
```