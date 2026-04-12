---
audience: pg-coder
type: specification
updated: 2026-04-11
status: draft
metadata_definition: "%definition.#:JobStatus"
metadata_instance: "%#:JobStatus:N"
---

# #JobStatus Enum

<!-- @c:types -->

Runtime `##Enum` type available in every `.pg` file. Represents the lifecycle state of a job as observed by a `{*}` collector.

---

## Definition

```polyglot
{#} #JobStatus
   [%] .description << "Job lifecycle state as observed by collectors"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "jobstatus"
   [.] .Running
   [.] .Completed
   [.] .Failed
   [.] .Cancelled
```

---

## Variants

| Variant | Description |
|---------|-------------|
| `.Running` | Job is still executing — variable value may be partial |
| `.Completed` | Job completed successfully — variable value is Final |
| `.Failed` | Job failed — trigger fires with error context |
| `.Cancelled` | Job was cancelled by a collector release |

---

## Related

- [[IncomingDataFrame]] — uses `#JobStatus` in `.status` field
- [[enums]] — other pglib enum types
- [[technical/spec/collector-definitions\|Collector Definitions]] — `{*}` block specification
