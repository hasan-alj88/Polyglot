---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.W:HTTP.Session"
metadata_instance: "%W:HTTP.Session:N"
---

# -W.HTTP.Session

Creates HTTP client on setup, closes on cleanup.

## Definition

```polyglot
{N} -W.HTTP.Session
   [%] .Kind << #NativeKind.Wrapper
   [%] .Rust << "WHttpSession"
   [%] .description << "Creates HTTP client on setup, closes on cleanup."
   (-) <baseUrl;string    [ ] Base URL for all requests in this session
   (-) <timeout;int       [ ] Request timeout in milliseconds
   (-) >httpClient        [ ] Configured HTTP client handle
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `$baseUrl` | `#string` | Base URL for all requests in this session |
| `$timeout` | `#int` | Request timeout in milliseconds |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `$httpClient` | client handle | Configured HTTP client handle |

## Errors

None.

## Permissions

Web.Request

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.W:HTTP.Session` | Compile-time pipeline template |
| Instance | `%W:HTTP.Session:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/W/INDEX|-W.* Wrappers]]
