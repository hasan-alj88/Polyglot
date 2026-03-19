---
issue: "002"
title: Document the full % metadata schema tree
related: PGE-206 (Rule 2.6)
priority: documentation
status: resolved
resolved: 2026-03-18
created: 2026-03-18
---

# 002 — Document the full `%` metadata schema tree

## Problem

The `%` metadata system is partially documented across `types.md`, `blocks.md`, and `identifiers.md`, but there is no single reference that describes the complete schema tree, instance references, or which fields are `live` vs non-live.

## What needs documenting

### Schema tree structure
Metadata is organized by block type:
- `%pipeline.*` — metadata for `{=}` pipeline definitions
- `%#.*` — metadata for `{#}` data type definitions
- `%M.*` — metadata for `{M}` macro definitions

### Instance references
The `*` in `%pipeline.*` is the instance reference. One pipeline definition may spawn several concurrent instances, each with its own metadata set. The schema is **fixed** per block type — all instances share the same field structure, but each instance has its own values.

### `live` vs non-live fields
- **`live` fields**: Runtime-managed, pull-only (PGE-206 applies). Examples: `%status`, `%state`, `%errors`, `%isSuccess`, `%instanceCount`, `%lastRun`, `%duration`, `%triggerCount`
- **Non-live fields**: User-assignable, follow normal lifecycle rules. Examples: `%description`, aliases

### Existing partial docs
- `docs/user/syntax/types.md` — `live` type modifier semantics
- `docs/user/syntax/blocks.md` — `[%]` block element, lists some `live` fields
- `docs/user/syntax/identifiers.md` — `%` accessor overview
- `docs/user/concepts/variable-lifecycle.md` — `$var%state` usage

## Resolution

Created `docs/user/concepts/metadata.md` as the consolidated spec. Contains the full metadata tree, all `live` and user-declared fields, instance reference semantics, and related types. Existing docs (`pipelines.md`, `blocks.md`, `types.md`, `variable-lifecycle.md`) now reference it via `[[metadata]]` wikilinks instead of duplicating content.

## See also

- [metadata.md](../../user/concepts/metadata.md)
