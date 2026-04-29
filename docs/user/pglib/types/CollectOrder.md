---
audience: automation-builder
type: specification
updated: 2026-04-11
status: draft
metadata_definition: "%definition.#:CollectOrder"
metadata_instance: "%#:CollectOrder:N"
---

# #CollectOrder Enum

<!-- @c:types -->

Runtime `##Enum` type available in every `.aj3` file. Controls the ordering of collected fragments in append-style collectors (`*Into.Text.Append`, `*Into.CSV.Rows`).

---

## Definition

```aljam3
{#} #CollectOrder
   [%] .description << "Ordering strategy for collected fragments"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "collectorder"
   [.] .ExpandIndex
   [.] .Arrival
```

---

## Variants

| Variant | Description |
|---------|-------------|
| `.ExpandIndex` | Order by expand index (default) — preserves original item order |
| `.Arrival` | Order by arrival time — first-completed first |

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:CollectOrder` | Compile-time type template |
| Instance | `%#:CollectOrder:0` | Runtime instance (enum — one active field) |

---

## Related

- [[enums]] — other pglib enum types
- [[syntax/types/INDEX|types]] — full type system specification
