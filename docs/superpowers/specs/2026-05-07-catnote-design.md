# catnote command design

## Summary

Add a `catnote` command (alias `ca`) that prints note file contents to the terminal with optional line limiting.

## Syntax

```
catnote <filename> [-n <lines>] [-t <lines>]
ca     <filename> [-n <lines>] [-t <lines>]
```

- No flags: print entire file
- `-n N`: print first N lines (head)
- `-t N`: print last N lines (tail)
- `-n` and `-t` are mutually exclusive

## Files to modify

| File | Change |
|------|--------|
| `src/command.rs` | Add `Catnote` variant + parse logic |
| `src/core/catnote.rs` | New: read file, truncate lines, print |
| `src/core/mod.rs` | Register `catnote` module |
| `src/handler.rs` | Add `handle_catnote` + dispatch |
| `src/repl.rs` | Add `catnote`/`ca` to completions |
| `src/core/help.rs` | Add catnote to help text |

## Parsing

Hand-rolled arg parsing via `splitn`, matching `-n`/`-t` flags. No new dependencies.

## Errors

- File not found → `StorageError::NoteNotFound`
- Both `-n` and `-t` → error message
- N not a positive integer → error message
