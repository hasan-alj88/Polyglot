---
audience: pg-coder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.##:Int"
metadata_instance: "%#:String:int"
---

# Scalar Subtypes

<!-- @types -->

Scalars are `#` types that compose the `##String` parameterized schema — regex constraints on `#String`. A `##` schema describes a property of a struct; it lives at `%##` on the metadata tree. `{#}` can define any tier of the `#`/`##`/`###` prefix system.

> **Naming convention:** Each scalar subtype has three names at different levels:
>
> | Level | Example | Purpose |
> |-------|---------|---------|
> | `##Int` | Schema descriptor | Compiler-enforced metadata on `#String` -- lives at `%##` |
> | `#Int` / `#int` | User alias | What you write in type annotations (lowercase is shorthand) |
> | `:int` | Tree path | Flexible-level key under `%#:String` on the metadata tree |
>
> `##` describes `#` the way `###` describes leaf fields -- they are syntax sugar for setting metadata that the compiler enforces on data structs. A `#` struct can compose multiple `##` schemas as long as they don't contradict. See [[metadata-tree/string-subtypes#Alias Resolution]] for the full resolution table.

All scalar subtypes compose `##String` with a specific `<regex` parameter:

## Summary Table

| Type | Alias | `.regex` Pattern | Example Values |
|------|-------|---------------|----------------|
| `#Int` | `int` | `^-?[0-9]+$` | `42`, `-7`, `007` |
| `#UnsignedInt` | `uint` | `^[0-9]+$` | `0`, `1`, `42` |
| `#Float` | `float` | `^-?[0-9]+\.[0-9]+$` | `3.14`, `-0.5`, `007.00` |
| `#Sci` | `sci` | `^-?[0-9]+(\.[0-9]+)?[eE][+-]?[0-9]+$` | `1e10`, `3.14e-2` |
| `#Eng` | `eng` | `^-?[0-9]+(\.[0-9]+)?[kKMGTPE]?$` | `1.5k`, `2.47M` |
| `#Dimension` | `dim` | `^[0-9]+D$` | `0D`, `1D`, `2D`, `3D` |
| `#KeyString` | `key` | `^[a-zA-Z_][a-zA-Z0-9_]*$` | `name`, `id`, `my_key` |
| `#NestedKeyString` | `nestedkey` | `^[a-zA-Z_][a-zA-Z0-9_.]*$` | `File.Permission.Denied` |
| `#CommaSeparatedList` | `csvlist` | `^[a-zA-Z_][a-zA-Z0-9_]*(,[a-zA-Z_][a-zA-Z0-9_]*)*$` | `product,price,quantity` |
| `#DataTypeString` | `dtstring` | `^[A-Z][a-zA-Z0-9]*(:[A-Z][a-zA-Z0-9]*)*$` | `Array1D:Int`, `Map:String` |

---

## #Int

```polyglot
{#} #Int
   [%] %alias << "int,integer,Integer"
   [#] ##String
      (#) <regex << "^-?[0-9]+$"
```

## #UnsignedInt

```polyglot
{#} #UnsignedInt
   [%] %alias << "uint"
   [#] ##String
      (#) <regex << "^[0-9]+$"
```

## #Float

```polyglot
{#} #Float
   [%] %alias << "float"
   [#] ##String
      (#) <regex << "^-?[0-9]+\.[0-9]+$"
```

## #Sci

```polyglot
{#} #Sci
   [%] %alias << "sci"
   [#] ##String
      (#) <regex << "^-?[0-9]+(\.[0-9]+)?[eE][+-]?[0-9]+$"
```

## #Eng

```polyglot
{#} #Eng
   [%] %alias << "eng"
   [#] ##String
      (#) <regex << "^-?[0-9]+(\.[0-9]+)?[kKMGTPE]?$"
```

## #Dimension

```polyglot
{#} #Dimension
   [%] %alias << "dim"
   [#] ##String
      (#) <regex << "^[0-9]+D$"
```

## #KeyString

```polyglot
{#} #KeyString
   [%] %alias << "key"
   [#] ##String
      (#) <regex << "^[a-zA-Z_][a-zA-Z0-9_]*$"
```

`#KeyString` excludes characters reserved by Polyglot syntax. Enum variant names used in `%##Fields` must conform to `#KeyString`; otherwise the compiler raises PGE11004.

## #NestedKeyString

```polyglot
{#} #NestedKeyString
   [%] %alias << "nestedkey"
   [#] ##String
      (#) <regex << "^[a-zA-Z_][a-zA-Z0-9_.]*$"
```

`#NestedKeyString` allows `.` separators but still excludes whitespace, `<`, and `>`. Used as the element type for `%##Alias` -- alias values may contain `.` to reference paths in the definition tree.

## #CommaSeparatedList

```polyglot
{#} #CommaSeparatedList
   [%] %alias << "csvlist"
   [#] ##String
      (#) <regex << "^[a-zA-Z_][a-zA-Z0-9_]*(,[a-zA-Z_][a-zA-Z0-9_]*)*$"
```

## #DataTypeString

```polyglot
{#} #DataTypeString
   [%] %alias << "dtstring"
   [#] ##String
      (#) <regex << "^[A-Z][a-zA-Z0-9]*(:[A-Z][a-zA-Z0-9]*)*$"
```

`#DataTypeString` validates `{x}` definition name format -- uppercase-initial segments separated by `:`.

## Metadata

Each scalar subtype has a definition and appears as an instance under #String:

| Subtype | Definition Path | Instance Path |
|---------|----------------|---------------|
| `#Int` | `%definition.#:Int` | `%#:String:int` |
| `#UnsignedInt` | `%definition.#:UnsignedInt` | `%#:String:uint` |
| `#Float` | `%definition.#:Float` | `%#:String:float` |
| `#Sci` | `%definition.#:Sci` | `%#:String:sci` |
| `#Eng` | `%definition.#:Eng` | `%#:String:eng` |
| `#Dimension` | `%definition.#:Dimension` | `%#:String:dim` |
| `#KeyString` | `%definition.#:KeyString` | `%#:String:key` |
| `#NestedKeyString` | `%definition.#:NestedKeyString` | `%#:String:nestedkey` |
| `#CommaSeparatedList` | `%definition.#:CommaSeparatedList` | `%#:String:csvlist` |
| `#DataTypeString` | `%definition.#:DataTypeString` | `%#:String:dtstring` |

## Related

- [[string]] -- #String foundation type
- [[boolean]] -- #Boolean (independent, not a #String subtype)
- [[schemas/String|##String]] -- parameterized schema used by all scalars
- [[concepts/collections/INDEX|collections]] -- collection types using scalar constraints
- [[syntax/types/INDEX|types]] -- full type system specification
