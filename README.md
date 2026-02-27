# dot-config

A web-based dotfile configuration UI built with [Leptos](https://leptos.dev/) and [Axum](https://github.com/tokio-rs/axum). Configure your dotfiles using dropdowns, toggles, and number inputs instead of reading manuals and memorizing syntax.

## Features

- **Schema-driven UI** — options render dynamically from schemas; adding a new option requires zero UI code changes
- **Live preview** — see the generated config file update in real-time as you change settings
- **Auto-import** — existing dotfiles are parsed and loaded into the UI on page load
- **Preserved content** — unrecognized lines (comments, custom functions, plugin configs) are kept verbatim
- **Automatic backups** — saves a timestamped copy to `~/.dot-config-backups/` before overwriting

## Supported Dotfiles

| Dotfile | Path | Options |
|---------|------|---------|
| Vimrc | `~/.vimrc` | ~40 (appearance, indentation, search, behavior, files) |
| Bash/Zsh | `~/.bashrc` | ~15 (editor, history, shell opts, aliases) |
| Git Config | `~/.gitconfig` | ~12 (user, core, branch/merge, UI) |
| Tmux | `~/.tmux.conf` | ~12 (prefix, mouse, display, status) |
| SSH Config | `~/.ssh/config` | ~10 per host (multi-host model) |

## Prerequisites

- [Rust](https://rustup.rs/) (nightly toolchain)
- `wasm32-unknown-unknown` target
- [cargo-leptos](https://github.com/leptos-rs/cargo-leptos)

```sh
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
cargo install cargo-leptos
```

## Quick Start

```sh
git clone https://github.com/madeye/dot-config.git
cd dot-config
cargo leptos watch
```

Open http://127.0.0.1:3000 in your browser.

## Project Structure

```
src/
├── app.rs              # Root component, router, sidebar
├── models/             # Schemas defining each dotfile's options
├── components/         # Reusable UI: dropdown, toggle, number, text, preview
├── generators/         # Produce valid config syntax from state
├── parsers/            # Parse existing dotfiles into state
├── server/             # File I/O, backup, dotfile detection
└── pages/              # Per-dotfile page components + home dashboard
```

## Tests

```sh
cargo test
```

## License

[MIT](LICENSE) — Max Lv <max.c.lv@gmail.com>
