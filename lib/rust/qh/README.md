# Rust — Queue Handler Native Operations

Native implementations for `#NativeKind.Queue` operations, dispatched by the Queue Handler.

## Operations

| Pipeline | Function | Description |
|----------|----------|-------------|
| `=Q.Default` | `QueueDefault` | Default FIFO queue strategy |
| `=Q.Pause.Hard` | `QueuePauseHard` | Hard pause (suspend immediately) |
| `=Q.Pause.Soft` | `QueuePauseSoft` | Soft pause (finish current, don't start new) |
| `=Q.Resume` | `QueueResume` | Resume paused jobs |
| `=Q.Kill.Graceful` | `QueueKillGraceful` | Graceful termination |

## Contract

Queue functions manage job scheduling state. They follow the standard JSON wire protocol.

```rust
pub fn queue_default(request: &str) -> Result<String, String>
```

See `docs/technical/spec/native-dispatch.md#Queue` for dispatch details.
