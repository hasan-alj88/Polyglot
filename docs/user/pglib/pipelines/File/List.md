---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.=:File.List"
metadata_instance: "%=:File.List:N"
---

# =File.List

List all files in a folder, returning an array of paths.

## Definition

```polyglot
{N} =File.List
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "FileList"
   [%] .description << "List files in folder"
   [=] <folder#path
   [=] >files#array.path
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<folder` | `#path` | Path to the folder to list |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>files` | `#array.path` | Array of file paths in the folder |

## Errors

| Error | When |
|-------|------|
| `!Folder.NotFound` | Folder does not exist at path |
| `!Folder.ReadError` | Folder exists but cannot be read (permissions) |

## Permissions

Requires `File.Read` capability.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.=:File.List` | Compile-time pipeline template |
| Instance | `%=:File.List:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/File/INDEX|=File.* File Pipelines]]
- [[pglib/pipelines/File/Access|=File.Access]]
