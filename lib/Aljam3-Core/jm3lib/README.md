# jm3lib — Aljam3 Standard Library

Aljam3 Code (`.jm3`) definitions for all jm3lib pipelines, types, and errors. These files are the compiler's source of truth for jm3lib.

## Organization

| Folder | Contains | #NativeKind |
|--------|----------|-------------|
| `tm/` | Trigger definitions (`=T.*`) | `.Trigger` |
| `qh/` | Queue definitions (`=Q.*`) | `.Queue` |
| `runner/` | Execution + Wrapper definitions (`=File.*`, `=Math.*`, `=DT.*`, `=W.*`, etc.) | `.Execution`, `.Wrapper` |
| `intrinsics/` | Compiler intrinsics (`=DoNothing`, `=#.*`) | `.Intrinsic` |
| `types/` | Data type definitions (`{#}`) | N/A |
| `errors/` | Error tree definitions (`{!}`) | N/A |
| `compiler/` | Compiler-specific definitions | N/A |

See `docs/technical/spec/native-dispatch.md` for the dispatch architecture.
