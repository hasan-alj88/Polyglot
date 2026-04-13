---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
---

# #Sci

<!-- @c:types -->

```polyglot
{#} #Sci
   [%] %alias << "sci"
   [#] ##String
      (#) <regex << "^-?[0-9]+(\.[0-9]+)?[eE][+-]?[0-9]+$"
```

| Alias | `.regex` Pattern | Example Values |
|-------|---------------|----------------|
| `sci` | `^-?[0-9]+(\.[0-9]+)?[eE][+-]?[0-9]+$` | `1e10`, `3.14e-2` |

| Level | Name | Purpose |
|-------|------|---------|
| Definition | `%definition.#:Sci` | Schema descriptor |
| Instance | `%#:String:sci` | Tree path under `#String` |
