# Language Syntax Reference

[← Back to README](../README.md)

## Table of Contents
- [Structural Block Elements](#structural-block-elements)
- [Binary Operators](#binary-operators)
- [Pipeline Structure](#pipeline-structure)
- [Element Execution Order](#element-execution-order)
- [Comments and Multi-line](#comments-and-multi-line)
- [Complete Examples](#complete-examples)

## Structural Block Elements

Polyglot supports both **compact** and **verbose** notation. Compact form is recommended for cleaner code.

| Compact                    | Verbose      | Purpose                                     |
|----------------------------|--------------|---------------------------------------------|
| `[#]`                      | `[Enum]`     | Define an enumeration                       |
| `[@]`                      | `[Import]`   | Declare file namespace for packaging        |
| `[\|]`                     | `[Pipeline]` | Define a pipeline                           |
| `[i]`, `[I]`               | `[input]`    | Define an input                             |
| `[t]`, `[T]`               | `[trigger]`  | Define a trigger condition                  |
| `[q]`, `[Q]`               | `[Queue]`    | Define queue conditions/configuration       |
| `[\]`                      | `[setup]`    | Pre-processing and resource setup           |
| `[r]`, `[R]`               | `[run]`      | Main process sequential execution           |
| `[/]`                      | `[clean]`    | Post-processing and resource cleanup        |
| `[x]`, `[X]`               | `[exit]`     | End a pipeline or branch                    |
| `[w]`, `[W]`               | `[wrap]`     | Apply a wrapper macro (setup/cleanup)       |
| `[?]`                      | `[switch]`   | Switch (flow control) statement             |
| `[!]`                      | `[error]`    | Error handler                               |
| `[f]`, `[F]`               | `[fork]`     | Start a parallel (forked) branch            |
| `[j]`, `[J]`, `[y]`, `[Y]` | `[join]`     | Join parallel branches                      |
| `[b]`, `[B]`               | `[back]`     | Start a background branch (fire-and-forget) |
| `[o]`                      | `[output]`   | Pipeline output                             |
| `[m]`, `[M]`               | `[macro]`    | Define or use a macro                       |
| `[v]`, `[V]`               | `[inject]`   | Injection slot (used inside macros)         |
| `[~]`                      | `[sub]`      | Element expansion (introduce sub-property)  |
| `[^]`                      | `[con]`      | Continue above line (multi-line)            |
| `[D]`                      | `[Define]`   | Define custom types and imports             |
| `[E]`                      | `[Enum]`     | Enumeration definition                      |
| `[>]`                      | `[argin]`    | Input argument binding (block element)      |
| `[<]`                      | `[orgout]`   | Output argument binding (block element)     |

## Binary Operators

| Operator | 1st Operand            | 2nd Operand                    | Purpose                            |
|----------|------------------------|--------------------------------|------------------------------------|
| `!>`     | Error capture          | ErrorType                      | If equal to error type, run branch |
| `?>`     | Block element\Variable | Hashable value\Boolean pipline | If equal or True, then run branch  |
| `<\|<`   | Chained pipeline       | Previous pipeline label        | Pipeline chain operator            |
| `@`      | Namespace accessor     | Library name or alias          | Access imported pipeline           |
| `\| `    | Pipline accessor       | Predefined pipline             | Call a predefined pipeline         |
| `~`      | Array accessor         | Array expansion type           | use Array items programmatically   |

## Pipeline Structure

Every pipeline follows this structure:

```polyglot
[|] PipelineName
[i] input_name: type              \\ Inputs (Mandatory, if none use `[i] None`)
[t] |TriggerPipeline               \\ Triggers (at least one required)
[Q] |QueueConfig                   \\ Queue configuration (optional)
[w] |WrapperMacro                  \\ Wrapper (required at least one [w] or {[\],[/]})

[\] |SetupOperation                \\ Setup phase (required at least one [w] or {[\],[/]})
[>] .arg1: type = setup_input

[r] |MainOperation                 \\ Run phase (required)
[>] .arg1: type = input
[<] .arg2: type = output

[/] |CleanupOperation              \\ Cleanup phase (required at least one [w] or {[\],[/]})
[>] .arg1: type = cleanup_input

[o] .output_name: type = output     \\ Output (required)
[x]                                \\ Exit marker
```

## Element Execution Order

### Phase Order

1. **Declaration Phase** - `[@]` namespace/imports, `[#]` enums
2. **Pipeline Header** - `[|]` name, `[i]` inputs, `[t]` triggers
3. **Queue Configuration** - `[Q]` blocks (checked before execution)
4. **Wrapper Application** - `[w]` macro setup
5. **Setup Phase** - `[\]` sequential execution
6. **Run Phase** - `[r]` sequential, `[f]` parallel, `[b]` background
7. **Join Phase** - `[j]` or `[Y]` synchronization
8. **Cleanup Phase** - `[/]` sequential execution
9. **Output Phase** - `[o]` sequential execution
10. **Exit** - `[x]` termination

### Execution Modes

| Phase          | Elements             | Mode                | Can Fork? |
|----------------|----------------------|---------------------|-----------|
| Declaration    | `[@]`, `[#]`         | Compile-time        | No        |
| Header         | `[\|]`, `[i]`, `[t]` | Registration        | No        |
| Queue          | `[Q]`                | Pre-execution check | No        |
| Wrapper        | `[w]`                | Setup wrapper       | No        |
| Setup          | `[\]`                | Sequential          | No        |
| Run            | `[r]`                | Sequential          | No        |
| Run Fork       | `[f]`                | Parallel            | Yes       |
| Run Background | `[b]`                | Fire-and-forget     | Yes       |
| Join           | `[j]`, `[Y]`         | Synchronization     | No        |
| Cleanup        | `[/]`                | Sequential          | No        |
| Output         | `[o]`                | Sequential          | No        |

## Comments and Multi-line

### Single-line Comments
```polyglot
\\ This is a comment
[r] |Operation  \\ Inline comment
```

### Multi-line 

```polyglot
\*
Multi-line 
Comment
*\
```

### Multi-line Continuation
```polyglot
[D] @AliasOfLongLib = @io.github.organization>VeryLongProjectName
[^] >VeryLongModuleName>SubModule

```

### Element Expansion

There are implicit expansion and explict expansion. There is an element block by definition an expansion for example pipline defition
`[\|]`
- `[t]`
- `[i]`
- `[Q]`
- `[\]`
- `[/]`
- `[w]`
- `[r]`
- `[f]`
- `[?]`
- `[b]`
- `[!]`
- `[x]` - marks the end of pipeline definition block

`[#]`
- `[D]`
- `[x]` - marks the end of pipeline definition block

`[ ] |`, `[ ] ~` pre-defined pipline or array accesser
- `[>]`
- `[<]`


```polyglot
[r] |Some.Operation
[>] .arg1: py\dict = input1
[>] .arg2: py\list = input2
[<] result: py\dict = output
```



explict expansion uses the expansion block `[~]`
- `[!]`
- `[r]`
- `[f]`
- `[?]`
- `[b]`

```polyglot
[f] |SomeRiskyPipeline
[<] .input_argument: pg\string = "Input value"
[~][!] >! pg\!Timeout
[~][~][b] |HandleError
[~][~][b] |Log.Error
[~][~][<] .message: pg\string = "Time out" 
[~][~][x] |U.Exit 
[~][~][<] .code: pg\int = 408
```

## Type Notation

Format: `language\datatype`.

The language is predefined supported languages, and datatype must be exactly as you would declare it in the native language.

Examples:
```polyglot
py\str                    \\ Python string
py\dict                   \\ Python dictionary
py\list                   \\ Python list
py\int                    \\ Python integer
py\float                  \\ Python float

rust\String               \\ Rust String
rust\Vec<i32>             \\ Rust vector of i32
rust\HashMap<String,i64>  \\ Rust HashMap

js\string                 \\ JavaScript string
js\number                 \\ JavaScript number
js\object                 \\ JavaScript object

cpp\std::string           \\ C++ std::string
cpp\std::vector<int>      \\ C++ std::vector<int>

pg\string                 \\ Polyglot string
pg\int                    \\ Polyglot integer
pg\float                  \\ Polyglot float
pg\bool                   \\ Polyglot boolean
pg\bytes                  \\ Polyglot byte array
pg\path                   \\ Polyglot path (file/directory)
pg\Enum                   \\ Polyglot enumeration
```

## Error Type Notation

Format: `language\!ErrorType`

Examples:
```polyglot
py\!ValueError
py\!TypeError
rust\!Panic
rust\!IOError
js\!TypeError
cpp\!Exception
pg\!NetworkError
pg\!ResourceError
```

## Complete Examples

### Simple Sequential Pipeline
```polyglot
[@] com.example>simple
[X]

[|] ProcessFile
[i] file_path: pg\path
[t] |T.Call

[w] |W.Python3.10

[r] |U.Python.File.Text.Read
[>] path: pg\path = file_path
[<] content: py\str = content

[r] |U.Python.String.ToUpperCase
[>] .text: py\str = content
[<] .result: py\str = upper

[r] |U.Python.Console.Print
[>] .message: py\str = upper

[o] .result: py\str = upper
[x]
```

### Parallel Processing Pipeline
```polyglot
[@] com.example>parallel
\\ import lib
[D] @Mylib = @Hasan>Examples>PythonCodes>Examples>DataProcessing1
\\ datatype alias
[D] rust\HashMap = rust\HashMap<string, string> 
[X]

[|] ParallelAnalysis
[i] folder: py\path
[i] Default rust_hevy_compute_file: pg\path = //FileDir//hevy.rs
[t] |T.Folder.NewFiles
[<] .folder: py\path = folder

[w] |W.Python3.10

[r] @Mylib|ValidateData
[>] .data: py\dict = dataset
[<] .validated_data: py\dict = validated

[f] @Mylib|StatisticalAnalysis
[>] .data: py\dict = validated
[<] .statistics: py\dict = stats

[f] |Run.Rust1.8
[<] .file: pg\path = rust_hevy_compute_file
[>] .arg.data: rust\HashMap = validated \\ implict conversion
[<] .predictions: rust\vector<uint> = predictions_rs
[~][r] predictions: py\list = predictions_rs \\ implict conversion
[~][o] predictions

[f] @Mylib|Visualization
[>] .data: py\dict = validated
[<] .charts: py\bytes = charts

[j] |JoinAll
[<] ... stats
[<] ... predictions
[<] ... charts

[r] @Mylib|CombineResults 
[>] .stats: py\dict = stats
[>] .predictions: py\list = predictions
[>] .charts: py\bytes = charts
[<] .report: py\dict = report

[o] final_report: py\dict = report
[x]
```

### Error Handling Pipeline
```polyglot
[@] com.example>errors
[X]

[|] RobustOperation
[i] url: pg\string
[t] |T.Call

[w] |W.Python3.10

[r] |U.Network.HttpGet
[>] url: pg\string = url
[<] response: py\dict = response

[!] !> pg\!NetworkError
[~][r] |U.Log
[~][>] level: pg\string = #LogLevel.Warning
[~][>] message: pg\string = "Network error, retrying"
[~]
[~][r] |U.Error.RetryWithBackoff
[~][>] max_attempts: pg\int = 3
[~]
[~][r] |U.Network.HttpGet
[~][>] url: pg\string = url
[~][<] response: py\dict = response

[!] !> pg\!TimeoutError
[~][r] |U.Log
[~][>] level: pg\string = #LogLevel.Error
[~][>] message: pg\string = "Timeout, using cached data"
[~]
[~][r] |FetchFromCache
[~][>] url: pg\string = url
[~][<] cached_data: py\dict = response

[o] result: py\dict = response
[x]
```

### Switch Statement Pipeline
```polyglot
[@] com.example>switch
[X]

[|] ProcessByStatus
[i] status: pg\string
[t] |T.Call

[?] status ?> "pending"
[~][r] |U.Log
[~][>] .message: pg\string = "Status is pending"
[~]
[~][r] |HandlePending

[?] status ?> "complete"
[~][r] |U.Log
[~][>] .message: pg\string = "Status is complete"
[~]
[~][r] |HandleComplete
[~][x] |Exit
[~][>] .code: pg\int = 0

[?] status ?> "failed"
[~][r] |U.Log.Error
[~][>] .message: pg\string = "Status is failed"
[~]
[~][r] |HandleFailure
[~][x] |Exit
[~][>] .code: pg\int = 1

[r] |U.Log
[>] .message: pg\string = "Unknown status"

[x]
```

### Complex Queue Configuration
```polyglot
[@] com.example>queued
[X]

[|] ResourceIntensiveTask
[i] large_data: py\dict
[t] |T.FileChanged
[>] pattern: pg\string = "data/*.csv"

[Q] |Q.ToQueue
[>] queue: pg\string = #Queue.Default

[Q] |Q.Priority
[>] level: pg\int = 2

[Q] |Q.CpuAvailable
[>] percent: pg\float = 70.0

[Q] |Q.MemoryAvailable
[>] megabytes: pg\int = 8192

[Q] |Q.MaxAttempts
[>] attempts: pg\int = 5

[Q] |Q.RetryStrategy
[>] strategy: pg\string = #RetryStrategy.Exponential

[Q] |Q.Kill.CpuLimit
[>] percent: pg\float = 90.0

[Q] |Q.Kill.MemoryLimit
[>] percent: pg\float = 95.0

[Q] |Q.Kill.ExecutionTimeout
[>] duration: pg\string = T"30:"

[w] |W.Python3.10

[\] |U.Log
[>] message: pg\string = "Starting intensive task"

[r] |ProcessLargeData
[>] data: py\dict = large_data
[<] result: py\dict = result

[/] |U.Log
[>] message: pg\string = "Completed intensive task"

[o] output: py\dict = result
[x]
```

---

[Next: Type System →](03-type-system.md)