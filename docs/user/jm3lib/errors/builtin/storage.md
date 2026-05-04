---
audience: automation-builder
type: specification
updated: 2026-04-10
status: complete
metadata_definition: "%!.Storage"
---

# `!Storage`

No `[@]` import needed — jm3lib built-in.

```aljam3
{!} !Storage
   [.] .Space#Error
      (-) .MessageTemplate << "Insufficient storage space: {required} needed"
      (-) .Info
         [:] :required#string
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Tree path | `%!.Storage` | Fixed error namespace (no instances) |

See [[metadata-tree/FULL-TREE|Full Metadata Tree]] for the authoritative tree.
