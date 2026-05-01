# Aljam3-Integration

Language-specific SDKs for integrating Aljam3 into host-language codebases.

Each language folder provides packages for embedding Aljam3 and implementing native operations.

## Structure

| Folder | Language | Contents |
|--------|----------|----------|
| `rust/` | Rust | Integrator SDK, TM, QH, Runner native implementations |
| `go/` | Go | Integrator SDK (scaffold) |
| `python/` | Python | Integrator SDK (scaffold) |

## Standard Packages per Language

| Package | Purpose |
|---------|---------|
| `integrator/` | Bidirectional SDK: call Aljam3 + be called by Aljam3 |
| `tm/` | Trigger Monitor native operations |
| `qh/` | Queue Handler native operations |
| `runner/` | Runner native operations (data ops + wrapper lifecycle) |

## Adding a Language

1. Create `<language>/` with the standard subfolders
2. Implement native functions following the JSON wire contract
3. Add `[%] .<Language>` bindings to `{N}` definitions in jm3lib `.jm3` files
4. Update the YAML config
