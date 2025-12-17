---
archived: true
archived_date: 2025-12-17
archived_reason: "Superseded by v0.0.4 sync"
original_path: user/advanced/macro-system.md
---

<!-- ARCHIVED: 2025-12-17 | Reason: Superseded by v0.0.4 sync -->

# Polyglot Macro System

**Version:** 0.0.2  
**Last Updated:** 2025-12-02

---

## Macro Definition

Macros encapsulate reusable setup/cleanup patterns with RAII semantics:

```polyglot
[M] DatabaseTransaction
[\]                                // Setup (FIFO
[r] |DB.BeginTransaction
[r] |DB.AcquireLock
[/]                                // Cleanup (LIFO - reverse order!
[r] |DB.ReleaseLock               // Runs second
[r] |DB.CommitOrRollback          // Runs first
[X]
```

---

## Unwrapping Macros `[W]`

Use `[W]` to unwrap macro at pipeline scope:

```polyglot
[|] UpdateUser
[i] .user_id:pg.int
[t] |T.Call
[W] |DatabaseTransaction           // Unwrap macro

[r] |ValidateUser
[r] |UpdateDatabase
[r] |LogChange

[o] !No.Output
[X]
```

**Execution order:**
1. `DB.BeginTransaction` (setup
2. `DB.AcquireLock` (setup
3. `ValidateUser`, `UpdateDatabase`, `LogChange` (body
4. `DB.ReleaseLock` (cleanup - LIFO!
5. `DB.CommitOrRollback` (cleanup - LIFO!

---

## RAII Pattern

**Resource Acquisition Is Initialization**

Setup runs in **FIFO** (First In, First Out:
```
[\] Step1 → Step2 → Step3
```

Cleanup runs in **LIFO** (Last In, First Out:
```
[/] Step3 → Step2 → Step1  (reverse!
```

---

## Scope Markers

### `[{]` Scope In

Marks where macro code begins:

```polyglot
[M] WithLogging
[\]
[r] |LogStart
[{]                                // Macro body insertion point
[r] .log_context:pg.serial << ...
[]
[/]
[r] |LogEnd
[X]
```

### `[]` Scope Out

Marks where macro code ends.

---

## Type-Based Insertion

Macros can insert code based on type:

```polyglot
[M] TypedCleanup
[\]
[r] .resource_type:pg.string << "{.*.pgvar.type"
[/]
[?] .resource_type =? "file"
[~][r] |CloseFile
[~]
[?] .resource_type =? "socket"
[~][r] |CloseSocket
[~]
[?] *?
[~][r] |GenericCleanup
[~]
[X]
```

---

## Multiple Wrappers

Stack multiple macros:

```polyglot
[|] Pipeline
[i] .data:pg.string
[t] |T.Call
[W] |DatabaseTransaction           // First wrapper
[W] |WithLogging                   // Second wrapper
[W] |ErrorHandling                 // Third wrapper

[r] |ProcessData

[o] .result:pg.string
[X]
```

**Execution:**
```
DB.Begin → Log.Start → ErrorHandler.Setup →
  ProcessData →
ErrorHandler.Cleanup → Log.End → DB.Commit
```

---

## Built-In Wrapper: `|W.Polyglot.Scope`

Default wrapper when no setup/cleanup needed:

```polyglot
[|] SimplePipeline
[i] !No.Input
[t] |T.Call
[W] |W.Polyglot.Scope              // No-op wrapper
[o] !No.Output
[X]
```

**Purpose:**
- RAII-style variable cleanup
- Makes explicit that no custom setup/cleanup
- Required if no other wrappers present

---

## Complete Example

### File Processing with Cleanup

```polyglot
[M] FileHandler
[\]                                // Setup
[r] |OpenFile
[<] .path << .file_path
[>] .handle >> .file_handle
[/]                                // Cleanup
[r] |CloseFile
[<] .handle << .file_handle
[X]


[|] ProcessFile
[i] .file_path:pg.path
[t] |T.Call
[W] |FileHandler                   // Auto cleanup!

[r] |ReadContents
[<] .handle << .file_handle
[>] .data >> .contents

[r] |ProcessData
[<] .input << .contents
[>] .result >> .processed

[o] .processed:pg.string
[X]
// File automatically closed!
```

---

**Next:** [Expansion Operator →](expansion-operator.md
