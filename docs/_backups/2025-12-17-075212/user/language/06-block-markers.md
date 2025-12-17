# Block Markers

**Version:** 0.0.3
**Last Updated:** 2025-12-10
**Status:** Complete

---

## Overview

Block markers are special syntax elements that define the structure and behavior of Polyglot code. Every line of valid Polyglot code must start with a block marker.

**Two Types of Markers:**
- **Registry markers:** Enclosed in curly braces `{marker}` - define operators and registries
- **Execution markers:** Enclosed in square brackets `[marker]` - control execution flow

**Key Characteristics:**
- Case-sensitive: `[r]` ≠ `[R]`, `{|}` ≠ `{|]`
- Every valid code line starts with a block marker
- Define code structure and execution flow
- Registry markers close with `{x}`
- Nesting uses backslash prefix/suffix: `\marker\`

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
13. [Wrapper System Markers](#wrapper-system-markers)
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
{|} |Pipeline.MyPipeline
[<} .input:pg.string
[r] |ProcessData
\<\ <data << .input
{x}

// ✗ INVALID - Missing block marker
{|} |Pipeline.MyPipeline
.input:pg.string  // Missing [<}
|ProcessData       // Missing [r]
{x}
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

### Marker Syntax

**Registry Markers** use curly braces `{ }`:

```polyglot
{marker}  // General format

{@}  // Registry declaration
{|}  // Pipeline definition
{#}  // Enumeration definition
{!}  // Error definition
{W}  // Wrapper definition
{x}  // Close registry marker
```

**Execution Markers** use square brackets `[ ]`:

```polyglot
[marker]  // General format

[r]  // Run sequential
[p]  // Parallel execution
[<}  // Input definition
[>}  // Output definition
```

**Nesting Markers** use backslash prefix/suffix:

```polyglot
\marker\  // Nested format

\<\  // Nested input
\>\  // Nested output
\~\  // Expand above marker
```

---

## Package Declaration Markers

### `{@}` - Registry Declaration

**Purpose:** Declares the registry identity and dependencies for the current file

**Position:** MUST be the first block in every Polyglot file

**Must be paired with:** `{x}` to close

**Full Syntax:**
```polyglot
{@} @Scope::Namespace.Name:Version
[A] @Alias
[.] @Dependency << @Scope::Package:Version
[.] @AnotherDependency << @Scope::Package:Version
{x}
```

---

#### Components

**Scope:**
- Local - Local development registries
- Community.username - Community shared registries with user namespacing
- Company - Company/organization registries

**Namespace:**
- Double-colon `::` separator: `Scope::Namespace`
- Dot-separated hierarchical path after `::`: `Namespace.Name`
- Example: `@Local::Examples.MyApp`, `@Community.user123::DataProcessing`

**Version:**
- Semantic versioning: `Major.Minor.Patch.Build`
- Example: `1.0.0.0`, `2.1.5.3`

**Alias:**
- `[A] @AliasName` - Create short alias for current registry
- Used for convenient referencing

**Dependencies:**
- `[.] @Dependency << @Scope::Package:Version`
- Each dependency uses `[.]` field marker
- Pull operator `<<` assigns dependency

---

#### Examples

**Simple Registry:**
```polyglot
{@} @Local::Examples.HelloWorld:1.0.0.0
[A] @HelloWorld
{x}
```

**Registry with Dependencies:**
```polyglot
{@} @Local::Examples.DataProcessor:1.0.0.0
[A] @DataProc
[.] @FileHelpers << @Community.user123::FileHelpers:2.0.0.1
[.] @CSVParser << @Community.user456::CSVParser:1.5.3.0
{x}
```

**Registry with Multiple Dependencies:**
```polyglot
{@} @Local::Examples.MyApp:1.0.0.0
[A] @MyApp
[.] @DataProcess << @Community.user123::DataProcessing:2.0.2.1
[.] @Stats << @Community.user143::Statistics:latest
{x}
```

---

#### Registry Naming Rules

**Scope Format:**
- `@Scope::` prefix required
- Local, Community.username, or Company
- Case-sensitive: `Local` not `local`

**Namespace Rules:**
- Use PascalCase for each segment
- Dot-separated hierarchy: `Examples.MyApp.Module`
- No special characters except dots
- Should reflect logical organization

**Version Format:**
- Must follow semantic versioning
- Format: `Major.Minor.Patch.Build`
- All four components required
- Examples: `1.0.0.0`, `2.1.5.3`, `10.3.7.0`

---

#### Best Practices

**1. Always Declare Registry First:**
```polyglot
// ✓ CORRECT - {@} is first
{@} @Local::Examples.MyApp:1.0.0.0
[A] @MyApp
{x}

{|} |Pipeline.MyPipeline
{x}

// ✗ WRONG - Missing {@} block
{|} |Pipeline.MyPipeline
{x}
```

**2. Use Semantic Versioning:**
- Major: Breaking changes
- Minor: New features, backward compatible
- Patch: Bug fixes, backward compatible
- Build: Build number/revision

**3. Organize Dependencies Logically:**
```polyglot
{@} @Local::Examples.WebAPI:2.0.0.0
[A] @WebAPI
// Core dependencies first
[.] @Logger << @Community.user123::Logger:3.0.0.0
[.] @Config << @Community.user123::Config:2.1.0.0
// Feature-specific dependencies
[.] @HTTPServer << @Community.user456::HTTPServer:5.0.0.0
[.] @Database << @Community.user789::Database:4.2.1.0
{x}
```

**4. Use Aliases for Convenience:**
- Create short aliases with `[A]`
- Makes referencing easier
- Scoped to current registry

---

## Pipeline Structure Markers

### `{|}` - Pipeline Definition

**Purpose:** Defines a pipeline (fundamental unit of execution)

**Must be paired with:** `{x}` to close

**Syntax:**
```polyglot
{|} |Pipeline.Name
// ... pipeline contents ...
{x}
```

**Example:**
```polyglot
{|} |Pipeline.ProcessData
[<} .input:pg.string
[r] |TransformData
\<\ <data << .input
\>\ >result >> .transformed
[>} .output:pg.string << .transformed
{x}
```

---

### `{x}` - Registry Close Marker

**Purpose:** Closes all registry markers

**Closes:** `{@}`, `{|}`, `{#}`, `{!}`, `{W}` blocks

**Syntax:**
```polyglot
{x}
```

**Examples:**
```polyglot
// Close pipeline
{|} |Pipeline.MyPipeline
{x}

// Close enumeration
{#} #MyEnumeration
[.] .field:pg.string << "value"
{x}

// Close error definition
{!} !MyError
[.] .message:pg.string << "Error"
[.] .code:pg.int << 5000
[.] .trace:pg.string << ""
{x}

// Close registry
{@} @Local::Examples.MyApp:1.0.0.0
{x}
```

---

## Input/Output Markers

### `[<}` - Input Declaration

**Purpose:** Declares pipeline inputs

**Three forms:**
1. **Required** - Caller MUST provide (no default)
2. **Constant** - Fixed value with `<<`
3. **Default** - Optional with `<~`

---

#### Required Input

```polyglot
[<} .parameter:pg.type
```

**Example:**
```polyglot
{|} |Pipeline.ProcessFile
[<} .file_path:pg.path  // Required - caller must provide
[r] |ReadFile
\<\ <path << .file_path
{x}
```

---

#### Constant Input

```polyglot
[<} .parameter:pg.type << value
```

**Example:**
```polyglot
{|} |Pipeline.ConnectToAPI
[<} .api_key:pg.string << "secret-key-123"  // Cannot be overridden
[<} .timeout:pg.int << 30
[r] |MakeAPICall
\<\ <key << .api_key
{x}
```

---

#### Default Input (Optional)

```polyglot
[<} .parameter:pg.type <~ default_value
```

**Example:**
```polyglot
{|} |Pipeline.ProcessWithOptions
[<} .input:pg.string  // Required
[<} .max_size:pg.int <~ 1024  // Optional, defaults to 1024
[<} .debug:pg.bool <~ #False   // Optional, defaults to False

[r] |Process
\<\ <data << .input
\<\ <size << .max_size
{x}
```

---

### `[>}` - Output Declaration

**Purpose:** Declares pipeline outputs

**Syntax:**
```polyglot
[>} .output:pg.type
[>} #None  // No output
```

**Examples:**
```polyglot
// Pipeline with output
{|} |Pipeline.Transform
[<} .input:pg.string
[r] |ProcessData
\<\ <data << .input
\>\ >result >> .output
[>} .output:pg.string
{x}

// Pipeline with no output
{|} |Pipeline.LogMessage
[<} .message:pg.string
[r] |U.Log.Info
\<\ <msg << .message
[>} #None
{x}
```

---

## Execution Control Markers

### `[r]` - Run Sequential

**Purpose:** Executes operations sequentially (one after another)

**Syntax:**
```polyglot
[r] |PipelineName
[r] .variable:pg.type << value
```

**Examples:**
```polyglot
// Sequential pipeline calls
[r] |Step1
[r] |Step2
[r] |Step3

// Sequential variable assignments
[r] .x:pg.int << 5
[r] .y:pg.int << 10
[r] .sum:pg.int << .x + .y
```

---

### `[p]` - Parallel Execution

**Purpose:** Parallel foreach-style execution with pack/unpack operations

**Characteristics:**
- Uses `~ForEach` or similar unpack operator
- Input/output with `\<\` and `\>\`
- Nested operations with `\~\` expansion
- Use `[v]` join to synchronize results

**Syntax:**
```polyglot
[p] ~ForEach
\<\ <array << .source
\>\ >item >> .current
\~\[r] |Operation
\~\[v] ~V.JoinAll
```

**Example:**
```polyglot
{|} |Pipeline.ProcessInParallel
[<} .data:pg.array

[p] ~ForEach
\<\ <array << .data
\>\ >item >> .current_item
\~\[r] |ProcessItem
\~\\<\ <in << .current_item
\~\\>\ >out >> .result
\~\[v] ~V.JoinAll
\~\\<\ <append << .result
\~\\>\ >array >> .final_results

[>} .output:pg.array << .final_results
{x}
```

---

## Data Flow Markers

### `[.]` - Field Definition

**Purpose:** Define fields in enumerations, errors, or registries

**Syntax:**
```polyglot
[.] .field:pg.type << value
```

**Example in Enumeration:**
```polyglot
{#} #Configuration
[.] .host:pg.string << "localhost"
[.] .port:pg.int << 8080
{x}
```

**Example in Error:**
```polyglot
{!} !CustomError
[.] .message:pg.string << "Error"
[.] .code:pg.int << 5000
[.] .trace:pg.string << ""
{x}
```

**Example in Registry (Dependencies):**
```polyglot
{@} @Local::Examples.MyApp:1.0.0.0
[.] @Logger << @Community.user123::Logger:1.0.0.0
{x}
```

---

### `\<\` - Pass Input (Nested)

**Purpose:** Pass input to predefined pipeline calls

**Context:** Nested under `[r]`, `[p]`, or other execution markers

**Syntax:**
```polyglot
\<\ <parameter << value
```

**Examples:**
```polyglot
// Pass input to pipeline
[r] |ProcessData
\<\ <input << "value"
\<\ <max_size << 1024

// Nested in parallel block
[p] ~ForEach
\<\ <array << .data
\~\[r] |Process
\~\\<\ <in << .current
```

---

### `\>\` - Pass Output (Nested)

**Purpose:** Extract/pull values FROM predefined pipelines

**Context:** Nested under `[r]`, `[p]`, or other execution markers

**Syntax:**
```polyglot
\>\ >field >> variable
```

**Examples:**
```polyglot
// Extract from pipeline output
[r] |ProcessData
\<\ <input << .data
\>\ >result >> .output_var

// Extract from error
[!] .handle_error
\<\ <error_type << !FileSystem.NotFound
\>\ >message >> .err_msg
\>\ >code >> .err_code
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
[s] .config:pg.serial << |JSON.Load"\\Config\\app.json"
[s] .users:#UserData << |YAML.Load"\\Data\\users.yaml"
[s] .settings:pg.serial << |TOML.Load"settings.toml"
```

**Parallel Loading (Automatic Join):**
```polyglot
[s] .db_config << |JSON.Load"db.json"       // Load in parallel
[s] .api_config << |JSON.Load"api.json"     // Load in parallel
[s] .cache_config << |JSON.Load"cache.json" // Load in parallel
// Automatic join before next operation
[r] |SetupDatabase
\<\ <config << .db_config
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

[>} .config.error  // Fail pipeline with error
```

**Scope-Level Error Handling (`[s][!]`):**
```polyglot
[s] .db_config << JSON"db.json"
[s] .api_config << JSON"api.json"

[s][!] !File.NotFound         // Catches ALL serial NotFound in scope
\>\ >message >> .err_msg
[r] |U.Log.Error
\<\ <msg << "Config file not found: {.err_msg}"
[>} !ConfigurationError

[s][!] !JSON.ParseError       // Catches ALL serial ParseError in scope
[r] |HandleParseError
```

**Error Precedence:**
- `\~\[!]` (specific - previous block) overrides `[s][!]` (general - all serial)

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
\<\ <data << .config
```

---

#### Type Loading & Validation

**Flexible Types (No Validation):**
```polyglot
[s] .data:pg.serial << JSON"data.json"          // No validation
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
[y] .env =? #Production
\~\ [s] .config << JSON"prod.json"
\~\ [s][!] !File.NotFound

[y] .env =? #Development
\~\ [s] .config << JSON"dev.json"
\~\ [s][!] !File.NotFound
```

**Execution Order:**
- Position: `[t]` > `[<}` > `[Q]` > `[\]` > **`[r],[p],[s],[b],[v]`** > `[/]` > `[>}` > `{x}`
- `[s]` sits alongside `[r]`, `[p]`, `[b]` in execution blocks
- **NOT allowed in `[t]` triggers** (execution-only block)

**Immutability:**
- Default: loaded variables are immutable
- Discourages mutable variables in automation (reduces edge cases)

---

#### Complete Example

```polyglot
{@} @Local::MyApp.ConfigLoader:1.0.0.0
[A] @ConfigLoader
[.] @FileHelpers << @Community.utils::FileHelpers:2.0.0.0
{x}

{|} |Pipeline.LoadApplicationConfig
[<} .env:#Environment
[t] |T.Call

[s] .base_config << JSON"\\Config\\base.json"
[s] .env_config << JSON"\\Config\\{.env}.json"
[s] .secrets << JSON.FilenameKey"\\Secrets\\*.json".ExcludeFileName"*example*"

[s][!] !File.NotFound
\>\ >message >> .err_msg
[r] |U.Log.Error
\<\ <msg << "Config file not found: {.err_msg}"
[>} !ConfigurationError

[s][!] !JSON.ParseError
[r] |U.Log.Error
\<\ <msg << "Invalid JSON in config"
[>} !ConfigurationError

// Merge configs
[r] |MergeConfigs
\<\ <base << .base_config
\<\ <env << .env_config
\<\ <secrets << .secrets
\>\ >merged >> .final_config

[>} .final_config:pg.serial
{x}
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

### `[v]` - Join Block

**Purpose:** Joins/merges parallel execution results (visual: ∨ merge symbol)

**Used with:** Pack operators like `~V.JoinAll`, `~V.JoinFirst`

**Syntax:**
```polyglot
\~\[v] ~V.JoinOperator
\~\\<\ <input_params
\~\\>\ >output_results
```

**Example:**
```polyglot
{|} |Pipeline.ParallelWorkflow
[<} .input:pg.array

[p] ~ForEach
\<\ <array << .input
\>\ >item >> .current
\~\[r] |ProcessItem
\~\\<\ <data << .current
\~\\>\ >result >> .processed
\~\[v] ~V.JoinAll
\~\\<\ <append << .processed
\~\\>\ >array >> .final_results

[>} .output:pg.array << .final_results
{x}
```

**Key Point:** `[v]` is used within the parallel block structure with the `\~\` expansion prefix to indicate merging of parallel results.

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
\<\ <parameter << value
```

---

#### Time-Based Triggers

```polyglot
// Daily trigger
[t] |T.Daily
\<\ <time << |DT"09:00:"

// Every minute
[t] |T.Every.Minute

// Every hour
[t] |T.Every.Hour
\<\ <minute << 0

// Custom interval
[t] |T.Every.Seconds
\<\ <interval << 30
```

---

#### File-Based Triggers

```polyglot
// File modified
[t] |T.File.Modified
\<\ <path << \\DataDir\\file.txt

// File created
[t] |T.File.Created
\<\ <path << \\DataDir\\

// File deleted
[t] |T.File.Deleted
\<\ <path << \\DataDir\\file.txt
```

---

#### Event-Based Triggers

```polyglot
// Custom event trigger (API TBD)
[t] |T.Event
\<\ <event_name << "user.registered"
```

---

### `[Q]` - Queue Control

**Purpose:** Controls queue operations within pipeline

**Note:** Uppercase `Q` (case-sensitive)

**Syntax:**
```polyglot
[Q] |Q.Operation
\<\ <parameter << value
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
\<\ <queue << #Queues.Background
```

---

#### Conditional Queue Control

```polyglot
// Pause if condition
[Q] |Q.PauseIf.RAM.Available.LessThan
\<\ <mb << 512

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
\<\ <script << "analyze.py"

// Fixed Node version
[W] |W.Node20
[r] |RunJavaScript
\<\ <script << "process.js"

// Rust
[W] |W.Rust
[r] |RunRustCode
\<\ <binary << "processor"
```

---

#### Dynamic Version Wrappers

```polyglot
// Latest Python 3.x
[W] |W.Python
[r] |RunScript
\<\ <script << "script.py"

// Latest Node.js
[W] |W.Node
[r] |RunScript
\<\ <script << "app.js"
```

---

#### Multiple Wrappers in One Pipeline

```polyglot
{|} |Pipeline.MultiRuntime
[<} .data:pg.string

// Python processing
[W] |W.Python3.11
[r] |PythonAnalyze
\<\ <input << .data
\>\ >result >> .python_result

// Node.js processing
[W] |W.Node20
[r] |NodeTransform
\<\ <input << .python_result
\>\ >result >> .final_result

{x}
```

---

## Type Definition Markers

### `{#}` - Enumeration Definition

**Purpose:** Defines enumerations (immutable data structures)

**Must be paired with:** `{x}` to close

**Syntax:**
```polyglot
{#} #Enumeration.Name
[.] .field:pg.type << value
{x}
```

**Example:**
```polyglot
{#} #AppConfiguration
[.] .host:pg.string << "localhost"
[.] .port:pg.int << 8080
[.] .debug:pg.bool << #False
{x}
```

---

### `{!}` - Error Definition

**Purpose:** Defines custom error types

**Must be paired with:** `{x}` to close

**Must include:** Three reserved fields (`.message`, `.code`, `.trace`)

**Syntax:**
```polyglot
{!} !Error.Name
[.] .message:pg.string << "message"
[.] .code:pg.int << code
[.] .trace:pg.string << ""
[.] .custom_field:pg.type << value  // Optional
{x}
```

**Example:**
```polyglot
{!} !MyApp.ValidationError
[.] .message:pg.string << "Validation failed"
[.] .code:pg.int << 4000
[.] .trace:pg.string << ""
[.] .field_name:pg.string << ""
[.] .invalid_value:pg.string << ""
{x}
```

---

### `[A]` - Alias Definition

**Purpose:** Creates scoped alias for registries, enumerations, pipelines, errors, or wrappers

**Syntax:**
```polyglot
[A] @AliasName  // For registry
[A] #AliasName  // For enumeration
[A] |AliasName  // For pipeline
[A] !AliasName  // For error
[A] |W.AliasName  // For wrapper
```

**Examples:**
```polyglot
// Registry alias
{@} @Local::Examples.MyApp:1.0.0.0
[A] @MyApp
{x}

// Enumeration alias
{#} #Path.Identifiers.MyApp.DataDirectory
[A] #DataDir
[.] .unix:pg.path << "\\UnixRoot\\opt\\data\\"
[.] .windows:pg.path << "\\C\\Data\\"
{x}

// Error alias
{!} !MyApp.Authentication.InvalidCredentials
[A] !InvalidCreds
[.] .message:pg.string << "Invalid credentials"
[.] .code:pg.int << 4010
[.] .trace:pg.string << ""
{x}
```

---

## Error Handling Markers

### `[!]` - Error Catching (Context-Dependent)

**Purpose:** Catches specific error types

**Note:** Same marker as error definition `{!}`, but `[!]` used in execution context for catching

**Syntax:**
```polyglot
[!] !ErrorType
\>\ >field >> variable  // Optional field extraction with nested marker
```

**Example:**
```polyglot
{|} |Pipeline.FileOperation
[<} .file_path:pg.path
[t] |T.Call
{W} |W.NoSetup.NoCleanup

[r] |ReadFile
\<\ <path << .file_path
\>\ >content >> .file_content
\~\[!] !pg.FileSystem.NotFound
\~\\>\ >message >> .err_msg
\~\[r] |U.Log.Error
\~\\<\ <msg << .err_msg
\~\[!] !pg.FileSystem.PermissionDenied
\~\\>\ >message >> .denied_msg
\~\[r] |U.Log.Error
\~\\<\ <msg << "Access denied: {.denied_msg}"

[>} .content:pg.string
{x}
```

---

## Expansion & Nesting Markers

### `\~\` - Expand-Above Marker

**Purpose:** Expands the parent marker above with nested content

**When to use:**
- Nesting operations under parent markers
- Operations inside parallel blocks
- Expanding conditional branches

**Syntax:**
```polyglot
\~\[marker] ...
\~\\marker\ ...  // Two levels deep (note double backslash)
```

---

#### Explicit Expansion

```polyglot
[p] ~ForEach
\<\ <array << .input
\>\ >item >> .current
\~\[r] |ProcessItem         // \~\ expands [p] with nested [r]
\~\\<\ <in << .current      // \~\\ = nested under \~\[r]
\~\\>\ >out >> .result
```

---

#### Multiple Nesting Levels

```polyglot
[p] ~ForEach
\<\ <array << .data
\>\ >item >> .current
\~\[r] |ProcessItem              // Level 1: nested under [p]
\~\\<\ <data << .current         // Level 2: nested under \~\[r]
\~\\>\ >result >> .processed
\~\[v] ~V.JoinAll                // Level 1: join under [p]
\~\\<\ <append << .processed     // Level 2: nested under \~\[v]
\~\\>\ >array >> .final
```

---

#### When NOT to Use `\~\`

**Direct nesting** with `\<\` and `\>\` is used for immediate children:

```polyglot
// Direct nesting - no \~\ needed
[r] |SomeOperation
\<\ <input << "value"    // Direct child of [r]
\>\ >output >> .result   // Direct child of [r]
```

---

## Wrapper System Markers

### `{W}` - Wrapper Definition

**Purpose:** Define reusable wrapper for runtime environments or setup/cleanup patterns

**Must be paired with:** `{x}` to close

**Syntax:**
```polyglot
{W} |W.Wrapper.Name
[{] .input_param:pg.type  // Scope input
[}] .output_param:pg.type  // Scope output

[\] |Setup.Step
[r] |ExecutionStep
[/] |Cleanup.Step
{x}
```

**Example:**
```polyglot
{W} |W.Database.Setup
[{] .db_host:pg.string  // Input from caller
[}] .db_conn:pg.db      // Output to caller

[\] |U.DB.Connect
\<\ <host << .db_host
\>\ >connection >> .db_conn

[/] |U.DB.Disconnect
\<\ <conn << .db_conn
{x}
```

---

### `[{]` - Scope Input

**Purpose:** Declare variables flowing INTO wrapper from caller

**Context:** Within `{W}` wrapper definitions

**Syntax:**
```polyglot
[{] .variable_name:pg.type
```

**Example:**
```polyglot
{W} |W.Logger.Setup
[{] .log_level:pg.string    // Input from caller
[{] .message:pg.string      // Input from caller
[}] .log_file:pg.file       // Output to caller

[\] |U.Log.Open
\<\ <level << .log_level
\>\ >file >> .log_file
{x}
```

**Flow:** Variables declared with `[{]` must be provided by caller when using wrapper

---

### `[}]` - Scope Output

**Purpose:** Declare variables flowing OUT of wrapper to caller

**Context:** Within `{W}` wrapper definitions

**Syntax:**
```polyglot
[}] .variable_name:pg.type
```

**Example:**
```polyglot
{W} |W.Cache.Setup
[{] .cache_size:pg.int      // IN from caller
[}] .cache_handle:pg.db     // OUT to caller

[\] |U.Cache.Initialize
\<\ <size << .cache_size
\>\ >handle >> .cache_handle
{x}
```

**Flow:** Variables declared with `[}]` become available in caller after wrapper execution

---

## Boolean Logic Markers

**Note:** Boolean markers use `\` prefix when nested under `[t]` triggers or `[y]` switches.

### `\&\` - AND Modifier (Nested)

**Purpose:** Logical AND modifier - combines multiple conditions

**Context:** Nested within `[t]` trigger blocks or `[y]` switch blocks

**Syntax:**
```polyglot
\&\ condition
```

**Example:**
```polyglot
[t] |T.Daily"9AM"
\&\ .hour >? 9
\&\ .hour <? 17
[r] |ProcessFile
```

---

### `\|\` - OR Modifier (Nested)

**Purpose:** Logical OR modifier - alternate conditions

**Context:** Nested within `[t]` trigger blocks or `[y]` switch blocks

**Syntax:**
```polyglot
\|\ condition
```

**Example:**
```polyglot
[y] .status =? "active"
\|\ .status =? "pending"
[r] |ProcessItem
```

---

### `\^\` - XOR Modifier (Nested)

**Purpose:** Exclusive OR modifier - exactly one condition must be true

**Context:** Nested within trigger or switch blocks

**Syntax:**
```polyglot
\^\ condition
```

**Example:**
```polyglot
[y] .has_password =? #Boolean.True
\^\ .has_oauth =? #Boolean.True    // XOR - exactly one must be true
[r] |Authenticate
```

---

### `[*]` - Boolean Grouping

**Purpose:** Groups boolean expressions for precedence

**Context:** Complex boolean expressions with `[t]` or `[y]`

**Syntax:**
```polyglot
\~\[*]
\~\\|\ condition1
\~\\&\ condition2
```

**Example:**
```polyglot
[t] |T.Daily"9AM"
\~\[*]  // Group conditions
\~\\|\ .is_weekday
\~\\&\ .is_business_hours
[r] |SendAlert
```

---

## String Processing Markers

### `[+]` - Line Continuation

**Purpose:** Syntactic line continuation for readability

**Context:** Any context where multi-line expressions needed

**Syntax:**
```polyglot
[+] <content>
[+] <more content>
```

**Example:**
```polyglot
[+] "This is a long message "
[+] +" that continues on the next line "
[+] +" for better readability."
```

**Behavior:**
- Whitespace outside strings stripped
- Comments stripped before joining
- Scope ends at next non-`[+]` line
- NOT semantic - purely syntactic sugar

**Note:**
- `[*]` is now used for Boolean-Grouping (see Boolean Logic Markers)

---

## Switch/Conditional Enhancements

### `[y]` - Enhanced Switch Block

**Purpose:** Conditional branching with exhaustive matching (visual: Y fork)

**Context:** Within pipelines

**Comprehensive features:**
- Comparison operators: `>?`, `<?`, `=>?`, `=<?`, `=?`, `=!?`
- Range operators: `?[a,b]`, `?(a,b)`, `?[a,b)`, `?(a,b]`
- Pattern matching: `*?` (wildcard), `re?` (regex)
- Boolean logic (nested): `\&\` (AND), `\|\` (OR), `\^\` (XOR), `[*]` (grouping)

**Example with exhaustive matching:**
```polyglot
[y] .age
[y] .age <? 18
[r] |ProcessMinor

[y] .age ?[18, 65]
[r] |ProcessAdult

[y] .age >? 65
[r] |ProcessSenior

[y] *?  // Catchall (replaces Default keyword)
[r] |ProcessUnknown
```

**Exhaustive Matching Rules:**
- Compiler enforces total probability = 1
- All possible values must be covered
- Use `[y] *?` for catchall (no `Default` keyword)

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

### Top-Level Parent: `{|}` Pipeline

**`{|}` is parent of:**
- `[<}` Input declaration
- `[>}` Output declaration
- `[t]` Trigger
- `[Q]` Queue control
- `[r]` Run/operation
- `[p]` Parallel execution
- `{W}` Wrapper definition
- `[b]` Background execution
- `[\]` Setup block
- `[/]` Cleanup block

```polyglot
{|} |Pipeline.MyPipeline  // Parent
[<} .input:pg.string      // Child (implicit expansion)
[t] |T.Daily              // Child (implicit expansion)
[r] |Operation            // Child (implicit expansion)
{x}
```

---

### Operation Parents

**Any block with operation/pipeline call is parent of:**
- `\<\` Input assignment (push INTO) - nested marker
- `\>\` Output assignment (pull FROM) - nested marker

```polyglot
[r] |SomeOperation       // Parent
\<\ <input << value      // Child (implicit expansion with nested marker)
\>\ >output >> result    // Child (implicit expansion with nested marker)
```

---

### Explicit vs Implicit Expansion

**Implicit** - No `\~\` needed (automatic):
```polyglot
[r] |Operation
\<\ <input << value    // Implicit - child of [r] using nested marker
```

**Explicit** - `\~\` required (manual):
```polyglot
[p] ~ForEach
\<\ <array << .items
\>\ >item >> .current
\~\[r] |Nested           // Explicit - WITHIN parallel context
\~\\<\ <in << .current   // Implicit - child of \~\[r] (note double backslash)
```

---

### Hierarchy Visual

```
{|} Pipeline
├── [<} Input (implicit)
├── [>} Output (implicit)
├── [t] Trigger (implicit)
│   ├── \<\ Trigger params (implicit, nested)
│   └── \>\ Trigger outputs (implicit, nested)
├── [Q] Queue control (implicit)
│   └── \<\ Queue params (implicit, nested)
├── [r] Run operation (implicit)
│   ├── \<\ Operation input (implicit, nested)
│   └── \>\ Operation output (implicit, nested)
├── [p] Parallel (implicit)
│   ├── \<\ Parallel input (implicit, nested)
│   ├── \>\ Parallel output (implicit, nested)
│   └── \~\[r] Nested operation (explicit with expand-above)
│       ├── \~\\<\ Nested input (double backslash for second nesting level)
│       └── \~\\>\ Nested output (double backslash for second nesting level)
└── {x} End marker (implicit)
```

---

## Best Practices

### 1. Always Start Lines with Block Markers

```polyglot
// ✓ CORRECT
{|} |Pipeline.Process
[<} .input:pg.string
[r] |Operation
{x}

// ✗ WRONG - Missing block markers
Pipeline
.input:pg.string
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

### 3. Always Close Registry Definitions

```polyglot
// ✓ CORRECT - Closed with {x}
{|} |Pipeline.Process
{x}

{#} #Configuration
{x}

// ✗ WRONG - Missing {x}
{|} |Pipeline.Process

{#} #Configuration
```

---

### 4. Use Appropriate Input Types

```polyglot
// ✓ CORRECT - Clear intent
[<} .file:pg.path                     // Required
[<} .api_key:pg.string << "secret"    // Constant (fixed value)
[<} .timeout:pg.int <~ 30             // Default (optional)

// ✗ AVOID - Unclear requirements
[<} .file:pg.path           // Is this required?
[<} .api_key:pg.string      // Should this be fixed?
```

---

### 5. Use `\~\` Only When Needed

```polyglot
// ✓ CORRECT - Implicit expansion
[r] |Operation
\<\ <input << value

// ✗ UNNECESSARY - Don't use \~\ for implicit children
[r] |Operation
\~\\<\ <input << value  // \~\ not needed here
```

---

### 6. Consistent Indentation

```polyglot
// ✓ CORRECT - Clear hierarchy
{|} |Pipeline.Process
[<} .input:pg.string
[r] |Operation
\<\ <data << .input
\>\ >result >> .output
{x}

// ✗ HARDER TO READ - Inconsistent indentation
{|} |Pipeline.Process
    [<} .input:pg.string
[r] |Operation
    \<\ <data << .input
        \>\ >result >> .output
{x}
```

---

### 7. Group Related Operations

```polyglot
// ✓ CORRECT - Grouped by purpose
{|} |Pipeline.ProcessData
// Inputs
[<} .file:pg.path
[<} .max_size:pg.int <~ 1024

// Triggers
[t] |T.File.Modified
\<\ <path << .file

// Operations
[r] |ReadFile
\<\ <path << .file
\>\ >content >> .data

[r] |ProcessContent
\<\ <input << .data
\>\ >result >> .output

{x}
```

---

### 8. Use Descriptive Pipeline Names

```polyglot
// ✓ CORRECT
{|} |Pipeline.ProcessUserRegistration
{|} |Pipeline.ValidateEmailAddress
{|} |Pipeline.SendWelcomeEmail

// ✗ AVOID
{|} |Pipeline.Process
{|} |Pipeline.Validate
{|} |Pipeline.Send
```

---

## See Also

### Language Specification
- [Complete Syntax Reference](01-syntax-complete.md) - All block markers overview
- [Operators](05-operators.md) - Operators used with block markers
- [Parallel Execution](08-parallel-execution.md) - `[p]` and `[v]` details
- [Expansion Operator](09-expansion-operator.md) - `\~\` detailed usage

### Type Definitions
- [Enumerations](03-enumerations.md) - `{#}` enumeration blocks
- [Error Handling](04-error-handling.md) - `{!}` error blocks

### Standard Library
- [Runtime Wrappers](../standard-library/01-runtime-wrappers.md) - `{W}` wrapper usage
- [Queue Control](../standard-library/02-queue-control.md) - `[Q]` queue operations
- [Triggers](../standard-library/04-triggers.md) - `[t]` trigger types

### Examples
- [Hello World](../examples/hello-world.md) - Basic block marker usage
- [Complete Workflows](../examples/complete-workflows.md) - Complex block patterns

### Planning
- [Decision Log](../decision-log.md) - Block marker decisions (#7, #15)

---

**End of Block Markers Reference**