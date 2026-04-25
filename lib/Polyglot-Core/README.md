# Polyglot-Core

The core Polyglot language compiler, implemented in Rust.

## Crate: `polyglot-core`

| Module | Description |
|--------|-------------|
| `lexer` | Tokenizes `.pg` source files into a token stream |
| `compiler` | Validates tokens, enforces compile rules (PGE/PGW), and produces AST JSON |

### Pipeline

```
.pg source → Lexer → Token Stream → Compiler → AST JSON
```

## `pglib/` — Polyglot Standard Library

The `pglib/` directory contains `.pg` source definitions for all standard-library pipelines, types, and errors. These files are the compiler's source of truth for pglib.

| Folder | Contains |
|--------|----------|
| `tm/` | Trigger definitions (`=T.*`) |
| `qh/` | Queue definitions (`=Q.*`) |
| `runner/` | Execution + Wrapper definitions |
| `intrinsics/` | Compiler intrinsics (`=DoNothing`, `=#.*`) |
| `types/` | Data type definitions (`{#}`) |
| `errors/` | Error tree definitions (`{!}`) |
