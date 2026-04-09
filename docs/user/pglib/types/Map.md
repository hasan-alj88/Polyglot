---
audience: pg-coder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:Map"
metadata_instance: "%#:Map:N"
---

# #Map Collection

<!-- @types -->

Sparse key-value pairs. `#Map` is a generic type with `[#] <param` inputs. Child access uses the `<` operator (`$myMap<key`).

---

## Definition

```polyglot
{#} #Map
   [#] <#KeyType
   [#] <#ValueType <~ #
   [#] << ##Map
      [#] <#KeyType << <#KeyType
      [#] <#ValueType << <#ValueType
   [#] %##Alias << "map"
```

The `<#KeyType` parameter sets the key type. The `<#ValueType` parameter sets the value type (defaults to `#` -- any type). The `##Map` parameterized schema provides the structural constraints: flat, sparse, flexible.

---

## Schema Properties

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Depth.Max` | `1` (via ##Flat) | One level of flexible children |
| `%##Gap` | `#True` (via ##Sparse) | Gaps allowed in keys |
| `%##Flexible` | `#FlexKind.Flexible` | User adds/removes entries |
| `%##Key` | `<#KeyType` | Key type from parameter |
| `%###Type` | `<#ValueType` | Value type constraint |

---

## Usage

The `:` separator binds positionally to `[#] <param` declarations:

```polyglot
[ ] #map:string:int → KeyType=String, ValueType=Int
[-] $lookup#map:string:int <~ {}

[ ] #map:string → KeyType=String, ValueType-# (default, any type)
[-] $config#map:string <~ {}
```

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:Map` | Compile-time type template |
| Instance | `%#:Map:N` | Runtime instance (N = instance number) |

---

## Related

- [[collections]] -- collection type overview
- [[Array]] -- contiguous ordered collection (composes ##Array)
- [[schemas/Map|##Map]] -- parameterized schema
- [[syntax/types/INDEX|types]] -- full type system specification
