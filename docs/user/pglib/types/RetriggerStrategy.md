---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:RetriggerStrategy"
metadata_instance: "%#:RetriggerStrategy:N"
---

# #RetriggerStrategy Enum

<!-- @c:types -->

Runtime `##Enum` type available in every `.pg` file. Uses `[#] ##Enum` (enum classification), `[#] ##Scalar` (depth 1), and `[#] ###ScalarEnum` (leaf content is variant selection).

---

## Definition

```polyglot
{#} #RetriggerStrategy
   [%] .description << "Queue configuration for handling duplicate trigger conditions while pipeline is queued or running"
   [%] .version << "1.0.0"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "retriggerstrategy"
   [.] .Disallow
   [.] .Allow
   [.] .NoDuplicate
   [.] .QueueAfter
```

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:RetriggerStrategy` | Compile-time type template |
| Instance | `%#:RetriggerStrategy:0` | Runtime instance (enum — one active field) |

---

## Related

- [[enums]] — other pglib enum types
- [[Queue]] — queue configuration
- [[concepts/pipelines/io-triggers|Triggers]] — trigger system
- [[syntax/types/INDEX|types]] — full type system specification
