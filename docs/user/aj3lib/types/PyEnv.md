---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.#:PyEnv"
metadata_instance: "%#:PyEnv:N"
---

# #PyEnv Struct

<!-- @c:types -->

Runtime environment handle produced by `-W.RT:Python:*` wrapper setup.

---

## Definition

```aljam3
{#} #PyEnv
   [.] .version#string
   [.] .handle#RawString
```

---

## Fields

| Field | Type | Purpose |
|-------|------|---------|
| `.version` | `#string` | Python version string (e.g., `"3.14"`) |
| `.handle` | `#RawString` | Opaque runtime handle used by `-RT.Python.*` pipelines |

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:PyEnv` | Compile-time type template |
| Instance | `%#:PyEnv:N` | Runtime instance (N = instance number) |

---

## Related

- [[rt]] — runtime types overview
- [[aj3lib/pipelines/RT/INDEX|-RT.*]] — runtime execution pipelines
- [[aj3lib/pipelines/W/INDEX|-W.*]] — wrapper pipelines
- [[syntax/types/INDEX|types]] — full type system specification
