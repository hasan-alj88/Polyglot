# lib/ — Native Implementations & Integrator SDKs

This directory contains host-language packages for Polyglot's native operations and integrator SDKs.

**Spec:** See `docs/technical/spec/native-dispatch.md` for the full architecture.

## Structure

Each language folder contains five packages:

| Folder | Purpose | #NativeKind |
|--------|---------|-------------|
| `integrator/` | Bidirectional SDK: call Polyglot + be called by Polyglot | N/A |
| `tm/` | Trigger Monitor native operations | `.Trigger` |
| `qh/` | Queue Handler native operations | `.Queue` |
| `runner/` | Runner native operations (data ops + wrapper lifecycle) | `.Execution`, `.Wrapper` |
| `pgcompiler/` | Compiler: lexer, parser, compile rule enforcement | N/A |

## Contract

All native functions follow the same JSON wire protocol:

- **Input:** JSON request envelope with typed parameters
- **Output:** JSON response envelope (success with outputs, or error with id + detail)
- **Error IDs** must match the `[=] !` declarations in the `{N}` definition

See `docs/technical/spec/native-dispatch.md#Native Function Contract` for details.

## Configuration

The YAML config (`native:` section) selects which language implements each operation:

```yaml
native:
  defaults:
    tm: Rust
    qh: Rust
    runner: Rust
    pgcompiler: Rust
  overrides:
    "Math.Add": Go
```

## Adding a Language

1. Create `lib/<language>/` with the five standard subfolders
2. Implement native functions following the JSON contract
3. Add `[%] .<Language>` bindings to `{N}` definitions in pglib `.pg` files
4. Update the YAML config
