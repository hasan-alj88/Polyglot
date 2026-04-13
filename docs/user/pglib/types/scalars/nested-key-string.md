---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
---

# #NestedKeyString

<!-- @c:types -->

```polyglot
{#} #NestedKeyString
   [%] %alias << "nestedkey"
   [#] ##String
      (#) <regex << "^[a-zA-Z_][a-zA-Z0-9_.]*$"
```

| Alias | `.regex` Pattern | Example Values |
|-------|---------------|----------------|
| `nestedkey` | `^[a-zA-Z_][a-zA-Z0-9_.]*$` | `File.Permission.Denied` |

| Level | Name | Purpose |
|-------|------|---------|
| Definition | `%definition.#:NestedKeyString` | Schema descriptor |
| Instance | `%#:String:nestedkey` | Tree path under `#String` |

`#NestedKeyString` allows `.` separators but still excludes whitespace, `<`, and `>`. Used as the element type for `%##Alias` -- alias values may contain `.` to reference paths in the definition tree.
