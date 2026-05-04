---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.W:Cache.Scope"
metadata_instance: "%W:Cache.Scope:N"
---

# -W.Cache.Scope

Connects cache on setup, flushes + disconnects on cleanup.

## Definition

```aljam3
{N} -W.Cache.Scope
   [%] .Kind << #NativeKind.Wrapper
   [%] .Rust << "WCacheScope"
   [%] .description << "Connects cache on setup, flushes + disconnects on cleanup."
   (-) <cacheUrl;string   [ ] Cache server connection URL
   (-) >cache             [ ] Active cache connection handle
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

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.W:Cache.Scope` | Compile-time pipeline template |
| Instance | `%W:Cache.Scope:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[jm3lib/pipelines/W/INDEX|-W.* Wrappers]]
