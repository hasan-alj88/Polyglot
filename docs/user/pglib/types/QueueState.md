---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.#:QueueState"
metadata_instance: "%#:QueueState:N"
---

# #QueueState Enum

<!-- @types -->

Runtime `##Enum` type available in every `.pg` file. Uses `[#] << ##Enum` (enum classification), `[#] << ##Scalar` (depth 1), and `[#] << ###ScalarEnum` (leaf content is variant selection).

---

## Definition

```polyglot
{#} #QueueState
   [%] .description << "Pipeline state within queue system"
   [%] .version << "2.0.0"
   [#] << ##Enum
   [#] << ##Scalar
   [#] << ###ScalarEnum
   [#] %##Alias << "queuestate"
   [.] .Pending
   [.] .Executing
   [.] .SuspendedSoft
   [.] .SuspendedHard
   [.] .Resuming
   [.] .Teardown
   [.] .Completed
   [.] .Failed
   [.] .Killed
```

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:QueueState` | Compile-time type template |
| Instance | `%#:QueueState:0` | Runtime instance (enum — one active field) |

---

## Related

- [[enums]] — other pglib enum types
- [[Job]] — job lifecycle
- [[Queue]] — queue configuration
- [[syntax/types/INDEX|types]] — full type system specification
