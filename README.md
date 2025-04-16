# rust_shell

A minimal shell written in Rust – work in progress.

## Features

- ✅ Run external system commands (`ls`, `echo`, `grep`, etc.)
- ✅ Built-in support for:
  - `cd` – change directory
  - `exit` – exit the shell
- ✅ Basic pipe support (e.g. `ls | grep txt`)

## Limitations

- No support yet for:
  - Quoted arguments (`echo "hello world"` will break)
  - Redirection (`>`, `<`)
  - Logic (`&&`, `||`)
  - Background execution (`&`)
- No command history or autocompletion
- Not yet handling signals (like `Ctrl+C`)

## Usage

Clone and run:

```bash
git clone https://github.com/BogdanR6/rust_shell
cd rust_shell
cargo run
```

You’ll see a `>` prompt where you can type shell commands.

Example:

```bash
> cd /tmp
> ls | grep .rs
> exit
```

## 📂 Structure

- `src/main.rs` – the main shell loop and command handling logic.

## Plans

- Support for quoted arguments, redirects and logical operands like && and || 
- Built-in command history
- Config file support
- Proper error messages and recovery

## 🤝 Contributing

Right now it’s a solo project, but suggestions or ideas are welcome via issues or discussions.
