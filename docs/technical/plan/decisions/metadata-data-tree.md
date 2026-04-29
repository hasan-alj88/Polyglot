---
audience: developer
type: decision
updated: 2026-04-05
---

# Decision: Metadata Data Tree

**Date:** 2026-03-21
**Status:** Decided

## Context

All Aljam3 objects are serialized data (trees of RawStrings). Every object needs a `%` URL path. The string-re-subfields decision referenced `%Data.#:*:String:int` paths but the full tree was unspecified.

## Decisions

### Path Patterns

| Tree | Pattern | Example |
|------|---------|---------|
| Schema (compile-time) | `%definition.{type}:{ref}` | `%definition.#:Boolean` |
| Instance (runtime) | `%{type}:{ref}:{instance}.{fields}` | `%#:Boolean:0.True` |

No `%Data` prefix — instance paths go directly to `%{type}:{ref}:{instance}.{fields}`.

### Object Type Branches

The `%` root has fixed branches for each object type prefix:

| Branch | Objects | Name level |
|--------|---------|------------|
| `%#` | Structs | Flexible (`:type`) |
| `%-` | Pipelines | Flexible (`:name`) |
| `%=` | Expanders | Flexible (`:name`) |
| `%*` | Collectors | Flexible (`:name`) |
| `%$` | Variables | Flexible (`:name`) |
| `%!` | Errors | Flexible (`:namespace`) |
| `%@` | Packages | Flexible (`:address`) |

Plus `%definition` (fixed) for compile-time schema templates.

### String Subtypes Nested with Alias

`int` lives at `%#:String:int` (nested under `:String`). User code `;int` is an alias for `;String.int`. Subtypes use `#String` as their schema with `.regex` pre-filled.

### Enum Instances — Active-Field-Only

An enum instance collapses to ONE active field. `%#:Boolean:0.True` exists; `%#:Boolean:0.False` does NOT for that instance. The definition (`%definition.#:Boolean`) lists all valid branches.

**Architecture safeguard:** Runtime enforces exactly one active enum field per instance:
- Push atomically clears previous field and sets new one
- Read on non-active field returns no path
- Compiler rejects multi-set on same instance in same scope

### IO Ports Nested

`.<` and `.>` are fixed typed data sections within each pipeline/expander/collector instance. Parameter names are flexible within.

### Definition = Structural Template

`%definition.X:{ref}` ensures all `%X:{ref}:{n}` instances share the same structure.

### `#string` Fields Expand to #String

Any field typed `#string` expands to `.string#RawString` + `.regex#RawString` in the tree (the full `#String` struct).

## Updated Files

- `docs/user/syntax/types.md` — path notation corrected
- `docs/technical/plan/decisions/string-re-subfields.md` — paths corrected
- `docs/technical/brainstorming/string-re-subfields.md` — deferred item marked decided
- GitHub Issues — doc gap marked done (previously tracked in `docs/technical/plan/TODO.md`)
- `docs/draft.md` — full tree specification (source of truth for placement)
