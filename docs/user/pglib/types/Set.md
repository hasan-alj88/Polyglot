---
audience: pg-coder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:Set"
metadata_instance: "%#:Set:N"
---

# #Set Collection

<!-- @types -->

Collection of unique values. `#Set` is a generic type with `[#] <param` inputs. Duplicates are rejected at runtime.

---

## Definition

```polyglot
{#} #Set
   [#] <#ValueType
   [#] << ##Set
      [#] <#ValueType << <#ValueType
   [#] %##Alias << "set"
```

The `<#ValueType` parameter sets the element type. The `##Set` parameterized schema provides the structural constraints: sparse storage with a uniqueness guarantee.

---

## Schema Properties

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Gap` | `#True` (via ##Sparse) | Gaps allowed |
| `%##Flexible` | `#FlexKind.Flexible` | User adds/removes entries |
| `%###Type` | `<#ValueType` | Element type constraint |
| `%###Unique` | `#True` | No duplicate values |

---

## Usage

```polyglot
[ ] #set:string → ValueType=String
[r] $tags#set:string <~ {}

[ ] #set:int → ValueType=Int
[r] $ids#set:int <~ {}
```

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:Set` | Compile-time type template |
| Instance | `%#:Set:N` | Runtime instance (N = instance number) |

---

## Related

- [[collections]] -- collection type overview
- [[schemas/Set|##Set]] -- parameterized schema
- [[syntax/types/INDEX|types]] -- full type system specification
