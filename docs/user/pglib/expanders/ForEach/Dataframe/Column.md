---
audience: pg-coder
type: specification
updated: 2026-03-29
status: deprecated
---

# ~ForEach.Dataframe.Column — DEPRECATED

**Deprecated:** With the row-oriented Dataframe redesign (Issue #94), column iteration is no longer an expander operation. Use the `=#.Column` pipeline to extract an entire column as an array, then `~ForEach.Array` to iterate it.

```polyglot
[ ] Extract column, then iterate
[r] =#.Column
   [=] <data << $sales
   [=] <column << .price
   [=] >values >> $prices

[p] ~ForEach.Array
   [~] <Array << $prices
   [~] >item >> $price
   ...
```

See also: [[concepts/collections/expand#Expand Operators]], [[pglib/pipelines/#]]
