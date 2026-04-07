---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.#:QueueStrategy"
metadata_instance: "%#:QueueStrategy:N"
---

# #QueueStrategy Enum

<!-- @types -->

Runtime `##Enum` type available in every `.pg` file. Uses `[#] << ##Enum` (enum classification), `[#] << ##Scalar` (depth 1), and `[#] << ###ScalarEnum` (leaf content is variant selection).

---

## Definition

```polyglot
{#} #QueueStrategy
   [%] .description << "Queue ordering strategy"
   [%] .version << "1.0.0"
   [#] << ##Enum
   [#] << ##Scalar
   [#] << ###ScalarEnum
   [#] %##Alias << "queuestrategy"
   [.] .FIFO
   [.] .LIFO
   [.] .Priority
```

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:QueueStrategy` | Compile-time type template |
| Instance | `%#:QueueStrategy:0` | Runtime instance (enum — one active field) |

---

## Related

- [[enums]] — other pglib enum types
- [[Queue]] — queue configuration
- [[syntax/types/INDEX|types]] — full type system specification
