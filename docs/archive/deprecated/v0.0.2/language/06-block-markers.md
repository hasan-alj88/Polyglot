---
status: deprecated
archived: 2026-03-22
source_branch: main
note: Superseded by docs/user/ and docs/technical/ in v0.0.4 documentation rewrite
replaced_by: docs/user/syntax/blocks.md
---
<!-- @d:user/syntax/blocks -->
> **Deprecated:** This document is superseded. See the current spec for up-to-date content.

# Block Markers

**Version:** 0.0.2
**Last Updated:** 2025-11-11
**Status:** Complete

---

## Overview

Block markers are special syntax elements enclosed in square brackets `[ ]` that define the structure and behavior of Polyglot code. Every line of valid Polyglot code must start with a block marker.

**Key Characteristics:**
- Enclosed in square brackets: `[marker]`
- Case-sensitive: `[r]` ≠ `[R]`
- Every valid code line starts with a block marker
- Define code structure and execution flow
- Cannot be combined or nested within brackets

---

## Table of Contents

1. [Block Marker Fundamentals](#block-marker-fundamentals)
2. [Pipeline Structure Markers](#pipeline-structure-markers)
3. [Input/Output Markers](#inputoutput-markers)
4. [Execution Control Markers](#execution-control-markers)
5. [Data Flow Markers](#data-flow-markers)
6. [Synchronization Markers](#synchronization-markers)
7. [Trigger & Queue Markers](#trigger--queue-markers)
8. [Runtime Wrapper Markers](#runtime-wrapper-markers)
9. [Type Definition Markers](#type-definition-markers)
10. [Error Handling Markers](#error-handling-markers)
11. [Expansion & Nesting Markers](#expansion--nesting-markers)
12. [Additional Markers](#additional-markers)
13. [Block Element Hierarchy](#block-element-hierarchy)
14. [Best Practices](#best-practices)

---

## Block Marker Fundamentals

### What are Block Markers?

<!-- @u:user/syntax/blocks#block-markers -->
**Block markers** are syntax elements that:
- Define the type of operation or declaration
- Start every valid line of Polyglot code
- Control execution flow
- Provide structure to pipelines

---

### Universal Rule

**Every line of valid Polyglot code MUST start with a block marker.**

```polyglot
// ✓ VALID - All lines start with block markers
[|] MyPipeline
[i] .input: pg\string
[r] |ProcessData
[<] .data: pg\string << .input
[X]

// ✗ INVALID - Missing block marker
[|] MyPipeline
.input: pg\string  // Missing [i]
|ProcessData       // Missing [r]
[X]
```

---

### Case Sensitivity

**Critical Rule:** Block markers are case-sensitive.

```polyglot
// Different meanings
[r]  // Run sequential (lowercase)
[R]  // NOT VALID - case matters

[i]  // Input declaration (lowercase)
[I]  // NOT VALID - case matters

[Q]  // Queue control (uppercase)
[q]  // NOT VALID - must be uppercase
```

---

### Square Bracket Syntax

Block markers use square brackets `[ ]` with the marker inside:

```polyglot
[marker]  // General format

[|]  // Pipeline definition
[r]  // Run sequential
[<]  // Pass input / Define field
[>]  // Pass output
```

---

## Pipeline Structure Markers

### `[|]` - Pipeline Definition

**Purpose:** Defines a pipeline (fundamental unit of execution)

**Must be paired with:** `[X]` to close

**Syntax:**
```polyglot
[|] PipelineName
// ... pipeline contents ...
[X]
```

**Example:**
```polyglot
[|] ProcessData
[i] .input: pg\string
[r] |TransformData
[<] .data: pg\string << .input
[>] .result: pg\string >> output
[X]
```

---

### `[X]` - End Marker

**Purpose:** Closes pipeline, enumeration, or error definitions

**Closes:** `[|]`, `[#]`, `[!]` blocks

**Syntax:**
```polyglot
[X]
```

**Examples:**
```polyglot
// Close pipeline
[|] MyPipeline
[X]

// Close enumeration
[#] MyEnumeration
[<] .field: pg\string << "value"
[X]

// Close error definition
[!] !MyError
[<] .message: pg\string << "Error"
[<] .code: pg\int << 5000
[<] .trace: pg\string << ""
[X]
```

---

## Input/Output Markers

### `[i]` - Input Declaration

**Purpose:** Declares pipeline inputs

**Three forms:**
1. **Required** - Caller MUST provide
2. **Fixed** - Caller CANNOT override (constant)
3. **Default** - Caller CAN override (optional)

---

#### Required Input

```polyglot
[i] .parameter: type
```

**Example:**
```polyglot
[|] ProcessFile
[i] .file_path: pg\path  // Required - caller must provide
[r] |ReadFile
[<] .path: pg\path << .file_path
[X]
```

---

#### Fixed Input (Constant)

```polyglot
[i] Fixed .parameter: type << value
```

**Example:**
```polyglot
[|] ConnectToAPI
[i] Fixed .api_key: pg\string << "secret-key-123"  // Cannot be overridden
[i] Fixed .timeout: pg\int << 30
[r] |MakeAPICall
[<] .key: pg\string << .api_key
[X]
```

---

#### Default Input (Optional)

```polyglot
[i] Default .parameter: type << value
```

**Example:**
```polyglot
[|] ProcessWithOptions
[i] .input: pg\string  // Required
[i] Default .max_size: pg\int << 1024  // Optional, defaults to 1024
[i] Default .debug: pg\bool << False   // Optional, defaults to False

[r] |Process
[<] .data: pg\string << .input
[<] .size: pg\int << .max_size
[X]
```

---

### `[o]` - Output Declaration

**Purpose:** Declares pipeline outputs

**Syntax:**
```polyglot
[o] .output: type
[o] #None  // No output
```

**Examples:**
```polyglot
// Pipeline with output
[|] Transform
[i] .input: pg\string
[r] |ProcessData
[<] .data: pg\string << .input
[>] .result: pg\string >> output
[o] .result: pg\string
[X]

// Pipeline with no output
[|] LogMessage
[i] .message: pg\string
[r] |U.Log.Info
[<] .msg: pg\string << .message
[o] #None
[X]
```

---

## Execution Control Markers

### `[r]` - Run Sequential

**Purpose:** Executes operations sequentially (one after another)

**Syntax:**
```polyglot
[r] |PipelineName
[r] .variable: type << value
```

**Examples:**
```polyglot
// Sequential pipeline calls
[r] |Step1
[r] |Step2
[r] |Step3

// Sequential variable assignments
[r] .x: pg\int << 5
[r] .y: pg\int << 10
[r] .sum: pg\int << .x + .y
```

---

### `[p]` - Parallel Execution

**Purpose:** Executes as mini-pipeline in parallel with other `[p]` blocks

**Characteristics:**
- Copy-in semantics (implicit input from outer scope)
- Explicit copy-out with `[>]` and `>>`
- Independent execution
- Use `[Y]` join to synchronize results

**Syntax:**
```polyglot
[p] |PipelineName
[<] .input: type << value
[>] .output >> variable
```

**Example:**
```polyglot
[|] ProcessInParallel
[i] .data: pg\string

// Parallel block 1
[p] |ProcessPartA
[<] .input: pg\string << .data
[>] .result >> result_a

// Parallel block 2
[p] |ProcessPartB
[<] .input: pg\string << .data
[>] .result >> result_b

// Join results
[Y] |Y.Join
[>] result_a
[>] result_b

// Use synchronized results
[r] |CombineResults
[<] .a: pg\string << result_a
[<] .b: pg\string << result_b

[X]
```

---

## Data Flow Markers

### `[<]` - Pass Input / Define Field (Dual Purpose)

**Purpose 1:** Pass input to pipeline calls

**Syntax:**
```polyglot
[<] .parameter: type << value
```

**Example:**
```polyglot
[r] |ProcessData
[<] .input: pg\string << "value"
[<] .max_size: pg\int << 1024
```

---

**Purpose 2:** Define fields in enumerations/errors

**Syntax:**
```polyglot
[<] .field: type << value
```

**Example:**
```polyglot
[#] Configuration
[<] .host: pg\string << "localhost"
[<] .port: pg\int << 8080
[X]

[!] !CustomError
[<] .message: pg\string << "Error"
[<] .code: pg\int << 5000
[<] .trace: pg\string << ""
[X]
```

---

### `[>]` - Pass Output

**Purpose:** Extracts/pulls values FROM source

**Syntax:**
```polyglot
[>] .field: type >> variable
```

**Examples:**
```polyglot
// Extract from pipeline output
[r] |ProcessData
[>] .result: pg\string >> output_var

// Extract from error fields
[!] !pg.FileSystem.NotFound
[>] .message: pg\string >> err_msg
[>] .code: pg\int >> err_code

// Copy out from parallel block
[p] |ProcessInParallel
[>] .output >> result
```

---

## Synchronization Markers

### `[Y]` - Join Block

**Purpose:** Synchronizes variables from parallel scopes to outer scope

**Always paired with:** `|Y.Join` pipeline

**Syntax:**
```polyglot
[Y] |Y.Join
[>] variable1
[>] variable2
```

**Example:**
```polyglot
[|] ParallelWorkflow
[i] .input: pg\string

// Initialize result variables
[r] .result1: pg\string << ""
[r] .result2: pg\string << ""

// Parallel block 1
[p] |ProcessPartA
[<] .data: pg\string << .input
[>] .output >> result1

// Parallel block 2
[p] |ProcessPartB
[<] .data: pg\string << .input
[>] .output >> result2

// Join - synchronize results
[Y] |Y.Join
[>] result1
[>] result2

// After join, result1 and result2 are synchronized
[r] |CombineResults
[<] .a: pg\string << result1
[<] .b: pg\string << result2

[X]
```

**Key Point:** Use `[>]` in join blocks (not `[<]`) because we're pulling/extracting FROM parallel scopes.

---

## Trigger & Queue Markers

<!-- @c:reference/glossary#Trigger Monitor -->
### `[t]` - Trigger

**Purpose:** Defines when pipeline should activate

**Types:**
- Time-based triggers
- File-based triggers
- Event-based triggers

**Syntax:**
```polyglot
[t] |T.TriggerType
[<] .parameter: type << value
```

---

#### Time-Based Triggers

```polyglot
// Daily trigger
[t] |T.Daily
[<] .time: pg\dt << DT"09:00:"

// Every minute
[t] |T.Every.Minute

// Every hour
[t] |T.Every.Hour
[<] .minute: pg\int << 0

// Custom interval
[t] |T.Every.Seconds
[<] .interval: pg\int << 30
```

---

#### File-Based Triggers

```polyglot
// File modified
[t] |T.File.Modified
[<] .path: pg\path << \\DataDir\\file.txt

// File created
[t] |T.File.Created
[<] .path: pg\path << \\DataDir\\

// File deleted
[t] |T.File.Deleted
[<] .path: pg\path << \\DataDir\\file.txt
```

---

#### Event-Based Triggers

```polyglot
// Custom event trigger (API TBD)
[t] |T.Event
[<] .event_name: pg\string << "user.registered"
```

---

<!-- @c:reference/glossary#Queue Manager -->
### `[Q]` - Queue Control

**Purpose:** Controls queue operations within pipeline

**Note:** Uppercase `Q` (case-sensitive)

**Syntax:**
```polyglot
[Q] |Q.Operation
[<] .parameter: type << value
```

---

#### Queue Operations

```polyglot
// Pause pipeline
[Q] |Q.Pause

// Resume pipeline
[Q] |Q.Resume

// Kill pipeline
[Q] |Q.Kill

// Priority bump
[Q] |Q.PriorityBump

// Assign to queue
[Q] |Q.Queue.Assign
[<] .queue: #Queues << #Queues.Background
```

---

#### Conditional Queue Control

```polyglot
// Pause if condition
[Q] |Q.PauseIf.RAM.Available.LessThan
[<] .mb: pg\uint << 512

// Dispatch with priority
[Q] |Q.Dispatch.Priority.High
```

---

## Runtime Wrapper Markers

### `[w]` - Wrapper Context

**Purpose:** Establishes runtime environment for executing code in other languages

**Supported Runtimes:**
- Python (various versions)
- Node.js (various versions)
- Rust
- Go
- Ruby
- Deno

**Syntax:**
```polyglot
[w] |W.Runtime
[w] |W.Runtime.Version
```

---

#### Fixed Version Wrappers

```polyglot
// Fixed Python version
[w] |W.Python3.11
[r] |RunPythonScript
[<] .script: pg\path << "analyze.py"

// Fixed Node version
[w] |W.Node20
[r] |RunJavaScript
[<] .script: pg\path << "process.js"

// Rust
[w] |W.Rust
[r] |RunRustCode
[<] .binary: pg\path << "processor"
```

---

#### Dynamic Version Wrappers

```polyglot
// Latest Python 3.x
[w] |W.Python
[r] |RunScript
[<] .script: pg\path << "script.py"

// Latest Node.js
[w] |W.Node
[r] |RunScript
[<] .script: pg\path << "app.js"
```

---

#### Multiple Wrappers in One Pipeline

```polyglot
[|] MultiRuntimePipeline
[i] .data: pg\string

// Python processing
[w] |W.Python3.11
[r] |PythonAnalyze
[<] .input: pg\string << .data
[>] .result: pg\string >> python_result

// Node.js processing
[w] |W.Node20
[r] |NodeTransform
[<] .input: pg\string << python_result
[>] .result: pg\string >> final_result

[X]
```

---

## Type Definition Markers

### `[#]` - Enumeration Definition

**Purpose:** Defines enumerations (immutable data structures)

**Must be paired with:** `[X]` to close

**Syntax:**
```polyglot
[#] EnumerationName
[<] .field: type << value
[X]
```

**Example:**
```polyglot
[#] AppConfiguration
[<] .host: pg\string << "localhost"
[<] .port: pg\int << 8080
[<] .debug: pg\bool << False
[X]
```

---

### `[!]` - Error Definition

**Purpose:** Defines custom error types

**Must be paired with:** `[X]` to close

**Must include:** Three reserved fields (`.message`, `.code`, `.trace`)

**Syntax:**
```polyglot
[!] !ErrorName
[<] .message: pg\string << "message"
[<] .code: pg\int << code
[<] .trace: pg\string << ""
[<] .custom_field: type << value  // Optional
[X]
```

**Example:**
```polyglot
[!] !MyApp.ValidationError
[<] .message: pg\string << "Validation failed"
[<] .code: pg\int << 4000
[<] .trace: pg\string << ""
[<] .field_name: pg\string << ""
[<] .invalid_value: pg\string << ""
[X]
```

---

### `[A]` - Alias Definition

**Purpose:** Creates package-scoped alias for enumerations or errors

**Syntax:**
```polyglot
[A] AliasName
```

**Examples:**
```polyglot
// Enumeration alias
[#] Path.Identifiers.MyApp.DataDirectory
[A] DataDir
[<] .unix: pg\path << \\UnixRoot\\opt\data\
[<] .windows: pg\path << \\C\\Data\
[X]

// Use alias
[r] .file: pg\path << \\DataDir\\records.csv

// Error alias
[!] !MyApp.Authentication.InvalidCredentials
[A] !InvalidCreds
[<] .message: pg\string << "Invalid credentials"
[<] .code: pg\int << 4010
[<] .trace: pg\string << ""
[X]
```

---

## Error Handling Markers

<!-- @u:user/concepts/errors#error-handler -->
### `[!]` - Error Catching (Context-Dependent)

**Purpose:** Catches specific error types

**Note:** Same marker as error definition, but different context

**Syntax:**
```polyglot
[!] !ErrorType
[>] .field: type >> variable  // Optional extraction
```

**Example:**
```polyglot
[|] FileOperation
[i] .file_path: pg\path
[t] |T.Call
[w] |W.NoSetup.NoCleanup

[r] |ReadFile
[<] .path: pg\path << .file_path
[>] .content: pg\string >> .file_content
[~]
[~][!] !pg.FileSystem.NotFound
[~][>] .message: pg\string >> .err_msg
[~]
[~][r] |U.Log.Error
[~][<] .msg: pg\string << .err_msg
[~]
[~][!] !pg.FileSystem.PermissionDenied
[~][>] .message: pg\string >> .denied_msg
[~]
[~][r] |U.Log.Error
[~][<] .msg: pg\string << "Access denied: {.denied_msg}"

[o] .content: pg\string
[X]
```

---

## Expansion & Nesting Markers

### `[~]` - Expansion/Nesting Prefix

**Purpose:** Indicates operation runs WITHIN parent context

**When to use:**
- Nesting operations inside expanded contexts
- Operations inside parallel blocks
- Operations inside unpack iterations

**Syntax:**
```polyglot
[~][marker] ...
[~][~][marker] ...  // Two levels deep
```

---

#### Explicit Expansion

```polyglot
[p] |ParallelBlock
[<] .data: pg\string << input
[~][r] |NestedOperation         // [~] means: runs WITHIN parallel block
[~][<] .input: pg\string << .data  // Child of [~][r] - implicit expansion
[~][>] .result >> temp
```

---

#### Multiple Nesting Levels

```polyglot
// Level 0 (outer scope)
[r] .items: pg\array{pg\string} << array{"a", "b", "c"}

// Level 1 - unpack array
[r] ~.items
[~][r] .item: pg\string << .items.current

// Level 2 - nested operation
[~][r] ~String.Split
[~][~][r] |ProcessToken              // WITHIN nested expansion
[~][~][<] .token: pg\string << .current_token
```

---

#### When NOT to Use `[~]`

**Implicit expansion** happens automatically for child operations:

```polyglot
// No [~] needed - implicit expansion
[r] |SomeOperation
[<] .input: pg\string << "value"    // Implicit child of [r]
[>] .output >> result               // Implicit child of [r]
```

---

## Additional Markers

### `[\]` - Setup Block (If Confirmed)

**Purpose:** Runs before pipeline execution (initialization)

**Status:** Pending confirmation

**Syntax:**
```polyglot
[\] |SetupOperation
```

---

### `[/]` - Cleanup Block (If Confirmed)

**Purpose:** Runs after pipeline execution (cleanup)

**Status:** Pending confirmation

**Syntax:**
```polyglot
[/] |CleanupOperation
```

---

### `[b]` - Batch Processing (If Confirmed)

**Purpose:** Batch processing operations

**Status:** Pending confirmation, details TBD

**Syntax:**
```polyglot
[b] |BatchOperation
```

---

## Block Element Hierarchy

### Parent-Child Relationships

Block markers have hierarchical relationships with **implicit expansion** built-in.

---

### Top-Level Parent: `[|]` Pipeline

**`[|]` is parent of:**
- `[i]` Input declaration
- `[o]` Output declaration
- `[t]` Trigger
- `[Q]` Queue control
- `[r]` Run/operation
- `[p]` Parallel execution
- `[w]` Wrapper context
- `[b]` Batch processing
- `[\]` Setup block
- `[/]` Cleanup block

```polyglot
[|] MyPipeline          // Parent
[i] .input: pg\string   // Child (implicit expansion)
[t] |T.Daily            // Child (implicit expansion)
[r] |Operation          // Child (implicit expansion)
[X]
```

---

### Operation Parents

**Any block with operation/pipeline call is parent of:**
- `[<]` Input assignment (push INTO)
- `[>]` Output assignment (pull FROM)

```polyglot
[r] |SomeOperation      // Parent
[<] .input << value     // Child (implicit expansion)
[>] .output >> result   // Child (implicit expansion)
```

---

### Explicit vs Implicit Expansion

**Implicit** - No `[~]` needed (automatic):
```polyglot
[r] |Operation
[<] .input << value    // Implicit - child of [r]
```

**Explicit** - `[~]` required (manual):
```polyglot
[p] |Parallel
[~][r] |Nested         // Explicit - WITHIN parallel context
[~][<] .input << value // Implicit - child of [~][r]
```

---

### Hierarchy Visual

```
[|] Pipeline
├── [i] Input (implicit)
├── [o] Output (implicit)
├── [t] Trigger (implicit)
│   ├── [<] Trigger params (implicit)
│   └── [>] Trigger outputs (implicit)
├── [Q] Queue control (implicit)
│   └── [<] Queue params (implicit)
├── [r] Run operation (implicit)
│   ├── [<] Operation input (implicit)
│   └── [>] Operation output (implicit)
├── [p] Parallel (implicit)
│   ├── [<] Parallel input (implicit)
│   ├── [>] Parallel output (implicit)
│   └── [~][r] Nested operation (explicit)
│       ├── [~][<] Nested input (implicit, but within [~] context)
│       └── [~][>] Nested output (implicit, but within [~] context)
└── [X] End marker (implicit)
```

---

## Best Practices

### 1. Always Start Lines with Block Markers

```polyglot
// ✓ CORRECT
[|] Pipeline
[i] .input: pg\string
[r] |Operation
[X]

// ✗ WRONG - Missing block markers
Pipeline
.input: pg\string
|Operation
```

---

### 2. Respect Case Sensitivity

```polyglot
// ✓ CORRECT
[r] |Run
[Q] |Q.Pause  // Uppercase Q

// ✗ WRONG
[R] |Run      // Wrong case
[q] |Q.Pause  // Wrong case
```

---

### 3. Always Close Definitions

```polyglot
// ✓ CORRECT - Closed with [X]
[|] Pipeline
[X]

[#] Enumeration
[X]

// ✗ WRONG - Missing [X]
[|] Pipeline

[#] Enumeration
```

---

### 4. Use Appropriate Input Types

```polyglot
// ✓ CORRECT - Clear intent
[i] .file: pg\path  // Required
[i] Fixed .api_key: pg\string << "secret"  // Constant
[i] Default .timeout: pg\int << 30  // Optional

// ✗ AVOID - Unclear requirements
[i] .file: pg\path  // Is this required?
[i] .api_key: pg\string  // Should this be fixed?
```

---

### 5. Use `[~]` Only When Needed

```polyglot
// ✓ CORRECT - Implicit expansion
[r] |Operation
[<] .input << value

// ✗ UNNECESSARY - Don't use [~] for implicit children
[r] |Operation
[~][<] .input << value  // [~] not needed here
```

---

### 6. Consistent Indentation

```polyglot
// ✓ CORRECT - Clear hierarchy
[|] Pipeline
[i] .input: pg\string
[r] |Operation
[<] .data: pg\string << .input
[>] .result: pg\string >> output
[X]

// ✗ HARDER TO READ - Inconsistent indentation
[|] Pipeline
    [i] .input: pg\string
[r] |Operation
    [<] .data: pg\string << .input
        [>] .result: pg\string >> output
[X]
```

---

### 7. Group Related Operations

```polyglot
// ✓ CORRECT - Grouped by purpose
[|] ProcessData
// Inputs
[i] .file: pg\path
[i] Default .max_size: pg\int << 1024

// Triggers
[t] |T.File.Modified
[<] .path: pg\path << .file

// Operations
[r] |ReadFile
[<] .path: pg\path << .file
[>] .content: pg\string >> data

[r] |ProcessContent
[<] .input: pg\string << data
[>] .result: pg\string >> output

[X]
```

---

### 8. Use Descriptive Pipeline Names

```polyglot
// ✓ CORRECT
[|] ProcessUserRegistration
[|] ValidateEmailAddress
[|] SendWelcomeEmail

// ✗ AVOID
[|] Process
[|] Validate
[|] Send
```

---

## See Also

### Language Specification
- [Complete Syntax Reference](01-syntax-complete.md) - All block markers overview
- [Operators](05-operators.md) - Operators used with block markers
- [Parallel Execution](08-parallel-execution.md) - `[p]` and `[Y]` details
- [Expansion Operator](09-expansion-operator.md) - `[~]` detailed usage

### Type Definitions
- [Enumerations](03-enumerations.md) - `[#]` enumeration blocks
- [Error Handling](04-error-handling.md) - `[!]` error blocks

### Standard Library
- [Runtime Wrappers](../standard-library/01-runtime-wrappers.md) - `[w]` wrapper usage
- [Queue Control](../standard-library/02-queue-control.md) - `[Q]` queue operations
- [Triggers](../standard-library/04-triggers.md) - `[t]` trigger types

### Examples
- [Hello World](../examples/hello-world.md) - Basic block marker usage
- [Complete Workflows](../examples/complete-workflows.md) - Complex block patterns

### Planning
- [Decision Log](../decision-log.md) - Block marker decisions (#7, #15)

---

**End of Block Markers Reference**