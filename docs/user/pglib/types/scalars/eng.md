---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:Eng"
metadata_instance: "%#:String:eng"
---

# #Eng

<!-- @c:types -->

```polyglot
{#} #Eng
   [%] %alias << "eng"
   [#] ##String
      (#) <regex << "^-?[0-9]+(\.[0-9]+)?[kKMGTPE]?$"
```

| Alias | `.regex` Pattern | Example Values |
|-------|---------------|----------------|
| `eng` | `^-?[0-9]+(\.[0-9]+)?[kKMGTPE]?$` | `1.5k`, `2.47M` |

| Level | Name | Purpose |
|-------|------|---------|
| Definition | `%definition.#:Eng` | Schema descriptor |
| Instance | `%#:String:eng` | Tree path under `#String` |

See [[metadata-tree/FULL-TREE|Full Metadata Tree]] for the authoritative tree.
