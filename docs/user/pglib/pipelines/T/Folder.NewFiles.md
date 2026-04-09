---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.T:Folder.NewFiles"
metadata_instance: "%T:Folder.NewFiles:N"
---

# -T.Folder.NewFiles

Fires when new files appear in the specified folder. Folder path provided via inline call: `-T.Folder.NewFiles"/inbox/"`.

## Definition

```polyglot
{N} -T.Folder.NewFiles
   [%] .Kind << #NativeKind.Trigger
   [%] .Rust << "TFolderNewFiles"
   [%] .description << "Fires when new files appear in the specified folder."
   <Folder#path
   >NewFiles#array.path
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `Folder` | `#path` | Path to the folder to watch. Provided inline: `-T.Folder.NewFiles"/inbox/"`. |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `NewFiles` | `#array.path` | Array of paths for newly detected files. |

## Errors

None.

## Permissions

File.Read

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.T:Folder.NewFiles` | Compile-time pipeline template |
| Instance | `%T:Folder.NewFiles:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/T/INDEX|-T.* Trigger Pipelines]]
