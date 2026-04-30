---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:PipelineStatus"
metadata_instance: "%#:PipelineStatus:N"
---

# #PipelineStatus Enum

<!-- @c:types -->

Runtime `##Enum` type available in every `.aj3` file. Uses `[#] ##Enum` (enum classification), `[#] ##Scalar` (depth 1), and `[#] ###ScalarEnum` (leaf content is variant selection).

---

## Definition

```aljam3
{#} #PipelineStatus
   [%] .description << "Pipeline instance status"
   [%] .version << "2.0.0"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "pipelinestatus"
   [.] .AwaitTrigger
   [.] .Disabled
   [.] .Pending
   [.] .Executing
   [.] .SuspendedSoft
   [.] .SuspendedHard
   [.] .Teardown
   [.] .Completed
   [.] .Failed
```

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:PipelineStatus` | Compile-time type template |
| Instance | `%#:PipelineStatus:0` | Runtime instance (enum — one active field) |

---

## Related

- [[enums]] — other aj3lib enum types
- [[concepts/pipelines/INDEX|pipelines]] — pipeline concepts
- [[syntax/types/INDEX|types]] — full type system specification
