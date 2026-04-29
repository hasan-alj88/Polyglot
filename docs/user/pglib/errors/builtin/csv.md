---
audience: automation-builder
type: specification
updated: 2026-04-10
status: complete
metadata_definition: "%!.CSV"
---

# `!CSV`

No `[@]` import needed — pglib built-in.

```aljam3
{!} !CSV
   [:] :Parse
      [.] .MalformedRow#Error
         (-) .MessageTemplate << "Malformed CSV row at line {lineNumber}: {reason}"
         (-) .Info
            [:] :lineNumber#int
            [:] :reason#string
      [.] .Empty#Error
         (-) .MessageTemplate << "CSV input is empty"
      [.] .InvalidDelimiter#Error
         (-) .MessageTemplate << "Invalid CSV delimiter: {delimiter}"
         (-) .Info
            [:] :delimiter#string
   [:] :Collect
      [.] .SchemaMismatch#Error
         (-) .MessageTemplate << "Row schema does not match header: {reason}"
         (-) .Info
            [:] :reason#string
      [.] .EmptyResult#Error
         (-) .MessageTemplate << "CSV collection produced empty result"
   [:] :Merge
      [.] .HeaderConflict#Error
         (-) .MessageTemplate << "CSV merge header conflict: {reason}"
         (-) .Info
            [:] :reason#string
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Tree path | `%!.CSV` | Fixed error namespace (no instances) |

See [[metadata-tree/FULL-TREE|Full Metadata Tree]] for the authoritative tree.
