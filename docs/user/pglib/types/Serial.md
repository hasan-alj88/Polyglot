---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.#:Serial"
metadata_instance: "%#:Serial:N"
---

# #Serial Collection

<!-- @types -->

Unconstrained collection with unlimited depth. Any keys, any types, any nesting. No compile-time validation of shape. Child access uses the `<` operator (`$data<key`). `#Serial` is a plain `{#}` definition — no macro needed.

---

## Definition

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

---

## Schema Properties

| Property | Value | Constraint Removed |
|----------|-------|--------------------|
| `%##Alias` | `"serial"` | — (shorthand `#serial`) |
| `##Deep` | `%##Depth.Max << -1` | Depth limit |
| `##Sparse` | `%##Children.Gap << #True` | No-gap requirement |
| `##Heterogeneous` | `%##Children.Uniform << #False` | Same-schema requirement |
| `%##Children.Ordered` | `#False` | Ordering requirement |
| `%##Children.Regular` | `#False` | Regularity requirement |
| `%##Children.Max` | `-1` | Max children limit |

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:Serial` | Compile-time type template |
| Instance | `%#:Serial:N` | Runtime instance (N = instance number) |

---

## Related

- [[collections]] — collection type overview
- [[Map]] — structured key-value alternative
- [[syntax/types/INDEX|types]] — full type system specification
