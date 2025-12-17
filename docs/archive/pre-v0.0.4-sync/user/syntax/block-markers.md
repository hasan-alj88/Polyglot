---
archived: true
archived_date: 2025-12-17
archived_reason: "Superseded by v0.0.4 sync"
original_path: user/syntax/block-markers.md
---

<!-- ARCHIVED: 2025-12-17 | Reason: Superseded by v0.0.4 sync -->

# Polyglot Block Markers Reference

**Version:** 0.0.2
**Complete Reference:** All 23+ Block Markers
**Last Updated:** 2025-12-02

---

## Introduction

Block markers are **3-character sequences** (`[x]` that define structure and execution semantics in Polyglot. Unlike traditional languages that use keywords and indentation, Polyglot's block markers make execution behavior **explicit and unambiguous**.

---

## Complete Block Marker Table

| Marker | Name         | Category     | Context               | Purpose                          |
| ------ | ------------ | ------------ | --------------------- | -------------------------------- |
| `[@]`  | Package      | Registrey    | File-scope            | Package declaration              |
| `[\|]` | Pipeline     | Registrey    | File-scope            | Pipeline definition              |
| `[#]`  | Enumeration  | Registrey    | File-scope            | Enumeration definition           |
| `[!]`  | Error        | Registrey    | File-scope / Pipeline | Error definition or handler      |
| `[M]`  | Macro        | Macro        | File-scope            | Macro definition                 |
| `[X]`  | Terminator   | Structure    | All blocks            | Block terminator                 |
| `[i]`  | Input        | Data Flow    | Pipeline              | Input declaration                |
| `[o]`  | Output       | Data Flow    | Pipeline              | Output declaration               |
| `[<]`  | Pass Input   | Data Flow    | Child block           | Input binding (PUSH              |
| `[>]`  | Pass Output  | Data Flow    | Child block           | Output binding (PULL             |
| `[r]`  | Sequentional | Execution    | Pipeline              | Sequential execution             |
| `[p]`  | Parallel     | Execution    | Pipeline              | Parallel execution               |
| `[b]`  | Background   | Execution    | Pipeline              | Background execution             |
| `[s]`  | Serial       | Execution    | Enum/Pipeline         | Serial file I/O (YAML/JSON/TOML) |
| `[Y]`  | Join         | Execution    | Join `[p]` variables  | Join/synchronize parallel tasks  |
| `[?]`  | Switch       | Control Flow | Pipeline              | Conditional check                |
| `[t]`  | Trigger      | Control Flow | Pipeline              | Trigger declaration              |
| `[Q]`  | Queue        | Control Flow | Pipeline              | Queue control                    |
| `[W]`  | Wrapper      | Control Flow | Pipeline              | Macro Injector wrapper           |
| `[~]`  | Expansion    | Scope        | Nested                | Nesting/expansion indicator      |
| `[A]`  | Alias        | Special      | Enumeration           | Alias definition                 |
| `[\]`  | Setup        | Special      | Pipeline              | Setup block                      |
| `[/]`  | Cleanup      | Special      | Pipeline              | Cleanup block                    |
| `[{]`  | Scope In     | Special      | Macro                 | Macro scope entry                |
| `[}]`  | Scope Out    | Special      | Macro                 | Macro scope exit                 |
| `[&]`  | AND          | Boolean      | Conditional           | Logical AND                      |
| `[+]`  | OR           | Boolean      | Conditional           | Logical OR                       |
| `[-]`  | NOT          | Boolean      | Conditional           | Logical NOT                      |
| `[^]`  | XOR          | Boolean      | Conditional           | Logical XOR                      |
| `[.]`  | Group        | Boolean      | Conditional           | Grouping operator                |
| `[*]`  | Continue     | Special      | Any                   | Line continuation                |

---

## Structure Markers

### `[@]` Package Declaration

Declares package metadata and dependencies.

```polyglot
[@] @Local::MyProject:1.0.0.0
[<] @utils << @Community::StringUtils:2.0.0.0  // Dependency
[X]
```

### `[|]` Pipeline Definition

Defines an executable pipeline with inputs, triggers, and outputs.

```polyglot
[|] ProcessData
[i] .input:pg.string
[t] |T.Call
[W] |W.Polyglot.Scope
[o] .result:pg.string
[X]
```

### `[#]` Enumeration Definition

Defines enum types or serial structures.

```polyglot
[#] #UserRole
[<] .Admin                         // Enum field (no type
[<] .User                          // Enum field (no type
[X]

[#] #Config
[<] <timeout:pg.int << 30         // Serial field (has type
[X]
```

### `[!]` Error Definition

Defines error types with required fields: `.message`, `.code`, `.trace`.

```polyglot
[!] NetworkTimeout
[<] <message:pg.string << "Network operation timed out"
[<] <code:pg.int << 1001
[<] <trace:pg.string << ""        // Populated at runtime
[<] <retry_after:pg.int << 5      // Custom field
[X]
```

### `[M]` Macro Definition

Defines reusable code blocks with setup/cleanup.

```polyglot
[M] DatabaseTransaction
[\]                                // Setup
[r] |DB.BeginTransaction
[/]                                // Cleanup
[r] |DB.Commit
[X]
```

### `[X]` Block Terminator

Closes ALL blocks (package, pipeline, enum, error, macro.

```polyglot
...
[|] Pipeline
[i] !No.Input
...
[X]                                // Closes pipeline
```

**Note:** `...` indicates additional code not relevant to this example.

---

## Data Flow Markers

### `[i]` Input Declaration

Declares pipeline inputs. **MANDATORY** - at least one required.

`[i]` acts as an **implicit trigger** - execution begins when the input variable reaches the `Ready` state.

**See:** [Variable State System](../variable-state-system.md for state transitions and the `Ready` state.

```polyglot
[i] .data:pg.string               // Required input (triggers when Ready
[i] .timeout:pg.int <~ 30         // Default input (can override
[i] .version:pg.string << "1.0"   // Constant input (Ready immediately
[i] !No.Input                      // No inputs marker
```

### `[o]` Output Declaration

Declares pipeline outputs. **MANDATORY** - at least one required.

```polyglot
[o] .result:pg.string             // Normal output
[o] #StatusCode                    // Enumeration output
[o] !No.Output                     // No output marker
```

### `[<]` Input Binding (Pass Input

**REQUIRES PARENT BLOCK** with **Predefined Pipeline** - pushes value into operation.

**Push and Pull direction:** When reading left-to-right, `<<` pushes data (from right to left, while `>>` pulls data (from left to right. The direction depends on reading perspective - one direction is always push, the other is pull.

**See:** [Pipeline Definition](../pipeline-definition.md for data flow patterns.

```polyglot
[r] |ProcessData                   // Parent block
[<] <input:pg.string << .data     // Input binding (PUSH data into pipeline
[>] >output:pg.string >> .result  // Output binding (PULL data from pipeline

[p] ~ForEach                       // Parent unpack
[<] .items                         // Variable only (type infrerd/push
[>] .item
```

**Common Mistake:**
```polyglot
// ✗ WRONG: [<] at pipeline scope (no parent
[<] <variable:pg.int << 42        // ERROR!

// ✓ CORRECT: Use [r] at pipeline scope
[r] .variable:pg.int << 42
```

### `[>]` Output Binding (Pass Output

**REQUIRES PARENT BLOCK**  with **Predefined Pipeline**- pulls value from operation.

```polyglot
[r] |FetchUser                     // Parent block
[<] .id << .user_id
[>] >user: #UserProfile >> .result // Output binding
```

---

## Special Data Flow Operators

### `<~` PUSH Default

Pushes default value, can be overridden once.

```polyglot
[i] .timeout:pg.int <~ 30         // DefaultReady state
[i] .retries:pg.int <~ 3          // Can override
...
[r] .timeout << 60                 // Override (1 push remaining
[r] .timeout << 20                 // Compile Error: No more push allowed
```

**Note:** Default values can only be overridden **once**. A second push attempt results in a compile-time error.

---

## Execution Markers

### `[r]` Run (Sequential OR Variable Declaration

**Context-dependent** behavior:

**Variable Declaration:**

Variable declaration can be done in **all execution blocks** (`[r]`, `[p]`, `[b]`, `[s]`, `[?]`, etc., not just `[r]`. However, `[r]` is the most common location.

**See:** [Variable State System](../variable-state-system.md for state transitions and declarations.

```polyglot
[r] .user:pg.string               // Declared state
[r] .count:pg.int << 42           // Ready state
[r] .timeout:pg.int <~ 30         // DefaultReady state

[?] .condition =? #Boolean.True    // Variables can be declared in any block
[~][r] .nested_var:pg.int << 10   // Declaration in nested scope
[~]
```

**In execution flow = Sequential pipeline call:**
```polyglot
[r] |Step1                         // Run sequentially
[<] .input << .data
[r] |Step2                         // Waits for Step1 to complete
[<] .input << .step1_result
```

### `[p]` Parallel Execution

Executes multiple operations in parallel.

```polyglot
[p] ~ForEach                       // Parallel unpack
[<] .items
[>] .item
[~][r] |ProcessItem                // Runs in parallel for each item
[~]
```

### `[b]` Background Execution

Fire-and-forget execution (no join.

```polyglot
[b] |LogEvent                      // Background task
[<] .event << .user_action         // Doesn't block pipeline
```

### `[s]` Serial File Loading (Enumerations)

**Context:** Enumeration definitions

Serial blocks load data files (YAML, JSON, TOML) using pipeline calls. All `[s]` blocks at the same nesting level run **in parallel** with shared error handling.

**⚠️ Safety Mechanism:** Must declare error handling with `[s][!] *` (default or `[s][!]` (custom.

```polyglot
[#] #Config
[<] .api_key:pg.string
[<] .timeout:pg.int <~ 30
[s] |YAML.Load"config.yaml"     // Load from YAML file
[s][!] *                        // Default error handling (required
[X]

// Multiple files load in parallel
[#] #AppConfig
[<] .database_url:pg.url
[<] .cache_url:pg.url
[s] |YAML.Load"database.yaml"   // Parallel load
[s] |YAML.Load"cache.yaml"      // Parallel load
[s][!] *                        // Shared error handling
[X]

// Custom error handling
[#] #Secrets
[<] .api_key:pg.string
[s] |YAML.Load".env.secrets"
[s][!]
[r] |U.Log.Error"Failed to load secrets: {!.message"
[r] |U.Process.Exit"1"
[X]
```

**See:** [Safety Mechanisms](safety-mechanisms.md#safety-mechanism-4-s---explicit-serial-error-handling) and [Safety Mechanism 5](safety-mechanisms.md#safety-mechanism-5-s----serial-schema-enforcement)

---

### `[s]` Serial File Loading (Pipelines)

**Context:** Pipeline execution

Serial file loading in pipeline context uses explicit I/O bindings for dynamic file processing.

```polyglot
[|] LoadData
[i] .json_file:pg\path
[t] |T.Call
[W] |W.Polyglot.Scope

[s] |JSON.Load.Files
[<] <files:pg\array.pg\path << .json_files
[>] >json_data:pg\serial >> .data

[s][!] *

[o] .data:pg\serial
[X]
```

**Note:** Unlike enumeration `[s]` blocks, pipeline `[s]` blocks do NOT require schema declarations because variables are explicitly typed.

---

### `[Y]` Join Point

Synchronizes parallel tasks and collects results.

**Operators:**
- **`~*`** - Unpack operator (expands collection into individual items
- **`~Y.*`** - Pack operator (collects individual items back into collection

**See:** [Handling Collections](../handling-collections.md for collection operations.

```polyglot
[p] ~ForEach                       // ~* unpack operator (implicit
[<] .items
[>] .item
[~][r] |ProcessItem
[~][<] .data << .item
[~][>] .result >> .processed
[~]
[~][Y] ~Y.IntoArray                // ~Y.* pack operator (join/collect
[~][<] .processed                  // Collect results
[~][>] .all_results                // Output array
[~]
```

---

## Control Flow Markers

### `[?]` Conditional (Switch

Conditional branching. **MUST include catch-all `[?] *?`**.

**Range operators:**
- `?[a,b]` - Closed interval (inclusive both ends
- `?[a,b` - Half-open interval (inclusive start, exclusive end
- `?(a,b]` - Half-open interval (exclusive start, inclusive end
- `?(a,b` - Open interval (exclusive both ends

```polyglot
[?] .value >? 100
[~][r] .result:pg.string << "high"
[~]

[?] .value ?[50,100               // 50 <= value < 100
[~][r] .result:pg.string << "medium"
[~]

[?] .value ?(0,50]                 // 0 < value <= 50
[~][r] .result:pg.string << "low"
[~]

[?] *?                             // REQUIRED catch-all
[~][r] .result:pg.string << "other"
[~]
```

### `[t]` Trigger Declaration

Defines when pipeline executes. **MANDATORY** - pipeline won't run without trigger!

```polyglot
[t] |T.Call                        // Manual invocation

[t] |T.DT.Daily                    // Daily at 3 AM
[<] <hour:pg.int << 3
[<] <minute:pg.int << 0

[t] |T.DT.Weekly                   // Every Friday at 5 PM
[<] <day:pg.string << "Friday"
[<] <hour:pg.int << 17
[<] <minute:pg.int << 0

[t] |T.File.Modified               // File system event
[<] <path:pg.path << "/data"
```

### `[Q]` Queue Control

Queue configuration and prioritization.

```polyglot
[Q]
[<] <priority:pg.int << 7         // Higher priority
[<] <max_retries:pg.int << 3
```

### `[W]` Wrapper Declaration

Runtime environment wrapper. **MANDATORY** if no `[\][/]` setup/cleanup.

```polyglot
[W] |W.RT.Python3.14               // Python runtime
[W] |W.RT.Rust1.8                  // Rust runtime
[W] |W.RT.Nodejs                   // Node runtime
[W] |W.Polyglot.Scope              // Safety placeholder (no external code
```

---

## Nesting and Scope Markers

### `[~]` Expansion/Nesting Indicator

Marks nested scope within parent block.

```polyglot
[?] .condition =? #Boolean.True
[~][r] .nested_var:pg.int << 1    // Nested within condition
[~][r] |NestedPipeline             // Nested execution
[~]                                // End nesting
```

### `[\]` Setup Block

Setup phase (runs before execution. **FIFO order**.

```polyglot
[\]
[r] |InitDatabase
[r] |LoadConfig
[/]
```

### `[/]` Cleanup Block

Cleanup phase (runs after execution. **LIFO order** (reverse of setup.

```polyglot
[\]
[r] |OpenFile
[/]
[r] |CloseFile                     // Runs in reverse order
```

### `[{]` Macro Scope In

Marks where macro code is inserted.

```polyglot
[M] WithLogging
[{]                                // Macro insertion point
[r] |LogStart
[]
[X]
```

### `[]` Macro Scope Out

Marks end of macro code insertion.

---

## Boolean Logic Markers

### `[&]` AND Operator

Logical AND - all conditions must be true.

```polyglot
[?] .x >? 10
[&] .y <?  20                      // x > 10 AND y < 20
[~][o] .result:pg.bool << #Boolean.True
[~]
```

### `[+]` OR Operator

Logical OR - any condition can be true.

```polyglot
[?] .status =? #Status.Active
[+] .status =? #Status.Pending     // Active OR Pending
[~][r] |Process
[~]
```

### `[-]` NOT Operator

Logical NOT - negates condition.

```polyglot
[?] .enabled =? #Boolean.True
[-] .disabled =? #Boolean.True     // NOT disabled
[~][r] |Execute
[~]
```

### `[^]` XOR Operator

Logical XOR - exactly one must be true.

```polyglot
[?] .mode_a =? #Boolean.True
[^] .mode_b =? #Boolean.True       // mode_a XOR mode_b
[~][o] .valid:pg.bool << #Boolean.True
[~]
```

### `[.]` Grouping Operator

Groups boolean expressions for precedence.

```polyglot
[?]
[.] .a >? 10
[.] [&] .b <? 20
[+] .c =? #Boolean.True            // (a > 10 AND b < 20 OR c == True
[~][o] .result:pg.bool << #Boolean.True
[~]
```

---

## Special Markers

### `[A]` Alias Definition

Creates shorthand alias for enumeration field.

```polyglot
[#] #Boolean
[<] .True
[A] True                           // Alias: #True → #Boolean.True
[<] .False
[A] False                          // Alias: #False → #Boolean.False
[X]
```

Usage:
```polyglot
.flag:pg.bool << #True            // Using alias
.flag:pg.bool << #Boolean.True    // Full path
```

### `[*]` Line Continuation

Continues logical line across multiple physical lines.

```polyglot
[r] .message:pg.string << "This is a very long message "
[*] +"that spans multiple lines for readability"
[*] +"and continues with proper concatenation."
```

---

## Block Marker Hierarchy Rules

### Parent-Child Relationships

Some markers **REQUIRE** parent context:

**`[<]` and `[>]` require parent:**
- Pipeline call `[r] |Pipeline`
- Unpack operator `[p] ~ForEach`
- Queue control `[Q]`
- Trigger config `[t]`
- Wrapper config `[W]`

**Example:**
```polyglot
[r] |Pipeline                      // Parent
[<] .input << .data                // Child (valid
[>] .output >> .result             // Child (valid

[<] .standalone << .data           // ERROR: No parent!
```

### Nesting Rules

**`[~]` marks nesting depth:**

```polyglot
[?] .outer =? #Boolean.True
[~][?] .inner =? #Boolean.True     // One level deep
[~][~][r] .deeply_nested:pg.int << 1  // Two levels deep
[~][~]
[~]
```

---

## Mandatory Pipeline Structure

Every `[|]` pipeline **MUST** include these sections **in order**:

1. **`[i]` Inputs** - At least one (or `[i] !No.Input`
2. **`[t]` Triggers** - At least one (or pipeline NEVER runs
3. **`[Q]` Queue** - Optional
4. **`[W]` Wrapper OR `[\][/]` Setup/Cleanup** - MANDATORY
5. **Execution blocks** - `[r]`, `[p]`, `[?]`, etc.
6. **`[o]` Outputs** - At least one (or `[o] !No.Output`

**Minimal example:**
```polyglot
[|] Minimal
[i] !No.Input
[t] |T.Call
[W] |W.Polyglot.Scope
[o] !No.Output
[X]
```

---

## Common Patterns

### Data Processing Pipeline

```polyglot
[|] ProcessData
[i] .input_file:pg.path
[t] |T.Call
[W] RT.Python"process.py"

[r] .raw_data:pg.string << ""
[r] |ReadFile
[<] .path << .input_file
[>] .content >> .raw_data

[r] |Transform
[<] .data << .raw_data
[>] .result >> .transformed

[o] .transformed:pg.string
[X]
```

### Parallel Processing with Join

```polyglot
[|] ParallelProcess
[i] .items:pg.array.pg.string
[t] |T.Call
[W] |W.Polyglot.Scope

[p] ~ForEach
[<] .items
[>] .item
[~][r] |ProcessItem
[~][<] .data << .item
[~][>] .result >> .processed
[~]
[~][Y] ~Y.IntoArray
[~][<] .processed
[~][>] .all_results
[~]

[o] .all_results:pg.array.pg.string
[X]
```

### Error Handling

```polyglot
[|] WithErrorHandling
[i] .url:pg.string
[t] |T.Call
[W] |W.Polyglot.Scope

[r] |FetchURL
[<] .url << .url
[>] .content >> .result
[~]
[~][!] !NetworkTimeout
[~][r] |HandleTimeout
[~][o] !NetworkTimeout
[~]
[~][!] *                           // Catch all other errors
[~][o] !UnknownError
[~]

[o] .result:pg.string
[X]
```

---

## See Also

- [Syntax Overview](overview.md - Core principles
- [Operators Reference](operators.md - All operators
- [Type System](type-system.md - Type declarations
- [Examples](/docs/user/examples/ - Practical examples

---

**Next:** [Operators →](operators.md
