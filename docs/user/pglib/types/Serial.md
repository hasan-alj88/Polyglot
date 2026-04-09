---
audience: pg-coder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:Serial"
metadata_instance: "%#:Serial:N"
---

# #Serial Collection

<!-- @types -->

Unconstrained collection with unlimited depth. Any keys, any types, any nesting. No compile-time validation of shape. Child access uses the `<` operator (`$data<key`). `#Serial` is a plain `{#}` definition -- no generic parameters needed.

---

## Definition

```polyglot
{#} #Serial
   [#] %##Alias << "serial"
   [#] << ##Deep
   [#] << ##Sparse
   [#] %##Ordered << #False
   [#] %##Regular << #False
   [#] %##Count << .Inf
   [:] :*#*
```

---

## Schema Properties

| Property | Value | Constraint Removed |
|----------|-------|--------------------|
| `%##Alias` | `"serial"` | -- (shorthand `#serial`) |
| `##Deep` | `%##Depth.Max << .Inf` | Depth limit |
| `##Sparse` | `%##Gap << #True` | No-gap requirement |
| `%##Ordered` | `#False` | Ordering requirement |
| `%##Regular` | `#False` | Regularity requirement |
| `%##Count` | `.Inf` | Max children limit |

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:Serial` | Compile-time type template |
| Instance | `%#:Serial:N` | Runtime instance (N = instance number) |

---

## Related

- [[collections]] -- collection type overview
- [[Map]] -- structured key-value alternative
- [[syntax/types/INDEX|types]] -- full type system specification
