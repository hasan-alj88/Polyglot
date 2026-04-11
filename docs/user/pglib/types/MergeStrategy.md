---
audience: pg-coder
type: specification
updated: 2026-04-11
status: draft
metadata_definition: "%definition.#:MergeStrategy"
metadata_instance: "%#:MergeStrategy:N"
---

# #MergeStrategy Enum

<!-- @c:types -->

Runtime `##Enum` type available in every `.pg` file. Controls how `*Into.Text.Merge` and `*Into.CSV.Merge` collectors handle conflicting changes from parallel jobs modifying the same line.

---

## Definition

```polyglot
{#} #MergeStrategy
   [%] .description << "Conflict resolution strategy for k-way merge collectors"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "mergestrategy"
   [.] .FirstWins
   [.] .LastWins
   [.] .KeepBoth
   [.] .SkipConflicts
   [.] .DiffOnly
```

---

## Variants

| Variant | Description |
|---------|-------------|
| `.FirstWins` | First job's change wins on conflict |
| `.LastWins` | Last job's change wins on conflict |
| `.KeepBoth` | Both changes kept, marked in output |
| `.SkipConflicts` | Apply non-conflicting changes only; output conflicts separately in `>result.conflicts` |
| `.DiffOnly` | Produce merged diff record without modifying anything |

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:MergeStrategy` | Compile-time type template |
| Instance | `%#:MergeStrategy:0` | Runtime instance (enum — one active field) |

---

## Related

- [[MergeResult]] — output type containing resolved text and unresolved conflicts
- [[MergeConflict]] — individual conflict record
- [[enums]] — other pglib enum types
- [[syntax/types/INDEX|types]] — full type system specification
