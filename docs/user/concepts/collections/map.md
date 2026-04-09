---
audience: pg-coder
type: spec
updated: 2026-03-30
---

<!-- @concepts/collections/INDEX -->

## #Map — Base Collection

`#Map` is the universal flat key-value collection — a generic `{#}` type with `[#] <#KeyType` and `[#] <#ValueType` parameters. Use `:` positional binding: `#map:string:int`.

See [[pglib/types/Map|#Map]] for the full definition and [[syntax/types/generic-types|Generic Types]] for the `[#] <param` syntax.

### Schema composition

`#Map` composes the `##Map` parameterized schema, which sets:

- `%##Depth.Max << 1` — flat (one level of children)
- `%##Flexible << #FlexKind.Flexible` — user-controlled `:` fields
- `%##Gap << #True` — sparse (gaps allowed in keys)

### Access

Use `<` to access flexible children by key:

```polyglot
[r] $ages#map:string:int << ...
[r] $aliceAge#int << $ages<alice
[r] $bobAge#int << $ages<bob
```

## See Also

- [[concepts/collections/expand|Expand Operators]] — `~ForEach.Map` iteration over key-value pairs
- [[concepts/collections/collect|Collect Operators]] — `*Into.Map` collection
- [[syntax/types/flexible-fields|Typed Flexible Fields]] — typed `:` fields in struct definitions
