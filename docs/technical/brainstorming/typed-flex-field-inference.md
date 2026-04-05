---
audience: developer
type: brainstorming
updated: 2026-04-05
---

# Brainstorm: Typed Flexible Field Inference

**Status:** Decided (2026-03-20)

## Problem

When pushing to a typed flexible field (`[:] :*#Type`), must the user explicitly state the type on the new key, or can the compiler infer it from the struct definition?

## Decision

**Inferred.** The compiler knows the type from the `[:] :*#Type` declaration — no explicit annotation needed. Stating the type explicitly is not required.

- **Omitted** — compiler infers from `[:] :*#Type` declaration
- **Stated and contradicts** — PGE04001 (type mismatch)

```polyglot
{#} #Registry
   [.] .plugins
      [:] :*#Handler

[ ] compiler infers :myPlugin is #Handler from [:] :*#Handler
[r] $registry.plugins:myPlugin << ...
```

## Compiler Inference Mechanism

This documents how the compiler resolves types on flexible field paths — needed for compiler implementation.

### Algorithm

1. **Resolve the parent path** — when the compiler sees `$registry.plugins:myPlugin`, resolve `$registry.plugins` to the `#Registry.plugins` struct level
2. **Check the level's field declarations** — find `[:] :*#Handler` at that level
3. **Apply the wildcard type** — any new `:key` at that level inherits `#Handler`
4. **Validate the pushed value** — the value being assigned must satisfy the `Handler` schema (PGE04002 if incomplete, PGE04001 if wrong type)

### Multi-Level Resolution

For nested typed flexible fields, the compiler resolves one level at a time. Each `:` level resolves its wildcard type, then uses that type's definition to continue resolution.

```polyglot
{#} #Config
   [.] .sections
      [:] :*#Section

{#} #Section
   [:] :*#Setting

{#} #Setting
   [.] .value#string
```

Path: `$config.sections:auth:timeout.value`

| Step | Path segment | Resolved from | Inferred type |
|------|-------------|---------------|---------------|
| 1 | `.sections` | `#Config` definition | known fixed field |
| 2 | `:auth` | `.sections` has `[:] :*#Section` | `#Section` |
| 3 | `:timeout` | `#Section` has `[:] :*#Setting` | `#Setting` |
| 4 | `.value` | `#Setting` definition | `#string` |

### Edge Cases

- **No `[:] :*#Type` at level** — untyped flexible field. No inference. Value is treated as `#serial` (schema-free).
- **Individually declared flex fields** (e.g., `[:] :specific#SomeType`) — the compiler matches the key name first. If no exact match, falls back to `[:] :*#Type` wildcard if present.

## Related

- [types.md §Typed Flexible Fields](../../user/syntax/types.md) — user-facing docs
- [TYPE-IDENTITY.md](../compile-rules/TYPE-IDENTITY.md) — structural matching rules
- PGE04001 — Type Mismatch
- PGE04002 — Schema Mismatch
