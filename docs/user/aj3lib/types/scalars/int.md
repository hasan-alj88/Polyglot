---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.##:Int"
metadata_instance: "%#:String:int"
---

# #Int

<!-- @c:types -->

```aljam3
{#} #Int
   [%] %alias << "int,integer,Integer"
   [#] ##String
      (#) <regex << "^-?[0-9]+$"
```

| Alias | `.regex` Pattern | Example Values |
|-------|---------------|----------------|
| `int` | `^-?[0-9]+$` | `42`, `-7`, `007` |

| Level | Name | Purpose |
|-------|------|---------|
| Definition | `%definition.#:Int` | Schema descriptor |
| Instance | `%#:String:int` | Tree path under `#String` |

See [[aj3lib/types/scalars/INDEX\|Scalar Subtypes]] for the full summary table.
