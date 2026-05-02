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

### `=ForEach.Level` — Level-Specific Iteration

Unlike `=ForEach.Serial` which iterates all keys, `=ForEach.Level` iterates only the siblings at a specific level of a serialized structure. The `.=` suffix on the input path marks the level iteration point — analogous to `.*` wildcard, `.=` means "expand siblings at this level":

```aljam3
[-] =ForEach.Level
   (=) <level << #SomeData.SubField.=
   (=) >key >> $key
   (=) >item >> $item
```

## See Also

- [[concepts/collections/collect|Collect Operators]] — `*Collect` and `*Agg` collectors that gather expanded items
- [[concepts/collections/reassemble|Reassemble Operators]] — `=*` atomic expand + collect operators
- [[concepts/collections/examples|Examples]] — complete expand/transform/collect patterns
- [[concepts/pipelines/execution|Execution]] — execution body where expand operators are used


