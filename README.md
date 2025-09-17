# Polyglot Event-Driven Language Documentation

## Overview

Polyglot is an event-driven orchestration language designed to seamlessly integrate code from multiple programming languages into a single, coherent pipeline. Its core philosophy is pragmatic: **leverage existing legacy code** instead of reinventing the wheel. It acts as a universal glue, allowing you to use the right tool for each job within an asynchronous, event-driven framework.

**Note:** This language is currently in the brainstorming phase. Nothing concrete has been built yet, and we need to plan and collaborate to make it happen. This documentation serves two audiences: developers who will build this language, and future users who need to understand what to expect and how to use it.

### Core Philosophy

- **Embrace Legacy Code:** Polyglot isn't another general-purpose language. It's a bridge builder, designed to integrate and orchestrate existing code from Python, JavaScript, Rust, C++, and more.
- **Asynchronous by Default:** Every operation is inherently async, enabling efficient coordination between interpreted runtime scripts and compiled build-time processes.
- **The Right Tool for the Job:** The language remains agnostic, allowing you to choose the best language for each specific task in a workflow.
- **Pipeline-Centric Thinking:** Unlike traditional synchronous functions, Polyglot uses pipelines - compositions of chained, parallel, and switching events that trigger asynchronously when conditions allow.

### Primary Goals

- **Polyglot Integration:** Seamlessly call functions and share data between different programming languages.
- **Event-Driven Automation:** Construct complex workflows using pure event-driven paradigms.
- **Provably Correct Concurrency:** Enforce data dependencies at compile-time to prevent race conditions.
- **Explicit Resource Management:** Mandatory setup and cleanup phases ensure resources are properly managed across all runtimes.
- **Practical Orchestration:** Built-in support for sequential chains, parallel tasks, error handling, and flow control.

## Core Concepts & Architecture

### The Event: The Fundamental Unit

An **Event** is a single block of execution with defined inputs, processing, and outputs. Every pipeline is composed of interconnected events that execute asynchronously when their triggers are satisfied.

Every Event must be structured in one of two ways:

**Explicit Form:**

1. **Triggers (`[t]`)** - The conditions that allow the event to execute.
2. **Inputs (`[i]`)** - The data the event requires.
3. **Setup (`[\]`)** - Pre-processing to acquire resources (e.g., open files, start runtimes).
4. **Process (`[r]`)** - The main execution (run code, call functions).
5. **Clean-up (`[/]`)** - Post-processing to release resources.

**Context Manager Form (Simplified):**

1. **Triggers (`[t]`)**
2. **Inputs (`[i]`)** - The data the event requires.
3. **Context Manager (`[^]`)** - A predefined macro that handles both setup and cleanup automatically.
4. **Process (`[r]`)**

### Terminology

| Term | Meaning |
| --- | --- |
| **Event** | A single execution block with inputs, wrapped processing, and outputs. |
| **Pipeline** | A series of events connected through chaining, parallel execution, or switching. |
| **Trigger** | An asynchronous condition that permits an event to execute. |
| **Macro** | A reusable template for events, often containing a standard setup/cleanup pattern. |

### Under the Hood

Polyglot code compiles down to an **async orchestration layer** that manages:

- **Language Runtimes:** Persistent or on-demand processes for Python, JS, Rust, Go, C++, etc.
- **Bridges:** There are two kinds of binding:-
  - first is though usual binding basically it will repeat the steps you would have done using exiting tool to bind which leverages existing tools like PyBind11, Node's `child_process`, FFI, and legacy tool that will bind lanuage to other under the hood.
  - Taking advantage to async nature of polyglot by complie and use on the fly truning things that are complie-time only into a runtime. For example if you have a rust function that will accept array of fixed size. you convert you python list into fixed rust array then use that and convert it back.
- **Resource Lifecycle:** Manages connection pools, file handles, and memory cleanup across languages.
- **Dependency Graph:** Analyzes variables to enforce correct execution order and prevent races.

## Syntax Reference

### Structural Elements (Dual Form Support)

Polyglot supports both compact and verbose notation for all elements. However, I would recommad using compact form for nicely formatted code.

| Compact | Verbose | Purpose |
| --- | --- | --- |
| `[@]` | `[e]` / `[event]` | Define or start a pipeline. |
| `[i]` | `[input]` | Define an input. |
| `[t]` | `[trigger]` | Define a trigger condition. |
| `[\]` | `[setup]` | Pre-processing and resource setup. |
| `[r]` | `[run]` | Main process execution. |
| `[/]` | `[clean]` | Post-processing and resource cleanup. |
| `[x]` | `[exit]` | End a pipeline or branch. |
| `[^]` | `[wrap]` | Apply a context manager (macro with built-in setup/cleanup). |
| `[?]` | `[switch]` | Switch (flow control) statement. |
| `[!]` | `[error]` | Error handler. |
| `[f]` | `[fork]` | Start a parallel (forked) branch. |
| `[j]` | `[join]` | Join parallel branches and consolidate results. |
| `[b]` | `[back]` | Start a background branch (fire-and-forget). |
| `[M]` | `[macro]` | Define or use a Macro. |
| `[v]` | `[inject]` | Injection slot (used inside Macros). |
| `[~]` |     | Branch depth indicator - always means "as above". |
| `[D]` | `[Define]` | Used to define custom types and imports. |

### The `[~]` Element: "As Above" Consistency

The `[~]` element **always** means "as above" throughout Polyglot:

- **In branching:** Indicates this line belongs to the same branch as the operation above (`[f]`, `[?]`, `[!]`)
- **In error capture:** `![~]` means "capture error from the operation above"
- **In multiline expressions:** Continues the expression from the line above
- **In nested structures:** Shows depth level within branches

This consistency enables visual scanning of complex nested pipelines at a glance.

### Square Elements Design Philosophy

The compact Square elements use fixed 3-character notation (`[x]`) to:

1. **Minimize branching footprint:** Complex pipelines with extensive type conversions and compilations remain visually compact
2. **Enable visual scanning:** Humans, computers, and AI can quickly identify nested branches and pipeline structure
3. **Reinforce event structure:** Every event must have triggers `[t]`, setup `[\]`, processes `[r]`, and cleanup `[/]`

### Syntax Construction Pattern

**Unified Syntax:** `[SquareElements] [Label]: [DataType] = [BodyElements]`

Components:

- **Square Element**: The operation type (`[r]`, `[i]`, `[t]`, etc.)
- **Label**: Optional variable name (omit if return value unused or void)
- **DataType**: Type specification (required for all variables, optional for Events/Pipelines)
- **Body Elements**: The actual operation or value using function-call syntax

### Element Construction Examples

**With Labels and Types:**

```polyglot
[r] Result: py\int = @[CalculateSum](a: py\int, b: py\int)
[i] UserData: py\dict = @[LoadUser](user_id: py\int)
[t] FileReady: boolean = t[FileExists](path: string)
```

**Without Labels (void or unused returns):**

```polyglot
[r] @[LogMessage]("Process started")
[/] @[CloseConnection](db_handle: string)
[r] r[py]("print('Hello World')")
```

**Input Types:**

```polyglot
 // Will receive input from pipline call
[i] RequiredVar: py\string
// Will be overwritten by pipeline call else defualt
[i] Default DefaultVar: py\int = some_value    // Default value
[i] ConstVar: py\int = some_const_value        // Constant value
```

### Body Elements & Labels

Elements are defined with the new unified form:

- **`[r] Label: DataType = @[PipelineLabel](args: type,...)`**: Call another pipeline with result assignment.
- **`[t] Label: boolean = t[TriggerType](args: type,...)`**: Define a trigger condition.
- **`[r] Label: DataType = r[Lang](code: string, args: type, ...)`**: Run code in specified language.
- **`[?] SwitchLabel: DataType = ?[var: DataType]`**: Capture value for switch statement.
- **`[!] ErrorLabel: ErrorType = ![~]`**: Capture error from operation above.
- **`[i] Label: DataType2 = value: DataType1`**: Define input value (conversion if types differ).

**Label Rules:**

- **Labels are optional**: Omit when return value is unused or operation is void
- **DataType required**: All variables must specify datatype except Events/Pipelines
- **Assignment syntax**: Use `=` for all assignments and function calls

## Data Types

Polyglot uses a flexible type system to bridge different languages while preserving native semantics.

### Native Polyglot Types

- **`Event` / `Pipeline`**: For calling other pipelines
- **`bool`**: True/False values
  - bit-operations keywords `NOT`, `OR`,`AND`, `XOR`.  
- **`Time`**: Formatted as `YYYY-MM-DDThh:mm:ss.000000`
  - 1ns = `T"1"`
  - 1ms = `T"1000"`
  - 1s = `T"1."`
  - 1m = `T"1:."`
  - 1h = `T"1::."`
  - 1 day = `T"1T"`
  - 1 month = `T"-1-"`
  - 1 year = `T"1--"`
- **`uint8`**: 8-bit unsigned integer
- **`json`**: JSON-formatted string for universal data exchange
- **`string`**: Character array
- **`type`**: An enumeration of supported datatypes
- **`pg\outputs`**: Automatic output container type
- **`pg\error`**: Error status type

### Language-Specific Types

Types are specified with consistent prefix notation:

#### Built-in Types (using `\` - "pre-made")

- **`py\int`**: Python integers
- **`rust\i32`**: Rust 32-bit signed integers
- **`rust\&str`**: Rust string slices
- **`js\string`**: JavaScript strings
- **`c++\std::string`**: Full C++ STL string type
- **`c++\int`**: C++ integers

#### Collections

Collections use the format: `Lang\array[ElementsDatatype, CollectionType, size]`

- **`py\array[int, list]`**: Python list of integers
- **`rust\array[usize, Vec]`**: Rust vector of usize
- **`c++\array[int, std::array, 10]`**: C++ fixed-size array of 10 integers
- **`js\array[string, Array]`**: JavaScript array of strings

#### Custom Data Types (using `/` - "post-made")

Custom datatypes are specified with the prefix `Language/`:

- **`py/Person`**: Custom Python class
- **`c++/CustomClass`**: Custom C++ class
- **`rust/MyStruct`**: Custom Rust struct

#### Error Types (using `\!`)

Error types follow the pattern `Lang\!ErrorName`:

- **`py\!TypeError`**: Python TypeError
- **`py\!ValueError`**: Python ValueError
- **`rust\!ParseError`**: Rust parse errors
- **`js\!ReferenceError`**: JavaScript reference errors

### Polyglot Error Types

Polyglot provides built-in error types using the `pg\!` prefix:

- **`pg\!ConversionError`**: Type conversion failures
- **`pg\!TypeMismatchError`**: Type compatibility issues
- **`pg\!ResourceError`**: Resource allocation/deallocation failures
- **`pg\!TimeoutError`**: Operation timeout errors
- **`pg\!ConcurrencyError`**: Race condition or synchronization errors
- **`pg\!BridgeError`**: Language bridge communication failures
- **`pg\!CompilationError`**: Code compilation failures
- **`pg\!RuntimeError`**: General runtime errors

*Note: More error types will be added as per needed.*

### Type System Rules

- **`\` denotes "pre-made"**: Used for built-in language types
- **`/` denotes "post-made"**: Used for user-defined types
- **`\!` denotes error types**: For exception handling
- **Exact Specification**: Types must be expressed exactly as in the source language
- **Custom Type Registration**: Custom types must be defined through `[D]` pipelines before use

### Type Shorthand and Conversion

```polyglot
// Datatype shorthand definition
[D] c++\str = c++\std::string
// below: X not allowed - types must mention language
// [D] str = c++\std::string

// Automatic type conversion
[?] MySwitch: py\int = ?[Var: rust\i32]
// If types don't match, triggers implicit conversion
// with t[Convert](rust\i32, py\int)
```

### pg\outputs Behavior

The `pg\outputs` type automatically adapts based on pipeline outputs:

```polyglot
[@] SomePipeline
[r] OutputLabel: py\int = some_calculation
[x]

[@] CallingPipeline
[r] PipelineOutputs: pg\outputs = @[SomePipeline]
[r] Results: py\int = @[Process](PipelineOutputs.OutputLabel)

// If pipeline has only one output, pg\outputs becomes that type automatically
[r] PipelineOutputs: py\int = @[SomePipeline]  // Also valid
[x]
```

### Custom Type Definition Example

```polyglot
[D] cppstring = type_definition
[t] = t[type](c++)
[^] = C++11SetupWrapper
[i] CppType: string = type_input
[r] ImportOp: boolean = r[ImportCpp](CppType: string)
[!] = ![~]
[!] <| Default MyError
[~][r] = @[Panic]("Failed to Import c++/{0}", ImportCpp)
[x]
```

## Triggers

Triggers are the fundamental async conditions that enable an event or pipeline to execute. They represent the "why" and "when" of execution in Polyglot's event-driven model.

### Universal Trigger Rules

- **Implicit AND:** Multiple triggers within a single event have an implicit `AND` relationship. *All* specified triggers must evaluate to `True` for the event to run.
- **Dependency Types:** Triggers are categorized as Independent or Dependent, which dictates how they can initiate execution.
- **Conversion Triggers:** When types don't match, `t[Convert](typeFrom, typeTo)` triggers automatically behind the scenes.

### Independent Triggers (Prime Movers)

These triggers can start a pipeline on their own, acting as the initial catalyst for a workflow.

#### Core System Triggers

- **`t[Call]`**: The primary trigger for synchronous-style behavior. Returns `True` if the event is explicitly called via `@[EventLabel]`.
- **`t[RunPolyglot]`**: Triggered by the command line (e.g., `polyglot -p <parameter>`).
- **`t[OnStartup]`**: Trigger when the Polyglot runtime starts.

#### Time-Based Triggers

- **`t[Schedule]`**: Time-based scheduling (e.g., cron jobs, recurring intervals, specific start times).
- **`t[At]`**: One-time execution at specific time.
- **`t[Cron]`**: Cron-style scheduling (e.g., "0 2 * * *" for daily 2 AM).
- **`t[BusinessHours]`**: Execute only during business hours.

#### External Event Triggers

- **`t[HTTPEndpoint]`**: Listen for HTTP requests on a specific path.
- **`t[Webhook]`**: Trigger from an incoming webhook payload.
- **`t[MessageQueue]`**: React to a new message in a queue (e.g., RabbitMQ, SQS).
- **`t[GitPush]`**: Trigger on a Git push event.
- **`t[DockerEvent]`**: React to Docker lifecycle events.

### Dependent Triggers (Reactive)

These triggers respond to conditions or state changes but **require an independent trigger to have already initiated the pipeline's execution**. They cannot start a pipeline alone.

#### File System Triggers

- **`t[IfChange]`**: Returns `True` if a specified file has been modified since last execution.
- **`t[NotCompiled]`**: Returns `True` if the target code at the path has not been compiled or is outdated.
- **`t[FileExists]`**: Checks for the existence of a file.
- **`t[FileSize]`**: Triggers based on file size conditions.

#### Process & Service State Triggers

- **`t[IsActive]`**: Checks if a process ID is currently running.
- **`t[ProcessExit]`**: Process exits with specific code.
- **`t[ServiceUp]`**: Service becomes available.
- **`t[ServiceDown]`**: Service becomes unavailable.

#### Conversion and Uniqueness Triggers

- **`t[Convert]`**: Triggered automatically for type conversions.
- **`t[UniqueTriggers]`**: Panics if multiple pipelines have identical trigger sets.

## Flow Control

### 1. Sequential Chaining

Use the `<=` operator to chain events sequentially. Data flows automatically from one event to the next.

```polyglot
[e] NextEvent <=
[i] PreviousOutput: py\int = previous_data
[r] ProcessedResult: py\int = @[Process](PreviousOutput: py\int)
[x]
```

### 2. Parallel Execution & Joining

Use `[f] <|` to fork parallel branches and `[j]` to join them and collect results.

```polyglot
[f] <| Task1
[~][r] ResultArs: rust\i32 = @[ProcessA](input: rust\i32)
[~][r] ResultA: py\int = ResultArs  // implicit conversion

[f] <| Task2
[~][r] ResultB: py\int = @[ProcessB](input: rust\i32)

[j] ResultA: py\int = f[Task1]    // include Task1 to watch list
[j] ResultB: py\int = f[Task2]    // include Task2 to watch list
[j] FinalResult: py\int = j[JoinFirst]
```

**Joining Rules:**

- All forked branches must join before the end of the whole pipeline
- The returns from branches can be used only in `[j]` otherwise compile error
- Race condition must be specified:
  - `j[JoinFirst]`: assigns the first received result to the join label
  - `j[JoinAll]`: awaits all watch list to join before proceeding

### 3. Switching

Switch statements provide conditional branching with automatic type conversion.

```polyglot
[?] MySwitch: py\int = ?[Var: rust\i32]
// Note: If HashableTypes don't match, triggers implicit conversion t[Convert](rust\i32, py\int)

[?] <| MySwitch == value1
[~][r] = @[HandleValue1]()
[~][x]

[?] <| MySwitch == value2
[~][r] = @[HandleValue2]()
[~][x]

[?] <| Default MySwitch
[~][r] = @[HandleDefaultCase]()
[~][x]
```

### 4. Error Handling

Errors bubble up unless caught. Use `[!]` to define error handlers.

```polyglot
[r] Var: OutputType = @[PanicedPipeline]()
[!] MyError: ErrorType = ![~]
// ![~] captures error from the operation above

[!] <| MyError == py\!TypeError
[~][r] = @[HandleTypeError]()
[~][x]  // Continue pipeline after this branch

[!] <| MyError == py\!ValueError
[~][r] = @[HandleValueError]()
[~][x] @[Exit]  // Terminate entire pipeline

[!] <| Default MyError
[~][r] = @[HandleDefaultError]()
[~][x]
```

**Error Status Access:**

```polyglot
[@] SomePipeline
[r] OutputLabel: Type = some_calculation
[x]

[@] Other
[r] PipelineOutputs: pg\outputs = @[SomePipeline]()
[r] Results: Type = @[Process](PipelineOutputs.OutputLabel)
[!] MyError: pg\error = ![PipelineOutputs]  // Explicit error access
[!] MyError2: pg\error = ![~]               // Shorthand for above operation
[x]
```

**Branch Termination Options:**

- `[~][x]`: Continue pipeline after the branch
- `[~][x] @[Exit]`: Terminate the entire pipeline
- Panic: Halt program and force compilation error fixes

## Macros

Macros (`[M]`) are reusable event templates. The `[v]` element is a placeholder inside a macro where other code can be injected. The `[^]` element is a shortcut for applying a macro that wraps your code in setup and cleanup logic.

### Macro Definition

```polyglot
[M] MyMacro
[i] FilePath: string = file_input
[t] = t[Call]()
[r] FileHandle: c++\ofstream = @[OpenFile](FilePath: string)
[v] = CodeToInject  // This is the injection slot
[/] = @[CloseFile](FileHandle: c++\ofstream)
[x]
```

### Macro Invocation

```polyglot
[@] MyPipeline 
[i] Path: string
[M] FileHandle: c++\ofstream = M[MyMacro](MyPipeline[r])
[r] FileContent: string = @[ReadFile](FileHandle: c++\ofstream)
[x]
```

### ## Context Managers: The `[^]` Wrapper Element

The `[^]` (wrap) element is a powerful syntax sugar designed to make resource management safe, explicit, and incredibly easy to use. It encapsulates the common pattern of "setup a resource, use it, then clean it up" into a single, clean line of code.

### For the User: What to Expect

Imagine you need to open a file, process it, and ensure it gets closed even if an error occurs. In traditional languages, this requires careful `try/finally` blocks. In Polyglot, you just wrap your process:

polyglot

[@] SafeFileProcessing
[i] FilePath: string = "data.txt"
// The clean, user-friendly way:
[^] FileHandler: c++\ofstream = @[FileManager](FilePath) // Setup & Cleanup handled here
[r] Content: string = @[ReadFile](FileHandler) // Your code runs in the middle
[x]

**Functionally, the `[^]` line means:** "Before the next step runs, execute the setup phase of the `@[FileManager]` macro. After the subsequent steps in this event finish (whether they succeed or fail), automatically execute its cleanup phase."

This ensures resources like file handles, database connections, network sockets, and language runtime contexts are never leaked.

### Under the Hood: It's a Macro

The `[^]` element is not magic; it's elegant syntax sugar. **It compiles directly to a Macro invocation.**

The example above is functionally identical to writing:

polyglot

[@] SafeFileProcessing
[i] FilePath: string = "data.txt"
// What the [^] element compiles to:
[M] FileHandler: c++\ofstream = M[FileManager](SafeFileProcessing[r]) // Inject the next [r] block
[r] Content: string = @[ReadFile](FileHandler)
[x]

This means every `[^]` context manager **must** be backed by a Macro (`[M]`) definition that includes:

1. A `[\]` (setup) section
  
2. A `[v]` (injection slot) for your code
  
3. A `[/]` (cleanup) section
  

### For the Developer: How to Build This

Implementing the `[^]` element requires two components in the compiler:

1. **Syntax Translation:** During parsing, a line starting with `[^]` must be transformed into the longer Macro invocation form. The compiler needs to identify the subsequent `[r]` elements and pass them as the injection argument.
  
2. **A Rich Standard Library:** The power of `[^]` comes from pre-defined macros in the standard library. We will need to build macros like:
  
  - `@[FileManager]` - for file handling
    
  - `@[DatabaseConnection]` - for database connections
    
  - `@[PythonRuntime]` - for managing a Python interpreter context
    
  - `@[HTTPClient]` - for managing network connections
    

### Example: Building a Context Manager Macro

Here is what the `@[FileManager]` macro, used in the example above, would look like:

```polyglot
// Macro Definition: The engine behind the [^] sugar
[M] FileManager
[i] Path: string
[t] = t[Call]()
[\] FileHandle: c++\ofstream = @[SystemOpenFile](Path: string) // SETUP
[v] // INJECTION SLOT
[/] = @[SystemCloseFile](FileHandle: c++\ofstream) // CLEANUP
[x]
```

The `[^]` element hides this complexity from the user, providing the safety of explicit resource management with the simplicity of "magic."

### Examples

### Language Integration

```polyglot
// Simple cross-language calls
[r] PyResult: py\str = r[py]("return data.upper()", "hello": py\str)
[r] JsResult: js\Object = r[js]("fetch('https://api.example.com')")
[r] RustResult: rust\i32 = r[rust]("fn add(a: i32, b: i32) -> i32 { a + b }", 5: i32, 3: i32)
```

**Multiline Body Elements:** Body element expressions can get long. Use `[~]` as line continuation indicator:

```polyglot
// Long line
[r] RustResult: rust\i32 = r[rust]("fn add(a: i32, b: i32) -> i32 { a + b }", 5: i32, 3: i32)
// Equivalent to
[r] RustResult: rust\i32
[~] = r[rust]("fn add(a: i32, b: i32) -> i32 { a + b }", 5: i32, 3: i32)
[r] // some other thing
```

### Complete Workflow Example

```polyglot
[@] DataProcessingWorkflow
[i] InputData: py\string = input_data

// Step 1: Process in Python
[^] = PythonEnvironmentSetup
[r] ProcessedData: py\dict 
[~] = @[PythonFunction](InputData: py\string)

// Step 2: Send to a Rust service for heavy computation in parallel
[f] <| RustCompute
[~][^] RustSetup
[~][r] RustData: rust\&str = ProcessedData  // implicit conversion
[~][r] RustResult: py\string = @[RustFunction](RustData: rust\&str)
[~][x]

// Step 3: Also log the action in the background
[b] <| Logger
[~][r] = r[log]("info", "Started processing for ${InputData}")
[~][x]

// Step 4: Wait for Rust to finish and handle result
[j] RustResult: rust\&str = f[RustCompute]
[r] FinalResult: py\dict = @[Convert](RustResult: py\string)
[r] = r[log]("info", "Workflow complete. Result: ${FinalResult}")
[x]
```

### Array Processing Example

```polyglot
[@] ArrayProcessingPipeline
[i] Numbers: py\array[int, list] = input_numbers
[i] Matrix: rust\array[f64, Vec] = input_matrix

// Process Python list in parallel
[f] <| PythonSort 
[~][r] SortedNumbers: py\array[int, list] 
[~][~] = @[python_sort](Numbers: py\array[int, list])

// Process Rust vector in parallel  
[f] <| RustMath 
[~][r] Result: rust\array[f64, Vec] 
[~][~] = @[rust_matrix_multiply](Matrix: rust\array[f64, Vec])

// Join results
[j] SortedNumbers: py\array[int, list] = f[PythonSort]
[j] Result: rust\array[f64, Vec] = f[RustMath]

// Convert and combine
[r] RustNumbers: rust\array[i32, Vec] = SortedNumbers  // implicit conversion
[r] FinalOutput: rust\array[i32, Vec] = @[CombineResults](
[~]   RustNumbers: rust\array[i32, Vec], 
[~]   Result: rust\array[f64, Vec]
[~] )
[x]
```

### Advanced Error and Switch Handling

```polyglot
[@] ConditionalProcessing
[i] UserInput: py\int = input_value
[i] DataFile: string = file_path

// Switch with automatic type conversion
[?] InputSwitch: py\int = ?[UserInput: py\int]

[?] <| InputSwitch == 1     // Process file
[~][r] FileData: py\dict = @[LoadFile](DataFile: string)
[~][!] FileError: pg\error = ![~]
[~][~][!] <| FileError == py\!FileNotFoundError
[~][~][~][r] ErrorMsg: string = "File not found, using default data"
[~][~][~][x]
[~][x]

[?] <| InputSwitch == 2     // Process database
[~][r] DbData: py\dict = @[LoadDatabase](connection: string)
[~][x]

[?] <| Default InputSwitch     // Default case
[~][r] DefaultData: py\dict = @[GetDefaultData]()
[~][x]
[x]
```
