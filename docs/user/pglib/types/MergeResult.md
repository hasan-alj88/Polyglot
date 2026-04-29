---
audience: automation-builder
type: specification
updated: 2026-04-11
status: draft
metadata_definition: "%definition.#:MergeResult"
metadata_instance: "%#:MergeResult:N"
---

# #MergeResult Struct

<!-- @c:types -->

Output from `*Into.Text.Merge` and `*Into.CSV.Merge` collectors. Contains the merged text and any unresolved conflicts. When `#MergeStrategy` is `.SkipConflicts`, non-conflicting changes are applied to `.text` while conflicting changes appear in `.conflicts` for later processing.

---

## Definition

```aljam3
{#} #MergeResult
   [%] .description << "Output from k-way merge collector"
   [#] ##Flat
   [.] .text#String
   [.] .conflicts#Array.MergeConflict
```

---

## Fields

| Field | Type | Purpose |
|-------|------|---------|
| `.text` | `#String` | Merged text output (with resolved changes applied) |
| `.conflicts` | `#Array.MergeConflict` | Unresolved conflicts (empty if no conflicts or strategy resolves all) |

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:MergeResult` | Compile-time type template |
| Instance | `%#:MergeResult:N` | Runtime instance (N = instance number) |

---

## Related

- [[MergeConflict]] — individual conflict records in .conflicts
- [[MergeStrategy]] — how conflicts are resolved
- [[TextDiff]] — diff entries that feed into the merge
- [[structs]] — other pglib struct types
- [[syntax/types/INDEX|types]] — full type system specification
