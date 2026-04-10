---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.W:Log.Context"
metadata_instance: "%W:Log.Context:N"
---

# -W.Log.Context

Opens structured log scope on setup, closes on cleanup.

## Definition

```polyglot
{N} -W.Log.Context
   [%] .Kind << #NativeKind.Wrapper
   [%] .Rust << "WLogContext"
   [%] .description << "Opens structured log scope on setup, closes on cleanup."
   (-) <traceId;string   [ ] Trace ID for the log scope
   (-) >logScope         [ ] Active structured log scope
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `$traceId` | `#string` | Trace ID for the log scope |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `$logScope` | log scope | Active structured log scope |

## Errors

None.

## Permissions

None — pure computation.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.W:Log.Context` | Compile-time pipeline template |
| Instance | `%W:Log.Context:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/W/INDEX|-W.* Wrappers]]
