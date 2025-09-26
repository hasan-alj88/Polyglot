# Polyglot Event-Driven Language Documentation

## 1. Overview & Philosophy

Polyglot is an event-driven orchestration language designed to seamlessly integrate code from multiple programming languages into a single, coherent pipeline. Its core philosophy is pragmatic: **leverage existing legacy code** instead of reinventing the wheel. It acts as universal glue, allowing you to use the right tool for each job within an asynchronous, event-driven framework.

**Note:** This language is currently in the brainstorming phase. Nothing concrete has been built yet, and we need to plan and collaborate to make it happen. This documentation serves two audiences: developers who will build this language, and future users who need to understand what to expect and how to use it.

### Core Philosophy

- **Don't Reinvent the Wheel, Embrace Legacy Code:** Polyglot isn't another general-purpose language. It's a bridge builder, designed to integrate and orchestrate existing code from Python, JavaScript, Rust, C++, and more.
- **Asynchronous by Default:** Every operation is inherently async, enabling efficient coordination between interpreted runtime scripts and compiled build-time processes. As such, technically Polyglot is an interpreted language in this sense.
- **Use the Right Tool for the Job:** Polyglot loves all programming languages. Every programming language has pros and cons depending on the task. Polyglot allows you to choose the best language for each task in a workflow.
- **Pipeline-Centric Thinking:** Unlike traditional synchronous functions, Polyglot uses pipelines—compositions of chained, parallel, and switching events that trigger asynchronously when conditions allow.
- **Divide and Conquer Programming Language Translation:** Integrating programming languages to each other can get very complex. We envision with the evolution of this language the problem is to be subdivided into smaller problems such as converting one datatype from one language to other instead of having the whole implementation integrated.
- **Minimalist Polyglot Footprint:** Polyglot is intended to orchestrate integration between languages with as little intervention as possible.

### Primary Goals

- **Polyglot Integration:** Seamlessly call functions and share data between different programming languages.
- **Minimize Translation Time and Memory Footprint:** At the beginning, the time and memory footprint will not be negligible. Hence, Polyglot must aim for integration optimization as we divide and conquer the language integration problem into smaller and more specific subproblems. Optimization goal will be an achievable goal.
- **Event-Driven Automation:** Construct complex workflows using pure event-driven paradigms.
- **Concurrency Discipline:** Enforce data dependencies at compile-time to prevent race conditions by only using data of joined forked branches. Since using data of incomplete branches will cause corruption.
- **Explicit Resource Management:** Mandatory setup and cleanup phases ensure resources are properly managed across all runtimes.
- **Practical Orchestration:** Built-in support for sequential chains, parallel tasks, error handling, and flow control.

## 2. Getting Started

### Hello World Example

Here's a simple "Hello World" Pipeline to demonstrate basic Polyglot syntax:

```polyglot
[|] HelloWorld
[i] name: py\str
[t] |Call
[w] PythonEnvironment
[r] name: py\str >> |Print >> None
[o] None
[x]
```

**Explanation:**

- `[|] HelloWorld` - Defines a Pipeline named HelloWorld
- `[i] name: py\str` - Requires a string input parameter
- `[t] |Call` - Triggered when called from other pipelines
- `[w] PythonEnvironment` - Sets up Python runtime environment
- `[r]` - Executes the main process (print the name)
- `[o] None` - No output returned
- `[x]` - Ends the Pipeline

### Basic Chain Example

This example shows sequential execution where each step runs after the previous completes:

```polyglot
[|] DataProcessor
[i] raw_data: py\str
[t] |Call
[w] PythonEnvironment
\\ Sequential execution: each step runs after the previous completes
[r] raw_data: py\str >> |CleanData >> clean_data: py\str
[r] clean_data: py\str >> |ValidateData >> valid_data: py\str  
[r] valid_data: py\str >> |SaveData >> result: py\bool
[o] result
[x]
```

**Key Points:**

- Data flows sequentially through each processing step
- Each `[r]` element waits for the previous one to complete
- The output of one step becomes the input of the next

## 3. Core Concepts

### Terminology

| Term | Meaning |
| --- | --- |
| **Event** | A single execution block with inputs, wrapped processing, and outputs. |
| **Pipeline** | A series of events connected through chaining, parallel execution, or switching. |
| **Trigger** | An asynchronous condition that permits an event to execute. |
| **Macro** | A reusable template for events, often containing a standard setup/cleanup pattern. |
| **Element** | Square-bracketed components that define Pipeline structure and behavior. |
| **Element Expansion** | Using `[~]` to define detailed properties of square elements. |

### The Event: The Fundamental Unit

An **Event** is a single block of execution with defined inputs, processing, and outputs. Every Pipeline is composed of interconnected events that execute asynchronously when their triggers are satisfied.

**Critical Rule:** Every Event must have all required elements, or a **compile error** will be raised.

Every Event must be structured in one of two ways:

#### Explicit Form:

1. **Inputs (`[i]`)** - The data the event requires. Use `[i] None` if no input needed.
2. **Triggers (`[t]`)** - The conditions that allow the event to execute.
3. **Setup (`[\]`)** - Pre-processing to acquire resources (e.g., open files, start runtimes).
4. **Process (`[r]`)** - The main execution (run code, call functions).
5. **Cleanup (`[/]`)** - Post-processing to release resources.
6. **Outputs (`[o]`)** - Pipeline outputs. Use `[o] None` if no output needed.

#### Wrapper Form (Simplified):

1. **Inputs (`[i]`)** - The data the event requires. Use `[i] None` if no input needed.
2. **Triggers (`[t]`)** - The conditions that allow the event to execute.
3. **Wrapper (`[w]`)** - A predefined macro that handles both setup and cleanup automatically (equivalent to RAII and Python's context manager).
4. **Process (`[r]`)** - The main execution.
5. **Outputs (`[o]`)** - Pipeline outputs. Use `[o] None` if no output needed.

## 4. Language Syntax Reference

### Structural Elements (Dual Form Support)

Polyglot supports both compact and verbose notation for all elements. However, it is recommended to use compact form for nicely formatted code.

| Compact | Verbose | Purpose | Reference |
| --- | --- | --- | --- |
| `[\|]` | `[Pipeline]` | Define or start a Pipeline. | \|PipelineName |
| `[i]` | `[input]` | Define an input. | Within Pipeline: `input_name: datatype`. Outside Pipeline: `\|PipelineName<input_name` |
| `[t]` | `[trigger]` | Define a trigger condition. | -   |
| `[\]` | `[setup]` | Pre-processing and resource setup. | -   |
| `[r]` | `[run]` | Main process *Sequential* execution. | variable: datatype |
| `[/]` | `[clean]` | Post-processing and resource cleanup. | -   |
| `[x]` | `[exit]` | End a Pipeline or branch. | -   |
| `[w]` | `[wrap]` | Apply a context manager (macro with built-in setup/cleanup). | ^WrapperName |
| `[?]` | `[switch]` | Switch (flow control) statement. | ?SwitchName |
| `[!]` | `[error]` | Error handler. | !ErrorHandler |
| `[f]` | `[fork]` | Start a parallel (forked) branch. | \\|BranchName |
| `[j]`, `[Y]` | `[join]` | Join parallel branches and consolidate results. | -   |
| `[b]` | `[back]` | Start a background branch (fire-and-forget). | \\|BranchName |
| `[o]` | `[output]` | Pipeline output | `\\|PipelineName>output_name` |
| `[M]` | `[macro]` | Define or use a Macro. | ^MacroName |
| `[v]` | `[inject]` | Injection slot (used inside Macros). | -   |
| `[~]` |     | Element Expansion | -   |
| `[^]` |     | Continue above line. Used for multi-lines. |     |
| `[D]` | `[Define]` | Used to define custom types and imports. | -   |
| `[#]` | `[Enum]` | Enumeration Definition | #EnumName.Element |

### Binary Operators

| Operator | 1st Operand | 2nd Operand | Purpose |
| --- | --- | --- | --- |
| `<<` | Square element | variable | Input Stream |
| `>>` | variable | Square element | Output Stream |
| `$>` | Pipeline instance \ Branch | label | "Label as" |
| `!>` | Error capture label | ErrorType | If Equal to this Error type run branch |
| `?>` | variable | hashable value | If Equal to then run Branch |
| `<-` | Chained Pipeline | Previous Pipeline label | Pipeline Chain operator |
| `.` | Pipeline instance \ Branch | square element | The square element subset. Used for Macro and variable extractions |

### IO Stream Syntax

**Critical Syntax Rule:** IO streams must match the flow with proper type declarations.

**Inline Format:**

```polyglot
[r] input1:type, input2:type >> |PredefinedPipeline >> output1:type, output2:type
```

**Expansion Format (Equivalent):**

```polyglot
[r] |PredefinedPipeline
[~] << input1: type
[~] << input2: type
[~] >> output1: type  
[~] >> output2: type
```

**Shorthand for Single Input/Output:** For single input or output operations, you may omit `>> None` and `None <<`:

```polyglot
\\ Allowed shorthand:
[r] data: py\str >> |ProcessData      \\ Implies >> None
[r] |GetData >> result: py\dict       \\ Implies None <<

\\ However, for multiple inputs/outputs, use expansion for better visualization:
[r] |ComplexProcess
[~] << input1: py\str
[~] << input2: py\int  
[~] >> output1: py\dict
[~] >> output2: py\bool
```

**Recommendation:** Only use shorthand for single input/output operations. The expansion element provides better formatting and visualization for complex operations.

### Square Elements Design Philosophy

The compact Square elements use fixed 3-character notation (`[x]`) to:

1. **Minimize branching footprint:** Complex Pipelines with extensive type conversions and compilations remain visually compact
2. **Enable visual scanning:** Humans, computers, and AI can quickly identify nested branches and Pipeline structure
3. **Reinforce event structure:** Every event must have triggers `[t]`, setup `[\]`, processes `[r]`, and cleanup `[/]`

### Element Expansion `[~]`

To define and extract properties of square elements, Element Expansion `[~]` is used. For example, `[~][!]` is used to define the expanded element error handling.

- Not to be confused with multi-line `[^]` where it is used to multi-line a long line.

```polyglot
[r] |RiskyPipeline
[~][!] !TimeOut << T"3:"   \\ wait 3min
[~][!] !CpuPercentageLimit \\ Set
[~][^] << 80.5             \\ Continuing line above    
```

This consistency enables visual scanning of complex nested Pipelines at a glance.

### Polyglot Data Type System

Polyglot uses a comprehensive type system that bridges multiple programming languages with explicit type declarations.

#### Type Declaration Format

**General Format:** `language\datatype`

Where:

- `language` specifies the programming language context
- `datatype` is the full literal data type as it exists in the native language

#### Supported Languages

Currently supported languages: `c++`, `py`, `rust`, `pg`

**Examples of Language-Specific Types:**

```polyglot
c++\int                 \\ C++ integer
c++\std::string         \\ C++ standard string
c++\std::vector<int>    \\ C++ vector of integers
py\str                  \\ Python string
py\dict                 \\ Python dictionary
py\list                 \\ Python list
rust\&str               \\ Rust string slice
rust\Vec<i32>           \\ Rust vector of 32-bit integers
rust\HashMap<String, i32> \\ Rust hash map
```

#### Type Shortening

Long type names can be shortened using the `[D]` Define element within a Macro:

```polyglot
[M] TypeDefinitions
[D] c++\std::string >> c++\str
[D] c++\std::vector<int> >> c++\intvec
[D] rust\HashMap<String, i32> >> rust\strmap
\\ Note: c++\std::string >> str  \\ ❌ Not allowed, must specify language
[x]
```

#### Language Reference Type

To reference a language itself, use the format `lang\language_name`:

```polyglot
[i] target_language: lang\c++    \\ References C++ language
[i] script_language: lang\py     \\ References Python language
```

#### Native Polyglot Types

Polyglot provides universal types prefixed with `pg\`:

##### Basic Types

- `pg\string` - Universal string type
- `pg\int` - Universal integer
- `pg\float` - Universal floating point
- `pg\bool` - Universal boolean
- `pg\blocks` - Code blocks (used for macros)
- `pg\Enum` - Enumeration type

##### Specialized Types

**`pg\Datetime` - Universal DateTime Type**

Format: `T"YYYY-MM-DD|hh:mm:ss.0000"`

Missing parts are assumed as zero, using separators as guides:

```polyglot
T"2--"         \\ 2 years
T"2-3-"        \\ 2 years and 3 months  
T"3|"          \\ 3 days
T"3:"          \\ 3 minutes
T"3:30"        \\ 3 minutes and 30 seconds
T"3:30:"       \\ 3 hours and 30 minutes
T"200"         \\ 200 nanoseconds
T"5.600"       \\ 5 seconds and 600 nanoseconds
```

**`pg\floatrange` - Numeric Range Type**

Since Polyglot does not have comparison operators, ranges are used for bounds checking:

```polyglot
[r] None >> |CpuPercentage >> cpu_percent: pg\float
[r] |FloatRange
[~] << 80.0: pg\float
[~] << 90.0: pg\float  
[~] >> high_cpu: pg\floatrange

[r] |FloatRange
[~] << 90.0: pg\float
[~] << None  \\ +infinity
[~] >> critical_cpu: pg\floatrange

\\ The ?> operator checks if value is in range
[?] cpu_percent ?> high_cpu 
[~][r] level: pg\Enum, msg: pg\string >> |Log >> None
[~][~] << #LogLevel.warning
[~][~] << "CPU usage in high range"

[?] cpu_percent ?> critical_cpu
[~][r] level: pg\Enum, msg: pg\string >> |Log >> None  
[~][~] << #LogLevel.critical
[~][~] << "CPU usage critical"
```

#### Enumeration Definition

Enumerations are defined using the `[#]` element:

```polyglot
[#] Color
[D] Red
[D] Blue  
[D] Green
[x]

[#] LogLevel
[D] debug
[D] info
[D] warning
[D] error
[D] critical
[x]

\\ Usage in code:
[r] level: pg\Enum, color: pg\Enum >> |DisplayMessage >> None
[~] << #LogLevel.error
[~] << #Color.Red
```

#### Type Conversion System

**Implicit Conversion Rules:**

Implicit conversion occurs during IO stream operations when types are compatible. If conversion is not defined, a **compiler error** is raised.

**Assignment-Based Conversion:**

```polyglot
\\ Allowed when conversion Pipeline exists:
[r] out_pg: pg\string << out_py: py\str    \\ ✅ Converts py\str to pg\string

\\ Compiler error if no conversion defined:
[r] |SomePythonFunction >> out: pg\string  \\ ❌ If function outputs py\str
```

**Conversion Examples:**

```polyglot
\\ Automatic conversions (when conversion Pipelines exist):
py\str → pg\string      \\ ✅ String conversion
py\int → rust\i32       \\ ✅ Integer conversion (if value fits)
py\list → c++\vector    \\ ✅ Container conversion (with element conversion)

\\ Compiler errors (no implicit conversion available):
py\str → py\int         \\ ❌ No implicit string to integer conversion
py\dict → rust\&str     \\ ❌ Incompatible types
c++\std::string → py\int \\ ❌ No logical conversion path
```

#### Error Types

Error types follow the format `language\!ErrorType`:

**Polyglot Native Errors:**

- `pg\!CompilerError` - Syntax/compilation issues
- `pg\!TypeError` - Type conversion failures
- `pg\!RuntimeError` - Execution failures
- `pg\!TimeoutError` - Operation timeouts
- `pg\!ResourceError` - Resource management issues
- `pg\!NetworkError` - Network operation failures
- `pg\!FileError` - File system operation failures

**Language-Specific Errors:**

```polyglot
py\!ValueError          \\ Python ValueError
py\!KeyError            \\ Python KeyError  
rust\!PanicError        \\ Rust panic
c++\!SegmentationFault  \\ C++ segmentation fault
c++\!std::runtime_error \\ C++ standard runtime error
```

**Error Usage Example:**

```polyglot
[r] |RiskyOperation
[~][!] !TimeOut << T"30:" 
[~][~][!] !> timeout_error: pg\!TimeoutError
[~][~][r] |Log
[~][~][~] msg: pg\string << #LogLevel.error
[~][~][~] level: pg\Enum << "Operation timed out after 30 seconds"
[~][~][x] |Exit << 408

[~][!] !PythonError !> py\!ValueError  
[~][~][r] error: py\!ValueError >> |HandlePythonError
[~][~][x] |Exit << 500
```

### Syntax Construction Pattern

**Unified Syntax:** `[SquareElements] ..BodyElements..`

## 5. Advanced Features

### Input Elements `[i]`

The Input elements define the Pipeline's input parameters:

```polyglot
[|] ExamplePipeline
\\ [i] argument\input label\name : datatype
[i] arg1: py\str
[i] arg2: py\int
[t] |Call
[w] PythonEnvironment  
[r] arg1: py\str, arg2: py\int >> |ProcessData >> result: py\dict
[o] result
[x]
```

#### Default Input Parameters

The `Default` keyword sets values when parameters are not provided:

```polyglot
[|] ConfigurablePipeline
[i] message: py\str
[i] Default level: py\int << 1
[i] Default debug: py\bool << False
[t] |Call
[w] PythonEnvironment
[r] message: py\str, level: py\int, debug: py\bool >> |LogMessage
[o] None
[x]
```

### Trigger Elements `[t]`

Trigger elements define conditions that allow Pipeline execution. They must output exactly one boolean value, or a compiler error will be raised.

#### Basic Call Trigger

The `|Call` Pipeline is a standard Pipeline that enables the current Pipeline:

```polyglot
[|] BasicPipeline
[i] data: py\str
\\ Triggered when `BasicPipeline << data` is called from other Pipelines
[t] |Call 
[w] PythonEnvironment
[r] data: py\str >> |ProcessData >> result: py\dict
[o] result
[x]
```

#### Conditional Triggers

Triggers with arguments using Element Expansion:

```polyglot
[|] FileWatcher
[i] file_path: py\str
[t] |Call
[t] |IsFileChanged
[~] << file_path: py\str
[w] FileSystemWrapper
[r] file_path: py\str >> |ProcessFile >> result: py\dict
[o] result
[x]
```

#### Complex Trigger Combinations

Boolean combinations between triggers using Element Expansion:

```polyglot
[|] SecureProcessor
[i] username: py\str
[i] password: py\str
[i] Default api_key: py\str << ""
[t] |Call
[t] |SessionCheck AND (|LocalAuth OR |RemoteAuth)
[~] |SessionCheck << session_id: py\str
[~] |LocalAuth << username: py\str
[~] |LocalAuth << password: py\str
[~] |RemoteAuth << api_key: py\str
[w] AuthenticationWrapper
[r] username: py\str >> |ProcessSecureData >> result: py\dict
[o] result
[x]
```

### Parallel Execution

#### Sequential vs Parallel Elements

| Sequential | Parallel |
| --- | --- |
| `[r]`, `[\]`, `[/]`, `[w]`, `[i]`, `[o]` | `[  |

#### Parallel Execution Rules

- All Sequential elements run after the previous Sequential elements complete
- Outputs of parallel elements can only be used *after* joining via `[j]` with race condition specified
- All `[f]` (fork) elements must be joined at some point before `[x]`, if not used consider using `[b]` for fire-and-forget

#### Advanced Parallel Example

```polyglot
[|] DataAnalysisPipeline
[i] dataset: py\dict
[i] config: py\dict
[t] |Call
[w] AnalysisEnvironment
\\ Sequential validation
[r] dataset: py\dict >> |ValidateData >> clean_dataset: py\dict
\\ Parallel analysis branches
[f] |StatisticalAnalysis
[~] << clean_dataset: py\dict
[~] << config: py\dict
[~] >> stats: py\dict
[f] |MachineLearningAnalysis  
[~] << clean_dataset: py\dict
[~] << config: py\dict
[~] >> ml_results: py\dict, model: py\object
[f] |VisualizationGeneration
[~] << clean_dataset: py\dict
[~] >> charts: py\list
\\ Join all parallel results
[j] |JoinAll
[~] << stats
[~] << ml_results
[~] << charts
[r] stats: py\dict, ml_results: py\dict, charts: py\list >> |CombineResults >> final_report: py\dict
[o] final_report
[x]
```

### Join Element Race Condition Management

The `[j]` Join Element manages different race conditions:

- **JoinAll:** Waits until all parallel processes capture their outputs
- **JoinFirst:** Captures the first one and dumps the rest (may add option to forcefully kill redundant processes) *And stores it in a variable*
- **JoinLast:** Captures the last parallel output *And stores it in a variable*
- **JoinNth:** Captures the nth parallel output *And stores it in a variable*

```polyglot
\\ Cannot use any parallel output variables before join
[j] |JoinAll
[~] << parallel_output_var1
[~] << parallel_output_var2
[~] << parallel_output_var_nth
\\ Now you can use the parallel output variables 
```

```polyglot
\\ Cannot use any parallel output variables before join
[j] |JoinFirst
[~] << parallel_output_var1
[~] << parallel_output_var2
[~] << parallel_output_var_nth
[~] >> result: datatype
\\ Still cannot use any parallel output variables
\\ Only can use 'result'
```

### Switch Pipelines

Using `[?]` you can switch between Pipelines based on conditions:

```polyglot
[|] LoadBalancedProcessor
[i] data: py\dict
[t] |Call
[w] SystemMonitorWrapper
[r] None >> |GetCPUAndMemoryLoadRating >> load_rating: py\int

[?] load_rating ?> 0
[~][r] data: py\dict >> |HeavyComputation >> results: py\dict

[?] load_rating ?> 1
[~][r] data: py\dict >> |MidComputation >> results: py\dict

[?] load_rating ?> 2
[~][r] data: py\dict >> |LightComputation >> results: py\dict

[?] load_rating ?> Default
[~][r] level: pg\Enum, msg: pg\string >> |Log >> None
[~][~] << #LogLevel.error
[~][~] << f"Not enough memory and CPU to compute"
[~][x] |Exit << 430  \\ Exit from whole Pipeline with code 430

[o] results
[x]
```

### Error Handling `[!]`

Error monitors run in parallel to associated Pipelines and can affect execution:

```polyglot
[r] |RiskyPipeline
[~][!] !TimeOut << T"3:" !> timeout1      \\ wait 3min
[~][~] !> #pgErrors.TimeOut
[~][~][r] level: pg\Enum, msg: pg\string >> |Log >> None
[~][~][~] << #LogLevel.error
[~][~][~] << f"Timed out at 3 min" 
[~][~][x] |Exit << 408 \\ Exit from whole Pipeline with code 408

\\ CPU limit monitor  
[~][!] !CpuPercentageLimit
[~][~] << cpu_threshold: py\float << 80.0
[~][~] << level: pg\Enum << #ErrorLevel.warning 
[~][~] !> #pgErrors.CpuLimitReached
[~][~][r] level: pg\Enum, msg: pg\string >> |Log >> None
[~][~][~] << #LogLevel.warning
[~][~][~] << f"CPU at 80%, force close Pipeline" 
[~][~][x] \\ continue as normal

[r] \\ main process continues here
[o] None
[x]
```

### Wrapper Elements and Macros

The `[w]` element is elegant syntax sugar that compiles directly to a Macro invocation:

```polyglot
\\ Macro Definition: The engine behind the [w] sugar 
[M] FileReadCpp  
\\ Macro inputs
[i] run_blocks: pg\blocks
[i] Path: pg\string
\\ Macro Triggers 
[t] |Call 
\\ Setup
[\] Path: pg\string >> |SystemOpenFile >> FileHandle: c++\ofstream
\\ INJECTION SLOT
[r][v] << run_blocks 
\\ Cleanup
[/] FileHandle: c++\ofstream >> |SystemCloseFile >> None
[o] None
[x]

\\ Usage
[|] SomePipeline
[i] filepath: pg\string
[t] |Call
[w] FileReadCpp << filepath >> FileHandle: c++\ofstream
[r] FileHandle: c++\ofstream >> |ReadFileContent >> content: pg\string
[o] content
[x]
```

## 6. Architecture & Implementation

### Under the Hood

The Polyglot system is a micro-service architecture of two main components: the `Trigger Monitor` and `Executioner`. The Polyglot code registers Pipeline triggers, inputs, outputs, and functionality so that the Trigger Monitor knows what triggers to monitor, and when trigger conditions are met for a Pipeline, the Executioner runs the Pipeline.

#### The Executioner Components

- **Language Runtimes:** Persistent or on-demand processes for Python, JS, Rust, Go, C++, etc.
- **Bridges:** There are two kinds of bindings:
  - First is through usual binding, basically repeating the steps you would have done using existing tools to bind, leveraging existing tools like PyBind11, Node's `child_process`, FFI, and legacy tools that bind languages to each other under the hood.
  - Taking advantage of the async nature of Polyglot by compiling and using on the fly, turning things that are compile-time only into runtime. For example, if you have a Rust function that will only accept an array of fixed size, Polyglot can convert your Python list into a fixed Rust array, then use that Rust function and convert it back.
- **Resource Lifecycle:** Manages connection pools, file handles, and memory cleanup across languages.
- **Dependency Graph:** Analyzes variables to enforce correct execution order and prevent races.

#### Trigger Monitor Components

- **File Watch:** Listens to file changes.
- **REST API:** Web service endpoints.
- **Message Queue Listener:** Integration with message queuing systems.

## 7. User Experience & Integration

Users can run Polyglot Pipelines with `[t] |PolyglotRun << parameter: pg\string = parameter_name` trigger through `polyglot -r parameter_name`.

Alternatively, Polyglot may evolve to have packages in the supported languages where they may use the Polyglot Pipelines from the Polyglot service in the background.

### Python Integration Example

```python
import polyglot as pg
from .dataloader import load_file 

def heavy_calculations(data: dict) -> float:
    return pg.run_pipeline('RustCalculation', data)

def main():
    filename = 'data_file.json'
    data = load_file(filename)
    result = heavy_calculations(data)
    print(f'calculation on {filename} = {result}')

if __name__ == '__main__':
    main()
```

### Rust Integration Example

```rust
use polyglot::{run_pipeline, PolyglotErrors};  

fn run_on_python(input_string: &str) -> Result<f32, PolyglotErrors> {  
    run_pipeline(String::from("SomePythonPipeline"), input_string)
}  
```

## 8. Development Roadmap

**Note:** This language is currently in the brainstorming phase. Nothing concrete has been built yet, and we need to plan and collaborate to make it happen.

### Code Interpreter

- [ ] Lexer
  - [ ] Tokens
- [ ] Parser
- [ ] Interpreter
- [ ] Transpiler
- [ ] Compiler

### Trigger Monitor

- [ ] File and Folder watch
- [ ] Scheduler
- [ ] Polyglot CLI

### The Executioner

- [ ] Setup Listeners
- [ ] Implement the Standard Library
  - [ ] Files, {Read, Write}
  - [ ] Run
    - [ ] Python
    - [ ] C++
    - [ ] Rust