---
audience: automation-builder
type: specification
updated: 2026-04-10
status: complete
metadata_definition: "%!.Alias"
---

# `!Alias`

No `[@]` import needed — pglib built-in.

```aljam3
{!} !Alias
   [.] .Clash#Error
      (-) .MessageTemplate << "Alias {alias} clashes with existing name in {namespace}"
      (-) .Info
         [:] :alias#string
         [:] :namespace#string
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Tree path | `%!.Alias` | Fixed error namespace (no instances) |

See [[metadata-tree/FULL-TREE|Full Metadata Tree]] for the authoritative tree.

See [[pglib/errors/alias-clash|!Alias.Clash]] for the compile error behavior and `[<] !Alias.Clash` fallback chain pattern.
