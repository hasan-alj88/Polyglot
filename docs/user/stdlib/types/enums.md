---
audience: pg-coder
type: specification
updated: 2026-04-04
status: complete
---

# Enum Types

<!-- @types -->

Runtime and internal `##Enum` types available in every `.pg` file. All enums use `[#] << ##Enum` (enum classification), `[#] << ##Scalar` (depth 1), and `[#] << ###ScalarEnum` (leaf content is variant selection).

See [[boolean]] for `#Boolean` (also a `##Enum` type, documented separately).

---

## #OS

```polyglot
{#} #OS
   [%] .description << "Operating system enum"
   [%] .version << "1.0.0"
   [#] << ##Enum
   [#] << ##Scalar
   [#] << ###ScalarEnum
   [#] %##Alias << "os"
   [.] .Unix
   [.] .Windows
```

---

## #PipelineStatus

```polyglot
{#} #PipelineStatus
   [%] .description << "Pipeline instance status"
   [%] .version << "2.0.0"
   [#] << ##Enum
   [#] << ##Scalar
   [#] << ###ScalarEnum
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

## #QueueStrategy

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

## #RetriggerStrategy

```polyglot
{#} #RetriggerStrategy
   [%] .description << "Queue configuration for handling duplicate trigger conditions while pipeline is queued or running"
   [%] .version << "1.0.0"
   [#] << ##Enum
   [#] << ##Scalar
   [#] << ###ScalarEnum
   [#] %##Alias << "retriggerstrategy"
   [.] .Disallow
   [.] .Allow
   [.] .NoDuplicate
   [.] .QueueAfter
```

---

## #QueueState

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

## #KillPropagation

```polyglot
{#} #KillPropagation
   [%] .description << "How kill signals propagate from parent job to sub-jobs"
   [%] .version << "1.0.0"
   [#] << ##Enum
   [#] << ##Scalar
   [#] << ###ScalarEnum
   [#] %##Alias << "killpropagation"
   [.] .Cascade
   [.] .Downgrade
```

| Variant | Behavior |
|---------|----------|
| `#Cascade` | Sub-jobs receive the same kill type as parent (hard→hard, graceful→graceful) |
| `#Downgrade` | Hard kill on parent → graceful kill on sub-jobs (allows `[/]` cleanup) |

Default: `#Cascade`. Orphan jobs are never permitted — every sub-job must be terminated when its parent is killed.

---

## #ResourceTag

```polyglot
{#} #ResourceTag
   [%] .description << "Resource tag for pipeline dispatch constraints"
   [%] .version << "1.0.0"
   [#] << ##Enum
   [#] << ##Scalar
   [#] << ###ScalarEnum
   [#] %##Alias << "resourcetag"
   [.] .GPU
   [.] .HighRAM
   [.] .HighCPU
   [.] .HighIO
   [.] .Network
```

Used in `#Queue.resourceTags` for dispatch constraint checking. The Dispatch Coordinator enforces resource exclusion — e.g., only one `#GPU`-tagged pipeline executes at a time.

---

## #FileAccess

```polyglot
{#} #FileAccess
   [%] .description << "File access state"
   [%] .version << "1.0.0"
   [#] << ##Enum
   [#] << ##Scalar
   [#] << ###ScalarEnum
   [#] %##Alias << "fileaccess"
   [.] .Available
   [.] .Locked
   [.] .NotFound
```

---

## #VarState

```polyglot
{#} #VarState
   [%] .description << "Variable lifecycle state"
   [%] .version << "1.0.0"
   [#] << ##Enum
   [#] << ##Scalar
   [#] << ###ScalarEnum
   [#] %##Alias << "varstate"
   [.] .Declared
   [.] .Default
   [.] .Final
   [.] .Failed
   [.] .Released
```

## #FieldKind

```polyglot
{#} #FieldKind
   [%] .description << "Leaf content field type classifier"
   [%] .version << "1.0.0"
   [#] << ##Enum
   [#] << ##Scalar
   [#] << ###ScalarEnum
   [#] %##Alias << "fieldkind"
   [.] .Value
   [.] .Enum
   [.] .None
```

Used by the `%##Leafs.Kind` schema property to constrain what `###` field type all leafs in a type must be. For example, `[#] %##Leafs.Kind << #FieldKind.Enum` requires all leafs to be `###Enum` (no type annotation). See [[syntax/types/schema-properties#Approved ## Schema Types]].

---

## Related

- [[boolean]] -- #Boolean enum type
- [[structs]] -- #Queue struct (uses #QueueStrategy, #KillPropagation, #ResourceTag)
- [[syntax/types/INDEX|types]] -- full type system specification
