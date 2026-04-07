---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =W.Cache.Scope

Connects cache on setup, flushes + disconnects on cleanup.

## Definition

```polyglot
{N} =W.Cache.Scope
   [%] .Kind << #NativeKind.Wrapper
   [%] .Rust << "WCacheScope"
   [%] .description << "Connects cache on setup, flushes + disconnects on cleanup."
   [{] $cacheUrl#string   [ ] Cache server connection URL
   [}] $cache             [ ] Active cache connection handle
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `$cacheUrl` | `#string` | Cache server connection URL |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `$cache` | cache handle | Active cache connection handle |

## Errors

None.

## Permissions

Database.Read

## Related

- [[pglib/pipelines/W/INDEX|=W.* Wrappers]]
