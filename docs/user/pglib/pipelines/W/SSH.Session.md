---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =W.SSH.Session

Connects SSH on setup, disconnects on cleanup.

## Definition

```polyglot
{N} =W.SSH.Session
   [%] .Kind << #NativeKind.Wrapper
   [%] .Rust << "WSshSession"
   [%] .description << "Connects SSH on setup, disconnects on cleanup."
   [{] $host#string          [ ] SSH host address
   [{] $credentials#string   [ ] SSH credentials (key path or password reference)
   [}] $sshSession           [ ] Active SSH session handle
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

## Related

- [[pglib/pipelines/W/INDEX|=W.* Wrappers]]
