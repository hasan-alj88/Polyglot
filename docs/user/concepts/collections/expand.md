---
audience: automation-builder
type: spec
updated: 2026-03-30
---

<!-- @concepts/collections/INDEX -->

## Expand Operators (`=`)

Expand operators iterate over a collection, producing a **mini-pipeline** for each item. The `=` symbol represents parallelism — contrasting with `-` for a single sequential thread. The execution marker on the expand line controls which mode is used:
- `[=] =ForEach.Array` — mini-pipelines run in **parallel** (the default intent of `=`)
- `[-] =ForEach.Array` — mini-pipelines run **sequentially**

<!-- @c:variable-lifecycle -->
Variables declared inside a mini-pipeline are scoped to that iteration — they cannot be accessed outside. See [[variable-lifecycle#Released]].

| Operator | Iterates | IO |
|----------|----------|-----|
| `=ForEach.Array` | Each item in an array | `<Array`, `>item` |
| `=ForEach.Array.Enumerate` | Each item with index | `<Array`, `>index`, `>item` |
| `=ForEach.Map` | Each key-value pair in a map | `<Map`, `>key`, `>item` |
| `=ForEach.Serial` | All key-item pairs in a serial (all levels) | `<Serial`, `>key`, `>item` |
| `=ForEach.Level` | Siblings at a specified level only | `<level` (`.=` suffix), `>key`, `>item` |
| `=ForEach.Dataframe` | Each row (a Map of column values) | `<Dataframe`, `>row` |
| `=ForEach.Dataframe.Enumerate` | Each row with row index | `<Dataframe`, `>index`, `>row` |

The expand operator's IO must match its signature — `=ForEach.Array` requires `<Array` and `>item`, `=ForEach.Dataframe` requires `<Dataframe` and `>row` (PGE03007). Similarly, each collect operator's IO must match its contract (PGE03008).

Every expand scope must contain at least one [[concepts/collections/collect|collector]]. A nested expand without an inner collector is a compile error — inner items cannot flow to outer collectors (PGE03009). Conversely, a `*Into` or `*Agg` collector outside any expand scope is invalid (PGE03010).

### `=ForEach.Level` — Level-Specific Iteration

Unlike `=ForEach.Serial` which iterates all keys, `=ForEach.Level` iterates only the siblings at a specific level of a serialized structure. The `.=` suffix on the input path marks the level iteration point — analogous to `.*` wildcard, `.=` means "expand siblings at this level":

```polyglot
[-] =ForEach.Level
   (=) <level << #SomeData.SubField.=
   (=) >key >> $key
   (=) >item >> $item
```

## See Also

- [[concepts/collections/collect|Collect Operators]] — `*Into` and `*Agg` collectors that gather expanded items
- [[concepts/collections/examples|Examples]] — complete expand/transform/collect patterns
- [[concepts/pipelines/execution|Execution]] — execution body where expand operators are used


