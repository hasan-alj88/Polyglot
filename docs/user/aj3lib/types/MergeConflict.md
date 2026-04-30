---
audience: automation-builder
type: specification
updated: 2026-04-11
status: draft
metadata_definition: "%definition.#:MergeConflict"
metadata_instance: "%#:MergeConflict:N"
---

# #MergeConflict Struct

<!-- @c:types -->

An unresolved conflict from a k-way merge operation. When multiple parallel jobs produce `#TextDiffs` that modify the same line, the conflict is recorded here for user-driven resolution.

---

## Definition

```aljam3
{#} #MergeConflict
   [%] .description << "Unresolved conflict from k-way merge"
   [#] ##Flat
   [.] .line#int
   [.] .sources#Array.TextDiff
```

---

## Fields

| Field | Type | Purpose |
|-------|------|---------|
| `.line` | `#int` | Line number where conflict occurred |
| `.sources` | `#Array.TextDiff` | Competing diff entries from different jobs |

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:MergeConflict` | Compile-time type template |
| Instance | `%#:MergeConflict:N` | Runtime instance (N = instance number) |

---

## Related

- [[MergeResult]] — contains an array of #MergeConflict in .conflicts field
- [[MergeStrategy]] — conflict resolution strategies
- [[TextDiff]] — individual diff entries that may conflict
- [[structs]] — other aj3lib struct types
- [[syntax/types/INDEX|types]] — full type system specification
