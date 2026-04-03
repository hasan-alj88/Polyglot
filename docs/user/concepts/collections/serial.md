---
audience: pg-coder
type: spec
updated: 2026-03-30
---

<!-- @concepts/collections/INDEX -->

## #Serial — Schema-Free Tree

`#Serial` has no `##` schema constraints at all. It accepts any compilable tree structure — depth, ordering, gaps, and uniformity are all unconstrained. This is the "raw tree" escape hatch.

```polyglot
{#} #Serial
   [#] %##Alias << "serial"
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
