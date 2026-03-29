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

Column-oriented table. Columns are `###Enum` fixed fields from a `ColumnEnum` type parameter; each column holds an `#Array<CellType>` of rows. All columns share the same cell type (homogeneous). Column access uses `.` (fixed field), row access uses `<` (array index).

```polyglot
{#} #Dataframe<ColumnEnum<CellType
   [ ] Column-oriented table: enum fields are columns, each holds #Array of rows
   [#] <ColumnEnum#* << #*
      [ ] Columns must be an enum type
      [<] << ##Scalar
      [<] << ##EnumLeafs
   [#] <CellType#* << #*
      [ ] Cell values must be scalar
      [<] << ##Scalar
   [#] %##Alias << "dataframe"
   [#] << ##EnumLeafs
   [#] << ##Rectangular
   [ ] Depth = 1 (array rows); fixed column fields don't count
   [#] %##Depth.Max << 1
   [ ] Columns from ColumnEnum, each holds #Array<CellType>
   [.] .*ColumnEnum#Array<CellType
```

### Schema Properties

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Alias` | `"dataframe"` | Shorthand `#dataframe` |
| `##EnumLeafs` | `%##Leafs.Kind << #FieldKind.Enum` | Columns are compile-time `###Enum` fields |
| `##Rectangular` | `%##Children.Regular << #True`, `%##Children.Uniform << #True` | All columns same row count |
| `%##Depth.Max` | `1` | One flex level (rows); fixed column fields don't count |

### Field Expansion

`[.] .*ColumnEnum#Array<CellType` is a **field expansion** — the compiler reads `ColumnEnum`'s `###Enum` fields and stamps out one `[.]` per field, each typed as `#Array<CellType>`. See [[types#Field Expansion]].

### Usage

```polyglot
{#} #SalesColumns
   [#] << ##Scalar
   [#] << ###Enum
   [.] .product
   [.] .price
   [.] .quantity

[r] $sales#dataframe:SalesColumns:string <~ {}

[ ] Access: .column<row
[r] $name#string << $sales.product<0
[r] $price#string << $sales.price<2
[r] $prices#array:string << $sales.price    [ ] entire column
```

## Related

- [[string]] -- #String foundation type
- [[scalars]] -- scalar subtypes
- [[structs]] -- #path and #Queue struct types
- [[enums]] -- #FieldKind enum (used by `%##Leafs.Kind`)
- [[types]] -- full type system specification
