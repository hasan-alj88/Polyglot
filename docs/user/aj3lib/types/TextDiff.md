---
audience: automation-builder
type: specification
updated: 2026-04-11
status: draft
metadata_definition: "%definition.#:TextDiff"
metadata_instance: "%#:TextDiff:N"
---

# #TextDiff Struct

<!-- @c:types -->

A single diff entry representing one line-level change in a text comparison. Produced by `=Text.Diff` and consumed by `*Into.Text.Merge` and `*Into.CSV.Merge` collectors.

`#TextDiffs` is an alias for `#Array.TextDiff` — a collection of diff entries from a single comparison.

---

## Definition

```aljam3
{#} #TextDiff
   [%] .description << "Single line-level diff entry"
   [#] ##Flat
   [.] .line#int
   [.] .op#DiffOp
   [.] .content#String
```

---

## Fields

| Field | Type | Purpose |
|-------|------|---------|
| `.line` | `#int` | Line number in the original text (zero-based) |
| `.op` | `#DiffOp` | Operation type: .Add, .Delete, or .Replace |
| `.content` | `#String` | Line content (new content for Add/Replace, original for Delete) |

---

## Alias

| Alias | Expands To | Description |
|-------|-----------|-------------|
| `#TextDiffs` | `#Array.TextDiff` | Collection of diff entries from a single comparison |

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:TextDiff` | Compile-time type template |
| Instance | `%#:TextDiff:N` | Runtime instance (N = instance number) |

---

## Related

- [[DiffOp]] — operation type enum used by .op field
- [[DiffStats]] — summary statistics from =Text.Diff
- [[MergeResult]] — output of k-way merge using #TextDiffs
- [[structs]] — other jm3lib struct types
- [[syntax/types/INDEX|types]] — full type system specification
