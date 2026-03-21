---
audience: user
type: specification
updated: 2026-03-20
status: draft
---

# ~ForEach — Expand Operators

Expand operators iterate over a collection, producing a mini-pipeline per item. The execution marker on the expand line controls parallelism: `[p]` for parallel, `[r]` for sequential.

No `[@]` import needed.

```
~ForEach
   .Array
      <Array
      >item
      .Enumerate
         <Array
         >index
         >item
   .Serial
      <Serial
      >key
      >item
   .Level
      <level
      >key
      >item
      [ ] The ~ suffix on the input path marks the iteration point:
      [ ]   <level << #SomeData.SubField.~
```

See also: [collections.md §Expand Operators](../concepts/collections.md)
