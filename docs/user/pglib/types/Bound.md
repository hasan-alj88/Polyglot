---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:Bound"
metadata_instance: "%#:Bound:N"
---

# #Bound Type

<!-- @c:types -->

`#Bound` represents a numeric value that may be infinite. It composes `##Int` and `##Inf` -- exactly one branch is active at a time (`%##Active << #ActiveKind.One`). Use `#Bound` wherever a count or limit may be unbounded, replacing the old `-1` magic number convention.

---

## Definition

```aljam3
{#} #Bound
   [%] .description << "Numeric value or infinity"
   [%] .version << "1.0.0"
   [#] ##Int
   [#] ##Inf
   [#] %##Active << #ActiveKind.One
   [#] %##Alias << "bound"
```

When the integer branch is active, `#Bound` holds a concrete numeric value. When the `.Inf` branch is active, it represents infinity. The compiler validates that exactly one branch is present at any time.

### Usage

`#Bound` is the type of several `%##` schema properties:

| Property | Meaning |
|----------|---------|
| `%##Count` | Max children (`.Inf` = unlimited) |
| `%##Depth.Max` | Max tree depth (`.Inf` = unlimited) |

```aljam3
[ ] Concrete limit
[#] %##Count << 10

[ ] Unlimited
[#] %##Count << .Inf

[ ] Deep type with explicit depth
[#] %##Depth.Max << 3

[ ] Unlimited depth
[#] %##Depth.Max << .Inf
```

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:Bound` | Compile-time type template |
| Instance | `%#:Bound:0` | Runtime instance (one active branch) |

---

## Related

- [[syntax/types/schema-properties|Schema Properties]] -- properties that use `#Bound`
- [[syntax/types/INDEX|types]] -- full type system specification
