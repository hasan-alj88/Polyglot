---
audience: pg-coder
type: specification
updated: 2026-03-25
status: draft
metadata_definition: "%definition.-:Path"
metadata_instance: "%-:Path:N"
---

# -Path — Cross-Platform Path Creation

Creates a `#path` value from a string argument. No `[@]` import needed.

```polyglot
-Path
   <InlineStringLiteral#string <~ ""
   >result#path
```

## Permissions

No permissions required. All operations are pure computation. See [[permissions]].

## Behavior

- Inline usage: `-Path"/tmp/MyApp"`, `-Path"C:\MyApp"`, `-Path"{.}/logs"`
- Both `/` and `\` are treated as separators, normalized per OS
- `{$var}` interpolation supported — vars must be `#path` with both OS defined
- `{.}` and `{..}` are built-in `#path` shorthands (current dir, parent dir)
- `{{` and `}}` produce literal brace characters

See also: [types.md §Path Type](../syntax/types.md)

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:Path` | Compile-time pipeline template |
| Instance | `%-:Path:N` | Runtime pipeline instance (N = instance number) |

