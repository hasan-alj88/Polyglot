---
audience: user
type: specification
updated: 2026-03-28
status: complete
---

# Collection Types

<!-- @types -->

Polyglot provides three collection types: `#Map`, `#Array`, and `#Serial`. All are available in every `.pg` file without `[@]` import.

See [[types]] for the full type hierarchy and schema property definitions.

---

## #Map

Sparse, homogeneous key-value pairs with `#KeyString` keys. Child access uses the `<` operator (`$myMap<name`).

```polyglot
{#} #Map<KeyType<ValueType
   [ ] Type of map keys
   [#] <KeyType#KeyString << #KeyString
   [ ] Type of map values
   [#] <ValueType#* << #*
      [<] << ##Scalar
   [#] %##Alias << "map"
   [#] %##Children.Type << KeyType
   [#] << ##Flat
   [#] << ##Homogeneous
   [#] << ##Sparse
   [:] :*#ValueType
```

### Schema Properties

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Alias` | `"map"` | Shorthand `#map` |
| `%##Children.Type` | `KeyType` | Key type for flexible children |
| `##Flat` | `%##Depth.Max << 1` | One level of flexible children |
| `##Homogeneous` | `%##Children.Uniform << #True` | All values same type |
| `##Sparse` | `%##Children.Gap << #True` | Gaps allowed in keys |

---

## #Array

Contiguous, rectangular collection with typed elements and N-dimensional support. `#Array` is a `#Map` variant with `#UnsignedInt` keys.

```polyglot
{#} #Array<ValueType<Dim
   [#] <~ #Map<#UnsignedInt<ValueType
   [ ] Accepts any type as element type
   [#] <ValueType#* << #*
      [ ] Constraint: ValueType must be scalar/record (depth 0)
      [<] << ##Scalar
   [ ] Dimension parameter -- defaults to 1 if omitted
   [#] <Dim#Dimension << #Dimension
      [<] << ##Scalar
   [#] %##Alias << "array"
   [#] %##Children.Type << #UnsignedInt
   [#] << ##Contiguous
   [#] << ##Rectangular
   [ ] Depth equals dimension parameter value
   [#] %##Depth.Max << Dim
   [ ] All elements share the same type
   [:] :*#ValueType
```

### Schema Properties

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Alias` | `"array"` | Shorthand `#array` |
| `%##Children.Type` | `#UnsignedInt` | Integer indices `:0`, `:1`, `:2` ... |
| `##Contiguous` | `%##Children.Gap << #False`, `%##Children.Ordered << #True` | No gaps, insertion order preserved |
| `##Rectangular` | `%##Children.Regular << #True`, `%##Children.Uniform << #True` | Regular shape, uniform types |
| `%##Depth.Max` | `Dim` | Equals dimension parameter value |

### Inheritance from #Map

`#Array` inherits from `#Map` via `[#] <~ #Map`, overriding the key type to `#UnsignedInt` and adding contiguity and rectangularity constraints.

---

## #Serial

Schema-free collection with unlimited depth. Any keys, any types, any nesting. No compile-time validation of shape. Child access uses the `<` operator (`$data<key`).

```polyglot
{#} #Serial
   [#] %##Alias << "serial"
   [#] << ##Deep
   [#] << ##Sparse
   [#] << ##Heterogeneous
   [ ] Any key, any value type, any depth
   [:] :*#*
```

### Schema Properties

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Alias` | `"serial"` | Shorthand `#serial` |
| `##Deep` | `%##Depth.Max << -1` | Unlimited flexible nesting |
| `##Sparse` | `%##Children.Gap << #True` | Gaps allowed |
| `##Heterogeneous` | `%##Children.Uniform << #False` | Mixed value types |

---

## #Dataframe

<!-- STATUS: TBD -->

> **Note:** `#Dataframe` appears in earlier drafts but is not present in the authoritative type hierarchy (`syntax/types.md`). Its status is to be determined. The definition below is retained for reference only.

```polyglot
{#} #Dataframe<KeyType<ValueType
   [ ] Column name type (typically #string)
   [#] <KeyType#* << #*
      [<] << ##Scalar
   [ ] Cell value type
   [#] <ValueType#* << #*
      [<] << ##Scalar
   [#] %##Alias << "dataframe"
   [#] %##Children.Type << #UnsignedInt
   [#] %##Children.Gap << #False
   [#] %##Children.Ordered << #True
   [ ] Two levels: row (uint) -> column (KeyType) -> cell (ValueType)
   [#] %##Depth.Max << 2
   [ ] Each row is a flat dict of KeyType -> ValueType
   [:] :*#Map:KeyType:ValueType
```

## Related

- [[string]] -- #String foundation type
- [[scalars]] -- scalar subtypes
- [[structs]] -- #path and #Queue struct types
- [[types]] -- full type system specification
