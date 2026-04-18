---
audience: automation-builder
type: specification
updated: 2026-04-18
status: complete
metadata_definition: "%definition.#:LimitConfig"
metadata_instance: "%#:LimitConfig:N"
---

# #LimitConfig Struct

<!-- @c:types -->

Configuration for resource limit-exceeded behavior. Used in `{Q}` queue definitions. See [[concepts/pipelines/queue|Queue]].

## Definition

```polyglot
{#} #LimitConfig
   [%] .description << "Resource limit exceeded configuration"
   [%] .version << "1.0.0"
   [#] ##Record
   [#] %##Alias << "limitconfig"
   [.] .action#LimitAction
   [.] .gracePeriod#Duration
```

| Field | Type | Description |
|-------|------|-------------|
| `.action` | `#LimitAction` | What to do when the limit is exceeded |
| `.gracePeriod` | `#Duration` | Time between SIGTERM and SIGKILL for `.Kill` action. Ignored for `.Throttle`. |

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:LimitConfig` | Compile-time type template |
| Instance | `%#:LimitConfig:0` | Runtime instance |

## Related

- [[LimitAction]] -- limit action enum
- [[concepts/pipelines/queue|Queue]] -- queue limit-exceeded behavior
