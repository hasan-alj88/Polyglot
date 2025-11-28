# Polyglot Quick Language Reference

**Version:** 0.0.2
**Purpose:** Fast reference for generating valid Polyglot code
**Last Updated:** 2025-11-12

---

## 1. CRITICAL RULES

### Absolute Requirements

1. **EVERY valid line MUST start with a block element** `[x]`
2. **ALL pipelines MUST have a trigger `[t] |T.*`** - compiler error if missing
3. **Type separator is ALWAYS backslash** `\` (NEVER forward slash `/`)
4. **Comments use forward slash** `//` or `/* */` (NOT backslash)
5. **Block markers are case-sensitive** (`[r]` ≠ `[R]`, `[i]` ≠ `[I]`)
6. **All pipelines must have `[|]` and `[X]`** (paired markers)
7. **Assignment operators: `<<` (push INTO) and `>>` (pull FROM)**
8. **Pipeline operator `|` is required** for all pipeline calls
9. **Error type operator `!` is required** for all error references
10. **`[o]` is for DECLARATION ONLY** - Never use `<<` with `[o]`; use `[r]` for computation first
11. **All branches MUST output same fields** - Switch/parallel branches must have identical output sets (compiler-enforced)
12. **NO comparison operators** - `==`, `!=`, `<`, `>`, `<=`, `>=` do NOT exist; use explicit named operations
13. **Errors use `[!]` blocks for catching** - NOT `[?]` switches; catch errors after operations that might fail
14. **No need to check for "no error"** - Default behavior is success; only catch specific error types when needed

### Never Do This

```polyglot
// ✗ INVALID - Missing block marker
.x: pg\int << 5

// ✗ INVALID - Wrong type separator
.x: pg/int << 5

// ✗ INVALID - Wrong assignment direction
[<] .input: pg\string >> value

// ✗ INVALID - Missing pipeline operator
[r] ProcessData

// ✗ INVALID - Missing [X] close marker
[|] Pipeline
[i] .input: pg\string

// ✗ INVALID - Assignment in [o] block
[o] .result: pg\string << "value"

// ✗ INVALID - Comparison operators don't exist
[?] .age >= 18
[?] .name == "Alice"
[?] greeting_error != \\NoError\\

// ✗ INVALID - Using [?] for error handling
[?] greeting_error != \\NoError\\
[~][r] |HandleError

// ✗ INVALID - Inconsistent outputs across branches
[?] .condition ?> True
[~][o] .result: pg\string

[?] .condition ?> False
[~][o] .status: pg\int    // Different field!
```

### Always Do This

```polyglot
// ✓ VALID - Starts with block marker
[r] .x: pg\int << 5

// ✓ VALID - Correct type separator
[r] .x: pg\int << 5

// ✓ VALID - Correct assignment direction
[<] .input: pg\string << value

// ✓ VALID - Pipeline operator present
[r] |ProcessData

// ✓ VALID - Properly closed
[|] Pipeline
[i] .input: pg\string
[X]

// ✓ VALID - Computation then declaration
[r] .greeting: pg\string << "Hello, {.name}!"
[o] .greeting: pg\string

// ✓ VALID - Use explicit named operations instead of comparisons
[Q] |Q.DispatchIf.RAM.MoreThan
[<] .ram_gb: pg\uint << 3

// ✓ VALID - Or use range matching when appropriate
[?] .age ?> 18..65
[~][r] |HandleAdult

// ✓ VALID - Catch errors with [!] block
[r] |RiskyOperation
[<] .data: pg\string << .input
[~]
[~][!] !Error.Network.Timeout
[~][<] .timeout: pg\dt << DT"5m"
[~][r] |HandleTimeout

// ✓ VALID - Consistent outputs across all branches
[?] .status ?> #Status.Success
[~][o] .result: pg\string
[~][o] .code: pg\int

[?] .status ?> #Status.Failed
[~][o] .result: pg\string
[~][o] .code: pg\int
```

---

## 2. Block Markers

### Required Block Markers

| Marker | Purpose | Required Pairing | Usage |
|--------|---------|------------------|-------|
| `[\|]` | Pipeline definition | `[X]` | `[|] PipelineName` |
| `[X]` | Close marker | `[\|]`, `[#]`, `[!]` | `[X]` |
| `[i]` | Input declaration | - | `[i] .param: pg\string` |
| `[o]` | Output declaration | - | `[o] .result: pg\string` |
| `[r]` | Run sequential | - | `[r] |PipelineName` |
| `[p]` | Parallel execution | - | `[p] |PipelineName` |
| `[<]` | Pass input / Define field | - | `[<] .param << value` |
| `[>]` | Pass output | - | `[>] .result >> var` |
| `[Y]` | Join block | `|Y.Join` | `[Y] |Y.Join` |
| `[t]` | Trigger | - | `[t] |T.Daily` |
| `[Q]` | Queue control | - | `[Q] |Q.Pause` |
| `[W]` | Wrapper context | - | `[W] |W.Python3.11` |
| `[#]` | Enumeration definition | `[X]` | `[#] EnumName` |
| `[!]` | Error def/catching | `[X]` or standalone | `[!] !ErrorType` |
| `[A]` | Alias definition | - | `[A] AliasName` |
| `[~]` | Expansion/nesting | - | `[~][r] |Operation` |

### Block Marker Syntax Rules

```polyglot
// Pipeline structure
[|] PipelineName
[i] .input: type
[r] |Operation
[o] .output: type
[X]

// Enumeration structure
[#] EnumerationName
[<] .field: type << value
[A] Alias
[X]

// Error structure
[!] !ErrorName
[<] .message: pg\string << "msg"
[<] .code: pg\int << 5000
[<] .trace: pg\string << ""
[X]

// Input variants
[i] .required: pg\string                    // Required
[i] Fixed .constant: pg\string << "value"   // Immutable
[i] Default .optional: pg\int << 30         // Optional with default

// Nesting with [~]
[p] |Parallel
[~][r] |Nested           // Runs WITHIN parallel
[~][<] .param << value   // Child of [~][r]
```

---

## 3. Operators

### All Operators

| Operator | Name | Direction | Purpose | Example |
|----------|------|-----------|---------|---------|
| `\|` | Pipeline | - | Call pipeline | `\|ProcessData` |
| `~` | Unpack | - | Expand collection | `~Array.ForEach` |
| `@` | Package | - | Access from package | `@pkg|Pipeline` |
| `#` | Enumeration | - | Mark enumeration | `#MyEnum` |
| `!` | Error | - | Mark error type | `!CustomError` |
| `<<` | Push | **LEFT** | Push value INTO variable | `.x << value` |
| `>>` | Pull | **RIGHT** | Pull value FROM source | `.x >> output` |
| `\` | Type separator | - | Separate namespace/type | `pg\int` |

### Operator Usage

```polyglot
// Pipeline operator |
[r] |PipelineName
[r] |T.Daily
[r] |U.String.Format
[r] |W.Python3.11

// Unpack operator ~
[r] ~myArray
[r] ~Array.ForEach
[r] ~MyEnumeration

// Package operator @
[r] @packageName|PipelineName
[r] @Community.hasan@DataUtils|Transform

// Enumeration operator #
[#] MyEnum
//Enum values
[X]
// to used in pipeline definiation
[r] .config: #MyEnum << #MyEnum.Default

// Error operator !
[!] !MyApp.CustomError
[X]
//usage
[!] !pg.FileSystem.NotFound
[>] .message: pg\string >> err_msg

// Assignment operators << and >>
[<] .input: pg\string << "push INTO this"
[>] .output: pg\string >> "pull FROM this"
```

### Assignment Direction

```
PUSH INTO (<<):
  variable  <<  expression
   ↓              ↓
   destination  source

PULL FROM (>>):
  expression  >>  variable
   ↓            ↓
   source       destination
```

### Match Operator `?>` and Range Operator `..`

**CRITICAL: Polyglot does NOT use comparison operators (`==`, `!=`, `<`, `>`, `<=`, `>=`)**

Instead, use:

```polyglot
// Match operator ?> for switch/conditional matching
[?] .variable ?> value
[~][r] |HandleMatch

// Range operator .. for range matching
[?] .age ?> 18..65
[~][r] |HandleAdultAge

[?] .score ?> 90..100
[~][r] |HandleHighScore

// Enum matching
[?] .status ?> #Status.Success
[~][r] |HandleSuccess

[?] .status ?> #Status.Failed
[~][r] |HandleFailure
```

**For explicit comparisons, use named operations:**

```polyglot
// Instead of: .ram >= 3 (INVALID)
[Q] |Q.DispatchIf.RAM.MoreThan
[<] .ram_gb: pg\uint << 3

// Instead of: .name == "" (INVALID)
[Q] |Q.DispatchIf.String.IsEmpty
[<] .value: pg\string << .name

// Instead of: .count > 0 (INVALID)
[Q] |Q.DispatchIf.Number.GreaterThan
[<] .number: pg\int << .count
[<] .threshold: pg\int << 0
```

---

## 4. Type System

### Type Syntax

```
language\type
language.mutable\type
```

### Primitive Types

```polyglot
pg\int          // Signed integer
pg\uint         // Unsigned integer
pg\float        // Floating point
pg\string       // UTF-8 string
pg\bool         // Boolean (True/False)
pg\path         // File system path
pg\dt           // DateTime
```

### Collection Types

```polyglot
pg\array{element_type}         // Ordered array
pg\set{element_type}           // Unordered set
pg\array{pg\string}            // String array
pg\set{pg\int}                 // Integer set
pg\array{pg\array{pg\int}}     // 2D array
```

### Special Types

```polyglot
pg\serial       // Serializable data (mutable schema)
#EnumName       // Enumeration type (user-defined)
!ErrorName      // Error type (user-defined)
#None           // No value
```

### Mutable Types

```polyglot
pg.mutable\int
pg.mutable\string
pg.mutable\path
pg.mutable\array{pg\int}
```

### Type Declaration Examples

```polyglot
// Variables
[r] .count: pg\int << 0
[r] .name: pg\string << "Alice"
[r] .valid: pg\bool << #True
[r] .items: pg\array{pg\string} << array{"a", "b", "c"}
[r] .config: pg\serial << serial{"host": "localhost"}

// Inputs
[i] .file: pg\path
[i] .data: pg\array{pg\int}
[i] Default .timeout: pg\int << 30

// Outputs
[o] .result: pg\string
[o] .count: pg\int

// Mutable
[r] .counter: pg.mutable\int << 0
```

---

## 5. Required Elements Checklist

### Every Pipeline Must Have

- [x] `[|]` pipeline start marker
- [x] Pipeline name (after `[|]`)
- [x] **`[t] |T.*` trigger declaration** - MANDATORY (compiler error if missing)
- [x] `[X]` close marker

### Trigger Types (REQUIRED)

**ALL pipelines MUST have exactly ONE trigger:**

```polyglot
// T.Call - For pipelines called by other pipelines
[t] |T.Call

// T.Cli - For pipelines run via CLI (polyglot run)
[t] |T.Cli

// T.Daily - Scheduled daily trigger
[t] |T.Daily
[<] .time: pg\dt << DT"09:00:"

// T.File.Modified - File system trigger
[t] |T.File.Modified
[<] .path: pg\path << \\DataDir\\config.json

// T.Every.Seconds - Interval trigger
[t] |T.Every.Seconds
[<] .interval: pg\int << 30
```

**CRITICAL RULES:**
- **DO NOT use boolean variables as triggers** - `[t] .condition: pg\bool << expr` is INVALID
- Triggers are continuously-run operations, NOT conditional execution
- Missing trigger = compiler error

### Minimal Valid Pipeline

```polyglot
[|] MinimalPipeline
[t] |T.Call
[W] |W.NoSetup.NoCleanup
[X]
```

### Pipeline with Input (Common Pattern)

```polyglot
[|] PipelineName
[i] .input: pg\string
[r] |ProcessData
[<] .data: pg\string << .input
[X]
```

### Pipeline with Output (Common Pattern)

```polyglot
[|] PipelineName
[i] .input: pg\string
[r] |ProcessData
[<] .data: pg\string << .input
[>] .result: pg\string >> output
[o] .result: pg\string
[X]
```

### Pipeline with Trigger (Common Pattern)

```polyglot
[|] ScheduledPipeline
[t] |T.Daily
[<] .time: pg\dt << DT"09:00:"
[r] |DoWork
[X]
```

### Enumeration Must Have

- [x] `[#]` enumeration marker
- [x] Enumeration name
- [x] At least one field with `[<]`
- [x] `[X]` close marker

```polyglot
[#] MyApp.Config
[<] .host: pg\string << "localhost"
[<] .port: pg\int << 8080
[X]
```

### Error Must Have

- [x] `[!]` error marker
- [x] Error name with `!` prefix
- [x] `.message: pg\string` field
- [x] `.code: pg\int` field
- [x] `.trace: pg\string` field
- [x] `[X]` close marker

```polyglot
[!] !MyApp.CustomError
[<] .message: pg\string << "Error occurred"
[<] .code: pg\int << 5000
[<] .trace: pg\string << ""
[X]
```

---

## 6. Common Patterns

### Pattern: Sequential Operations

```polyglot
[r] |Step1
[r] |Step2
[r] |Step3
```

### Pattern: Variable Assignment

```polyglot
[r] .variable: pg\string << "value"
[r] .counter: pg\int << 0
```

### Pattern: Pipeline Call with I/O

```polyglot
[r] |PipelineName
[<] .input1: pg\string << "value"
[<] .input2: pg\int << 42
[>] .output: pg\string >> result_var
```

### Pattern: Parallel Execution

```polyglot
[p] |TaskA
[<] .data: pg\string << input
[>] .result >> result_a

[p] |TaskB
[<] .data: pg\string << input
[>] .result >> result_b

[Y] |Y.Join
[>] result_a
[>] result_b
```

### Pattern: Error Handling

```polyglot
[r] |MightFail
[!] !pg.FileSystem.NotFound
[>] .message: pg\string >> err_msg
[r] |HandleError
[<] .msg: pg\string << err_msg
```

### Pattern: Switch/Conditional with `[?]`

```polyglot
// ✓ CORRECT - Use range matching or enum matching
[?] .status ?> #Status.Success
[~][r] |HandleSuccess
[~][o] .result: pg\string

[?] .status ?> #Status.Failed
[~][r] |HandleFailure
[~][o] .result: pg\string

// ✓ CORRECT - Range matching with `..`
[?] .age ?> 18..65
[~][r] |HandleAdult

[?] .age ?> 0..17
[~][r] |HandleMinor

// ✓ CORRECT - Use explicit named operations for comparisons
[Q] |Q.DispatchIf.String.IsEmpty
[<] .value: pg\string << .input_name
[~][r] |HandleEmpty

[Q] |Q.DispatchIf.String.IsNotEmpty
[<] .value: pg\string << .input_name
[~][r] |HandleNotEmpty

// IMPORTANT: All branches MUST output the same fields (compiler-enforced)
```

### Pattern: Runtime Wrapper

```polyglot
[W] |W.Python3.11
[r] |RunScript
[<] .script: pg\path << "analyze.py"
[<] .data: pg\string << input_data
[>] .result: pg\string >> output
```

### Pattern: Nested Operations

```polyglot
[p] |Parallel
[<] .data: pg\string << input
[~][r] |NestedOp1            // WITHIN parallel
[~][<] .param << .data
[~][r] |NestedOp2            // WITHIN parallel
[>] .output >> result
```

### Pattern: Unpack Iteration

```polyglot
[r] .items: pg\array{pg\string} << array{"a", "b", "c"}
[r] ~.items
[~][r] |ProcessItem          // WITHIN iteration
[~][<] .item: pg\string << .current_item
```

---

## 7. String Syntax

### Basic String Literal

```polyglot
[r] .text: pg\string << "Hello, World!"
```

### Multiline String Continuation with `[^]` and `+""`

**CRITICAL RULE:** The `+""` prefix is REQUIRED for explicit string continuation on new lines to prevent accidental concatenation.

```polyglot
// ✓ CORRECT - Explicit continuation with +""
[r] .report: pg\string << "First line "
[^] +"second line "
[^] +"third line"
// Result: "First line second line third line"

// ✗ INVALID - Missing +"" prefix causes syntax error
[r] .report: pg\string << "First line "
[^] "second line"
// Result: "First line ""second line"  (invalid - unclosed quotes)
```

**Why `+""` is Required:**

Unlike Python where strings on consecutive lines are automatically concatenated, Polyglot requires **explicit continuation** to prevent accidental mistakes from missing commas in arrays or serial objects.

**Rules for `[^]` and `+""`:**
- `[^]` - Block marker indicating line continuation
- `+""` - String prefix for explicit concatenation (REQUIRED on continuation lines)
- Must start the line after `[^]`
- Preserves whitespace after `+"`
- Prevents syntax errors from missing commas

**Example with Interpolation:**
```polyglot
[r] .message: pg\string << "Hello, {name}! "
[^] +"Welcome to {city}. "
[^] +"Your account status is: {status}"
// All variables interpolated correctly across lines
```

### String Interpolation

```polyglot
// Simple interpolation - NO +"" prefix needed
[r] .greeting: pg\string << "Hello, {name}!"
[r] .result: pg\string << "Value: {count}, Status: {status}"
```

**Rules for String Interpolation:**
- `{variable}` syntax for interpolation in regular strings
- Variables must be in scope
- No special prefix required for single-line strings

### Serial (JSON-like) Syntax

```polyglot
[r] .data: pg\serial << serial{
[^]  "key1": "value1",
[^]  "key2": 123,
[^]  "nested": {
[^]    "inner": "value"
[^]  }
[^]}
```

### Array Literal Syntax

```polyglot
[r] .items: pg\array{pg\string} << array{
[^]  "item1",
[^]  "item2",
[^]  "item3"
[^]}

// Or single line
[r] .nums: pg\array{pg\int} << array{1, 2, 3, 4, 5}
```

---

## 8. Error Handling

### Error Type Definition

```polyglot
[!] !MyApp.CustomError
[<] .message: pg\string << "Default message"
[<] .code: pg\int << 5000
[<] .trace: pg\string << ""
// Optional custom fields
[<] .custom_field: pg\string << ""
[X]
```

### Raising Errors

```polyglot
// Raise an error in a switch block
[?] .some_condition ?> True
[~]
[~][r] .error: ! << !ValidationError
[~][<] .message: pg\string << "Validation failed"
[~][<] .code: pg\int << 1001
[~][<] .trace: pg\string << ""
[~][o] .error: !

// Note: Variable type is `!` (generic error type)
// Assign specific error type: << !ErrorType
// Output raises the error: [o] .error: !
```

### Catching Errors

```polyglot
[r] |OperationThatMightFail
[~]
[~][!] !pg.FileSystem.NotFound
[~][>] .message: pg\string >> err_msg
[~][>] .code: pg\int >> err_code
[~]
[~][r] |HandleError
[~][<] .msg: pg\string << err_msg

// Catch another error type
[~]
[~][!] !MyApp.ValidationError
[~][>] .message: pg\string >> validation_msg
[~]
[~][r] |HandleValidation
```

### Error Propagation

```polyglot
// Error propagates UP if not caught
[r] |MightFail         // Throws error
// No [!] catch block here - error propagates to caller
```

### Built-in Error Namespaces

```polyglot
!pg.FileSystem.NotFound
!pg.FileSystem.PermissionDenied
!pg.Network.Timeout
!pg.Network.ConnectionRefused
!pg.Validation.TypeMismatch
```

---

## 9. Reserved Enumerations

### Path Identifiers (Extendable)

```polyglot
[#] Path.Identifiers.MyApp.DataDir
[A] DataDir
[<] .unix: pg\path << \\UnixRoot\\opt\myapp\data\
[<] .windows: pg\path << \\C\\ProgramData\MyApp\Data\
[X]

// Usage
[r] .file: pg\path << \\DataDir\\config.json
```

**Required Fields:**
- `.unix: pg\path` - Unix/Linux/macOS path
- `.windows: pg\path` - Windows path

**Use `\\NoPath\\` if path doesn't exist on OS**

### System Queues (Non-extendable)

```polyglot
#Queues.Pending     // Waiting to run
#Queues.Dispatch    // Currently running
#Queues.Pause       // Paused instances
```

### Custom Queues (Extendable)

```polyglot
[#] Queues.Background
[<] .max_concurrent: pg\int << 5
[X]

// Usage
[Q] |Q.Queue.Assign
[<] .queue: #Queues << #Queues.Background
```

### Status Values (Non-extendable)

```polyglot
#Status.Success
#Status.Failed
#Status.Pending
#Status.Running
#Status.Cancelled
```

### None Value

```polyglot
#None               // Represents no value
[o] #None           // No output from pipeline
```

---

## 10. Quick Validation Checklist

### Before Running Your Code

- [ ] **ALL pipelines have `[t] |T.*` trigger** - CRITICAL (compiler error if missing)
- [ ] All lines start with block marker `[x]`
- [ ] All `[|]` have matching `[X]`
- [ ] All `[#]` have matching `[X]`
- [ ] All `[!]` definitions have matching `[X]`
- [ ] Type separators use `\` not `/`
- [ ] Comments use `//` or `/* */`
- [ ] Pipeline calls use `|` operator
- [ ] Error references use `!` operator
- [ ] Assignment direction correct: `<<` (push) vs `>>` (pull)
- [ ] Required error fields present: `.message`, `.code`, `.trace`
- [ ] Path identifiers have both `.unix` and `.windows` fields
- [ ] Nested operations use `[~]` when needed
- [ ] Parallel blocks have `[Y] |Y.Join`
- [ ] All inputs declared with `[i]`
- [ ] All outputs declared with `[o]` (if pipeline returns value)

### Common Validation Errors

```polyglot
// ✗ Missing block marker
.x << 5

// ✗ Wrong type separator
[r] .x: pg/int << 5

// ✗ Missing pipeline operator
[r] ProcessData

// ✗ Wrong assignment direction
[<] .input >> value

// ✗ Missing [X]
[|] Pipeline
[i] .input: pg\string

// ✗ Missing required error fields
[!] !MyError
[<] .message: pg\string << "msg"
[X]

// ✗ Nested without [~]
[p] |Parallel
[r] |Operation    // Should be [~][r]

// ✗ Parallel without join
[p] |Task
[>] .result >> output
// Missing [Y] |Y.Join

// ✓ All correct
[|] Pipeline
[i] .input: pg\string
[r] |ProcessData
[<] .data: pg\string << .input
[X]
```

---

## 11. Standard Library Quick Reference

### Runtime Wrappers (`|W.*`)

```polyglot
[W] |W.Python3.11       // Python 3.11
[W] |W.Python3.12       // Python 3.12
[W] |W.Python           // Latest Python
[W] |W.Node20           // Node.js 20
[W] |W.Node             // Latest Node
[W] |W.Rust             // Rust
[W] |W.Go               // Go
[W] |W.Ruby3.2          // Ruby 3.2
[W] |W.Deno             // Deno
```

### Queue Control (`|Q.*`)

```polyglot
[Q] |Q.Pause                                    // Pause instance
[Q] |Q.Resume                                   // Resume instance
[Q] |Q.Kill                                     // Kill instance
[Q] |Q.PriorityBump                             // Increase priority
[Q] |Q.Queue.Assign                             // Assign to queue
[Q] |Q.Status                                   // Get status
[Q] |Q.PauseIf.RAM.Available.LessThan          // Conditional pause
```

### Join Operations (`|Y.*`)

```polyglot
[Y] |Y.Join                 // Wait for all parallel blocks
[Y] |Y.JoinAny              // Wait for first to complete
[Y] |Y.JoinTimeout          // Wait with timeout
```

### Triggers (`|T.*`) - Catalog

```polyglot
[t] |T.Daily
[<] .time: pg\dt << DT"09:00:"

[t] |T.Every.Minute
[t] |T.Every.Hour
[t] |T.Every.Seconds
[<] .interval: pg\int << 30

[t] |T.File.Modified
[<] .path: pg\path << \\DataDir\\config.json

[t] |T.File.Created
[t] |T.File.Deleted
```

### Utilities (`|U.*`) - Catalog

```polyglot
// String operations
[r] |U.String.Format
[r] |U.String.Concat
[r] |U.String.Split
[r] |U.String.ToInt

// Array operations
[r] |U.Array.Length
[r] |U.Array.Append
[r] |U.Array.Filter
[r] |U.Array.Map

// Path operations
[r] |U.Path.Join
[r] |U.Path.Exists
[r] |U.Path.Parent

// Logging
[r] |U.Log.Info
[<] .msg: pg\string << "message"

[r] |U.Log.Warning
[<] .msg: pg\string << "warning"

[r] |U.Log.Error
[<] .msg: pg\string << "error"
```

---

## 12. DateTime System

### DateTime Literal Syntax

```polyglot
DT"HH:MM:"                  // Time only
DT"HH:MM:SS"                // Time with seconds
DT"YYYY-MM-DD"              // Date only
DT"YYYY-MM-DD HH:MM:"       // Date and time
DT"YYYY-MM-DD HH:MM:SS"     // Full datetime
```

### DateTime Examples

```polyglot
[r] .time: pg\dt << DT"09:00:"
[r] .date: pg\dt << DT"2024-01-15"
[r] .datetime: pg\dt << DT"2024-01-15 14:30:"
[r] .precise: pg\dt << DT"2024-01-15 14:30:45"
```

### Calendar Systems

```polyglot
DT.Gregorian"2024-01-15"
DT.Hijri"1445-06-28"
DT.Chinese"4722-01-15"
DT.Hebrew"5784-10-03"
DT.Persian"1402-10-25"
```

---

## 13. Reserved Keywords

Only 5 reserved keywords exist:

| Keyword | Purpose | Example |
|---------|---------|---------|
| `True` | Boolean true | `.valid: pg\bool << #True` |
| `False` | Boolean false | `.valid: pg\bool << #False` |
| `Fixed` | Immutable input | `[i] Fixed .key: pg\string << "secret"` |
| `Default` | Optional input with default | `[i] Default .timeout: pg\int << 30` |
| `Exposed` | Macro exposure (TBD) | *(macro system details TBD)* |

---

## 14. Comments

### Single-Line Comments

```polyglot
// This is a single-line comment
[r] .x: pg\int << 5  // Inline comment
```

### Multi-Line Comments

```polyglot
/*
 * This is a multi-line comment
 * spanning multiple lines
 */
[r] |Operation

[r] .config: pg\serial << serial{ /* inline block comment */
[^]  "host": "localhost"
[^]}
```

**Important:** Comments use `/` (forward slash), types use `\` (backslash)

---

## 15. Complete Minimal Examples

### Hello World

```polyglot
[|] HelloWorld
[r] |U.Log.Info
[<] .msg: pg\string << "Hello, World!"
[o] #None
[X]
```

### File Processing

```polyglot
[|] ProcessFile
[i] .file_path: pg\path

[r] |U.File.Read
[<] .path: pg\path << .file_path
[>] .content: pg\string >> file_data

[r] |U.String.ToUpper
[<] .input: pg\string << file_data
[>] .output: pg\string >> upper_data

[r] |U.File.Write
[<] .path: pg\path << \\DataDir\\output.txt
[<] .content: pg\string << upper_data

[o] #None
[X]
```

### Scheduled Task

```polyglot
[|] DailyReport
[t] |T.Daily
[<] .time: pg\dt << DT"09:00:"

[r] |GenerateReport
[>] .report: pg\string >> report_content

[r] |U.Log.Info
[<] .msg: pg\string << report_content

[o] #None
[X]
```

### Parallel Processing

```polyglot
[|] ParallelWork
[i] .data: pg\array{pg\int}

[p] |ProcessPartA
[<] .input: pg\array{pg\int} << .data
[>] .result >> result_a

[p] |ProcessPartB
[<] .input: pg\array{pg\int} << .data
[>] .result >> result_b

[Y] |Y.Join
[>] result_a
[>] result_b

[r] |CombineResults
[<] .a: pg\int << result_a
[<] .b: pg\int << result_b
[>] .final: pg\int >> output

[o] .final: pg\int
[X]
```

### Error Handling

```polyglot
[|] SafeFileRead
[i] .file: pg\path

[r] |U.File.Read
[<] .path: pg\path << .file
[>] .content: pg\string >> file_content

[!] !pg.FileSystem.NotFound
[>] .message: pg\string >> err_msg
[r] |U.Log.Error
[<] .msg: pg\string << err_msg
[r] .file_content: pg\string << "default content"

[o] .content: pg\string
[X]
```

---

## 16. Path System

### Path Identifiers

```polyglot
// Define path identifier
[#] Path.Identifiers.MyApp.DataDir
[A] DataDir
[<] .unix: pg\path << \\UnixRoot\\opt\myapp\data\
[<] .windows: pg\path << \\C\\ProgramData\MyApp\Data\
[X]

// Use path identifier
[r] .config_file: pg\path << \\DataDir\\config.json
[r] .log_file: pg\path << \\DataDir\\logs\app.log
```

### Built-in Path Roots

```polyglot
\\UnixRoot\\          // Root for Unix paths
\\WindowsRoot\\       // Root for Windows paths (C:\)
\\NoPath\\            // Path doesn't exist on this OS
```

### Path Examples

```polyglot
// Unix path
[<] .unix: pg\path << \\UnixRoot\\opt\app\data\

// Windows path
[<] .windows: pg\path << \\C\\ProgramData\App\Data\

// Cross-platform via identifier
[r] .file: pg\path << \\DataDir\\records.csv
```

---

## 17. Package System

### Package Access Syntax

```polyglot
// Pipeline from package
@packageName|PipelineName

// Enumeration from package
@packageName#EnumerationName

// Three-tier registry
@Local.mypackage|Pipeline           // Local package
@Community.hasan@utils|Transform    // Community package
@Company.acme@internal|Process      // Company package
```

### Package Examples

```polyglot
// Call pipeline from package
[r] @Community.hasan@DataUtils|Transform
[<] .input: pg\string << data
[>] .output: pg\string >> result

// Use enumeration from package
[i] .config: @Company.acme@InternalLib#Configuration
```

---

## Quick Syntax Cheat Sheet

```polyglot
// Pipeline
[|] Name [X]

// Input
[i] .name: type
[i] Fixed .name: type << value
[i] Default .name: type << value

// Output
[o] .name: type
[o] #None

// Run
[r] |Pipeline
[r] .variable: type << value

// Parallel
[p] |Pipeline
[Y] |Y.Join

// Error
[!] !Error
[!] !Error [X]

// Enum
[#] Enum [X]

// Types
pg\int, pg\string, pg\bool, pg\path, pg\dt
pg\array{type}, pg\set{type}, pg\serial
pg.mutable\type

// Operators
| pipeline, ~ unpack, @ package
# enum, ! error, << push, >> pull

// Comments
// single
/* multi */
```

---

**End of Quick Language Reference**