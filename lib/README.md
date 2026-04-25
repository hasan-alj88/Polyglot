# lib/ — Polyglot Core & Integration

This directory contains the Polyglot language compiler and host-language integration SDKs.

## Structure

| Folder | Purpose |
|--------|---------|
| `Polyglot-Core/` | The Polyglot compiler (Rust crate) — lexer, compiler, and the `.pg` standard library (`pglib/`) |
| `Polyglot-Integration/` | Language-specific SDKs for embedding Polyglot into host-language codebases (Rust, Go, Python) |

### Polyglot-Core vs Polyglot-Integration

- **Polyglot-Core** is the _implementation of the Polyglot language itself_ — tokenizing `.pg` source, validating it, and producing AST JSON. It happens to be written in Rust, but it _is_ Polyglot.
- **Polyglot-Integration** contains SDKs that let _other_ languages (Rust, Go, Python, etc.) call into Polyglot and be called by Polyglot at runtime. Each language folder implements the native-operation contract.

See each folder's `README.md` for details.
