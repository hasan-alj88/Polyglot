---
audience: automation-builder
type: specification
updated: 2026-04-10
status: complete
metadata_definition: "%!.Field"
---

# `!Field`

No `[@]` import needed — pglib built-in.

```polyglot
{!} !Field
   [.] .NotFound#Error
      (-) .MessageTemplate << "Field not found: {field}"
      (-) .Info
         [:] :field#string
   [.] .PathError#Error
      (-) .MessageTemplate << "Invalid field path: {path}"
      (-) .Info
         [:] :path#string
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Tree path | `%!.Field` | Fixed error namespace (no instances) |

See [[metadata-tree/FULL-TREE|Full Metadata Tree]] for the authoritative tree.
