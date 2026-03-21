---
audience: user
type: specification
updated: 2026-03-20
status: draft
---

# =Path — Cross-Platform Path Creation

Creates a `;path` value from a string argument. No `[@]` import needed.

```
=Path
   <InlineStringLiteral;string <~ ""
   >result;path
```

## Behavior

- Inline usage: `=Path"/tmp/MyApp"`, `=Path"C:\MyApp"`, `=Path"{.}/logs"`
- Both `/` and `\` are treated as separators, normalized per OS
- `{$var}` interpolation supported — vars must be `;path` with both OS defined
- `{.}` and `{..}` are built-in `;path` shorthands (current dir, parent dir)
- `{{` and `}}` produce literal brace characters

See also: [types.md §Path Type](../syntax/types.md)
