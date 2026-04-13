---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
---

# #Dimension

<!-- @c:types -->

```polyglot
{#} #Dimension
   [%] %alias << "dim"
   [#] ##String
      (#) <regex << "^[0-9]+D$"
```

| Alias | `.regex` Pattern | Example Values |
|-------|---------------|----------------|
| `dim` | `^[0-9]+D$` | `0D`, `1D`, `2D`, `3D` |

| Level | Name | Purpose |
|-------|------|---------|
| Definition | `%definition.#:Dimension` | Schema descriptor |
| Instance | `%#:String:dim` | Tree path under `#String` |
