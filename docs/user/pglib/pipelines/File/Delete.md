---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.=:File.Delete"
metadata_instance: "%=:File.Delete:N"
---

# =File.Delete

Delete a file at the given path.

## Definition

```polyglot
{N} =File.Delete
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "FileDelete"
   [%] .description << "Delete file"
   [=] <path#path
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<path` | `#path` | Path to the file to delete |

## Outputs

None. Success is signalled by `!NoError`.

## Errors

| Error | When |
|-------|------|
| `!File.NotFound` | File does not exist at path |
| `!File.DeleteError` | Cannot delete file (permissions, locked) |

## Permissions

Requires `File.Delete` capability.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.=:File.Delete` | Compile-time pipeline template |
| Instance | `%=:File.Delete:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/File/INDEX|=File.* File Pipelines]]
- [[pglib/pipelines/File/Move|=File.Move]]
