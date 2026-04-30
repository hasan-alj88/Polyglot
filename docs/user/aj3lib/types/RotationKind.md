---
audience: automation-builder
type: specification
updated: 2026-04-17
status: draft
metadata_definition: "%definition.#:RotationKind"
metadata_instance: "%#:RotationKind:N"
---

# #RotationKind Enum

<!-- @c:types -->

Runtime `##Enum` type available in every `.aj3` file. Uses `[#] ##Enum` (enum classification), `[#] ##Scalar` (depth 1), and `[#] ###ScalarEnum` (leaf content is variant selection).

Specifies how `-T.File.Rolled` detects that a file has been rotated.

---

## Definition

```aljam3
{#} #RotationKind
   [%] .description << "Rotation detection strategy"
   [%] .version << "1.0.0"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "rotationkind"
   [.] .Timestamp
   [.] .Sequence
   [.] .CopyTruncate
   [.] .Auto
```

## Variants

| Variant | Description |
|---------|-------------|
| `Timestamp` | Files named with timestamps (`app-2026-04-17.log`). Previous file identified by timestamp sort order. |
| `Sequence` | Files named with sequence numbers (`app.log.1`, `app.log.2`). Previous file identified as highest sequence before the new one. |
| `CopyTruncate` | No new file created. Active file is copied then truncated in place (logrotate `copytruncate` mode). Detected by observing file size decrease. |
| `Auto` | Infer strategy from observed naming patterns. Default. |

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:RotationKind` | Compile-time type template |
| Instance | `%#:RotationKind:0` | Runtime instance (enum — one active field) |

---

## Related

- [[enums]] — other aj3lib enum types
- [[aj3lib/pipelines/T/File.Rolled|-T.File.Rolled]] — trigger that uses this enum
- [[syntax/types/INDEX|types]] — full type system specification
