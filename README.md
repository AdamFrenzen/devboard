# devboard 🧑‍💻📋

A tiny TUI tool for saving and running terminal commands.

Helps prevent digging through shell history by providing easy access to saved commands.

![devboard screenshot](https://github.com/user-attachments/assets/662b312b-e6b8-4e28-8b49-a566458064d8)

---

## 🛠️ Features

- 📝 Save commands
- ⚡ Run them instantly with a keypress
- 🎮 Vim-like navigation with `esc`/`i`/`h`/`l`, `Enter` to run
- 💾 No more scrolling through your shell history

---

## 📦 Installation

### With Cargo

```bash
cargo install devboard
```

## 🚀 Usage

1. Press `i` to enter insert mode and type a command
2. Optional: press `Enter` to run input command without saving while in insert mode
3. Press `Esc` to return to normal mode
4. Press `s` (in normal mode) to save the current command
5. Use `h`/`l` (in normal mode) to navigate saved commands, `Enter` to run a saved command

## 💡 Example Use Case

You’re compiling a C++ file:

```
clang++ main.cpp -o main
./main
```

After a few rebuilds and edits, the ./main command gets buried in history.

With devboard, you just save both commands once — then quickly run them any time without scrolling.

## 📄 License

MIT © 2025 [AdamFrenzen](https://adamfrenzen.com)

See [LICENSE](./LICENSE) for full details.
