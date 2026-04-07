---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.#:Map"
metadata_instance: "%#:Map:N"
---

# #Map Collection

<!-- @types -->
<!-- @macros -->

Sparse key-value pairs. `#Map` has two `{M}` macro overloads dispatched by signature. Child access uses the `<` operator (`$myMap<key`).

---

## Definition

### Homogeneous Variant

Dispatched by signature `(<#, <#)` — all values share the same type.

```polyglot
{ } Homogeneous variant — dispatched by signature (<#, <#)
{M} #Map
   [#] <#KeyType
      [<] << ##EnumLeafs
   [#] <#ValueType
      [<] << ##Scalar

   [r] $UniformMapName##DataTypeString << "Map:{$KeyType%name}:{$ValueType%name}"
   {#} #{$UniformMapName}
      [#] %##Alias
         [:] << "map:{$KeyType%name}:{$ValueType%name}"
         [:] << "Map:{$KeyType%name}:{$ValueType%name}"
      [#] %##Children.Type << $KeyType
      [#] << ##Flat
      [#] << ##Homogeneous
      [#] << ##Sparse
      [:] :*#$ValueType
```

### Heterogeneous Variant

Dispatched by signature `(<#)` — values can be mixed types.

```polyglot
{ } Heterogeneous variant — dispatched by signature (<#)
{M} #Map
   [#] <#KeyType
      [<] << ##EnumLeafs

   [r] $MapName##DataTypeString << "Map:{$KeyType%name}"
   {#} #{$MapName}
      [#] %##Alias
         [:] << "map:{$KeyType%name}"
         [:] << "Map:{$KeyType%name}"
      [#] %##Children.Type << $KeyType
      [#] << ##Flat
      [#] << ##Heterogeneous
      [#] << ##Sparse
      [:] :*#*
```

---

## Schema Properties

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Children.Type` | `$KeyType` | Key type from enum parameter |
| `##Flat` | `%##Depth.Max << 1` | One level of flexible children |
| `##Homogeneous` | `%##Children.Uniform << #True` | All values same type (homogeneous variant) |
| `##Heterogeneous` | `%##Children.Uniform << #False` | Mixed value types (heterogeneous variant) |
| `##Sparse` | `%##Children.Gap << #True` | Gaps allowed in keys |

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:Map` | Compile-time type template |
| Instance | `%#:Map:N` | Runtime instance (N = instance number) |

---

## Related

- [[collections]] — collection type overview
- [[Array]] — contiguous ordered collection (inherits from #Map)
- [[scalars]] — scalar schema classifications
- [[syntax/types/INDEX|types]] — full type system specification
- [[macros]] — macro system
