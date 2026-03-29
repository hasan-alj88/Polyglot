---
audience: user
type: specification
updated: 2026-03-29
status: stable
---

# *Into.Dataframe — Collect into Dataframe

Gathers row outputs from mini-pipelines back into a single `#Dataframe`, accessible one level up from the expand scope. Each `<row` must match the compiler-synthesized row struct (all fields typed as `CellType`).

Collector invocation uses `[r]` (sequential) or `[p]` (parallel) execution markers. Collector IO lines use `[*]`.

No `[@]` import needed.

Row indices are auto-assigned (0, 1, 2...) like `*Into.Array`. The resulting dataframe is `##Rectangular` — all columns have equal row count.

## IO Signature

| Input | Output |
|-------|--------|
| `<row` | `>Dataframe` |

## Usage

```polyglot
[r] *Into.Dataframe
   [*] <row << $filteredRow
   [*] >Dataframe >> $result
```

See also: [[collections#Collect Operators]]
