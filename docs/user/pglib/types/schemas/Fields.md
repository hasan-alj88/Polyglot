---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.%##:Fields"
---

# %##Fields Property

<!-- @c:types -->

`%##Fields` is a branch-level schema property that describes how a type's children are organized. It replaces the former `##Fields` parameterized schema, `%##Key`, `%##Range`, and `%##Flexible` / `#FlexKind`.

## Values

`%##Fields` accepts either a `#FieldsDescriptor` variant or a `##Enum`-satisfying type reference:

| Value | Effect | Used By |
|-------|--------|---------|
| `#Range` | Integer-indexed children (`:0`, `:1`, `:2`, ...) | `##Array`, `##Dataframe` (L1) |
| `#SomeEnum` ref | Stamps one `:` child per enum variant | `##Record` |

### #Range (Integer-Indexed)

When `%##Fields << #Range`, children are addressed by integer index. The compiler generates `:0`, `:1`, `:2`, ... flexible fields up to `%##Count`:

```polyglot
{#} ##Array
   (#) <#ValueType
   (#) <Dim##Dimension <~ "1D"
   [#] %##Depth.Max << <Dim
   [#] %##Fields << #Range
   [#] %##Ordered << #True
   [#] %##Gap << #False
   [#] %##Propagate << #True
   [#] %###Type << <#ValueType
   [#] %###Kind << #FieldKind.Value
```

### Enum Reference (Variant-Stamped)

When `%##Fields` receives an enum type reference, the compiler reads the enum's variants and generates one `:` child per variant:

```polyglot
{#} ##Record
   (#) <#Fields << ##Enum
   (#) <#ValueType <~ #
   [#] ##Flat
   [#] %##Fields << <#Fields
   [#] %##Active << #ActiveKind.All
   [#] %###Type << <#ValueType
   [#] %###Kind << #FieldKind.Value
```

For example, `%##Fields << #DayOfWeek` stamps `.Monday`, `.Tuesday`, ... `.Sunday` as typed value fields.

## #FieldsDescriptor

The enum governing `%##Fields`:

```polyglot
{#} #FieldsDescriptor
   [#] ##Scalar
   [#] %##Active << #ActiveKind.One
   [.] .Range
      [#] %##Alias << "range"
   [.] .Enum
      [#] %##Alias << "enum"
```

`#Range` (alias for `#FieldsDescriptor.Range`) and `#Enum` (alias for `#FieldsDescriptor.Enum`) are the two variants.

## Retired

`%##Fields` replaces several former properties and types:

| Former | Replaced By |
|--------|-------------|
| `%##Key` | `%##Fields` with enum ref |
| `%##Range` | `%##Count` for bounds; `%##Fields << #Range` for integer indexing |
| `%##Flexible` / `#FlexKind` | `%##Fields` (range vs enum) |
| `##Fields` (parameterized schema) | `%##Fields` property |

## Related

- [[schemas/INDEX|## Schema Types]] -- all schema definitions
- [[syntax/types/schema-properties|Schema Properties]] -- full property reference
- [[schemas/Record|##Record]] -- primary consumer of enum-ref `%##Fields`
- [[schemas/Array|##Array]] -- primary consumer of `#Range` `%##Fields`
