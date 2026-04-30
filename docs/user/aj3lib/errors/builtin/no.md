---
audience: automation-builder
type: specification
updated: 2026-04-10
status: complete
metadata_definition: "%!.No"
---

# `!No`

No `[@]` import needed — aj3lib built-in.

```aljam3
{!} !No
   [.] .Input#Error
      (-) .MessageTemplate << "Missing required input: {name}"
      (-) .Info
         [:] :name#string
   [.] .Output#Error
      (-) .MessageTemplate << "Missing required output: {name}"
      (-) .Info
         [:] :name#string
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Tree path | `%!.No` | Fixed error namespace (no instances) |

See [[metadata-tree/FULL-TREE|Full Metadata Tree]] for the authoritative tree.
