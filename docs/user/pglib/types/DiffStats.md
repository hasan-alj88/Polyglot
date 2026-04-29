---
audience: automation-builder
type: specification
updated: 2026-04-11
status: draft
metadata_definition: "%definition.#:DiffStats"
metadata_instance: "%#:DiffStats:N"
---

# #DiffStats Struct

<!-- @c:types -->

Summary statistics from a `=Text.Diff` comparison. Reports the count of additions, deletions, and unchanged lines.

---

## Definition

```aljam3
{#} #DiffStats
   [%] .description << "Summary statistics from text diff comparison"
   [#] ##Flat
   [.] .additions#int
   [.] .deletions#int
   [.] .unchanged#int
```

---

## Fields

| Field | Type | Purpose |
|-------|------|---------|
| `.additions` | `#int` | Number of lines added |
| `.deletions` | `#int` | Number of lines deleted |
| `.unchanged` | `#int` | Number of lines unchanged |

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:DiffStats` | Compile-time type template |
| Instance | `%#:DiffStats:N` | Runtime instance (N = instance number) |

---

## Related

- [[TextDiff]] — individual diff entries that these stats summarize
- [[structs]] — other pglib struct types
- [[syntax/types/INDEX|types]] — full type system specification
