---
audience: automation-builder
type: specification
updated: 2026-04-10
status: complete
metadata_definition: "%!.Math"
---

# `!Math`

No `[@]` import needed — pglib built-in.

```polyglot
{!} !Math
   [.] .DivideByZero#Error
      (-) .MessageTemplate << "Division by zero: {expression}"
      (-) .Info
         [:] :expression#string
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Tree path | `%!.Math` | Fixed error namespace (no instances) |

See [[metadata-tree/FULL-TREE|Full Metadata Tree]] for the authoritative tree.
