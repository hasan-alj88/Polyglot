---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =W.HTTP.Session

Creates HTTP client on setup, closes on cleanup.

## Definition

```polyglot
{N} =W.HTTP.Session
   [%] .Kind << #NativeKind.Wrapper
   [%] .Rust << "WHttpSession"
   [%] .description << "Creates HTTP client on setup, closes on cleanup."
   [{] $baseUrl#string    [ ] Base URL for all requests in this session
   [{] $timeout#int       [ ] Request timeout in milliseconds
   [}] $httpClient        [ ] Configured HTTP client handle
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

## Related

- [[pglib/pipelines/W/INDEX|=W.* Wrappers]]
