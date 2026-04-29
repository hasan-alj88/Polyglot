---
audience: developer
type: specification
status: complete
updated: 2026-04-18
---

# Foreign Code Parsers

<!-- @c:technical/algorithms/foreign-code-analysis -->
<!-- @u:concepts/permissions/foreign-code -->

The Aljam3 compiler (written in Rust) needs to parse foreign code ASTs for permission analysis. This document specifies the parser toolchain, per-language strategies, and upgrade path.

## tree-sitter (Universal Base)

The `tree-sitter` crate with per-language grammar crates provides a uniform parsing framework. One dependency covers all supported languages.

| Language | Grammar Crate | AST Quality | Notes |
|----------|--------------|-------------|-------|
| Python | `tree-sitter-python` | Full CST | All call sites visible; sufficient for IO detection |
| Rust | `tree-sitter-rust` | Full CST | `use` declarations + call graph walkable |
| C | `tree-sitter-c` | Full CST | No preprocessor expansion; pragmatic for `[C]` blocks |
| C++ | `tree-sitter-cpp` | Full CST | No template resolution; covers call sites |
| JavaScript | `tree-sitter-javascript` | Full CST | Covers require/import + call detection |
| TypeScript | `tree-sitter-typescript` | Full CST | Type annotations ignorable for IO detection |
| Shell/Bash | `tree-sitter-bash` | Full CST | Command detection; variable expansion limited |

### Why tree-sitter

- **Single dependency** — uniform API across all languages
- **Actively maintained** — used by GitHub (code search), Neovim, Zed
- **CST detail** — concrete syntax tree is detailed enough to find `CallExpression` nodes and match function/module names
- **Single-file analysis** — no cross-module resolution, but IO calls are identifiable by name at the call site
- **Error recovery** — produces partial trees even for incomplete code (useful for `[C]` inline blocks)

## Per-Language Upgrades

For deeper analysis beyond call-site name matching, the compiler can upgrade to language-specific parsers:

| Language | Tool | Type | What It Adds Over tree-sitter |
|----------|------|------|-------------------------------|
| Python | `ruff_python_parser` | Rust-native crate | Full Python AST (not CST); import resolution within file; type-aware |
| Python | Python `ast` module via PyO3 | FFI | stdlib-level accuracy; requires Python runtime linked |
| Rust | `syn` | Rust-native crate | Full Rust AST; proc-macro level detail; very mature |
| Rust | `ra_ap_syntax` + `ra_ap_hir` | Rust-native crate | rust-analyzer's parser; cross-crate resolution; name resolution |
| C/C++ | `libclang` via `clang-sys` | C FFI | Full AST with macro expansion, cross-file resolution; requires clang |
| JS/TS | `oxc_parser` | Rust-native crate | Full JS/TS AST; very fast; from Oxc project |
| JS/TS | `swc_ecma_parser` | Rust-native crate | Full AST; mature; used by Next.js and Deno |

## Recommended Strategy

1. **Start with tree-sitter for all languages** — uniform, fast, sufficient for 90% of IO detection (call-site name matching + argument extraction)
2. **Upgrade Python first** — `ruff_python_parser` is Rust-native (no FFI), Python is the most common `-Run.*` target
3. **Upgrade others as needed** — `syn` for Rust, `libclang` for C/C++, `oxc_parser` for JS/TS
4. **tree-sitter remains the fallback** — for any language not yet upgraded, or for new languages added to `-Run.*`

## C/C++ Special Considerations

C/C++ has no serious Rust-native parser — the language is too complex (preprocessor, templates, overloads). The options are:

- **tree-sitter-c/cpp** — sufficient for `[C]` inline blocks where macro usage is limited and the code is short
- **`libclang` via `clang-sys` FFI** — the production answer for `-Run.C.*` file-based pipelines; provides full AST with macro expansion
- **Preprocessor limitation** — tree-sitter sees `#include` and `#define` as text, not expanded content. For `[C]` inline blocks this is acceptable. For full `.c` files, `libclang` is required to see the actual code after preprocessing.

## Integration Points

The parser output feeds directly into the detection algorithm ([[algorithms/foreign-code-analysis]]):

1. **Parser produces AST** — tree-sitter CST or language-specific AST
2. **Algorithm walks AST nodes** — `CallExpression`, `ImportDeclaration`, `Assignment`
3. **Import alias resolution** — parser identifies `import X as Y` nodes
4. **Call-site extraction** — parser provides callee name + argument positions
5. **Variable tracing** — parser provides assignment nodes for `trace_assignment`

The parser abstraction layer presents a uniform interface regardless of which parser backend is used:

```text
trait ForeignParser {
  fn parse(source: &str, language: Language) -> ForeignAst
  fn walk_calls(ast: &ForeignAst) -> Vec<CallNode>
  fn walk_imports(ast: &ForeignAst) -> Vec<ImportNode>
  fn walk_assignments(ast: &ForeignAst) -> Vec<AssignmentNode>
  fn is_banned_construct(node: &AstNode, language: Language) -> bool
}
```

## Related

- [[algorithms/foreign-code-analysis]] — the algorithm that consumes parser output
- [[compiler/io-registry]] — the sink tables matched against parsed call sites
- PGE10014 — banned constructs detection relies on parser identifying node types
