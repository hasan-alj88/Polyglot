---
audience: pg-coder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:VarState"
metadata_instance: "%#:VarState:N"
---

# #VarState Enum

<!-- @c:types -->

Runtime `##Enum` type available in every `.pg` file. Uses `[#] ##Enum` (enum classification), `[#] ##Scalar` (depth 1), and `[#] ###ScalarEnum` (leaf content is variant selection).

---

## Definition

```polyglot
{#} #VarState
   [%] .description << "Variable lifecycle state"
   [%] .version << "1.0.0"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "varstate"
   [.] .Declared
   [.] .Default
   [.] .Final
   [.] .Failed
   [.] .Released
```

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:VarState` | Compile-time type template |
| Instance | `%#:VarState:0` | Runtime instance (enum — one active field) |

---

## Related

- [[enums]] — other pglib enum types
- [[concepts/variables/variable-lifecycle|Variable Lifecycle]] — variable state transitions
- [[syntax/types/INDEX|types]] — full type system specification
