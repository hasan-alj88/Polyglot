---
audience: automation-builder
type: specification
updated: 2026-04-18
status: complete
metadata_definition: "%definition.#:LimitAction"
metadata_instance: "%#:LimitAction:N"
---

# #LimitAction Enum

<!-- @c:types -->

Action to take when a resource limit is exceeded. Used in `{Q}` queue configuration for limit-exceeded behavior. See [[concepts/permissions|Permissions]], [[concepts/pipelines/queue/INDEX|Queue]].

## Definition

```polyglot
{#} #LimitAction
   [%] .description << "Action when resource limit exceeded"
   [%] .version << "1.0.0"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "limitaction"
   [.] .Kill
   [.] .Throttle
   [.] .Retry
```

| Variant | Description | Applicable To |
|---------|-------------|---------------|
| `.Kill` | SIGTERM followed by SIGKILL after grace period. Job enters Failed state. | All categories |
| `.Throttle` | cgroup throttles the resource. Job continues at reduced capacity. | CPU, IO only (memory cannot be throttled) |
| `.Retry` | Kill and re-queue. Uses existing `-Q.*` retry configuration for retry count and backoff. | All categories |

> **Constraint:** `.Throttle` is only valid for `#CPU` and `#IO` categories. Using `.Throttle` with `#RAM`, `#GPU`, `#Processes`, or `#Duration` is a compile error — these resources can only be killed or retried, not throttled. The compiler validates this at the `{Q}` definition site.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:LimitAction` | Compile-time type template |
| Instance | `%#:LimitAction:0` | Runtime instance (enum — one active field) |

## Related

- [[LimitConfig]] -- limit configuration struct
- [[concepts/pipelines/queue/INDEX|Queue]] -- queue limit-exceeded behavior
- [[concepts/permissions|Permissions]] -- permission system overview
