---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
---

# #DataTypeString

<!-- @c:types -->

```polyglot
{#} #DataTypeString
   [%] %alias << "dtstring"
   [#] ##String
      (#) <regex << "^[A-Z][a-zA-Z0-9]*(:[A-Z][a-zA-Z0-9]*)*$"
```

| Alias | `.regex` Pattern | Example Values |
|-------|---------------|----------------|
| `dtstring` | `^[A-Z][a-zA-Z0-9]*(:[A-Z][a-zA-Z0-9]*)*$` | `Array1D:Int`, `Map:String` |

| Level | Name | Purpose |
|-------|------|---------|
| Definition | `%definition.#:DataTypeString` | Schema descriptor |
| Instance | `%#:String:dtstring` | Tree path under `#String` |

`#DataTypeString` validates `{x}` definition name format -- uppercase-initial segments separated by `:`.
