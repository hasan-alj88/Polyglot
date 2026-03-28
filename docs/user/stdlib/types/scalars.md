---
audience: user
type: specification
updated: 2026-03-28
status: complete
---

# Scalar Subtypes

<!-- @types -->

All scalar subtypes inherit `#String`'s schema via `[#] <~ #String` and override `.re` with a specific regex. See [[string]] for the foundation type.

## Summary Table

| Type | Alias | `.re` Pattern | Example Values |
|------|-------|---------------|----------------|
| `#Int` | `int` | `^-?[0-9]+$` | `42`, `-7`, `007` |
| `#UnsignedInt` | `uint` | `^[0-9]+$` | `0`, `1`, `42` |
| `#Float` | `float` | `^-?[0-9]+\.[0-9]+$` | `3.14`, `-0.5`, `007.00` |
| `#Sci` | `sci` | `^-?[0-9]+(\.[0-9]+)?[eE][+-]?[0-9]+$` | `1e10`, `3.14e-2` |
| `#Eng` | `eng` | `^-?[1-9]\.[0-9]{0,2}[eE][+-]?(0\|[369]\|[1-9][0-9]*[0369])$` | `1.5e3`, `2.47e-6` |
| `#Dimension` | `dim` | `^[0-9]+$` | `0`, `1`, `2`, `3` |
| `#KeyString` | `key` | `^[^\s.<>:]+$` | `name`, `id`, `my-key` |
| `#NestedKeyString` | `nestedkey` | `^[^\s<>]+$` | `File.Permission.Denied` |

---

## #Int

```polyglot
{#} #Int
   [ ] Inherits #String schema (.string, .re)
   [#] <~ #String
   [#] %##Alias << "int"
   [ ] Matches: 42, -7, 0, 007
   [.] .re#RawString << "^-?[0-9]+$"
```

## #UnsignedInt

```polyglot
{#} #UnsignedInt
   [ ] Non-negative integers -- array keys, dimensions
   [#] <~ #String
   [#] %##Alias << "uint"
   [ ] Matches: 0, 1, 42, 007
   [.] .re#RawString << "^[0-9]+$"
```

## #Float

```polyglot
{#} #Float
   [#] <~ #String
   [#] %##Alias << "float"
   [ ] Matches: 3.14, -0.5, 007.00
   [.] .re#RawString << "^-?[0-9]+\.[0-9]+$"
```

## #Sci

```polyglot
{#} #Sci
   [#] <~ #String
   [#] %##Alias << "sci"
   [ ] Scientific notation with optional decimal
   [ ] Matches: 1e10, 3.14e-2, -5E+3
   [.] .re#RawString << "^-?[0-9]+(\.[0-9]+)?[eE][+-]?[0-9]+$"
```

## #Eng

```polyglot
{#} #Eng
   [#] <~ #String
   [#] %##Alias << "eng"
   [ ] Engineering notation: exponents are multiples of 3
   [ ] Matches: 1.5e3, 2.47e-6, 9.99e12
   [.] .re#RawString << "^-?[1-9]\.[0-9]{0,2}[eE][+-]?(0|[369]|[1-9][0-9]*[0369])$"
```

## #Dimension

```polyglot
{#} #Dimension
   [ ] Array dimension parameters -- 0D for scalars, 1D, 2D, etc.
   [ ] The "D" suffix in :2D usage is syntax sugar -- :2D means dimension value = 2
   [#] <~ #String
   [#] %##Alias << "dim"
   [ ] Matches: 0, 1, 2, 3, 10 (allows 0D for scalars)
   [.] .re#RawString << "^[0-9]+$"
```

## #KeyString

```polyglot
{#} #KeyString
   [#] <~ #String
   [#] %##Alias << "key"
   [ ] Excludes whitespace, dot, colon, angle brackets
   [.] .re#RawString << "^[^\s.<>:]+$"
```

`#KeyString` excludes characters reserved by Polyglot syntax -- whitespace, `.`, `:`, `<`, `>`. Any type used as `%##Children.Type` must inherit from `#KeyString`; otherwise the compiler raises PGE-924.

## #NestedKeyString

```polyglot
{#} #NestedKeyString
   [#] <~ #String
   [#] %##Alias << "nestedkey"
   [ ] Allows dot and colon; excludes whitespace and angle brackets
   [.] .re#RawString << "^[^\s<>]+$"
```

`#NestedKeyString` allows `.` and `:` separators but still excludes whitespace, `<`, and `>`. Used as the element type for `%##Alias` -- alias values may contain `.` and `:` to reference paths in the definition tree.

## Related

- [[string]] -- #String foundation type
- [[boolean]] -- #Boolean (independent, not a #String subtype)
- [[types]] -- full type system specification
