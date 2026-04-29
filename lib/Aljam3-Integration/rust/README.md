# Rust — Native Implementations

Rust is the initial host language for all Aljam3 native operations.

## Packages

| Package | Description |
|---------|-------------|
| `integrator/` | Rust SDK for bidirectional Aljam3 integration |
| `tm/` | Trigger Monitor native ops (`=T.Call`, `=T.Daily`, etc.) |
| `qh/` | Queue Handler native ops (`=Q.Default`, `=Q.Pause.*`, etc.) |
| `runner/` | Runner native ops (`=File.*`, `=Math.*`, `=DB.*`, `=DT.*`, `=W.*`) |
| `pgcompiler/` | Aljam3 compiler: lexer, parser, compile rules |

## Contract

Every native function signature:

```rust
pub fn function_name(request: &str) -> Result<String, String>
```

- Input: JSON request envelope
- Output: JSON response envelope (Ok = success, Err = error envelope)

See `docs/technical/spec/native-dispatch.md` for the full wire protocol.
