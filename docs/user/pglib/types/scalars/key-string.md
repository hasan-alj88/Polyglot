---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:KeyString"
metadata_instance: "%#:String:key"
---

# #KeyString

<!-- @c:types -->

```aljam3
{#} #KeyString
   [%] %alias << "key"
   [#] ##String
      (#) <regex << "^[a-zA-Z_][a-zA-Z0-9_]*$"
```

| Alias | `.regex` Pattern | Example Values |
|-------|---------------|----------------|
| `key` | `^[a-zA-Z_][a-zA-Z0-9_]*$` | `name`, `id`, `my_key` |

| Level | Name | Purpose |
|-------|------|---------|
| Definition | `%definition.#:KeyString` | Schema descriptor |
| Instance | `%#:String:key` | Tree path under `#String` |

See [[metadata-tree/FULL-TREE|Full Metadata Tree]] for the authoritative tree.

`#KeyString` excludes characters reserved by Aljam3 syntax. Enum variant names used in `%##Fields` must conform to `#KeyString`; otherwise the compiler raises PGE11004.
