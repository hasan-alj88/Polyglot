---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.###:Value"
---

# ###Value Field Type

<!-- @c:types -->

`###Value` classifies a leaf node that holds typed data. The field is declared with a type annotation in the `{#}` block using `[.] .fieldName#type`.

The leaf stores an actual value of the specified type. This is the most common field type -- any field that carries data rather than acting as a variant selector uses `###Value`.

## Allows

```
#path [###Value]
├── .Unix     -> "/usr/bin"#string     ← typed data: #string annotation
└── .Windows  -> "C:\\bin"#string        each leaf holds a value
```

## Disallows

```
#path [###Value]
├── .Unix     -> "/usr/bin"#string     ✓ typed field
└── .Default                           ✗ no #type annotation — PGE05005
                                         siblings must all be same ### kind
```

## Declaration

```aljam3
{#} #path
   [.] .Unix#string
   [.] .Windows#string
```

Each `[.]` line with a `#type` annotation declares a `###Value` field. The compiler assigns `###Value` as the field type on the metadata tree.

## Example Types

<!-- @u:jm3lib/types/path -->
<!-- @u:jm3lib/types/Queue -->
<!-- @u:jm3lib/types/Job -->

| Type | Field | Annotation | Stored Data |
|------|-------|------------|-------------|
| [[path\|#path]] | `.Unix` | `#string` | Unix path string |
| [[path\|#path]] | `.Windows` | `#string` | Windows path string |
| [[Queue\|#Queue]] | `.strategy` | `#QueueStrategy` | Queue strategy enum |
| [[Job\|#Job]] | `.PID` | `#String` | Process identifier |

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.###:Value` | Field type definition template |

Field types are compile-time leaf content classifiers -- they have no runtime instances.

## Related

- [[schemas/Scalar]] -- most types using ###Value are ##Scalar
- [[syntax/types/INDEX|types]] -- full type system specification
