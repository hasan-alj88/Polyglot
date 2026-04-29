---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.#:Code"
metadata_instance: "%#:Code:N"
---

# #Code Struct

<!-- @c:types -->

Runtime execution output type. Output struct for all `-RT.*` pipeline modes. Language is a flexible field (`:Python`, `:Rust`, etc.), `.Output` is fixed.

---

## Definition

```aljam3
{#} #Code
   :Python
      .Output
         [.] .stdout#RawString
         [.] .stderr#RawString
         [.] .return#serial
   :Rust
      .Output
         [.] .stdout#RawString
         [.] .stderr#RawString
         [.] .return#serial
```

---

## Fields

| Field | Type | Purpose |
|-------|------|---------|
| `.stdout` | `#RawString` | Standard output capture (raw bytes) |
| `.stderr` | `#RawString` | Standard error capture (raw bytes) |
| `.return` | `#serial` | Function return value (shape varies by code). Empty for `.Script` and `.Bind` modes. |

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:Code` | Compile-time type template |
| Instance | `%#:Code:N` | Runtime instance (N = instance number) |

---

## Related

- [[rt]] — runtime types overview
- [[pglib/pipelines/RT/INDEX|-RT.*]] — runtime execution pipelines
- [[pglib/pipelines/W/INDEX|-W.*]] — wrapper pipelines
- [[syntax/types/INDEX|types]] — full type system specification
