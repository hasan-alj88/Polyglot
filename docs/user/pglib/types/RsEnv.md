---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.#:RsEnv"
metadata_instance: "%#:RsEnv:N"
---

# #RsEnv Struct

<!-- @c:types -->

Runtime environment handle produced by `-W.RT:Rust:*` wrapper setup.

---

## Definition

```aljam3
{#} #RsEnv
   [.] .version#string
   [.] .handle#RawString
```

---

## Fields

| Field | Type | Purpose |
|-------|------|---------|
| `.version` | `#string` | Rust version string (e.g., `"1.84"`) |
| `.handle` | `#RawString` | Opaque runtime handle used by `-RT.Rust.*` pipelines |

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:RsEnv` | Compile-time type template |
| Instance | `%#:RsEnv:N` | Runtime instance (N = instance number) |

---

## Related

- [[rt]] — runtime types overview
- [[pglib/pipelines/RT/INDEX|-RT.*]] — runtime execution pipelines
- [[pglib/pipelines/W/INDEX|-W.*]] — wrapper pipelines
- [[syntax/types/INDEX|types]] — full type system specification
