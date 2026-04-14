---
audience: automation-builder
type: specification
updated: 2026-04-10
status: complete
metadata_definition: "%!.File"
---

# `!File`

No `[@]` import needed — pglib built-in.

```polyglot
{!} !File
   [.] .NotFound#Error
      (-) .MessageTemplate << "File not found: {path}"
      (-) .Info
         [:] :path#path
   [.] .ReadError#Error
      (-) .MessageTemplate << "Cannot read file: {path}"
      (-) .Info
         [:] :path#path
   [.] .WriteError#Error
      (-) .MessageTemplate << "Cannot write file: {path}"
      (-) .Info
         [:] :path#path
   [.] .ParseError#Error
      (-) .MessageTemplate << "Parse error in {path}: {reason}"
      (-) .Info
         [:] :path#path
         [:] :reason#string
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Tree path | `%!.File` | Fixed error namespace (no instances) |

See [[metadata-tree/FULL-TREE|Full Metadata Tree]] for the authoritative tree.
