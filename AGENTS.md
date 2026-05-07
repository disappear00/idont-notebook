# AGENTS.md

## Quick start

```bash
cargo build --release
./target/release/idontnote
```

## Project overview

REPL-based notebook CLI written in Rust. Registers multiple note directories ("notebooks"), manages markdown/text notes with TOML metadata.

- **Binary name**: `idontnote`
- **Crate name**: `idont-notebook`
- **Rust edition**: 2024 (requires Rust 1.85+)
- **No tests** — none exist yet
- **No workspace** — single crate

## Architecture

```
src/
├── main.rs              # Entry point: creates Storage, runs REPL
├── command.rs           # Input parsing → Command enum (with aliases: mk, il, sl, etc.)
├── handler.rs           # Dispatch layer: Command → core module functions
├── repl.rs              # REPL loop (rustyline, dynamic prompt, tab completion)
├── storage/
│   ├── models.rs        # Data structs + constants (NotebookMeta, NoteMeta, GlobalConfig)
│   ├── config.rs        # GlobalConfig load/save (~/.idont/idont-notebook-config.toml)
│   ├── storage.rs       # Storage struct: notebook registry, note CRUD, selection state
│   └── error.rs         # StorageError enum (thiserror)
└── core/                # One file per command: initlib, selectlib, mknote, etc.
```

## Storage layout

- Global config: `~/.idont/idont-notebook-config.toml` (registered notebook paths)
- Per-notebook: `<notebook_path>/.notes/notes.toml` + `.notes/data/*.md`

## npm distribution

`bin/idontnote.js` is a Node.js wrapper that locates platform-specific binaries (`bin/idontnote-<target>`). Published via GitHub Actions on version tags (`v*`). The CI cross-compiles for 6 targets (linux x64/arm64, macOS x64/arm64, windows x64/arm64).

## Conventions

- **All user-facing text is in Chinese** (error messages, help output, REPL prompts)
- Commands have short aliases: `mk`, `il`, `sl`, `cl`, `ls`, `rm`, `ed`, `ca`, `log`
- Notes must end in `.md` or `.txt` (enforced in `command.rs:validate_note_filename`)
- `rmnote` prompts for confirmation before deletion
- REPL prompt shows current notebook: `idont(<name>)> `

## Dependencies

| Crate | Purpose |
|-------|---------|
| rustyline 15 | REPL (readline, completion) |
| serde + toml | Config serialization |
| thiserror | Error derive |
| chrono | Timestamps |
| dirs | Home directory |

## Build & release

- Local: `cargo build --release`
- npm publish: push a `v*` tag → GitHub Actions builds + publishes to npm
- `scripts/check-binaries.js` validates platform binaries exist before npm publish
