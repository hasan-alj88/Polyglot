---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
---

# #UnsignedInt

<!-- @c:types -->

```polyglot
{#} #UnsignedInt
   [%] %alias << "uint"
   [#] ##String
      (#) <regex << "^[0-9]+$"
```

| Alias | `.regex` Pattern | Example Values |
|-------|---------------|----------------|
| `uint` | `^[0-9]+$` | `0`, `1`, `42` |

| Level | Name | Purpose |
|-------|------|---------|
| Definition | `%definition.#:UnsignedInt` | Schema descriptor |
| Instance | `%#:String:uint` | Tree path under `#String` |
