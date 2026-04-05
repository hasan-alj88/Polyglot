---
audience: pg-coder
type: spec
updated: 2026-03-30
---

<!-- @concepts/collections/INDEX -->

## #Serial — Unconstrained Tree

`#Serial` achieves its unconstrained nature by explicitly removing every structural constraint through schema properties:

| Schema / Property | Constraint Removed | Effect |
|---|---|---|
| `##Deep` (`%##Depth.Max << -1`) | Depth limit | Unlimited nesting |
| `##Sparse` (`%##Children.Gap << #True`) | No-gap requirement | Gaps allowed in child keys |
| `##Heterogeneous` (`%##Children.Uniform << #False`) | Same-schema requirement | Children can be mixed types |
| `%##Children.Ordered << #False` | Ordering requirement | No assumed key order |
| `%##Children.Regular << #False` | Regularity requirement | Branches at same depth can have different child counts |
| `%##Children.Max << -1` | Max children limit | Unlimited children per node |

By removing all structural constraints, `#Serial` accepts any compilable tree shape. This is the "raw tree" escape hatch — it has schema properties, but every one is maximally permissive.

```polyglot
{#} #Serial
   [#] %##Alias << "serial"
   [#] << ##Deep
   [#] << ##Sparse
   [#] << ##Heterogeneous
   [#] %##Children.Ordered << #False
   [#] %##Children.Regular << #False
   [#] %##Children.Max << -1
   [:] :*#*
```

Access uses chained `<` at unlimited depth:

```polyglot
[r] $value << $data<key<subkey<deeperkey
```

`#Serial` is useful for JSON-like data, configuration trees, and any structure where the schema is unknown at compile time.

## See Also

- [[syntax/types/conversions|Type Conversions]] — struct-to-serial and serial-to-struct conversion rules
- [[concepts/collections/expand|Expand Operators]] — `~ForEach.Serial` and `~ForEach.Level` iteration
- [[concepts/collections/collect|Collect Operators]] — `*Into.Serial` and `*Into.Level` collection
