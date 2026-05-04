---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:DataTypeString"
metadata_instance: "%#:String:dtstring"
---

# #DataTypeString

<!-- @c:types -->

```aljam3
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

See [[metadata-tree/FULL-TREE|Full Metadata Tree]] for the authoritative tree.

`#DataTypeString` validates `{x}` definition name format -- uppercase-initial segments separated by `:`.
