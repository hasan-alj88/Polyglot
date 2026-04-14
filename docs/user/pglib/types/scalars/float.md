---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:Float"
metadata_instance: "%#:String:float"
---

# #Float

<!-- @c:types -->

```polyglot
{#} #Float
   [%] %alias << "float"
   [#] ##String
      (#) <regex << "^-?[0-9]+\.[0-9]+$"
```

| Alias | `.regex` Pattern | Example Values |
|-------|---------------|----------------|
| `float` | `^-?[0-9]+\.[0-9]+$` | `3.14`, `-0.5`, `007.00` |

| Level | Name | Purpose |
|-------|------|---------|
| Definition | `%definition.#:Float` | Schema descriptor |
| Instance | `%#:String:float` | Tree path under `#String` |

See [[metadata-tree/FULL-TREE|Full Metadata Tree]] for the authoritative tree.
