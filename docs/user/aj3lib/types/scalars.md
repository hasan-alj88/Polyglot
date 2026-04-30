---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
---

# Scalar Subtypes

<!-- @c:types -->

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

| Type | Alias | `.regex` Pattern | Example Values | Doc |
|------|-------|---------------|----------------|-----|
| `#Int` | `int` | `^-?[0-9]+$` | `42`, `-7`, `007` | [[aj3lib/types/scalars/int\|#Int]] |
| `#UnsignedInt` | `uint` | `^[0-9]+$` | `0`, `1`, `42` | [[aj3lib/types/scalars/unsigned-int\|#UnsignedInt]] |
| `#Float` | `float` | `^-?[0-9]+\.[0-9]+$` | `3.14`, `-0.5`, `007.00` | [[aj3lib/types/scalars/float\|#Float]] |
| `#Sci` | `sci` | `^-?[0-9]+(\.[0-9]+)?[eE][+-]?[0-9]+$` | `1e10`, `3.14e-2` | [[aj3lib/types/scalars/sci\|#Sci]] |
| `#Eng` | `eng` | `^-?[0-9]+(\.[0-9]+)?[kKMGTPE]?$` | `1.5k`, `2.47M` | [[aj3lib/types/scalars/eng\|#Eng]] |
| `#Dimension` | `dim` | `^[0-9]+D$` | `0D`, `1D`, `2D`, `3D` | [[aj3lib/types/scalars/dimension\|#Dimension]] |
| `#KeyString` | `key` | `^[a-zA-Z_][a-zA-Z0-9_]*$` | `name`, `id`, `my_key` | [[aj3lib/types/scalars/key-string\|#KeyString]] |
| `#NestedKeyString` | `nestedkey` | `^[a-zA-Z_][a-zA-Z0-9_.]*$` | `File.Permission.Denied` | [[aj3lib/types/scalars/nested-key-string\|#NestedKeyString]] |
| `#CommaSeparatedList` | `csvlist` | `^[a-zA-Z_][a-zA-Z0-9_]*(,[a-zA-Z_][a-zA-Z0-9_]*)*$` | `product,price,quantity` | [[aj3lib/types/scalars/comma-separated-list\|#CommaSeparatedList]] |
| `#DataTypeString` | `dtstring` | `^[A-Z][a-zA-Z0-9]*(:[A-Z][a-zA-Z0-9]*)*$` | `Array1D:Int`, `Map:String` | [[aj3lib/types/scalars/data-type-string\|#DataTypeString]] |
| `#Email` | `email` | `^[a-zA-Z0-9+_.-]+@[a-zA-Z0-9.-]+$` | `user@example.com`, `admin+tag@sub.domain.org` | [[aj3lib/types/scalars/email\|#Email]] |

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
| `#Email` | `%definition.#:Email` | `%#:String:email` |

## Related

- [[string]] -- #String foundation type
- [[boolean]] -- #Boolean (independent, not a #String subtype)
- [[schemas/String|##String]] -- parameterized schema used by all scalars
- [[concepts/collections/INDEX|collections]] -- collection types using scalar constraints
- [[syntax/types/INDEX|types]] -- full type system specification
