---
audience: automation-builder
type: specification
updated: 2026-04-10
status: complete
metadata_definition: "%!.Validation"
---

# `!Validation`

No `[@]` import needed — pglib built-in.

```polyglot
{!} !Validation
   [.] .Schema#Error
      (-) .MessageTemplate << "Schema validation failed: {reason}"
      (-) .Info
         [:] :reason#string
   [.] .Type#Error
      (-) .MessageTemplate << "Type mismatch: expected {expected}, got {actual}"
      (-) .Info
         [:] :expected#string
         [:] :actual#string
   [.] .Regex#Error
      (-) .MessageTemplate << "Value does not match pattern {pattern}: {value}"
      (-) .Info
         [:] :pattern#string
         [:] :value#string
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Tree path | `%!.Validation` | Fixed error namespace (no instances) |

See [[metadata-tree/FULL-TREE|Full Metadata Tree]] for the authoritative tree.
