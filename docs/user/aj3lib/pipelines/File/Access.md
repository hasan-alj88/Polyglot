---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:File.Access"
metadata_instance: "%-:File.Access:N"
---

# -File.Access

Check file access permissions at the given path. Returns a `#FileAccess` value describing what access is available.

## Definition

```aljam3
{N} -File.Access
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "FileAccess"
   [%] .description << "Check file access permissions"
   (-) <path#path
   (-) >access#FileAccess
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<path` | `#path` | Path to check access for |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>access` | `#FileAccess` | Access permissions available at path |

## Errors

None.

## Permissions

Requires `File.Read` capability.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:File.Access` | Compile-time pipeline template |
| Instance | `%-:File.Access:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[aj3lib/pipelines/File/INDEX|-File.* File Pipelines]]
- [[aj3lib/pipelines/File/List|-File.List]]
