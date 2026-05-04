---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.W:SSH.Session"
metadata_instance: "%W:SSH.Session:N"
---

# -W.SSH.Session

Connects SSH on setup, disconnects on cleanup.

## Definition

```aljam3
{N} -W.SSH.Session
   [%] .Kind << #NativeKind.Wrapper
   [%] .Rust << "WSshSession"
   [%] .description << "Connects SSH on setup, disconnects on cleanup."
   (-) <host;string          [ ] SSH host address
   (-) <credentials;string   [ ] SSH credentials (key path or password reference)
   (-) >sshSession           [ ] Active SSH session handle
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `$host` | `#string` | SSH host address |
| `$credentials` | `#string` | SSH credentials (key path or password reference) |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `$sshSession` | session handle | Active SSH session handle |

## Errors

None.

## Permissions

System.Process

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.W:SSH.Session` | Compile-time pipeline template |
| Instance | `%W:SSH.Session:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[jm3lib/pipelines/W/INDEX|-W.* Wrappers]]
