---
audience: automation-builder
type: spec
updated: 2026-03-30
---

<!-- @concepts/collections/INDEX -->
<!-- @u:technical/ebnf/12-collections -->
<!-- @u:io/collection-operators -->

## Expand Operators (`=`)

Expand operators iterate over a collection, producing a **mini-pipeline** for each item. The `=` symbol represents parallelism — contrasting with `-` for a single sequential thread. The execution marker on the expand line controls which mode is used:
- `[=] =ForEach` — mini-pipelines run in **parallel** (the default intent of `=`)
- `[-] =ForEach` — mini-pipelines run **sequentially**

<!-- @c:variable-lifecycle -->
Variables declared inside a mini-pipeline are scoped to that iteration — they cannot be accessed outside. See [[variable-lifecycle#Released]].

| Operator | Iterates | IO |
|----------|----------|-----|
| `=ForEach` | Universal expander: array items, map entries, etc. | `<Data`, `>item`, optionally `>key` |

The expand operator's IO must match its signature — `=ForEach` requires `<Data` and `>item` (PGE03007). Similarly, each collect operator's IO must match its contract (PGE03008).

Every expand scope must contain at least one [[concepts/collections/collect|collector]]. A nested expand without an inner collector is a compile error — inner items cannot flow to outer collectors (PGE03009). Conversely, a `*Collect` or `*Agg` collector outside any expand scope is invalid (PGE03010).

### Depth Limiting (`<Depth`)

Because Aljam3 fundamentally treats all collections as **DataTrees**, a generic `=ForEach` must know how far down the branches to traverse. By default, tree traversal walks down to the leaves (cell-by-cell). 

You can instruct the expander to yield entire sub-trees at a specific level using the `(<) <Depth` argument:

```aljam3
[=] =ForEach
   (=) <Data << $salesData##Dataframe
   (=) <Depth << 1           [ ] Stops at the first branch, yielding row Enums
   (=) >item >> $row         [ ] $row is the sub-tree Record for that row
   (=) >key >> $index        [ ] $index is the row Enum (e.g., .0)
```

- `<Depth << 1`: Iterates the immediate children (e.g., rows of a dataframe).
- `<Depth << 2`: Iterates the next level (e.g., cells of a dataframe).
- `<Depth << -1`: Exhaustive iteration; traverses uniformly to all leaf nodes across the tree.

## See Also

- [[concepts/collections/collect|Collect Operators]] — `*Collect` and `*Agg` collectors that gather expanded items
- [[concepts/collections/reassemble|Reassemble Operators]] — `=*` atomic expand + collect operators
- [[concepts/collections/examples|Examples]] — complete expand/transform/collect patterns
- [[concepts/pipelines/execution|Execution]] — execution body where expand operators are used


