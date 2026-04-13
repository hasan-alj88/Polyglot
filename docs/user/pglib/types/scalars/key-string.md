---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
---

# #KeyString

<!-- @c:types -->

```polyglot
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

`#KeyString` excludes characters reserved by Polyglot syntax. Enum variant names used in `%##Fields` must conform to `#KeyString`; otherwise the compiler raises PGE11004.
