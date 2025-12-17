# Complete Syntax Reference

**Version:** 0.0.2
**Last Updated:** 2025-11-18
**Status:** Complete

---

## Overview

This document provides a complete reference for all Polyglot syntax elements. It serves as the canonical syntax guide for v0.0.2, resolving all inconsistencies from v0.0.1.

**Polyglot** is an asynchronous automation language designed for orchestrating complex workflows across multiple runtime environments.

---

## Table of Contents

1. [Fundamental Concepts](#fundamental-concepts)
2. [Polyglot Formatting Guide (PFG)](#polyglot-formatting-guide-pfg)
3. [Block Markers](#block-markers)
4. [Operators](#operators)
5. [Assignment Direction](#assignment-direction)
6. [Type Syntax](#type-syntax)
7. [Comments](#comments)
8. [Pipeline Structure](#pipeline-structure)
9. [Block Element Hierarchy](#block-element-hierarchy)
10. [Terminology](#terminology)
11. [Quick Reference Tables](#quick-reference-tables)

---

## Fundamental Concepts

### Pipelines are the Foundation

**Pipeline** - The fundamental unit of execution in Polyglot. Like a function or blackbox with defined inputs and outputs.

```polyglot
[|] PipelineName
[i] .input: pg\string
[r] |ProcessData
[<] .data: pg\string << .input
[X]
```

### All Valid Code Starts with Block Elements

**Critical Rule:** Every line of valid Polyglot code MUST start with a block element.

```polyglot
// ✓ VALID - starts with block element
[r] .x: pg\int << 5

// ✗ INVALID - no block element
.x: pg\int << 5
```

### Asynchronous by Nature

All Polyglot pipelines are inherently asynchronous. They execute in response to triggers and events, managed by the queue system.

### Variable Naming - Universal Rule

**CRITICAL RULE:** ALL variables in Polyglot ALWAYS begin with a dot (`.`)

This rule applies universally to:
- **Declarations**: `.name: pg\string << "Alice"`
- **References**: `[<] .input << .name`
- **Inputs**: `[i] .input: pg\string`
- **Outputs**: `[o] .result: pg\string`
- **Input constants**: `[i] .max: pg\int << 3`
- **Macro scope**: `[{] .input: pg\string`, `[}] .output: pg\string`
- **All other contexts**: Always use dot prefix

```polyglot
// ✓ CORRECT - All variables with dot prefix
[r] .counter: pg\int << 0
[r] .name: pg\string << "Alice"
[<] .input << .name

// ✗ INVALID - Missing dot prefix
[r] counter: pg\int << 0     // WRONG
[r] name: pg\string << "Alice"  // WRONG
[<] input << name            // WRONG
```

**Why the dot prefix?**
- Distinguishes variables from other identifiers (pipelines, types, enumerations)
- Enables clear string interpolation: `"Hello {.name}"`
- Provides consistent, unambiguous syntax across all contexts

---

## Polyglot Formatting Guide (PFG)

### File Structure Spacing

**Critical Rule:** Three blank lines MUST precede all file-scope definitions

File-scope definitions include:
- `[|]` Pipeline definitions
- `[#]` Enumeration definitions
- `[!]` Error definitions
- `[M]` Macro definitions

**Why 3 blank lines?**
- Provides clear visual separation between definitions
- Makes file structure immediately scannable
- Prevents definitions from blending together
- Maintains consistency across all Polyglot files

---

### Correct Formatting

```polyglot
[@] Local@MyApp.Example:1.0.0
[#] 1
[X]



[#] Configuration
[<] .host: pg\string << "localhost"
[<] .port: pg\int << 8080
[X]



[!] !ValidationError
[<] .message: pg\string << "Validation failed"
[<] .code: pg\int << 4000
[<] .trace: pg\string << ""
[X]



[|] ProcessData
[i] .input: pg\string
[r] |TransformData
[<] .data: pg\string << .input
[X]
```

---

### Incorrect Formatting

```polyglot
[@] Local@MyApp.Example:1.0.0
[#] 1
[X]
[#] Configuration  // ✗ WRONG - Missing 3 blank lines
[<] .host: pg\string << "localhost"
[X]
[!] !ValidationError  // ✗ WRONG - Missing 3 blank lines
[<] .message: pg\string << "Error"
[X]
```

---

### Within-Definition Spacing

**No special spacing rules within definitions:**

```polyglot
[|] MyPipeline
[i] .input: pg\string
[i] .count: pg\int
[t] |T.Daily
[r] |ProcessData
[<] .data: pg\string << .input
[>] .result: pg\string >> output
[o] .result: pg\string
[X]
```

**Guidelines:**
- Single blank lines (optional) can group related operations
- No blank lines required between block markers
- Consistency within a file is more important than strict rules

---

### Package Declaration Spacing

**Exception:** The `[@]` package declaration is always first with NO preceding blank lines:

```polyglot
[@] Local@MyApp.Example:1.0.0
[#] 1
[<] @Community.utils@FileHelpers:2.0.0
[X]



[|] FirstPipeline  // 3 blank lines after [@] block closing
[X]
```

---

### Summary

| Context | Spacing Rule |
|---------|-------------|
| Before file-scope definitions | **3 blank lines** (mandatory) |
| Within definitions | No special rules (optional blank lines for grouping) |
| After `[@]` block | **3 blank lines** before first definition |
| Between block markers | No blank lines required |

---

## Block Markers

Block markers are special syntax elements enclosed in square brackets `[ ]` that define the structure and behavior of Polyglot code.

### Case Sensitivity

**Important:** Block markers are case-sensitive. `[r]` ≠ `[R]`, `[i]` ≠ `[I]`

### Complete Block Marker List

#### Pipeline Structure

**`[|]` - Pipeline Definition**
- Defines a pipeline (the fundamental unit)
- Must be paired with `[X]` to close

```polyglot
[|] MyPipeline
// ... pipeline contents ...
[X]
```

**`[X]` - End Marker**
- Closes pipeline, enumeration, or error definitions
- Marks the end of a block

```polyglot
[|] Pipeline
[X]

[#] Enumeration
[X]

[!] !Error
[X]
```

---

#### Input/Output

**`[i]` - Input Declaration**
- Declares pipeline inputs
- Three forms: required, fixed, default

```polyglot
// Required input - caller MUST provide
[i] .file_path: pg\path

// Fixed constant - caller CANNOT override
[i] .api_key: pg\string << "secret-123"

// Optional with default - caller CAN override
[i] .timeout: pg\int <~ 30
```

**`[o]` - Output Declaration**
- Declares pipeline outputs (if needed)
- Defines what the pipeline returns

```polyglot
[o] .result: pg\string
[o] #None  // No output
```

---

#### Execution Control

**`[r]` - Run Sequential**
- Executes operations sequentially
- One after another, in order

```polyglot
[r] |FirstOperation
[r] |SecondOperation
[r] |ThirdOperation
```

**`[p]` - Parallel Execution**
- Executes as mini-pipeline in parallel
- Copy-in semantics, explicit copy-out

```polyglot
[p] |ProcessPartA
[<] .data: pg\string << input_data
[>] .output >> result1

[p] |ProcessPartB
[<] .data: pg\string << input_data
[>] .output >> result2
```

---

#### Data Flow

**`[<]` - Passing Input (Dual Purpose)**

Purpose 1: Input to pipeline calls
```polyglot
[r] |SomePipeline
[<] .param: pg\string << "value"
```

Purpose 2: Field definition in enumerations/errors
```polyglot
[#] MyEnumeration
[<] .field: pg\string << "value"
[X]
```

**`[>]` - Passing Output**
- Extracts/pulls values FROM source
- Used for output from operations

```polyglot
[r] |SomePipeline
[>] .result: pg\string >> variable_name
```

---

#### Synchronization

**`[Y]` - Join Block**
- Synchronizes variables from parallel scopes
- Lists variables to pull into outer scope

```polyglot
[Y] |Y.Join
[>] result1
[>] result2
```

---

#### Triggers & Queues

**`[t]` - Trigger**
- Defines when pipeline should activate
- Time-based, file-based, event-based

```polyglot
[t] |T.Daily
[<] .time: pg\dt << DT"12:30:"

[t] |T.Every.Minute

[t] |T.File.Modified
[<] .path: pg\path << \\DataDir\\file.txt
```

**`[Q]` - Queue Control**
- Controls queue operations within pipeline
- Pause, resume, priority, assignment

```polyglot
[Q] |Q.PauseIf.RAM.Available.LessThan
[<] .mb: pg\uint << 2048

[Q] |Q.Dispatch.Priority.High
```

---

#### Runtime Wrappers

**`[W]` - Wrapper Context**
- Establishes runtime environment
- For Python, Node, Rust, etc.

```polyglot
[W] |W.Python3.11
[r] |RunPythonScript
[<] .script: pg\path << "analyze.py"
```

---

#### Type Definitions

**`[#]` - Enumeration Definition**
- Defines enumerations (immutable data structures)
- Can extend reserved enumerations

```polyglot
[#] MyApp.Configuration
[<] .host: pg\string << "localhost"
[<] .port: pg\int << 8080
[X]
```

**`[!]` - Error Definition**
- Defines custom error types
- Must include three reserved fields

```polyglot
[!] !MyApp.CustomError
[<] .message: pg\string << "Error occurred"
[<] .code: pg\int << 5000
[<] .trace: pg\string << ""
[X]
```

**`[A]` - Alias Definition**
- Creates alias for enumerations or errors
- Package-scoped only

```polyglot
[#] Path.Identifiers.MyApp.DataDirectory
[A] DataDir  // Use as \\DataDir\\
[<] .unix: pg\path << \\UnixRoot\\opt\data\
[<] .windows: pg\path << \\C\\Data\
[X]
```

---

#### Error Handling

**`[!]` - Error Catching** (context-dependent)
- Catches specific error types
- Can extract error fields

```polyglot
[r] |MightFail
[!] !pg.FileSystem.NotFound
[>] .message: pg\string >> err_msg
[r] |HandleError
[<] .msg: pg\string << err_msg
```

---

#### Expansion & Nesting

**`[~]` - Expansion/Nesting Prefix**
- Indicates operation runs WITHIN parent context
- Required when nesting operations

```polyglot
[p] |ParallelBlock
[<] .data: pg\string << input
[~][r] |NestedOperation     // Runs WITHIN parallel block
[~][<] .input: pg\string << .data
```

---

#### Additional Blocks

#### Macro System

**`[M]` - Macro Definition**
- Defines compile-time inline code templates
- Must be paired with `[X]`

```polyglot
[M] MacroName
[<] Macro.include"{\/"
[X]
```

**`[W]` - Macro Unwrap**
- Inlines macro at unwrap site
- Compile-time insertion

```polyglot
[W] |MacroName
```

**`[{]` - Scope Input**
- Variables flowing INTO macro from caller
- Used in macro definitions

```polyglot
[{] .input_var: pg\type
```

**`[}]` - Scope Output**
- Variables flowing OUT of macro to caller
- Used in macro definitions

```polyglot
[}] .output_var: pg\type
```

**`[i]` with `<<` - Constant Input**
- Constant input value (immediately Ready state)
- Cannot be overridden by caller

```polyglot
[i] .max_retries: pg\int << 3
```

**Note:** `[i]` supports three variations:
- `[i] .var: Type` - Required input (Declared, caller must provide)
- `[i] .var: Type <~ value` - Default input (DefaultReady, caller can override)
- `[i] .var: Type << value` - Constant input (Ready, caller cannot override)

#### Boolean Logic

**`[&]` - AND Block**
- Logical AND - all conditions must be true
- Used in triggers and switches

**`[+]` - OR Block**
- Logical OR - any condition can be true
- Used in triggers and switches

**`[-]` - NOT Block**
- Logical negation
- Used in triggers and switches

**`[^]` - XOR Block**
- Exclusive OR - exactly one must be true
- Used in triggers and switches

**`[.]` - Grouping Block**
- Logical grouping for precedence
- Used in complex boolean expressions

#### String Processing

**`[*]` - Line Continuation**
- Syntactic line continuation for readability
- NOT semantic - purely for source layout

```polyglot
[*] "Long text "
[*] +" "continued here"
```

**Note:** `[*]` is line continuation, `[^]` is XOR operator (different purposes)

#### Setup & Cleanup

**`[\]` - Setup Block**
- Runs before pipeline execution
- Initialization logic
- Confirmed for macro system

**`[/]` - Cleanup Block**
- Runs after pipeline execution
- Cleanup logic
- Confirmed for macro system (LIFO order)

**`[b]` - Background Execution**
- Parallel fire-and-forget execution
- Non-blocking background operations

---

### Block Marker Summary Table

| Marker | Purpose | Paired With | Case-Sensitive |
|--------|---------|-------------|----------------|
| `[|]` | Pipeline definition | `[X]` | ✓ |
| `[X]` | End marker | `[|]`, `[#]`, `[!]`, `[M]` | ✓ |
| `[i]` | Input declaration | - | ✓ |
| `[o]` | Output declaration | - | ✓ |
| `[r]` | Run sequential | - | ✓ |
| `[p]` | Parallel execution | - | ✓ |
| `[<]` | Pass input / Define field | - | ✓ |
| `[>]` | Pass output | - | ✓ |
| `[Y]` | Join block | `|Y.Join` | ✓ |
| `[t]` | Trigger | - | ✓ |
| `[Q]` | Queue control | - | ✓ (uppercase) |
| `[W]` | Wrapper context | - | ✓ |
| `[#]` | Enumeration definition | `[X]` | ✓ |
| `[!]` | Error def/catching | `[X]` or standalone | ✓ |
| `[A]` | Alias definition | - | ✓ |
| `[~]` | Expansion/nesting | - | ✓ |
| `[M]` | Macro definition | `[X]` | ✓ |
| `[W]` | Macro unwrap | - | ✓ |
| `[{]` | Scope input | - | ✓ |
| `[}]` | Scope output | - | ✓ |
| `[&]` | AND block | - | ✓ |
| `[+]` | OR block | - | ✓ |
| `[-]` | NOT block | - | ✓ |
| `[^]` | XOR block | - | ✓ |
| `[.]` | Grouping block | - | ✓ |
| `[*]` | Line continuation | - | ✓ |
| `[\]` | Setup block | - | ✓ |
| `[/]` | Cleanup block | - | ✓ |
| `[b]` | Background execution | - | ✓ |

---

## Operators

Operators are special symbols with specific semantic meanings in Polyglot.

### Pipeline Operator `|`

**Purpose:** Calls pipelines that are defined via `[|]`

**Usage:**
```polyglot
[r] |PipelineName
[r] |T.Daily
[r] |U.String.Format
[r] |Q.Pause
```

**Rules:**
- Always required when calling a pipeline
- Never combine with other operators (e.g., `|~` is invalid)

---

### Unpack Operator `~`

**Purpose:** Unpacks arrays, sets, and enumerations

**Usage:**
```polyglot
[r] ~myArray
[r] ~mySet
[r] ~MyEnumeration
[r] ~Array.ForEach
```

**Rules:**
- NOT a pipeline call - different operator
- Used for expanding collections and iterating

---

### Package Operator `@`

**Purpose:** Accesses pipelines and enumerations from other packages

**Usage:**
```polyglot
// Pipeline from package
[r] @packageName|PipelineName
[r] @Community.hasan@DataUtils|Transform

// Enumeration from package
[i] @packageName#EnumerationName
[i] @Company.acme@InternalLib#ErrorCodes
```

**Rules:**
- Combined with `|` for pipelines
- Combined with `#` for enumerations
- Three-tier registry: Local, Community, Company

---

### Enumeration Operator `#`

**Purpose:** Marks enumeration types and references

**Usage:**
```polyglot
// Define enumeration
[#] MyEnumeration
[X]

// Reference enumeration
[i] .config: #MyEnumeration << #MyEnumeration.Default

// Reserved enumerations
[i] .status: #Status << #Status.Success
```

---

### Error Type Operator `!`

**Purpose:** Marks error types

**Usage:**
```polyglot
// Define error
[!] !MyApp.CustomError
[X]

// Catch error
[!] !pg.FileSystem.NotFound

// Reference error type
[i] .error: !ErrorType
```

---

### Comparison Operators

**Question Mark Suffix Pattern:**
All comparison operators use `?` suffix for consistency

| Operator | Meaning | Example |
|----------|---------|---------|
| `>?` | Greater than | `.age >? 18` |
| `<?` | Less than | `.temp <? 0` |
| `=>?` | Greater or equal | `.count =>? 10` |
| `=<?` | Less or equal | `.score =<? 100` |
| `=?` | Equal | `.status =? "active"` |
| `=!?` | Not equal | `.value =!? 0` |

**Usage:** Primarily in `[?]` switch/conditional blocks

```polyglot
[?] .age =>? 18
[r] |ProcessAdult

[?] .age <? 18
[r] |ProcessMinor
```

---

### Range and Interval Operators

**Mathematical Interval Notation:**

| Operator | Meaning | Example |
|----------|---------|---------|
| `?[a, b]` | Closed (both inclusive) | `.age ?[18, 65]` |
| `?(a, b)` | Open (both exclusive) | `.score ?(0, 100)` |
| `?[a, b)` | Half-open (left inclusive) | `.value ?[0, 10)` |
| `?(a, b]` | Half-open (right inclusive) | `.value ?(0, 10]` |

**Usage:** Range checks in switch blocks

```polyglot
[?] .age ?[18, 65]
[r] |ProcessWorkingAge

[?] .age ?(0, 18)
[r] |ProcessMinor
```

---

### Pattern Matching Operators

**`*?` - Wildcard Match**
- Matches any value (catchall)
- Used in switch blocks

```polyglot
[?] .status *?
[r] |HandleAnyStatus
```

**`re?` - Regex Match**
- Matches against regex pattern
- String type only

```polyglot
[?] .email re? "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"
[r] |ProcessValidEmail
```

---

### String Operators

**`+"` - String Concatenation**
- Explicit concatenation of string literals
- Literals only (not variables)

```polyglot
[r] .msg: pg\string << "Hello" +" ", " +" "World!"
```

**`{.variable}` - String Interpolation**
- Embed variables in strings

```polyglot
[r] .greeting: pg\string << "Hello, {.name}!"
```

---

### Assignment Operators

**`<<` - Left/Push Assignment**
- Pushes value INTO variable
- Input direction

```polyglot
[<] .path: pg\path << "data.txt"
[r] .x: pg\int << 5
```

**`>>` - Right/Pull Assignment**
- Pulls value FROM source
- Output/extract direction

```polyglot
[>] .result: pg\string >> output_var
[>] .message: pg\string >> err_msg
```

**Visual Mnemonic:**
- `<<` arrows point left → data flows INTO variable (push)
- `>>` arrows point right → data flows FROM source (pull)

---

### Operator Summary Table

**Core Operators:**

| Operator | Name | Purpose | Example |
|----------|------|---------|---------|
| `|` | Pipeline | Call pipeline | `|ProcessData` |
| `~` | Unpack | Expand collection | `~Array.ForEach` |
| `@` | Package | Access from package | `@pkg|Pipeline` |
| `#` | Enumeration | Mark enumeration | `#MyEnum` |
| `!` | Error | Mark error type | `!CustomError` |
| `<<` | Push | Assign INTO variable | `.x << value` |
| `>>` | Pull | Extract FROM source | `.x >> output` |

**Comparison Operators** (? suffix pattern):

| Operator | Name | Purpose | Example |
|----------|------|---------|---------|
| `>?` | Greater than | Compare values | `.age >? 18` |
| `<?` | Less than | Compare values | `.temp <? 0` |
| `=>?` | Greater or equal | Compare values | `.count =>? 10` |
| `=<?` | Less or equal | Compare values | `.score =<? 100` |
| `=?` | Equal | Test equality | `.status =? "active"` |
| `=!?` | Not equal | Test inequality | `.value =!? 0` |

**Range/Interval Operators** (mathematical notation):

| Operator | Name | Purpose | Example |
|----------|------|---------|---------|
| `?[a, b]` | Closed interval | Both inclusive | `.age ?[18, 65]` |
| `?(a, b)` | Open interval | Both exclusive | `.score ?(0, 100)` |
| `?[a, b)` | Half-open | Left inclusive | `.value ?[0, 10)` |
| `?(a, b]` | Half-open | Right inclusive | `.value ?(0, 10]` |

**Pattern Matching Operators:**

| Operator | Name | Purpose | Example |
|----------|------|---------|---------|
| `*?` | Wildcard | Match any | `.value *?` (catchall) |
| `re?` | Regex | Match pattern | `.email re? "pattern"` |

**String Operators:**

| Operator | Name | Purpose | Example |
|----------|------|---------|---------|
| `+"` | Concatenate | Join literal strings | `"Hello" +" " +" "World"` |
| `{.var}` | Interpolate | Embed variable | `"Hello {.name}"` |

---

## Assignment Direction

Understanding assignment direction is critical in Polyglot.

### Push INTO with `<<`

Data flows INTO the variable (left side):

```polyglot
// Push literal INTO variable
[r] .x: pg\int << 5

// Push variable INTO parameter
[<] .input: pg\string << source_var

// Push enumeration INTO variable
[r] .status: #Status << #Status.Success
```

**Direction:** `value` → `<<` → `variable`

---

### Pull FROM with `>>`

Data flows FROM source (right side):

```polyglot
// Pull result FROM pipeline output
[>] .result: pg\string >> destination_var

// Pull field FROM error
[>] .message: pg\string >> err_msg

// Pull value FROM source
[>] .output >> result
```

**Direction:** `source` → `>>` → `variable`

---

### Why Two Operators?

**Semantic Clarity:**
- `<<` = Push INTO = Input direction = Giving data TO something
- `>>` = Pull FROM = Output direction = Taking data FROM something

**Context Determines Usage:**
- Input contexts use `<<` - pushing data into operations
- Output contexts use `>>` - pulling data from operations

---

## Type Syntax

### Type Separator: Backslash `\`

**Critical Rule:** The type separator is ALWAYS backslash `\`, never forward slash `/`

```polyglot
// ✓ CORRECT
pg\int
pg\string
pg\path

// ✗ WRONG
pg/int
pg/string
pg/path
```

---

### Basic Type Syntax

```polyglot
language\type
```

Examples:
```polyglot
pg\int       // Integer
pg\uint      // Unsigned integer
pg\string    // String
pg\bool      // Boolean
pg\path      // Path
pg\dt        // DateTime
```

---

### Mutable Types

```polyglot
language.mutable\type
```

Examples:
```polyglot
pg.mutable\int
pg.mutable\string
pg.mutable\path
```

---

### Collection Types

```polyglot
pg\array{element_type}
pg\set{element_type}
```

Examples:
```polyglot
pg\array{pg\string}
pg\set{pg\int}
pg\array{pg\path}
```

---

### Special Types

**`pg\serial`** - Serializable data structure
- Mutable schema (keys can change at runtime)
- Dynamic key-value pairs

**`pg\path`** - Path type with reserved fields
- `.unix: pg\path` - Unix/Linux/macOS path
- `.windows: pg\path` - Windows path

---

### Type Declaration Examples

```polyglot
// Variable declarations
[r] .count: pg\int << 0
[r] .name: pg\string << "Example"
[r] .is_valid: pg\bool << #True

// Input declarations
[i] .input_file: pg\path
[i] .items: pg\array{pg\string}

// Mutable types
[r] .counter: pg.mutable\int << 0
```

---

## Comments

### Single-Line Comments

Use double forward slash `//`:

```polyglot
// This is a single-line comment
[r] .x: pg\int << 5  // Comment at end of line
```

---

### Multi-Line Comments

Use C-style block comments `/* */`:

```polyglot
/*
 * This is a multi-line comment
 * that spans multiple lines
 * and can include detailed explanations
 */

[r] .config: pg\serial << serial{
  /* Configuration block
     with multi-line explanation */
[^]  "host": "localhost",
[^]  "port": 8080
[^]}
```

---

### Important Distinction

**Comments use forward slash `/`:**
- Single-line: `//`
- Multi-line: `/* */`

**Other syntax uses backslash `\`:**
- Type separator: `pg\int`
- Path identifiers: `\\DataDir\\`
- Enumeration aliases: `\\AliasName\\`

```polyglot
// This is a comment - uses forward slash
[r] .path: pg\path << \\DataDir\\file.txt  // Path identifier - uses backslash
```

---

## Pipeline Structure

### Minimal Pipeline

```polyglot
[|] MinimalPipeline
[X]
```

---

### Pipeline with Input

```polyglot
[|] ProcessData
[i] .input: pg\string
[r] |DoSomething
[<] .data: pg\string << .input
[X]
```

---

### Pipeline with Input and Output

```polyglot
[|] Transform
[i] .input: pg\string
[r] |ProcessData
[<] .data: pg\string << .input
[>] .result: pg\string >> output
[o] .result: pg\string
[X]
```

---

### Pipeline with Trigger

```polyglot
[|] ScheduledTask
[t] |T.Daily
[<] .time: pg\dt << DT"09:00:"
[r] |DoWork
[X]
```

---

### Complete Pipeline Example

```polyglot
[|] CompleteExample
// Input declarations
[i] .input_file: pg\path
[i] .timeout: pg\int <~ 30

// Trigger
[t] |T.File.Modified
[<] .path: pg\path << .input_file

// Queue control
[Q] |Q.PauseIf.RAM.Available.LessThan
[<] .mb: pg\uint << 512

// Sequential operations
[r] |ReadFile
[<] .path: pg\path << .input_file
[>] .content: pg\string >> file_data

[r] |ProcessContent
[<] .data: pg\string << file_data
[>] .result: pg\string >> processed

// Error handling
[!] !pg.FileSystem.NotFound
[>] .message: pg\string >> err_msg
[r] |U.Log.Error
[<] .msg: pg\string << err_msg

// Output
[o] .result: pg\string
[X]
```

---

## Block Element Hierarchy

### Parent-Child Relationships

All block elements have hierarchical relationships with **implicit expansion** built-in.

#### `[|]` Pipeline - Parent of:
- `[i]` Input declaration
- `[t]` Trigger
- `[Q]` Queue control
- `[r]` Run/operation
- `[p]` Parallel execution
- `[b]` Batch processing
- `[\]` Setup block
- `[/]` Cleanup block
- `[o]` Output declaration

#### Any Block with Pipeline Call - Parent of:
- `[<]` Input assignment (push INTO)
- `[>]` Output assignment (pull FROM)

---

### Implicit vs Explicit Expansion

**Implicit Expansion (No `[~]` needed):**

Parent-child relationships automatically expand:

```polyglot
[r] |SomeOperation
[<] .input: pg\string << "value"    // Implicit - no [~] needed
[>] .output >> result               // Implicit - no [~] needed
```

**Explicit Expansion (Use `[~]`):**

When adding operations WITHIN an expanded context:

```polyglot
[p] |ProcessPartA
[<] .data: pg\string << .shared_data
[~][r] |TransformData                   // [~] means: runs WITHIN parallel block
[~][<] .input: pg\string << .data       // Child of [~][r] - implicit
[~][r] |ValidateData                    // [~] means: runs WITHIN parallel block
[~][<] .value: pg\string << temp        // Child of [~][r] - implicit
```

---

### Nesting Depth

Each `[~]` adds one nesting level:
- `[~]` - One level deep (within parent)
- `[~][~]` - Two levels deep (within nested parent)
- `[~][~][~]` - Three levels deep

**Example:**
```polyglot
// Level 0 (outer scope)
[r] .data: pg\string << "input"

// Level 1 - operation expands
[r] ~Array.ForEach
[~][r] .item: pg\string

// Level 2 - nested expansion
[~][r] ~String.Split
[~][~][r] |ProcessToken              // WITHIN nested expansion
[~][~][<] .token: pg\string          // Child of [~][~][r] - implicit
```

---

## Terminology

Understanding Polyglot terminology is essential for clear communication.

### Pipeline

**Definition:** Individual unit of execution, like a function or blackbox.

**Characteristics:**
- Defined using `[|]...[X]`
- Has inputs and outputs
- Can be called from other pipelines
- Asynchronous by nature

**Usage:**
```polyglot
[|] MyPipeline
[i] .input: pg\string
[r] |ProcessData
[<] .data: pg\string << .input
[X]

// Call the pipeline
[r] |MyPipeline
[<] .input: pg\string << "value"
```

---

### Workflow

**Definition:** Collection of interconnected pipelines with triggers forming complete automation.

**Characteristics:**
- Multiple pipelines working together
- Includes triggers that activate pipelines
- Represents end-to-end automation process
- May span multiple files and packages

**Example:**
```polyglot
// Workflow: "Order Processing System"
// Consists of 3 interconnected pipelines

[|] ProcessNewOrders
[t] |T.Every.Minute
[r] |FetchOrders
[X]

[|] SendNotifications
[t] |T.Daily
[<] .time: pg\dt << DT"09:00:"
[r] |ComposeEmails
[X]

[|] UpdateInventory
[t] |T.File.Modified
[<] .path: pg\path << \\DataDir\\inventory.csv
[r] |SyncToDatabase
[X]
```

---

### Function

**Context-Dependent:**

**When discussing Polyglot:**
- "Function" = "Pipeline" (interchangeable)
- Prefer "pipeline" for clarity

**When discussing other languages:**
- "Function" refers to functions in Python, Rust, JavaScript, etc.
- Distinct from Polyglot pipelines

**Example:**
```polyglot
[W] |W.Python3.11
[r] |RunPythonScript
[<] .script: pg\path << "analyze.py"
```
"This pipeline calls Python functions defined in `analyze.py`."

---

### Pipeline Instance

**Definition:** Runtime instantiation of a pipeline definition, like an object from a class.

**Class/Object Analogy:**
- Pipeline definition = Class (template)
- Pipeline instance = Object (instantiation)

**Characteristics:**
- Has unique instance ID
- Has its own state and variables
- Independent from other instances
- Goes through lifecycle: created → queued → running → exit

**Example:**
```polyglot
// Definition (template)
[|] ProcessFile
[i] .filename: pg\path
[r] |ReadFile
[<] .path: pg\path << .filename
[X]

// Create 3 instances
[r] |ProcessFile
[<] .filename: pg\path << "file1.csv"  // Instance #1

[r] |ProcessFile
[<] .filename: pg\path << "file2.csv"  // Instance #2

[r] |ProcessFile
[<] .filename: pg\path << "file3.csv"  // Instance #3
```

---

### Execution vs Running

**Execution:**
- The act of actively running code
- Occurs when instance is in Dispatch queue
- Can be paused

**Running:**
- Broad state including execution AND paused
- Starts when dispatched, ends when exits
- Running = from dispatch until exit

**Example:**
```polyglot
// Instance is created → goes to Pending queue
[r] |LongProcess

// Dispatched → "Running" starts
// Actively executing → "Execution" happening

// Paused → still "Running" (not exited)
[r] |Q.Pause
[<] .pipeline_id: pg\string << instance_id

// Resumed → still "Running", execution continues

// Completes → "Running" ends, instance exits
```

---

## Quick Reference Tables

### Reserved Keywords

**IMPORTANT:** Polyglot has **ZERO reserved keywords**. All previously keyword-based functionality has been replaced with block markers or reserved enumerations.

**Deprecated Keywords (v0.0.1 - DO NOT USE):**

| Old Keyword | Replacement | Example |
|-------------|-------------|---------|
| `True` | `#Boolean.True` or alias `#True` | `.valid: pg\bool << #True` |
| `False` | `#Boolean.False` or alias `#False` | `.valid: pg\bool << #False` |
| `Fixed` | `[i]` with `<<` (constant) | `[i] .key: pg\string << "secret"` |
| `Default` | `[i]` with `<~` | `[i] .timeout: pg\int <~ 30` |
| `Exposed` | *(removed - macro system revised)* | N/A |

**Boolean Values:**

Polyglot uses the `#Boolean` reserved enumeration for boolean values:
- **Full form:** `#Boolean.True`, `#Boolean.False`
- **Aliases:** `#True`, `#False` (recommended for readability)

```polyglot
[r] .is_valid: pg\bool << #True
[r] .is_error: pg\bool << #False
```

---

### Standard Library Namespaces

| Namespace | Purpose | Status |
|-----------|---------|--------|
| `|W.*` | Runtime wrappers | ✓ Fully documented |
| `|Q.*` | Queue control | ✓ Fully documented |
| `|Y.*` | Join operations | ✓ Fully documented |
| `|U.*` | Utilities | ⚠ Catalog only (APIs TBD) |
| `|T.*` | Triggers | ⚠ Catalog only (APIs TBD) |

---

### Type Quick Reference

| Type | Description | Example |
|------|-------------|---------|
| `pg\int` | Integer | `.count: pg\int << 42` |
| `pg\uint` | Unsigned integer | `.size: pg\uint << 1024` |
| `pg\string` | String | `.name: pg\string << "Example"` |
| `pg\bool` | Boolean | `.valid: pg\bool << #True` |
| `pg\path` | File path | `.file: pg\path << \\DataDir\\data.csv` |
| `pg\dt` | DateTime | `.time: pg\dt << DT"12:30:"` |
| `pg\array{}` | Array | `.items: pg\array{pg\string}` |
| `pg\set{}` | Set | `.unique: pg\set{pg\int}` |
| `pg\serial` | Serializable | `.config: pg\serial` |

---

### Common Patterns

**Define a Pipeline:**
```polyglot
[|] PipelineName
[i] .input: pg\string
[r] |ProcessData
[<] .data: pg\string << .input
[X]
```

**Call a Pipeline:**
```polyglot
[r] |PipelineName
[<] .input: pg\string << "value"
```

**Handle an Error:**
```polyglot
[r] |MightFail
[!] !ErrorType
[>] .message: pg\string >> err_msg
```

**Run in Parallel:**
```polyglot
[p] |ProcessPartA
[<] .data: pg\string << input
[>] .output >> result1

[Y] |Y.Join
[>] result1
```

**Use Runtime Wrapper:**
```polyglot
[W] |W.Python3.11
[r] |RunScript
[<] .script: pg\path << "analyze.py"
```

---

## See Also

### Language Specification
- [Type System](type-system.md) - Complete type reference
- [Operators](operators.md) - Detailed operator semantics
- [Block Markers](block-markers.md) - Complete block marker reference
- [Comments](comments.md) - Comment syntax details

### Advanced Features
- [Parallel Execution](parallel-execution.md) - Parallel blocks and join
- [Expansion Operator](expansion-operator.md) - Nesting rules
- [Pipeline Lifecycle](pipeline-lifecycle.md) - Instance lifecycle

### Examples
- [Hello World](../examples/hello-world.md) - Basic examples
- [Complete Workflows](../examples/complete-workflows.md) - Full examples

### Planning
- [Decision Log](../decision-log.md) - All syntax decisions

---

**End of Complete Syntax Reference**