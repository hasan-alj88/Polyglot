# Rust — Trigger Monitor Native Operations

Native implementations for `#NativeKind.Trigger` operations, dispatched by the Trigger Monitor.

## Operations

| Pipeline | Function | Description |
|----------|----------|-------------|
| `=T.Call` | `TriggerCall` | Pipeline invoked by another pipeline |
| `=T.Daily` | `TriggerDaily` | Time-based daily trigger |
| `=T.Folder.NewFiles` | `TriggerFolderNewFiles` | File system watcher |
| `=T.Webhook` | `TriggerWebhook` | HTTP webhook receiver |

## Contract

Each function evaluates a trigger condition and returns `>IsTriggered#bool` plus any additional outputs.

```rust
pub fn trigger_call(request: &str) -> Result<String, String>
```

See `docs/technical/spec/native-dispatch.md#Trigger` for dispatch details.
