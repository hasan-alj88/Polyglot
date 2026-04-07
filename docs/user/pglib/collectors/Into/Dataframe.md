---
audience: pg-coder
type: specification
updated: 2026-04-07
status: stable
---

# *Into.Dataframe

Gathers row outputs from mini-pipelines back into a single `#Dataframe`, accessible one level up from the expand scope. Each `<row` must match the compiler-synthesized row struct (all fields typed as `CellType`).

Row indices are auto-assigned (0, 1, 2...) like `*Into.Array`. The resulting dataframe is `##Rectangular` -- all columns have equal row count.

## Syntax

```polyglot
[r] *Into.Dataframe
   [*] <row << $filteredRow
   [*] >Dataframe >> $result
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<row` | row struct | Row matching compiler-synthesized struct |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>Dataframe` | `#Dataframe` | Collected dataframe of all rows |

## Errors

None.

## Permissions

None.

## Related

- [[pglib/collectors/Into/INDEX|*Into Data Collectors]]
- [[concepts/collections/collect|Collect Operators]]
