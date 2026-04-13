---
audience: automation-builder
type: spec
updated: 2026-04-09
---

<!-- @concepts/collections/INDEX -->

## #Serial -- Unconstrained Tree

`#Serial` achieves its unconstrained nature by explicitly removing every structural constraint through direct schema properties:

| Property | Value | Constraint Removed |
|---|---|---|
| `%##Depth.Max << #Inf` | `#Inf` | Depth limit -- unlimited nesting |
| `%##Gap << #True` | `#True` | No-gap requirement -- gaps allowed in child keys |
| `%##Ordered << #False` | `#False` | Ordering requirement -- no assumed key order |
| `%##Regular << #False` | `#False` | Regularity requirement -- branches at same depth can have different child counts |
| `%##Count << #Inf` | `#Inf` | Max children limit -- unlimited children per node |

By removing all structural constraints, `#Serial` accepts any compilable tree shape. This is the "raw tree" escape hatch -- it has schema properties, but every one is maximally permissive.

```polyglot
{#} #Serial
   [#] %##Alias << "serial"
   [#] %##Depth.Max << #Inf
   [#] %##Gap << #True
   [#] %##Ordered << #False
   [#] %##Regular << #False
   [#] %##Count << #Inf
   [:] :*#*
```

Access uses chained `<` at unlimited depth:

```polyglot
[-] $value << $data<key<subkey<deeperkey
```

`#Serial` is useful for JSON-like data, configuration trees, and any structure where the schema is unknown at compile time.

## See Also

- [[syntax/types/conversions|Type Conversions]] -- struct-to-serial and serial-to-struct conversion rules
- [[concepts/collections/expand|Expand Operators]] -- `=ForEach.Serial` and `=ForEach.Level` iteration
- [[concepts/collections/collect|Collect Operators]] -- `*Into.Serial` and `*Into.Level` collection
