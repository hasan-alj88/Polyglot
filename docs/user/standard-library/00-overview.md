# Standard Library Overview

**Version:** 0.0.2
**Last Updated:** 2025-11-11
**Status:** Complete

---

## Overview

The Polyglot standard library provides built-in pipelines, operations, and enumerations that form the foundation of Polyglot applications. The library is organized into namespaces, each serving a specific purpose.

**Key Namespaces:**
- `|W.*` - Runtime wrappers (Python, Node, Rust, etc.)
- `|Q.*` - Queue control operations
- `|Y.*` - Join operations
- `|U.*` - Utility functions (catalog only - APIs TBD)
- `|T.*` - Trigger patterns (catalog only - APIs TBD)

**Reserved Enumerations:**
- `#Path.Identifiers.*` - Cross-platform path definitions
- `#Queues.*` - Queue configurations
- `#Status.*` - System status values
- `#None` - No value indicator
- `#DT.*` - DateTime configurations (TBD)

---

## Table of Contents

1. [Namespace Organization](#namespace-organization)
2. [Namespace Status](#namespace-status)
3. [Runtime Wrappers (|W.*)](#runtime-wrappers-w)
4. [Queue Control (|Q.*)](#queue-control-q)
5. [Join Operations (|Y.*)](#join-operations-y)
6. [Utilities (|U.*)](#utilities-u)
7. [Triggers (|T.*)](#triggers-t)
8. [Reserved Enumerations](#reserved-enumerations)
9. [Standard Library Philosophy](#standard-library-philosophy)
10. [Usage Guidelines](#usage-guidelines)

---

## Namespace Organization

### Naming Convention

All standard library pipelines follow a hierarchical dot notation:

```
|Namespace.Category.Operation
```

**Examples:**
```polyglot
|W.Python3.11          // Wrapper.Runtime.Version
|Q.Pause               // Queue.Operation
|U.String.Format       // Utility.Category.Function
|T.Every.Minute        // Trigger.Pattern.Interval
```

---

### Namespace Hierarchy

```
Standard Library
├── |W.* (Runtime Wrappers)
│   ├── |W.Python
│   ├── |W.Node
│   ├── |W.Rust
│   └── ...
│
├── |Q.* (Queue Control)
│   ├── |Q.Pause
│   ├── |Q.Resume
│   ├── |Q.Kill
│   └── ...
│
├── |Y.* (Join Operations)
│   └── |Y.Join
│
├── |U.* (Utilities)
│   ├── |U.String.*
│   ├── |U.Array.*
│   ├── |U.Path.*
│   └── ...
│
└── |T.* (Triggers)
    ├── |T.Daily
    ├── |T.Every.*
    ├── |T.File.*
    └── ...
```

---

## Namespace Status

### Fully Documented Namespaces

These namespaces have complete API specifications:

| Namespace | Purpose | Status | Documentation |
|-----------|---------|--------|---------------|
| `|W.*` | Runtime wrappers | ✓ Fully documented | [Runtime Wrappers](01-runtime-wrappers.md) |
| `|Q.*` | Queue control | ✓ Fully documented | [Queue Control](02-queue-control.md) |
| `|Y.*` | Join operations | ✓ Fully documented | [Join Operations](05-join-operations.md) |

---

### Catalog-Only Namespaces

These namespaces are referenced but full APIs are still being designed:

| Namespace | Purpose | Status | Documentation |
|-----------|---------|--------|---------------|
| `|U.*` | Utility functions | ⚠ Catalog only (APIs TBD) | [Utilities](03-utilities.md) |
| `|T.*` | Trigger patterns | ⚠ Catalog only (APIs TBD) | [Triggers](04-triggers.md) |

**Note:** Catalog-only namespaces have their operations referenced in examples and documentation, but complete API specifications will be designed after core syntax is finalized.

---

### Reserved Enumerations Status

| Enumeration | Purpose | Status |
|-------------|---------|--------|
| `#Path.Identifiers.*` | Cross-platform paths | ✓ Schema confirmed |
| `#Queues.*` | Queue configurations | ⚠ Schema pending |
| `#Status.*` | System status | ⚠ Definition pending |
| `#None` | No value | ⚠ Definition pending |
| `#DT.*` | DateTime configs | ⚠ Discovery needed |
| `#Errors.*` | Error types | ✗ Deprecated (replaced by `!Error`) |

**See:** [Reserved Enumeration Schemas](../reserved-enumeration-schema-decisions.md) for pending definitions.

---

## Runtime Wrappers (|W.*)

### Purpose

Execute code in other programming languages (Python, Node, Rust, Go, Ruby, Deno).

---

### Key Features

- Fixed version wrappers: `|W.Python3.11`, `|W.Node20`
- Dynamic version wrappers: `|W.Python`, `|W.Node`
- Multiple runtimes in same pipeline
- Implementation detail: Uses `uv` (not exposed to users)

---

### Example

```polyglot
[|] MultiRuntimePipeline
[i] .data:pg.string

// Python processing
[W] |W.Python3.11
[r] |PythonAnalyze
[<] .input:pg.string << .data
[>] .result:pg.string >> python_result

// Node processing
[W] |W.Node20
[r] |NodeTransform
[<] .input:pg.string << python_result
[>] .result:pg.string >> final_result

[X]
```

**Full Documentation:** [Runtime Wrappers](01-runtime-wrappers.md)

---

## Queue Control (|Q.*)

### Purpose

Control pipeline instance execution through queue operations.

---

### Key Operations

| Operation | Purpose |
|-----------|---------|
| `|Q.Pause` | Pause instance |
| `|Q.Resume` | Resume paused instance |
| `|Q.Kill` | Terminate instance |
| `|Q.PriorityBump` | Increase instance priority |
| `|Q.Queue.Assign` | Assign to specific queue |
| `|Q.Status` | Get queue status |

---

### System Queues

- `#Queues.Pending` - Waiting to run
- `#Queues.Dispatch` - Currently running
- `#Queues.Pause` - Temporarily stopped

---

### Example

```polyglot
[|] ControlledTask
[i] .data:pg.string

[r] |Step1

// Conditionally pause
[Q] |Q.PauseIf.RAM.Available.LessThan
[<] .mb:pg.uint << 512

[r] |Step2

[X]
```

**Full Documentation:** [Queue Control](02-queue-control.md)

---

## Join Operations (|Y.*)

### Purpose

Synchronize variables from parallel scopes to outer scope.

---

### Key Operation

**`|Y.Join`** - The primary join operation

Used with `[Y]` block marker and `[>]` to list variables to synchronize.

---

### Example

```polyglot
[|] ParallelWorkflow
[i] .data:pg.string

[p] |ProcessA
[<] .input:pg.string << .data
[>] .result >> result_a

[p] |ProcessB
[<] .input:pg.string << .data
[>] .result >> result_b

// Join - synchronize results
[Y] |Y.Join
[>] result_a
[>] result_b

// Now both results available
[r] |CombineResults
[<] .a:pg.string << result_a
[<] .b:pg.string << result_b

[X]
```

**Full Documentation:** [Join Operations](05-join-operations.md)

---

## Utilities (|U.*)

### Purpose

Provide common utility functions for string manipulation, array operations, file I/O, and more.

---

### Status

**⚠ Catalog Only - APIs TBD**

Utility operations are referenced throughout documentation and examples, but complete API specifications will be designed after core syntax is finalized.

---

### Referenced Categories

**String Operations:**
- `|U.String.Format`
- `|U.String.Concat`
- `|U.String.Split`
- `|U.String.ToInt`

**Array Operations:**
- `|U.Array.Length`
- `|U.Array.Append`
- `|U.Array.Filter`
- `|U.Array.Map`

**Path Operations:**
- `|U.Path.Join`
- `|U.Path.Exists`
- `|U.Path.Parent`

**Logging:**
- `|U.Log.Info`
- `|U.Log.Warning`
- `|U.Log.Error`

**And more...**

---

### Example Usage

```polyglot
// String formatting
[r] |U.String.Format
[<] .template:pg.string << "Hello, {}!"
[<] .value:pg.string << "World"
[>] .result:pg.string >> greeting

// Array operations
[r] |U.Array.Length
[<] .arr: pg.array.pg.string << items
[>] .length:pg.uint >> count

// Logging
[r] |U.Log.Info
[<] .msg:pg.string << "Processing started"
```

**Full Documentation:** [Utilities Catalog](03-utilities.md)

---

## Triggers (|T.*)

### Purpose

Define when pipelines should activate (time-based, file-based, event-based).

---

### Status

**⚠ Catalog Only - APIs TBD**

Trigger operations are referenced throughout documentation and examples, but complete API specifications will be designed after core syntax is finalized.

---

### Referenced Patterns

**Time-Based:**
- `|T.Daily`
- `|T.Every.Minute`
- `|T.Every.Hour`
- `|T.Every.Seconds`

**File-Based:**
- `|T.File.Modified`
- `|T.File.Created`
- `|T.File.Deleted`

**Event-Based:**
- `|T.Event` (API TBD)

---

### Example Usage

```polyglot
// Daily trigger
[|] DailyReport
[t] |T.Daily
[<] .time:pg.dt << DT"09:00:"
[r] |GenerateReport
[X]

// File change trigger
[|] FileWatcher
[t] |T.File.Modified
[<] .path:pg.path << \\DataDir\\config.json
[r] |ReloadConfig
[X]

// Interval trigger
[|] FrequentTask
[t] |T.Every.Seconds
[<] .interval:pg.int << 30
[r] |CheckStatus
[X]
```

**Full Documentation:** [Triggers Catalog](04-triggers.md)

---

## Reserved Enumerations

### Purpose

System-defined enumerations that provide core functionality.

---

### Confirmed Schemas

**`#Path.Identifiers.*`** - Cross-platform path definitions

Schema (confirmed):
```polyglot
[#] Path.Identifiers.{CustomName}
[<] .unix:pg.path      // REQUIRED
[<] .windows:pg.path   // REQUIRED
[X]
```

Example:
```polyglot
[#] Path.Identifiers.MyApp.DataDir
[A] DataDir
[<] .unix:pg.path << \\UnixRoot\\opt\myapp\data\
[<] .windows:pg.path << \\C\\ProgramData\MyApp\Data\
[X]

// Usage
[r] .file:pg.path << \\DataDir\\records.csv
```

---

### Pending Schemas

**`#Queues.*`** - Custom queue definitions (schema pending)

**`#DT.Business.Week.*`** - Business week definitions (schema pending)

**`#Status.*`** - System status values (definition pending)

**`#None`** - No value indicator (definition pending)

---

### Deprecated

**`#Errors.*`** - ✗ Completely replaced by `!Error` syntax in v0.0.2

Migration:
- Old: `#Errors.SomeError`
- New: `!MyApp.SomeError`

---

## Standard Library Philosophy

### Design Principles

1. **Minimal Core, Rich Library**
   - Core language is small and focused
   - Standard library provides rich functionality

2. **Namespace Organization**
   - Clear, hierarchical naming
   - Easy to discover related operations
   - Avoid namespace pollution

3. **Consistent Patterns**
   - Similar operations use similar patterns
   - Predictable parameter names
   - Uniform error handling

4. **Extensibility**
   - Users can create custom operations
   - Package system for sharing
   - No distinction between std and user code

5. **Documentation First**
   - Catalog references before implementation
   - Clear examples for all operations
   - Complete API specifications

---

### Implementation Strategy

**Phase 1: Core Syntax (Current)**
- Define language syntax
- Document standard library references
- Create catalog of operations

**Phase 2: API Design (Next)**
- Design complete APIs for |U.* and |T.*
- Specify input/output contracts
- Define error conditions

**Phase 3: Implementation (Future)**
- Implement compiler
- Build standard library
- Create runtime

---

## Usage Guidelines

### Calling Standard Library Pipelines

```polyglot
// Standard library pipelines use | operator
[r] |Q.Pause
[r] |U.String.Format
[r] |W.Python3.11
```

---

### Using Reserved Enumerations

```polyglot
// Reserved enumerations use # operator
[r] .status: #Status << #Status.Success
[r] .file:pg.path << \\DataDir\\file.txt

// Extend reserved enumerations
[#] Path.Identifiers.MyApp.CustomPath
[<] .unix:pg.path << \\UnixRoot\\path\
[<] .windows:pg.path << \\C\\path\
[X]
```

---

### Error Handling with Standard Library

```polyglot
[r] |U.File.Read
[<] .path:pg.path << file_path

// Standard library operations can throw errors
[!] !pg.FileSystem.NotFound
[>] .message:pg.string >> err_msg
[r] |U.Log.Error
[<] .msg:pg.string << err_msg
```

---

### Wrapper Context

```polyglot
// Use [W] block marker for wrappers
[W] |W.Python3.11
[r] |RunPythonScript
[<] .script:pg.path << "analyze.py"
```

---

### Queue Control

```polyglot
// Use [Q] block marker for queue operations
[Q] |Q.Pause
[Q] |Q.Resume
[Q] |Q.Kill
```

---

### Triggers

```polyglot
// Use [t] block marker for triggers
[t] |T.Daily
[<] .time:pg.dt << DT"09:00:"

[t] |T.File.Modified
[<] .path:pg.path << \\ConfigDir\\app.conf
```

---

## See Also

### Standard Library Documentation
- [Runtime Wrappers (|W.*)](01-runtime-wrappers.md) - Complete reference
- [Queue Control (|Q.*)](02-queue-control.md) - Complete reference
- [Utilities (|U.*)](03-utilities.md) - Catalog
- [Triggers (|T.*)](04-triggers.md) - Catalog
- [Join Operations (|Y.*)](05-join-operations.md) - Complete reference
- [Reserved Enumerations](06-reserved-enumerations.md) - Schemas (deferred)

### Language Specification
- [Block Markers](../language/06-block-markers.md) - `[W]`, `[Q]`, `[t]`, `[Y]`
- [Operators](../language/05-operators.md) - `|` pipeline operator
- [Enumerations](../language/03-enumerations.md) - Reserved enumerations

### Planning
- [Reserved Enumeration Schemas](../reserved-enumeration-schema-decisions.md) - Pending schemas

---

**End of Standard Library Overview**