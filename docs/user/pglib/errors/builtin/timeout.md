---
audience: automation-builder
type: specification
updated: 2026-04-10
status: complete
metadata_definition: "%!.Timeout"
---

# `!Timeout`

No `[@]` import needed — pglib built-in.

```polyglot
{!} !Timeout
   [.] .Connection#Error
      (-) .MessageTemplate << "Connection timed out after {duration}"
      (-) .Info
         [:] :duration#string
   [.] .Read#Error
      (-) .MessageTemplate << "Read timed out after {duration}"
      (-) .Info
         [:] :duration#string
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Tree path | `%!.Timeout` | Fixed error namespace (no instances) |

See [[metadata-tree/FULL-TREE|Full Metadata Tree]] for the authoritative tree.
