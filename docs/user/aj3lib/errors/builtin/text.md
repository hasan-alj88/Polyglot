---
audience: automation-builder
type: specification
updated: 2026-04-10
status: complete
metadata_definition: "%!.Text"
---

# `!Text`

No `[@]` import needed — aj3lib built-in.

```aljam3
{!} !Text
   [:] :Diff
      [.] .EmptyInput#Error
         (-) .MessageTemplate << "Diff input is empty: {side}"
         (-) .Info
            [:] :side#string
   [:] :Lines
      [.] .Empty#Error
         (-) .MessageTemplate << "Text has no lines"
   [:] :Append
      [.] .EmptyResult#Error
         (-) .MessageTemplate << "Append produced empty result"
   [:] :Merge
      [.] .InvalidLineNumber#Error
         (-) .MessageTemplate << "Invalid line number: {lineNumber}"
         (-) .Info
            [:] :lineNumber#int
      [.] .EmptyBase#Error
         (-) .MessageTemplate << "Merge base text is empty"
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Tree path | `%!.Text` | Fixed error namespace (no instances) |

See [[metadata-tree/FULL-TREE|Full Metadata Tree]] for the authoritative tree.
