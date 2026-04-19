---
audience: automation-builder
type: specification
updated: 2026-04-19
status: complete
metadata_definition: "%definition.T:Call"
metadata_instance: "%T:Call:N"
---

# -T.Call

Pipeline invoked when called from another pipeline. No additional parameters.

## Definition

```polyglot
{N} -T.Call
   [%] .Kind << #NativeKind.Trigger
   [%] .Rust << "TCall"
   [%] .description << "Pipeline is invoked when called from another pipeline."
   [ ] No additional parameters.
```

## Inputs

| Name | Type | Description |
|------|------|-------------|

## Outputs

| Name | Type | Description |
|------|------|-------------|

## Errors

None.

## Permissions

None.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.T:Call` | Compile-time pipeline template |
| Instance | `%T:Call:N` | Runtime pipeline instance (N = instance number) |

## Signal Flow

<!-- @c:spec/native-dispatch -->
<!-- @c:spec/polyglot-sdk -->

When foreign code calls `polyglot_sdk.call("-PipelineName", bindings)`, a NATS request-reply cycle connects the SDK to the Polyglot Service:

```text
Foreign Code
    │  polyglot_sdk.call("-ProcessData", bindings)
    ▼
Polyglot SDK
    │  1. Serialize each binding via to_polyglot()
    │  2. Publish to NATS: polyglot.call.ProcessData
    │  3. Subscribe to: polyglot.result.{correlation_id}
    ▼
Trigger Monitor (TM)
    │  4. Receives call on polyglot.call.* subscription
    │  5. Matches pipeline with [T] -T.Call
    │  6. Writes bindings to variable store (Redis)
    │  7. Emits command.enqueue → pipeline enters normal queue flow
    ▼
Queue Handler (QH) → Runner → executes pipeline body → writes outputs
    ▼
Trigger Monitor (TM)
    │  8. Receives runner.completed (or runner.failed) signal
    │  9. Reads output ports from variable store
    │  10. Publishes result to NATS: polyglot.result.{correlation_id}
    ▼
Polyglot SDK (waiting)
    │  Receives result, deserializes via from_polyglot()
    │  Returns outputs to foreign code
```

Unlike other triggers (`-T.Daily`, `-T.Webhook`) which are fire-and-forget, `-T.Call` is **synchronous from the caller's perspective** — the SDK blocks until the pipeline completes and results are returned.

For the full technical protocol (NATS topics, Redis key structure, error handling), see [[native-dispatch#-T.Call Signal Path]]. For the SDK API and payload schema, see [[polyglot-sdk#call]].

## Related

- [[pglib/pipelines/T/INDEX|-T.* Trigger Pipelines]]
- [[spec/native-dispatch#-T.Call Signal Path|Signal Path Technical Protocol]]
- [[spec/polyglot-sdk#call|SDK call() API]]
- [[queue-manager/nats-namespace|NATS Subject Registry]]
