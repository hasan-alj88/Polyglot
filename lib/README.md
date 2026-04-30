# lib/ — Aljam3 Core & Integration

This directory contains the Aljam3 language compiler and host-language integration SDKs.

## Structure

| Folder | Purpose |
|--------|---------|
| `Aljam3-Core/` | The Aljam3 compiler (Rust crate) — lexer, compiler, and the `.aj3` standard library (`aj3lib/`) |
| `Aljam3-Integration/` | Language-specific SDKs for embedding Aljam3 into host-language codebases (Rust, Go, Python) |

### Aljam3-Core vs Aljam3-Integration

- **Aljam3-Core** is the _implementation of the Aljam3 language itself_ — tokenizing `.aj3` source, validating it, and producing AST JSON. It happens to be written in Rust, but it _is_ Aljam3.
- **Aljam3-Integration** contains SDKs that let _other_ languages (Rust, Go, Python, etc.) call into Aljam3 and be called by Aljam3 at runtime. Each language folder implements the native-operation contract.

See each folder's `README.md` for details.
