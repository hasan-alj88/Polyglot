---
audience: user
type: specification
updated: 2026-03-28
status: complete
---

# Enum Types

<!-- @types -->

Runtime and internal enum types available in every `.pg` file. All enums use `[#] << ##Scalar` (depth 0) and `[#] << ###Enum` (leaf content is variant selection).

See [[boolean]] for `#Boolean` (also an enum, documented separately).

---

## #OS

```polyglot
{#} #OS
   [%] .description << "Operating system enum"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "os"
   [.] .Unix
   [.] .Windows
```

---

## #PipelineStatus

```polyglot
{#} #PipelineStatus
   [%] .description << "Pipeline instance status"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "pipelinestatus"
   [.] .AwaitTrigger
   [.] .Disabled
   [.] .Running
   [.] .Failed
```

---

## #QueueStrategy

```polyglot
{#} #QueueStrategy
   [%] .description << "Queue ordering strategy"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "queuestrategy"
   [.] .FIFO
   [.] .LIFO
   [.] .Priority
```

---

## #RetriggerStrategy

```polyglot
{#} #RetriggerStrategy
   [%] .description << "Behavior when pipeline is re-triggered while queued or running"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
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
   [%] .description << "Active queue pipeline state"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "queuestate"
   [.] .Running
   [.] .SoftPaused
   [.] .HardPaused
   [.] .Killed
```

---

## #FileAccess

```polyglot
{#} #FileAccess
   [%] .description << "File access state"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
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
   [#] << ##Scalar
   [#] << ###Enum
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
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "fieldkind"
   [.] .Value
   [.] .Enum
   [.] .None
```

Used by the `%##Leafs.Kind` schema property to constrain what `###` field type all leafs in a type must be. For example, `[#] %##Leafs.Kind << #FieldKind.Enum` requires all leafs to be `###Enum` (no type annotation). See [[syntax/types/schema-properties#Approved ## Schema Types]].

---

## Related

- [[boolean]] -- #Boolean enum type
- [[structs]] -- #Queue struct (uses #QueueStrategy and #RetriggerStrategy)
- [[syntax/types/INDEX|types]] -- full type system specification
