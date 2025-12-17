# Block Markers

**Version:** 0.0.2
**Last Updated:** 2025-11-18
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
2. [Package Declaration Markers](#package-declaration-markers)
3. [Pipeline Structure Markers](#pipeline-structure-markers)
4. [Input/Output Markers](#inputoutput-markers)
5. [Execution Control Markers](#execution-control-markers)
6. [Data Flow Markers](#data-flow-markers)
7. [Synchronization Markers](#synchronization-markers)
8. [Trigger & Queue Markers](#trigger--queue-markers)
9. [Runtime Wrapper Markers](#runtime-wrapper-markers)
10. [Type Definition Markers](#type-definition-markers)
11. [Error Handling Markers](#error-handling-markers)
12. [Expansion & Nesting Markers](#expansion--nesting-markers)
13. [Macro System Markers](#macro-system-markers)
14. [Boolean Logic Markers](#boolean-logic-markers)
15. [String Processing Markers](#string-processing-markers)
16. [Switch/Conditional Enhancements](#switchconditional-enhancements)
17. [Additional Markers](#additional-markers)
18. [Block Element Hierarchy](#block-element-hierarchy)
19. [Best Practices](#best-practices)

---

## Block Marker Fundamentals

### What are Block Markers?

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

## Package Declaration Markers

### `[@]` - Package Declaration

**Purpose:** Declares the package identity and dependencies for the current file

**Position:** MUST be the first block in every Polyglot file

**Must be paired with:** `[X]` to close

**Full Syntax:**
```polyglot
[@] Registry@Package.Full.Name:Major.Minor.Patch
[#] file_number
[<] @Registry.package@DependencyName:Version
[<] @Registry.package@AnotherDependency:Version
[X]
```

---

#### Components

**Registry Types:**
- `Local` - Local development packages
- `Community` - Community shared packages
- `Company` - Company/organization packages

**Package Path:**
- Dot-separated hierarchical path
- Example: `MyApp.DataProcessor`, `Utils.FileHelpers`

**Version:**
- Semantic versioning: `Major.Minor.Patch`
- Example: `1.0.0`, `2.1.5`

**File Number:**
- `[#] N` where N is the file number within the package
- Enables multi-file packages

**Dependencies:**
- `[<] @Registry.package@Name:Version`
- Note the `.package` separator between registry and package name
- Each dependency on its own `[<]` line

---

#### Examples

**Simple Package (Single File):**
```polyglot
[@] Local@MyApp.HelloWorld:1.0.0
[#] 1
[X]
```

**Package with Dependencies:**
```polyglot
[@] Local@MyApp.DataProcessor:1.0.0
[#] 1
[<] @Community.utils@FileHelpers:2.0.0
[<] @Community.data@CSVParser:1.5.3
[X]
```

**Multi-File Package:**
```polyglot
// File 1
[@] Local@MyApp.LargeProject:1.0.0
[#] 1
[<] @Community.utils@Logger:3.0.0
[X]

// File 2
[@] Local@MyApp.LargeProject:1.0.0
[#] 2
[<] @Community.utils@Logger:3.0.0
[X]
```

---

#### Package Naming Rules

**Registry Prefix:**
- Always starts with registry name: `Local@`, `Community@`, `Company@`
- Case-sensitive: `Local` not `local`

**Package Path Rules:**
- Use PascalCase for each segment
- Dot-separated hierarchy: `MyApp.Module.SubModule`
- No special characters except dots
- Should reflect logical organization

**Version Format:**
- Must follow semantic versioning
- Format: `Major.Minor.Patch`
- All three components required
- Examples: `1.0.0`, `2.1.5`, `10.3.7`

---

#### Best Practices

**1. Always Declare Package First:**
```polyglot
// ✓ CORRECT - [@] is first
[@] Local@MyApp.Processor:1.0.0
[#] 1
[X]

[|] MyPipeline
[X]

// ✗ WRONG - Missing [@] block
[|] MyPipeline
[X]
```

**2. Use Semantic Versioning:**
- Major: Breaking changes
- Minor: New features, backward compatible
- Patch: Bug fixes, backward compatible

**3. Organize Dependencies Logically:**
```polyglot
[@] Local@MyApp.WebAPI:2.0.0
[#] 1
// Standard library / Core dependencies first
[<] @Community.utils@Logger:3.0.0
[<] @Community.utils@Config:2.1.0
// Feature-specific dependencies
[<] @Community.web@HTTPServer:5.0.0
[<] @Community.data@Database:4.2.1
[X]
```

**4. Keep File Numbers Sequential:**
- Start at 1
- No gaps in numbering
- Use to split large packages into logical files

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
[t] |T.Call                         // MANDATORY: Trigger
[W] |W.Polyglot.Scope              // MANDATORY: Wrapper

[r] |TransformData
[<] .data: pg\string << .input
[>] .result: pg\string >> .output

[o] .output: pg\string              // MANDATORY: Output
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
[i] .parameter: type << value
```

**Example:**
```polyglot
[|] ConnectToAPI
[i] .api_key: pg\string << "secret-key-123"  // Cannot be overridden
[i] .timeout: pg\int << 30
[r] |MakeAPICall
[<] .key: pg\string << .api_key
[X]
```

---

#### Default Input (Optional)

**Correct Syntax:** Use DEFAULT PUSH operator `<~`

```polyglot
[i] .parameter: type <~ default_value
```

**Example:**
```polyglot
[|] ProcessWithOptions
[i] .input: pg\string                 // Required input
[i] .max_size: pg\int <~ 1024         // DEFAULT PUSH - optional, defaults to 1024
[i] .debug: pg\bool <~ #Boolean.False // DEFAULT PUSH - optional, defaults to False
[t] |T.Call
[W] |W.Polyglot.Scope

[r] |Process
[<] .data: pg\string << .input
[<] .size: pg\int << .max_size
[>] .result: pg\string >> .output

[o] .output: pg\string
[X]
```

**Key Points:**
- Use `<~` (DEFAULT PUSH) for optional parameters with defaults
- Do NOT use "Default" keyword (Polyglot has no keywords)
- Do NOT use `<<` (PUSH) for defaults - that makes them immediately Ready

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
[t] |T.Call                         // MANDATORY: Trigger
[W] |W.Polyglot.Scope              // MANDATORY: Wrapper

[r] |ProcessData
[<] .data: pg\string << .input
[>] .result: pg\string >> .output

[o] .output: pg\string              // Output declaration
[X]

// Pipeline with no output
[|] LogMessage
[i] .message: pg\string
[t] |T.Call                         // MANDATORY: Trigger
[W] |W.Polyglot.Scope              // MANDATORY: Wrapper

[r] |U.Log.Info
[<] .msg: pg\string << .message

[o] !NoError                        // No output (use error type)
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
[|] SequentialWorkflow
[i] .input: pg\string
[t] |T.Call
[W] |W.Polyglot.Scope

// Sequential pipeline calls
[r] |Step1
[<] .data: pg\string << .input
[>] .result1: pg\string >> .out1

[r] |Step2
[<] .data: pg\string << .out1
[>] .result2: pg\string >> .out2

[r] |Step3
[<] .data: pg\string << .out2
[>] .result3: pg\string >> .final

// Sequential variable declarations
[r] .x: pg\int << 5
[r] .y: pg\int << 10
[r] .sum: pg\int << .x + .y

[o] .final: pg\string
[X]
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

### `[s]` - Serial Load Block

**Purpose:** Load serialized data (JSON, YAML, TOML, XML) from files with parallel execution and automatic error handling

**Syntax:**
```polyglot
[s] .variable_name: type << Format"path"
```

**Supported Formats:**
- `JSON"path"` - Load JSON file
- `YAML"path"` - Load YAML file
- `TOML"path"` - Load TOML file
- `XML"path"` - Load XML file

**Key Characteristics:**
- **Parallel Execution:** All `[s]` blocks at the same scope/level load in parallel with automatic join
- **Error-Carrying Variables:** Variables hold either data + `!NoError` OR `#None.ErrorState` + specific error
- **Two-Level Error Handling:** Variable-level (`.var.error`) and scope-level (`[s][!]`)
- **Partial Success:** Successful loads complete even when others fail
- **Wildcard Support:** Load multiple files with glob patterns
- **Chained Pipelines:** Transform file paths and filter results

---

#### Basic Usage

**Single File Loading:**
```polyglot
[s] .config: pg\serial << JSON"\\Config\\app.json"
[s] .users: #UserData << YAML"\\Data\\users.yaml"
[s] .settings: pg\serial << TOML"settings.toml"
```

**Parallel Loading (Automatic Join):**
```polyglot
[s] .db_config << JSON"db.json"       // Load in parallel
[s] .api_config << JSON"api.json"     // Load in parallel
[s] .cache_config << JSON"cache.json" // Load in parallel
// Automatic join before next operation
[r] |SetupDatabase
[<] .config: pg\serial << .db_config
```

**Three-Step Process:**
1. **Collect paths** - Resolve file paths (including wildcards)
2. **Load parallel** - Load all files concurrently
3. **Assign** - Assign results to variables

---

#### Wildcard/Array Loading

**Basic Wildcard:**
```polyglot
[s] .configs << JSON"\\Config\\*.json"  // Loads all JSON files
```

**Combination Strategies:**

**1. FilenameKey** - Use filename as key:
```polyglot
[s] .configs << JSON.FilenameKey"\\Config\\*.json"
// Result: { "app": {...}, "db": {...}, "cache": {...} }
```

**2. Index** - Use array index:
```polyglot
[s] .configs << JSON.Index"\\Config\\*.json"
// Result: [0: {...}, 1: {...}, 2: {...}]
```

**3. Merge** - Merge all objects:
```polyglot
[s] .config << JSON.Merge"\\Config\\*.json"
// Result: Single merged object
```

**4. Concat** - Concatenate all arrays:
```polyglot
[s] .items << JSON.Concat"\\Data\\*.json"
// Result: Single concatenated array
```

**5. FlatMap** - Flatten nested arrays:
```polyglot
[s] .all_items << JSON.FlatMap"\\Data\\*.json"
// Result: Flattened array
```

---

#### Chained Literal Pipelines (NEW FEATURE)

**Purpose:** Transform file paths and filter results before loading

**Syntax:**
```polyglot
Format.Method1"pattern".Method2"filter".Method3"transform"
```

**Filtering:**
```polyglot
[s] .secrets << JSON.FilenameKey"\\Secrets\\*.json".ExcludeFileName"*example*"
[s] .configs << YAML.FilenameKey"\\Config\\*.yaml".ExcludeFileName"*test*"
```

**String Interpolation:**
```polyglot
[s] .env_config << JSON"\\Config\\{.env}.json"
// If .env = "production", loads: \\Config\\production.json
```

---

#### Error Handling

**Variable-Level Error Checking:**
```polyglot
[s] .config << JSON"config.json"

[!] .config.error =? !File.NotFound
[r] |UseDefaultConfig

[o] .config.error  // Fail pipeline with error
```

**Scope-Level Error Handling (`[s][!]`):**
```polyglot
[s] .db_config << JSON"db.json"
[s] .api_config << JSON"api.json"

[s][!] !File.NotFound         // Catches ALL serial NotFound in scope
[>] .message >> err_msg
[r] |U.Log.Error
[<] .msg: pg\string << "Config file not found: {err_msg}"
[o] !ConfigurationError

[s][!] !JSON.ParseError       // Catches ALL serial ParseError in scope
[r] |HandleParseError
```

**Error Precedence:**
- `[~][!]` (specific - previous block) overrides `[s][!]` (general - all serial)

**Error Types:**
- `!File.NotFound` - File not found or empty (0 bytes)
- `!JSON.ParseError`, `!YAML.ParseError`, etc. - Parsing failures
- `!Serial.ReservedEnumeration.MissingField` - Validation failure for reserved types
- `!Serial.ReservedEnumeration.FieldMismatch` - Type mismatch in reserved types

**Implicit Error Notification:**
- If no explicit `[s][!]` handler: automatic logging/console output
- Context-aware (console display vs. log files)
- Critical for automated pipelines (failure visibility required)

---

#### Error-Carrying Variables

**Variable States:**

**Success:**
```polyglot
.variable = actual data
.variable.error = !NoError
```

**Failure:**
```polyglot
.variable = #None.ErrorState
.variable.error = specific error (e.g., !File.NotFound)
```

**Both parts always accessible:**
```polyglot
[s] .config << JSON"config.json"

// Check error state
[!] .config.error =? !File.NotFound
[r] |HandleMissingConfig

// Or use value directly if no error handler
[r] |ProcessConfig
[<] .data: pg\serial << .config
```

---

#### Type Loading & Validation

**Flexible Types (No Validation):**
```polyglot
[s] .data: pg\serial << JSON"data.json"          // No validation
[s] .custom: #UserEnum << JSON"custom.json"      // User enum, flexible
```

**Reserved Enumeration Validation:**
```polyglot
[s] .timestamp: #DT.ISO8601 << JSON"time.json"   // Validates at assignment (step 3)
```

**Validation Process:**
- Happens at step 3 (assignment)
- Missing mandatory fields → `!Serial.ReservedEnumeration.MissingField`
- Field type mismatch → `!Serial.ReservedEnumeration.FieldMismatch`

**Standard Library Support:**
- `|U.JSON.Load` - JSON loading pipeline
- `|U.YAML.Load` - YAML loading pipeline
- `|U.TOML.Load` - TOML loading pipeline
- `|U.XML.Load` - XML loading pipeline
- String literal syntax: `JSON"path"` invokes `|U.JSON.Load` internally

---

#### Integration with Other Blocks

**Scope & Level:**
```polyglot
// Same scope - shared error handling
[s] .config1 << JSON"c1.json"
[s] .config2 << JSON"c2.json"
[s][!] !File.NotFound   // Handles both

// Different scope - separate handling
[?] .env =? #Production
[~] [s] .config << JSON"prod.json"
[~] [s][!] !File.NotFound

[?] .env =? #Development
[~] [s] .config << JSON"dev.json"
[~] [s][!] !File.NotFound
```

**Execution Order:**
- Position: `[t]` > `[i]` > `[Q]` > `[\]` > **`[r],[p],[s],[b],[Y]`** > `[/]` > `[o]` > `[X]`
- `[s]` sits alongside `[r]`, `[p]`, `[b]` in execution blocks
- **NOT allowed in `[t]` triggers** (execution-only block)

**Immutability:**
- Default: loaded variables are immutable
- Discourages mutable variables in automation (reduces edge cases)

---

#### Complete Example

```polyglot
[@] MyApp.ConfigLoader
[<] @Community.utils@FileHelpers
[#] 001

[|] LoadApplicationConfig
[i] .env: #Environment
[t] |T.Call

[s] .base_config << JSON"\\Config\\base.json"
[s] .env_config << JSON"\\Config\\{.env}.json"
[s] .secrets << JSON.FilenameKey"\\Secrets\\*.json".ExcludeFileName"*example*"

[s][!] !File.NotFound
[>] .message >> err_msg
[r] |U.Log.Error
[<] .msg: pg\string << "Config file not found: {err_msg}"
[o] !ConfigurationError

[s][!] !JSON.ParseError
[r] |U.Log.Error
[<] .msg: pg\string << "Invalid JSON in config"
[o] !ConfigurationError

// Merge configs
[r] |MergeConfigs
[<] .base: pg\serial << .base_config
[<] .env: pg\serial << .env_config
[<] .secrets: pg\serial << .secrets
[>] .merged: pg\serial >> final_config

[o] .final_config: pg\serial
[X]
```

---

#### MVP Scope

**Included in MVP:**
- ✓ Basic file loading (JSON, YAML, TOML, XML)
- ✓ Parallel execution with automatic join
- ✓ Error-carrying variables
- ✓ Two-level error handling (`[s][!]`)
- ✓ Wildcard/array loading
- ✓ Chained literal pipelines
- ✓ Combination strategies (5 types)
- ✓ Reserved enumeration validation
- ✓ Filter syntax (chained ExcludeFileName)

**Post-MVP:**
- Security (path traversal, permissions)
- Caching (file re-reading optimization)
- Streaming (large file handling)
- Remote loading (HTTP, database)
- Compression/encryption

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

### `[W]` - Wrapper Context

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
[W] |W.Runtime
[W] |W.Runtime.Version
```

---

#### Fixed Version Wrappers

```polyglot
// Fixed Python version
[W] |W.Python3.11
[r] |RunPythonScript
[<] .script: pg\path << "analyze.py"

// Fixed Node version
[W] |W.Node20
[r] |RunJavaScript
[<] .script: pg\path << "process.js"

// Rust
[W] |W.Rust
[r] |RunRustCode
[<] .binary: pg\path << "processor"
```

---

#### Dynamic Version Wrappers

```polyglot
// Latest Python 3.x
[W] |W.Python
[r] |RunScript
[<] .script: pg\path << "script.py"

// Latest Node.js
[W] |W.Node
[r] |RunScript
[<] .script: pg\path << "app.js"
```

---

#### Multiple Wrappers in One Pipeline

```polyglot
[|] MultiRuntimePipeline
[i] .data: pg\string

// Python processing
[W] |W.Python3.11
[r] |PythonAnalyze
[<] .input: pg\string << .data
[>] .result: pg\string >> python_result

// Node.js processing
[W] |W.Node20
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
[<] .debug: pg\bool << #False
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
[W] |W.Polyglot.Scope

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

## Macro System Markers

### `[M]` - Macro Definition

**Purpose:** Define compile-time inline code templates for reusable setup/cleanup patterns

**Context:** Top-level (outside pipelines)

**Syntax:**
```polyglot
[M] MacroName
[>] Macro.Include"{\/}"  // Declare included block types (macro PUSHES blocks)
// Macro body...
[X]
```

**Direction Note:** Use `[>]` (output) for Macro.Include because macros **push** blocks outward to the caller, not pull them in.

**Example:**
```polyglot
[M] DatabaseSetup
[>] Macro.Include"{\/}"  // Macro pushes these block types to caller
[{] .db_host: pg\string  // Scope input
[}] .db_conn: pg\db      // Scope output

[\] |U.DB.Connect
[<] .host: pg\string << .db_host
[>] .connection: pg\db >> .db_conn

[/] |U.DB.Disconnect
[<] .conn: pg\db << .db_conn
[X]
```

---

### `[W]` - Macro Unwrap

**Purpose:** Inline macro at unwrap site (compile-time insertion)

**Context:** Within pipelines

**CRITICAL: Macro unwrap uses EXACT same syntax as pipeline calls**

**Syntax:**
```polyglot
[W] |MacroName
[<] .input_param: type << value   // Pass inputs (same as pipeline call)
[>] .output_var: type >> variable  // Receive outputs (same as pipeline call)
```

**Example:**
```polyglot
[|] MyPipeline
[W] |DatabaseSetup       // Unwrap macro
[<] .db_host: pg\string << "localhost"  // Pass input to macro
[>] .db_conn: pg\db >> .db              // Receive output from macro
// .db now available for use!
[r] |QueryDatabase
[<] .connection: pg\db << .db
[X]
```

**Complete Example with Macro Definition:**
```polyglot
// Define macro
[M] DBJsonMacro
[>] Macro.Include"{\/}"
[{] .db_config_file: pg\path      // Macro input
[\] .db: pg\db << JSON.Load"{.db_config_file}"
[\] |U.DataBase.Connect
[<] .db: pg\db << .db
[}] .db                           // Macro output (type inferred)
[/] |U.DataBase.Disconnect
[X]

// Use macro (same syntax as pipeline call)
[|] MyWorkflow
[W] |DBJsonMacro
[<] .db_config_file: pg\path << .db_config  // Input
[>] .db: pg\db >> .db1                      // Output
// .db1 now contains the database connection
[X]
```

**Behavior:**
- **Same as pipeline calls:** Use `[<]` for inputs, `[>]` for outputs
- Blocks insert by TYPE, not position
- Multiple macros: FIFO setup (`[\]`), LIFO cleanup (`[/]`)
- Type safety enforced at unwrap site
- Output types can be inferred from assignment

---

### `[{]` - Scope Input

**Purpose:** Declare variables flowing INTO macro from caller

**Context:** Within `[M]` macro definitions

**Syntax:**
```polyglot
[{] .variable_name: pg\type
```

**Example:**
```polyglot
[M] Logger
[{] .log_level: pg\string    // Input from caller
[{] .message: pg\string       // Input from caller
[}] .log_file: pg\file        // Output to caller

[\] |U.Log.Open
[<] .level: pg\string << .log_level
[>] .file: pg\file >> .log_file
[X]
```

**Flow:** Variables declared with `[{]` must be provided by caller at unwrap site

---

### `[}]` - Scope Output

**Purpose:** Declare variables flowing OUT of macro to caller

**Context:** Within `[M]` macro definitions

**Syntax:**
```polyglot
[}] .variable_name: pg\type
```

**Example:**
```polyglot
[M] CacheSetup
[{] .cache_size: pg\int      // IN from caller
[}] .cache_handle: pg\db     // OUT to caller

[\] |U.Cache.Initialize
[<] .size: pg\int << .cache_size
[>] .handle: pg\db >> .cache_handle
[X]
```

**Flow:** Variables declared with `[}]` become available in caller after macro unwrap

---

### `[i]` Input Variations

**Purpose:** Declare pipeline inputs with three state variations

**Context:** Pipeline inputs, macro inputs

**Three Variations:**

1. **Required Input (Declared state):**
```polyglot
[i] .variable_name: pg\type
```

2. **Default Input (DefaultReady state):**
```polyglot
[i] .variable_name: pg\type <~ default_value
```

3. **Constant Input (Ready state):**
```polyglot
[i] .variable_name: pg\type << constant_value
```

**Example:**
```polyglot
[|] ConfigPipeline
[i] .server_url: pg\string            // Required - caller must provide
[i] .max_retries: pg\int <~ 3         // Default - caller can override
[i] .timeout_seconds: pg\int << 30    // Constant - caller cannot override
[X]
```

**Behavior:**
- **Required:** Caller must provide value (Declared state, push count: 0)
- **Default:** Caller can override default (DefaultReady state, allows 1 override)
- **Constant:** Value fixed, caller cannot change (Ready state, no pushes allowed)
- No `Fixed` keyword needed (block-based, no keywords)

---

## Boolean Logic Markers

### `[&]` - AND Modifier

**Purpose:** Logical AND modifier - combines multiple conditions

**Context:** Within `[t]` trigger blocks and `[?]` switch blocks

**Closing:** Boolean blocks close by context (when nesting level changes) - NO explicit closing markers

**Syntax:**
```polyglot
[&] condition
```

**Example:**
```polyglot
[t] |T.Daily
[t] |T.Time         // First-level implicit AND
[&] .hour >? 9      // AND modifier
[&] .hour <? 17     // AND modifier
[r] |ProcessFile
```

**Note:** First-level triggers have implicit AND - explicit `[&]` only needed for additional conditions

---

### `[+]` - OR Modifier

**Purpose:** Logical OR modifier - alternate conditions

**Context:** Within `[t]` trigger blocks and `[?]` switch blocks

**Closing:** Boolean blocks close by context (when nesting level changes) - NO explicit closing markers

**Syntax:**
```polyglot
[+] condition
```

**Example:**
```polyglot
[?] .status =? "active"
[+] .status =? "pending"   // OR modifier
[r] |ProcessItem
```

---

### `[-]` - NOT Modifier

**Purpose:** Logical negation modifier

**Context:** Within trigger and switch blocks

**Closing:** Boolean blocks close by context (when nesting level changes) - NO explicit closing markers

**Syntax:**
```polyglot
[-] condition
```

**Example:**
```polyglot
[?] .user_role =? "guest"
[-] .user_role =? "admin"  // NOT admin
[r] |GrantLimitedAccess
```

---

### `[^]` - XOR Modifier

**Purpose:** Exclusive OR modifier - exactly one condition must be true

**Context:** Within trigger and switch blocks

**Closing:** Boolean blocks close by context (when nesting level changes) - NO explicit closing markers

**Syntax:**
```polyglot
[^] condition
```

**Example:**
```polyglot
[?] .has_password =? #Boolean.True
[^] .has_oauth =? #Boolean.True    // XOR - exactly one must be true
[r] |Authenticate
```

---

### `[.]` - Grouping Modifier

**Purpose:** Logical grouping for precedence in complex boolean expressions

**Context:** Within complex boolean expressions

**Closing:** Boolean blocks close by context (when nesting level changes) - NO explicit closing markers

**Syntax:**
```polyglot
[.] // Start nested group - closes when scope changes
[~] conditions within group
```

**Example:**
```polyglot
[t] |T.Daily
[t] |T.Time
[+] |T.WebHook         // OR - either scheduled OR webhook
[.]                    // Group for complex condition
[~][t] .hour >? 9      // Nested within group
[~][&] .hour <? 17     // AND within group
[r] |SendAlert
```

---

## String Processing Markers

### `[*]` - Line Continuation

**Purpose:** Syntactic line continuation for readability

**Context:** Any context where multi-line expressions needed

**Syntax:**
```polyglot
[*] <content>
[*] <more content>
```

**Example:**
```polyglot
[*] "This is a long message "
[*] +" that continues on the next line "
[*] +" for better readability."
```

**Behavior:**
- Whitespace outside strings stripped
- Comments stripped before joining
- Scope ends at next non-`[*]` line
- NOT semantic - purely syntactic sugar

**NOT line continuation:**
- `[^]` is XOR operator (boolean logic)

---

## Switch/Conditional Enhancements

### `[?]` - Enhanced Switch Block

**Purpose:** Conditional branching with exhaustive matching

**Context:** Within pipelines

**Comprehensive features:**
- Comparison operators: `>?`, `<?`, `=>?`, `=<?`, `=?`, `=!?`
- Range operators: `?[a,b]`, `?(a,b)`, `?[a,b)`, `?(a,b]`
- Pattern matching: `*?` (wildcard), `re?` (regex)
- Boolean logic: `[&]`, `[+]`, `[-]`, `[^]`, `[.]`

**Example with exhaustive matching:**
```polyglot
[?] .age
[?] .age <? 18
[r] |ProcessMinor

[?] .age ?[18, 65]
[r] |ProcessAdult

[?] .age >? 65
[r] |ProcessSenior

[?] *?  // Catchall (replaces Default keyword)
[r] |ProcessUnknown
```

**Exhaustive Matching Rules:**
- Compiler enforces total probability = 1
- All possible values must be covered
- Use `[?] *?` for catchall (no `Default` keyword)

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

### `[b]` - Background Execution

**Purpose:** Parallel fire-and-forget background execution

**Status:** Confirmed

**Syntax:**
```polyglot
[b] |BackgroundOperation
```

**Behavior:**
- Executes operation in background (non-blocking)
- Fire-and-forget - no waiting for completion
- Parallel execution without synchronization

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
- `[W]` Wrapper context
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
[i] .file: pg\path                    // Required (Declared state)
[i] .api_key: pg\string << "secret"   // Constant (immediately Ready)
[i] .timeout: pg\int <~ 30            // Optional with default (DEFAULT PUSH)

// ✗ WRONG - Using incorrect keywords
[i] Fixed .api_key: pg\string << "secret"  // ❌ No "Fixed" keyword exists!
[i] Default .timeout: pg\int << 30         // ❌ No "Default" keyword! Use <~ operator
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
[i] .max_size: pg\int <~ 1024         // DEFAULT PUSH operator <~

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
- [Complete Syntax Reference](syntax-complete.md) - All block markers overview
- [Operators](operators.md) - Operators used with block markers
- [Parallel Execution](parallel-execution.md) - `[p]` and `[Y]` details
- [Expansion Operator](expansion-operator.md) - `[~]` detailed usage

### Type Definitions
- [Enumerations](enumerations.md) - `[#]` enumeration blocks
- [Error Handling](error-handling.md) - `[!]` error blocks

### Standard Library
- [Runtime Wrappers](../standard-library/runtime-wrappers.md) - `[W]` wrapper usage
- [Queue Control](../standard-library/queue-control.md) - `[Q]` queue operations
- [Triggers](../standard-library/triggers.md) - `[t]` trigger types

### Examples
- [Hello World](../examples/hello-world.md) - Basic block marker usage
- [Complete Workflows](../examples/complete-workflows.md) - Complex block patterns

### Planning
- [Decision Log](../decision-log.md) - Block marker decisions (#7, #15)

---

**End of Block Markers Reference**