---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: pipeline-structure
shard: false

# --- Classification ---
type: spec
topic: Pipeline Structure & Execution Order
summary: Pipeline Structure & Execution Order specification
keywords:
  - control-flow
  - execution
  - pipelines

# --- BMAD Agent Routing ---
agents:
  - architect
  - developer
phase: solutioning
workflow: any
module: bmm
complexity: medium

# --- Dependency Chain ---
prereqs:
  - language-syntax
  - type-system
unlocks:
  - advanced-features

# --- Relationships ---
related:
  []
parent: language-control-flow

# --- Metadata ---
status: stable
updated: 2025-12-16
version: 0.0.4
tags:
  - "#control-flow"
  - "#spec"
---
# Pipeline Structure & Execution Order

**What You'll Learn:**
- Complete pipeline execution order
- Input parameters as implicit triggers
- Required vs optional pipeline components
- Wrapper structure (setup + cleanup)
- Queue behavior and control
- Common pipeline patterns

---

## Pipeline Anatomy

A complete pipeline has the following structure:

```polyglot
{|} |PipelineName                  // 1. Definition
[%] %Doc << "Description"          // 2. Metadata (optional)

[|] <input_param :type             // 3. Input parameters
[|] >output_param :type            // 4. Output parameters

[t] |T.TriggerType                 // 5. Trigger (REQUIRED)
[Q] |Q.QueueType                   // 6. Queue (optional, has default)
[W] |W.WrapperType                 // 7. Wrapper (REQUIRED)

   // 8. Logic (pipeline body)
   [r] $variable << value
   // ... more logic ...

{x}                                // 9. Close
```

---

## Execution Order

When a pipeline is called, execution follows this **strict order**:

```
1. Inputs (implicit triggers)
   ↓
2. Trigger [t]
   ↓
3. Queue [Q]
   ↓
4. Wrapper [W] Setup
   ↓
5. Logic (pipeline body)
   ↓
6. Wrapper [W] Cleanup
   ↓
7. Outputs
```

### Why This Order Matters

**Inputs are checked first:**
- Pipeline **will not execute** until all inputs are **Final** or **Default**
- This makes inputs **implicit triggers**

**Trigger determines execution conditions:**
- When should this pipeline run?
- Examples: On call, on schedule, on event

**Queue controls concurrency:**
- How many instances can run simultaneously?
- Serial, parallel, limited pool, etc.

**Wrapper provides runtime environment:**
- Setup: Prepare execution environment
- Cleanup: Tear down environment (even on error)

**Logic executes in controlled environment:**
- All prerequisites met
- Environment prepared
- Cleanup guaranteed

---

## 1. Input Parameters (Implicit Triggers)

### Input Parameters as Triggers

**Critical Concept:** Pipeline execution is **triggered** when all input parameters reach **Final** or **Default** state.

```polyglot
{|} |ProcessOrder
[|] <order_id :string              // Input 1
[|] <customer_id :string           // Input 2

[t] |T.Call                        // Explicit trigger
[W] |W.Polyglot.Scope

   [r] $order << |Database.Orders.Find
   [|] <order_id << $order_id      // PULL from input (must be Final/Default)

{x}

// Calling the pipeline:
[r] |ProcessOrder
[|] <order_id << "ORD-123"         // Pending → Final
[|] <customer_id << "CUST-456"     // Pending → Final
// Pipeline executes when BOTH inputs are Final
```

**Behavior:**
- If any input is **Pending**, pipeline **waits**
- When all inputs are **Final** or **Default**, pipeline **executes**
- This creates **data-driven execution**

### Inputs with Defaults

```polyglot
{|} |ProcessWithDefaults
[|] <timeout :int
   [%] %default << "30"            // Default value

[|] <retries :int
   [%] %default << "3"             // Default value

[t] |T.Call
[W] |W.Polyglot.Scope

   // $timeout and $retries are at least Default (or Final if overridden)

{x}

// Call without providing values:
[r] |ProcessWithDefaults
// Pipeline executes with defaults (both inputs are Default)

// Call with overrides:
[r] |ProcessWithDefaults
[|] <timeout << 60                 // Override: Default → Final
// Pipeline executes with timeout=60, retries=3
```

**When inputs have defaults:**
- Input starts **Pending**
- Receives default value → **Default**
- May receive caller value → **Final**
- Pipeline executes when all inputs are at least **Default**

### Optional Inputs (Future Feature)

**Not available in v0.0.4**, but planned:

```polyglot
[|] <optional_param :type
   [%] %optional << "true"
```

Currently, use defaults with sentinel values:

```polyglot
[|] <optional_param :string
   [%] %default << ""              // Empty string = not provided
```

---

## 2. Trigger `[t]` - REQUIRED

### Purpose

Defines **when** and **how** the pipeline should execute.

### Syntax

```polyglot
[t] |T.TriggerType
```

### Common Trigger Types

#### `|T.Call` - On Explicit Call (Most Common)

```polyglot
{|} |MyPipeline
[|] <input :string

[t] |T.Call                        // Execute when called
[W] |W.Polyglot.Scope

   // Logic
{x}

// Call explicitly:
[r] |MyPipeline <input << "value"
```

**Use when:** Pipeline should only run when explicitly called.

#### `|T.Schedule` - On Schedule

```polyglot
{|} |DailyReport
[t] |T.Schedule
   [.] .cron << "0 0 * * *"        // Every midnight
[W] |W.Polyglot.Scope

   // Generate daily report
{x}
```

**Use when:** Pipeline should run on a schedule (cron-like).

#### `|T.Event` - On Event

```polyglot
{|} |OnOrderCreated
[|] <order_event :event.order

[t] |T.Event
   [.] .source << "orders"
   [.] .type << "created"
[W] |W.Polyglot.Scope

   // React to order creation
{x}
```

**Use when:** Pipeline should react to external events.

#### `|T.Watch` - On File/Resource Change

```polyglot
{|} |OnConfigChange
[t] |T.Watch
   [.] .path << "/etc/config.yaml"
[W] |W.Polyglot.Scope

   // Reload configuration
{x}
```

**Use when:** Pipeline should run when a file or resource changes.

### Trigger is REQUIRED

**Every pipeline MUST have a trigger:**

```polyglot
{|} |BadPipeline
[|] <input :string

// Missing [t] marker!
[W] |W.Polyglot.Scope              // ❌ ERROR: No trigger defined

   // Logic
{x}
```

**Fix:**

```polyglot
{|} |GoodPipeline
[|] <input :string

[t] |T.Call                        // ✅ Trigger defined
[W] |W.Polyglot.Scope

   // Logic
{x}
```

---

## 3. Queue `[Q]` - Optional (Has Default)

### Purpose

Controls **how many instances** of the pipeline can run concurrently.

### Syntax

```polyglot
[Q] |Q.QueueType
```

### Common Queue Types

#### `|Q.Serial` - One at a Time (FIFO)

```polyglot
{|} |ProcessPayment
[|] <payment_id :string

[t] |T.Call
[Q] |Q.Serial                      // Only one instance at a time
[W] |W.Polyglot.Scope

   // Process payment (must be sequential)
{x}
```

**Behavior:**
- If pipeline is running, new calls **wait in queue**
- Executes in **FIFO order**

**Use when:** Operations must be sequential (e.g., database transactions, payments).

#### `|Q.Parallel` - Unlimited Concurrent Instances

```polyglot
{|} |FetchUserData
[|] <user_id :string

[t] |T.Call
[Q] |Q.Parallel                    // Unlimited concurrent instances
[W] |W.Polyglot.Scope

   // Fetch user data (independent operations)
{x}
```

**Behavior:**
- Every call starts **new instance**
- No limit on concurrent instances

**Use when:** Operations are independent and can run concurrently.

#### `|Q.Pool` - Limited Concurrent Instances

```polyglot
{|} |ProcessLargeFile
[|] <file_path :string

[t] |T.Call
[Q] |Q.Pool
   [.] .size << 5                  // Max 5 concurrent instances
[W] |W.Polyglot.Scope

   // Process file (resource-intensive)
{x}
```

**Behavior:**
- Up to `size` instances run concurrently
- Additional calls **wait** until slot available

**Use when:** Operations are resource-intensive and need throttling.

#### `|Q.Debounce` - Ignore Rapid Calls

```polyglot
{|} |SearchAutocomplete
[|] <query :string

[t] |T.Call
[Q] |Q.Debounce
   [.] .delay << 300               // 300ms delay
[W] |W.Polyglot.Scope

   // Perform search
{x}
```

**Behavior:**
- Waits for `delay` milliseconds of inactivity
- Cancels previous pending calls

**Use when:** Handling rapid user input (search, autocomplete).

### Default Queue Behavior

**If `[Q]` is omitted:**

```polyglot
{|} |MyPipeline
[|] <input :string

[t] |T.Call
// No [Q] specified
[W] |W.Polyglot.Scope              // Uses default queue: |Q.Serial

   // Logic
{x}
```

**Default:** `|Q.Serial` (one instance at a time, FIFO).

---

## 4. Wrapper `[W]` - REQUIRED

### Purpose

Provides **runtime environment** with **setup** and **cleanup**.

### Structure

```
[W] |W.WrapperType
    ├── Setup Code (runs before logic)
    ├── Logic Execution Environment
    └── Cleanup Code (runs after logic, even on error)
```

**Wrapper contains:**
1. **Setup:** Initialize runtime, allocate resources
2. **Cleanup:** Tear down runtime, release resources (guaranteed)

### Common Wrapper Types

#### `|W.Polyglot.Scope` - Pure Polyglot Execution

```polyglot
{|} |MyPipeline
[|] <input :string

[t] |T.Call
[W] |W.Polyglot.Scope              // Pure Polyglot, no external runtime

   [r] $result << |ProcessInPolyglot <input << $input

{x}
```

**Use when:** Pipeline uses only Polyglot constructs.

#### `|W.RT.Python3.12` - Python Runtime

```polyglot
{|} |AnalyzeData
[|] <data :array.float

[t] |T.Call
[W] |W.RT.Python3.12               // Python 3.12 runtime

   [r] $analysis << |Python.NumPy.Analyze <data << $data
   // Python NumPy available in this scope

{x}
```

**Setup:**
- Initialize Python 3.12 interpreter
- Load NumPy libraries

**Cleanup:**
- Release Python objects
- Shut down interpreter

#### `|W.RT.Rust` - Rust Runtime

```polyglot
{|} |HighPerformanceCompute
[|] <matrix :array.array.float

[t] |T.Call
[W] |W.RT.Rust                     // Rust runtime

   [r] $result << |Rust.MatrixMultiply <input << $matrix

{x}
```

**Setup:**
- Initialize Rust runtime
- Allocate memory

**Cleanup:**
- Free memory
- Shut down runtime

#### `|W.DB.Transaction` - Database Transaction

```polyglot
{|} |UpdateUserAndLog
[|] <user_id :string
[|] <new_email :string

[t] |T.Call
[W] |W.DB.Transaction              // Database transaction wrapper
   [.] .connection << "main_db"

   [r] |Database.Users.Update
   [|] <user_id << $user_id
   [|] <email << $new_email

   [r] |Database.AuditLog.Insert
   [|] <user_id << $user_id
   [|] <action << "email_updated"

{x}
```

**Setup:**
- BEGIN TRANSACTION

**Cleanup (success):**
- COMMIT TRANSACTION

**Cleanup (error):**
- ROLLBACK TRANSACTION

### Wrapper is REQUIRED

**Every pipeline MUST have a wrapper:**

```polyglot
{|} |BadPipeline
[|] <input :string

[t] |T.Call
// Missing [W] marker!

   [r] $result << |Process         // ❌ ERROR: No wrapper defined
{x}
```

**Fix:**

```polyglot
{|} |GoodPipeline
[|] <input :string

[t] |T.Call
[W] |W.Polyglot.Scope              // ✅ Wrapper defined

   [r] $result << |Process
{x}
```

### Wrapper Cleanup is Guaranteed

**Even if an error occurs, cleanup ALWAYS runs:**

```polyglot
{|} |WithCleanup
[t] |T.Call
[W] |W.DB.Transaction

   [r] |Database.Insert             // Might throw error
   [z][!] *! ? "Insert failed"

{x}  // Cleanup (ROLLBACK) runs even if error occurred
```

This is similar to `finally` blocks in other languages.

---

## 5. Pipeline Logic

### Execution Environment

The pipeline body executes **inside the wrapper**:

```polyglot
{|} |MyPipeline
[t] |T.Call
[W] |W.Polyglot.Scope

   // This code runs AFTER wrapper setup
   // This code runs BEFORE wrapper cleanup

   [r] $step1 << |FirstStep
   [r] $step2 << |SecondStep <input << $step1
   [|] >result << $step2

{x}  // Wrapper cleanup runs here
```

### Execution is Sequential (Unless Marked Parallel)

```polyglot
{|} |SequentialSteps
[t] |T.Call
[W] |W.Polyglot.Scope

   [r] $step1 << |Step1            // Executes first
   [r] $step2 << |Step2            // Waits for step1, then executes
   [r] $step3 << |Step3            // Waits for step2, then executes

{x}
```

**Parallel execution:**

```polyglot
{|} |ParallelSteps
[t] |T.Call
[W] |W.Polyglot.Scope

   [p] $task1 << |Task1            // Executes concurrently
   [p] $task2 << |Task2            // Executes concurrently
   [p] $task3 << |Task3            // Executes concurrently

   // Wait for all to complete before proceeding

{x}
```

---

## 6. Output Parameters

### Outputs are Evaluated Last

```polyglot
{|} |MyPipeline
[|] <input :string
[|] >output :string                // Output declaration

[t] |T.Call
[W] |W.Polyglot.Scope

   [r] $result << |Process <input << $input
   [|] >output << $result          // Output assignment

{x}
```

**Execution order:**
1. Inputs validated (Final or Default)
2. Trigger fires
3. Queue checked
4. Wrapper setup
5. Logic executes (`$result` computed)
6. Outputs assigned (`>output << $result`)
7. Wrapper cleanup
8. Outputs available to caller

### Outputs Must Be Assigned Before Exit

```polyglot
{|} |BadPipeline
[|] >result :string                // Output declared

[t] |T.Call
[W] |W.Polyglot.Scope

   [r] $temp << "value"
   // Forgot to assign >result!

{x}  // ❌ ERROR: >result is still Pending
```

**Fix:**

```polyglot
{|} |GoodPipeline
[|] >result :string

[t] |T.Call
[W] |W.Polyglot.Scope

   [r] $temp << "value"
   [|] >result << $temp            // ✅ Output assigned

{x}
```

---

## 7. %Inline Metadata - Making Pipelines Inline-Callable

### Overview

**`%Inline` metadata** enables pipelines to be called using **inline syntax** with formatted string templates:

```polyglot
// Instead of:
[r] |U.Math.Add
[|] <x << 5
[|] <y << 3
[|] >result >> $sum

// Use inline syntax:
[r] $sum :pg.int << |U.Math.Add"{5}, {3}"
```

**This is the most common feature in Polyglot** - all standard library utilities support inline calls.

---

### %Inline Metadata Structure

```polyglot
{|} |MyPipeline
[%] %Inline
   [%] |MyPipeline.FormattedString.For.MyPipeline    // Formatter pipeline
   [|] <formatted_string:pg.string << %Inline.FormattedString  // Compiler-populated
   [|] >param1:type >> <input1                       // Wire formatter outputs
   [|] >param2:type >> <input2                       // to main inputs

[|] <input1:type
[|] <input2:type
[|] >result:type

[t] |T.Call
[W] |W.Polyglot.Scope
   // Pipeline logic
{x}
```

**Key Components:**
1. **`%Inline` metadata block** - Marks pipeline as inline-callable
2. **Formatter pipeline reference** - Parses formatted string into parameters
3. **`%Inline.FormattedString`** - Special variable (compiler-populated with formatted string)
4. **Output wiring** - `>formatter_output >> <main_input` connects formatter to main pipeline

---

### Complete Example

**Main Pipeline:**
```polyglot
{|} |U.Math.Add
[%] %Doc << "Add two numbers"

[%] %Inline
   [%] |U.Math.Add.FormattedString.For.Add
   [|] <formatted_string:pg.string << %Inline.FormattedString
   [|] >x:pg.int >> <x
   [|] >y:pg.int >> <y

[|] <x:pg.int
[|] <y:pg.int
[|] >result:pg.int

[t] |T.Call
[W] |W.Polyglot.Scope
   [r] $sum :pg.int << |Internal.Add <x << $x <y << $y
   [|] >result << $sum
{x}
```

**Formatter Pipeline:**
```polyglot
{|} |U.Math.Add.FormattedString.For.Add
[|] <formatted_string:pg.string       // Receives "5, 3"
[|] >x:pg.int
[|] >y:pg.int

[t] |T.Call
[W] |W.Polyglot.Scope
   // Parse: "5, 3" → split by comma → x=5, y=3
   [r] $parts << |U.String.Split <input << $formatted_string <delimiter << ", "
   [r] $x :pg.int << |Parse.Int"{$parts[0]}"
   [r] $y :pg.int << |Parse.Int"{$parts[1]}"

   [|] >x << $x
   [|] >y << $y
{x}
```

**Usage:**
```polyglot
[r] $sum :pg.int << |U.Math.Add"{$a}, {$b}"
// Behind the scenes:
// 1. Convert $a, $b to strings (parallel)
// 2. Substitute into template: "5, 3"
// 3. Formatter parses: x=5, y=3
// 4. Main pipeline computes: result=8
```

---

### Output Marking with %Inline.Output

For pipelines with **multiple outputs**, mark which one is returned inline:

```polyglot
{|} |Process
[|] <input:pg.string
[|] >result:pg.string
   [%] %Inline.Output << #True        // ✅ This is the inline result
[|] >debug_info:pg.string            // Ignored in inline calls

[t] |T.Call
[W] |W.Polyglot.Scope
   [|] >result << $main_result
   [|] >debug_info << $debug_data
{x}

// Usage:
[r] $output :pg.string << |Process"{$data}"  // Captures >result only
```

**Output Rules:**
- **Single output** → Implied (automatically used)
- **Multiple outputs + `%Inline.Output` marker** → That output used
- **Multiple outputs + no marker** → Returns `:pg.serial` with all outputs as key-value pairs

---

### Multiple Format Styles

Support multiple inline formats using conditionals:

```polyglot
{|} |FlexiblePipeline
[%] %Inline
   [f] %Inline.FormattedString re? "^\\d+,\\d+$"        // CSV format
      [%] |FlexiblePipeline.FormattedString.CSV
      [|] <formatted_string << %Inline.FormattedString
      [|] >x >> <x
      [|] >y >> <y

   [f] %Inline.FormattedString re? "^\\{.*\\}$"        // JSON format
      [%] |FlexiblePipeline.FormattedString.JSON
      [|] <formatted_string << %Inline.FormattedString
      [|] >x >> <x
      [|] >y >> <y

[|] <x:pg.int
[|] <y:pg.int
[|] >result:pg.string

[t] |T.Call
[W] |W.Polyglot.Scope
   [|] >result << $computed
{x}
```

---

### See Also

**For complete inline pipeline documentation:**
- [Inline Pipelines Specification](../../User/language/advanced/inline-pipelines.md) - Complete details
- [Formatted String Templates](../../User/language/advanced/inline-pipelines.md#formatted-string-template-syntax) - Template syntax
- [Three-Phase Execution](../../User/language/advanced/inline-pipelines.md#three-phase-execution-model) - How it works

---

## Complete Execution Flow Example

```polyglot
{|} |ProcessOrder
// --- DEFINITION ---
[%] %Doc << "Processes customer order"

// --- INPUTS (Implicit Triggers) ---
[|] <order_id :string              // Must be Final/Default to execute
[|] <priority :string
   [%] %default << "medium"        // Provides default (becomes Default state)

// --- OUTPUTS ---
[|] >result :string
[|] >status :int

// --- TRIGGER (REQUIRED) ---
[t] |T.Call                        // Execute when called

// --- QUEUE (Optional, defaults to |Q.Serial) ---
[Q] |Q.Serial                      // One instance at a time

// --- WRAPPER (REQUIRED) ---
[W] |W.Polyglot.Scope              // Pure Polyglot environment
   // Setup runs here (before logic)

   // --- LOGIC ---
   [r] $order << |Database.Orders.Find
   [|] <order_id << $order_id      // PULL from input (Final/Default)

   [r] $processed << |ProcessOrderLogic
   [|] <order << $order
   [|] <priority << $priority

   // --- OUTPUTS ASSIGNMENT ---
   [|] >result << "Order processed"
   [|] >status << 200

   // Cleanup runs here (after logic, even on error)

{x}

// --- CALLING THE PIPELINE ---
[r] |ProcessOrder
[|] <order_id << "ORD-123"         // Pending → Final
[|] <priority << "high"            // Pending → Final (overrides default)
[|] >result >> $order_result       // Capture output
[|] >status >> $http_status        // Capture output

// Execution order when called:
// 1. Check inputs: order_id (Final), priority (Final) ✅
// 2. Trigger fires: |T.Call ✅
// 3. Queue checks: |Q.Serial - is instance running? No ✅
// 4. Wrapper setup: |W.Polyglot.Scope initializes
// 5. Logic executes
// 6. Outputs assigned
// 7. Wrapper cleanup: |W.Polyglot.Scope tears down
// 8. Outputs returned to caller ($order_result, $http_status)
```

---

## Pipeline Composition with `|>`

### Purpose

The **pipeline composition operator** (`|>`) allows you to **chain multiple pipelines** in sequence, creating a **data transformation pipeline**.

**Syntax:**
```polyglot
[r] |Pipeline1 |> |Pipeline2               // Chain Pipeline1 → Pipeline2
[|] <input:datatype << $source            // Input to Pipeline1
[|] >output1:datatype >> <input2          // Pipeline1 output → Pipeline2 input
[|] |> |Pipeline3                          // Chain Pipeline2 → Pipeline3
[|] >output2:datatype >> <input3          // Pipeline2 output → Pipeline3 input
[|] |>                                     // End chain
[|] >final_output:datatype >> $result     // Capture Pipeline3 output
```

**Flow:**
```
$source → Pipeline1 → output1 → Pipeline2 → output2 → Pipeline3 → $result
```

**Critical Rule:** Each `|>` chain segment must be on its own line, following the "one line = one marker + one expression" principle. This eliminates ambiguity in parameter mapping.

### How It Works

**Each pipeline in the chain:**
1. Receives inputs (from variables or previous pipeline outputs)
2. Processes the data
3. Outputs are wired to next pipeline's inputs using `>param >> <nextParam`
4. Final pipeline outputs are wired to variables
5. Chain segments use `[r] |PipeA |> |PipeB` or `[|] |> |PipeC` format
6. Chain ends with `[|] |>` alone (no following pipeline)

**Key concept:** The `[|]` marker + `>>` operator **explicitly wires outputs to inputs**. Each chain segment gets its own line for clarity.

---

### Simple Composition

**Basic chaining:**
```polyglot
[r] |String.Trim |> |String.Lower             // Chain Trim → Lower
[|] <input:pg.string << $raw_input           // Input to Trim
[|] >trimmed:pg.string >> <input             // Trim output → Lower input
[|] |> |String.Validate                       // Chain Lower → Validate
[|] >lowered:pg.string >> <input             // Lower output → Validate input
[|] |>                                        // End chain
[|] >validated:pg.string >> $clean_email     // Capture Validate output
```

**Equivalent sequential calls:**
```polyglot
[r] |String.Trim
[|] <input:pg.string << $raw_input
[|] >trimmed:pg.string >> $temp1

[r] |String.Lower
[|] <input:pg.string << $temp1
[|] >lowered:pg.string >> $temp2

[r] |String.Validate
[|] <input:pg.string << $temp2
[|] >validated:pg.string >> $clean_email
```

**Benefit:** Composition is **more readable** and **eliminates temporary variables**.

---

### Composition with Multiple Inputs

**Passing additional parameters to pipelines:**
```polyglot
[r] |Parse.Int |> |Math.Double                // Chain Parse → Double
[|] <input:pg.string << $data                 // Input to Parse
[|] >value:pg.int >> <input                   // Parse output → Double input
[|] |> |Math.Add                               // Chain Double → Add
[|] >doubled:pg.int >> <x                     // Double output → Add's <x input
[|] <y:pg.int << 10                           // Additional input to Add
[|] |>                                         // End chain
[|] >result:pg.int >> $final_value            // Capture Add output
```

**How it works:**
1. `$data` → `|Parse.Int` → integer output `value`
2. `value` → `|Math.Double` → `doubled` output
3. `doubled` → `|Math.Add` with `<y << 10` → `result`
4. `result` → `$final_value`

**Additional inputs** (like `<y << 10`) are provided alongside chained outputs. Each chain segment (`|>`) gets its own line.

---

### Multi-Step Data Processing

**Real-world example:**
```polyglot
[r] |Clean.RemoveWhitespace |> |Transform.Normalize    // Chain Clean → Transform
[|] <input:pg.string << $raw_data                      // Input to Clean
[|] >cleaned:pg.string >> <input                       // Clean output → Transform input
[|] |> |Validate.Schema                                 // Chain Transform → Validate
[|] >normalized:pg.string >> <input                    // Transform output → Validate input
[|] <schema:pg.serial << $my_schema                    // Additional input to Validate
[|] |> |Store.Save                                      // Chain Validate → Store
[|] >validated:pg.serial >> <data                      // Validate output → Store input
[|] <table:pg.string << "processed_data"               // Additional input to Store
[|] |>                                                  // End chain
[|] >success:pg.bool >> $stored                        // Capture Store output
```

**Pattern:** Each step transforms data and wires output to next step's input. Each `|>` chain segment on its own line.

---

### Composition with Error Handling

**Handling errors in composed pipelines:**
```polyglot
[r] |Trim |> |Lower                              // Chain Trim → Lower
[|] <input:pg.string << $username                // Input to Trim
[|] >trimmed:pg.string >> <input                 // Trim output → Lower input
[|] |> |RemoveSpecialChars                       // Chain Lower → RemoveSpecialChars
[|] >lowered:pg.string >> <input                 // Lower output → RemoveSpecialChars input
[|] |> |Validate                                  // Chain RemoveSpecialChars → Validate
[|] >cleaned:pg.string >> <input                 // RemoveSpecialChars output → Validate input
[|] |>                                            // End chain
[|] >validated:pg.string >> $result              // Capture Validate output
[z][!] *! >> $validation_error
```

**Behavior:**
- If **any** pipeline in the chain fails (enters Faulted state), error propagates
- The `[z][!] *!` handler catches errors from the entire composition
- `$result` enters **Faulted** state if any step fails

---

### Composition vs Sequential Calls

**Using composition:**
```polyglot
[r] |Step1 |> |Step2                             // Chain Step1 → Step2
[|] <input:pg.string << $value                   // Input to Step1
[|] >out1:pg.int >> <input                       // Step1 output → Step2 input
[|] |> |Step3                                     // Chain Step2 → Step3
[|] >out2:pg.float >> <input                     // Step2 output → Step3 input
[|] |>                                            // End chain
[|] >final:pg.string >> $result                  // Capture Step3 output
```

**Using sequential calls:**
```polyglot
[r] |Step1
[|] <input:pg.string << $value
[|] >out1:pg.int >> $temp1

[r] |Step2
[|] <input:pg.int << $temp1
[|] >out2:pg.float >> $temp2

[r] |Step3
[|] <input:pg.float << $temp2
[|] >final:pg.string >> $result
```

**When to use composition:**
- Data flows linearly through transformations
- No need to inspect intermediate results
- Emphasize the **transformation chain**

**When to use sequential calls:**
- Need to inspect or branch on intermediate results
- Complex logic between steps
- Debugging intermediate values

---

### Composition Pattern Guidelines

**✅ Good use of composition:**
```polyglot
// Clear linear transformation
[r] |String.Trim |> |String.Lower                // Chain Trim → Lower
[|] <input:pg.string << $raw_input               // Input to Trim
[|] >trimmed:pg.string >> <input                 // Trim output → Lower input
[|] |> |Email.Validate                            // Chain Lower → Validate
[|] >lowered:pg.string >> <input                 // Lower output → Validate input
[|] |>                                            // End chain
[|] >email:pg.string >> $clean_email             // Capture Validate output
```

**❌ Avoid over-composition:**
```polyglot
// Too complex - violates one-line-one-expression rule
[r] |Step1 |> |Step2 |> |Step3 |> |Step4 |> |Step5 |> |Step6
[|] <input << $value
// ... too many wiring lines, hard to debug ...
```

**✅ Better - break into logical groups:**
```polyglot
[r] |Clean.Trim |> |Clean.Normalize              // Chain Trim → Normalize
[|] <input:pg.string << $raw_input               // Input to Trim
[|] >trimmed:pg.string >> <input                 // Trim output → Normalize input
[|] |>                                            // End chain
[|] >cleaned:pg.string >> $intermediate1         // Capture Normalize output

[r] |Validate.Format |> |Validate.Schema         // Chain Format → Schema
[|] <input:pg.string << $intermediate1           // Input to Format
[|] >formatted:pg.string >> <input               // Format output → Schema input
[|] |>                                            // End chain
[|] >validated:pg.string >> $intermediate2       // Capture Schema output

[r] |Transform.Encode |> |Store.Save             // Chain Encode → Save
[|] <input:pg.string << $intermediate2           // Input to Encode
[|] >encoded:pg.string >> <data                  // Encode output → Save input
[|] |>                                            // End chain
[|] >stored:pg.bool >> $result                   // Capture Save output
```

---

### Composition with Conditionals

**Conditional pipeline selection:**
```polyglot
[r] $processor:pg.pipeline << |Text.Parse

[f] $format =? "json"
   [r] $processor << |JSON.Parse
[f] $format =? "xml"
   [r] $processor << |XML.Parse

[r] $processor |> |Validate
[|] <input:pg.string << $raw_data
[|] >parsed:pg.serial >> <input
[|] |>
[|] >validated:pg.serial >> $result
```

**Pattern:** Select pipeline based on condition, then use in composition.

---

### See Also

- [Pipeline Composition Examples](../design-history/syntax-refinement/pipeline-composition-examples.md) - Detailed examples
- [Operators - Pipeline Composition](./operators.md#-pipeline-composition) - `|>` operator details

---

## Common Patterns

### Pattern 1: Simple Data Processing

```polyglot
{|} |Transform
[|] <input :string
[|] >output :string

[t] |T.Call
[W] |W.Polyglot.Scope

   [r] $result << |ProcessData <data << $input
   [|] >output << $result

{x}
```

### Pattern 2: With Default Parameters

```polyglot
{|} |ProcessWithDefaults
[|] <data :string
[|] <timeout :int
   [%] %default << "30"

[t] |T.Call
[W] |W.Polyglot.Scope

   [r] $result << |Process
   [|] <data << $data
   [|] <timeout << $timeout

{x}
```

### Pattern 3: Parallel Execution Pool

```polyglot
{|} |ExpensiveOperation
[|] <input :string

[t] |T.Call
[Q] |Q.Pool
   [.] .size << 10                 // Max 10 concurrent

[W] |W.RT.Python3.12

   [r] $result << |Python.HeavyCompute <data << $input

{x}
```

### Pattern 4: Database Transaction

```polyglot
{|} |UpdateWithTransaction
[|] <user_id :string
[|] <new_data :string

[t] |T.Call
[W] |W.DB.Transaction

   [r] |DB.Users.Update
   [|] <id << $user_id
   [|] <data << $new_data

   [r] |DB.AuditLog.Insert
   [|] <user_id << $user_id

{x}  // COMMIT if success, ROLLBACK if error
```

---

## Summary

### Execution Order (Strict)
1. **Inputs** - Implicit triggers (must be Final/Default)
2. **Trigger** `[t]` - When to execute
3. **Queue** `[Q]` - Concurrency control
4. **Wrapper Setup** `[W]` - Prepare environment
5. **Logic** - Pipeline body
6. **Wrapper Cleanup** `[W]` - Tear down (guaranteed)
7. **Outputs** - Return values

### Required Components
- **Trigger `[t]`** - REQUIRED, defines when pipeline executes
- **Wrapper `[W]`** - REQUIRED, provides runtime environment with cleanup

### Optional Components
- **Queue `[Q]`** - Optional (defaults to `|Q.Serial`)
- **Metadata `[%]`** - Optional documentation

### Key Concepts
- **Inputs are implicit triggers** - Pipeline waits until all inputs are Final/Default
- **Wrapper cleanup is guaranteed** - Runs even on error
- **Queue controls concurrency** - Serial, parallel, pool, debounce
- **Execution order is predictable** - Always the same sequence

---

## Related Documentation

- [Markers Reference](./markers.md) - `[t]`, `[Q]`, `[W]` markers
- [I/O Operators](./io-operators.md) - Input/output syntax
- [Variables & Lifecycle](./variables-lifecycle.md) - Variable states
- [Standard Library - Triggers](../standard-library/triggers/README.md) - Available trigger types
- [Standard Library - Wrappers](../User/stdlib/wrappers/README.md) - Available wrappers

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../README.md)
