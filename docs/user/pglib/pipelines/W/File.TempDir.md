---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.W:File.TempDir"
metadata_instance: "%W:File.TempDir:N"
---

# -W.File.TempDir

Creates temp directory on setup, deletes on cleanup.

## Definition

```polyglot
{N} -W.File.TempDir
   [%] .Kind << #NativeKind.Wrapper
   [%] .Rust << "WFileTempDir"
   [%] .description << "Creates temp directory on setup, deletes on cleanup."
   [{] $prefix#string   [ ] Prefix for the temporary directory name
   [}] $tempDir         [ ] Path to the created temporary directory
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `$prefix` | `#string` | Prefix for the temporary directory name |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `$tempDir` | path | Path to the created temporary directory |

## Errors

None.

## Permissions

File.Write

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.W:File.TempDir` | Compile-time pipeline template |
| Instance | `%W:File.TempDir:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/W/INDEX|-W.* Wrappers]]
- [[pglib/pipelines/W/File.Lock|-W.File.Lock]]
