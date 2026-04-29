---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:CommaSeparatedList"
metadata_instance: "%#:String:csvlist"
---

# #CommaSeparatedList

<!-- @c:types -->

```aljam3
{#} #CommaSeparatedList
   [%] %alias << "csvlist"
   [#] ##String
      (#) <regex << "^[a-zA-Z_][a-zA-Z0-9_]*(,[a-zA-Z_][a-zA-Z0-9_]*)*$"
```

| Alias | `.regex` Pattern | Example Values |
|-------|---------------|----------------|
| `csvlist` | `^[a-zA-Z_][a-zA-Z0-9_]*(,[a-zA-Z_][a-zA-Z0-9_]*)*$` | `product,price,quantity` |

| Level | Name | Purpose |
|-------|------|---------|
| Definition | `%definition.#:CommaSeparatedList` | Schema descriptor |
| Instance | `%#:String:csvlist` | Tree path under `#String` |

See [[metadata-tree/FULL-TREE|Full Metadata Tree]] for the authoritative tree.
