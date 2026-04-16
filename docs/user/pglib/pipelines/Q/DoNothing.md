---
audience: automation-builder
type: specification
updated: 2026-04-16
status: complete
metadata_definition: "%definition.Q:DoNothing"
metadata_instance: "%Q:DoNothing:N"
---

# -Q.DoNothing

No action — satisfies `*?` exhaustiveness in conditional blocks. The compiler emits a warning listing states not explicitly handled.

## Definition

```polyglot
{N} -Q.DoNothing
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QDoNothing"
   [%] .description << "No action — satisfies exhaustiveness in conditional blocks."
```

## Inputs

None.

## Outputs

None.

## Errors

None.

## Compiler Warning

When a `[Q]` block uses state guards and a `DoNothing` catch-all, the compiler emits a warning listing states not explicitly handled. This ensures the developer sees what they are implicitly ignoring. The warning is informational — sometimes `DoNothing` is genuinely correct.

## Runtime Behavior

No runtime signal. No NATS command. No Redis write. No Unix mechanism. Compiler no-op satisfying exhaustiveness in `*?` catch-all blocks.

See [[queue-manager/signal-map|Signal Map]] for the full cross-reference.

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:DoNothing` | Compile-time pipeline template |
| Instance | `%Q:DoNothing:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
