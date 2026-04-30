# Aljam3-Core

The core Aljam3 language compiler, implemented in Rust.

## Crate: `aljam3-core`

| Module | Description |
|--------|-------------|
| `lexer` | Tokenizes `.aj3` source files into a token stream |
| `compiler` | Validates tokens, enforces compile rules (PGE/PGW), and produces AST JSON |

### Pipeline

```
.aj3 source → Lexer → Token Stream → Compiler → AST JSON
```

## `aj3lib/` — Aljam3 Standard Library

The `aj3lib/` directory contains `.aj3` source definitions for all standard-library pipelines, types, and errors. These files are the compiler's source of truth for aj3lib.

| Folder | Contains |
|--------|----------|
| `tm/` | Trigger definitions (`=T.*`) |
| `qh/` | Queue definitions (`=Q.*`) |
| `runner/` | Execution + Wrapper definitions |
| `intrinsics/` | Compiler intrinsics (`=DoNothing`, `=#.*`) |
| `types/` | Data type definitions (`{#}`) |
| `errors/` | Error tree definitions (`{!}`) |
