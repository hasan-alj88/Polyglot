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
| `[\|]`                     | `[Pipeline]` | Define a pipline                            | Define a pipeline |
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

## Binary Operators

| Operator | 1st Operand             | 2nd Operand                    | Purpose                            |
|----------|-------------------------|--------------------------------|------------------------------------|
| `<<`     | Square element\pipline  | Variable                       | Input stream                       |
| `>>`     | Square element\pipline  | Variable                       | Output stream                      |
| `!>`     | Error capture           | ErrorType                      | If equal to error type, run branch |
| `?>`     | Square element\Variable | Hashable value\Boolean pipline | If equal or True, then run branch  |
| `<\|<`   | Chained pipeline        | Previous pipeline label        | Pipeline chain operator            |
| `@`      | Namespace accessor      | Library name or alias          | Access imported pipeline           |

## Pipeline Structure

Every pipeline follows this structure:

```polyglot
[|] PipelineName
[i] input_name: type              \\ Inputs (Madatory, if none use `[i] None`
[t] |TriggerPipeline               \\ Triggers (at least one required)
[Q] |QueueConfig                   \\ Queue configuration (optional)
[w] |WrapperMacro                  \\ Wrapper (optional)

[\] |SetupOperation                \\ Setup phase (optional)
[~] << setup_input

[r] |MainOperation                 \\ Run phase (required)
[~] << input >> output

[/] |CleanupOperation              \\ Cleanup phase (optional)
[~] << cleanup_input

[o] >> output                      \\ Output (required)
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

```ployglot
\*
Multi-line 
Comment
*\
```

### Multi-line Continuation
```polyglot
[D] @AliasOfLongLib << @io.github.organization>VeryLongProjectName
[^] >VeryLongModuleName>SubModule

[r] |SomePipeline << very_long_variable_name
[^] << another_long_variable
[^] >> output_result
```

### Element Expansion
```polyglot
[r] |Operation
[~] << input1: py\dict
[~] << input2: py\list  
[~] >> output: py\dict
```

## Type Notation

Format: `language\datatype`

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

cpp\string                \\ C++ std::string
cpp\vector<int>           \\ C++ std::vector<int>

pg\string                 \\ Polyglot string
pg\int                    \\ Polyglot integer
pg\float                  \\ Polyglot float
pg\bool                   \\ Polyglot boolean
pg\bytes                  \\ Polyglot byte array
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
[i] file_path: pg\string
[t] |T.Call

[w] |W.Python3.10

[r] |U.System.File.Text.Read << file_path >> content: py\str
[r] |U.String.ToUpperCase << content >> upper: py\str
[r] |U.Console.Print << upper

[o] >> upper
[x]
```

### Parallel Processing Pipeline
```polyglot
[@] com.example>parallel
[X]

[|] ParallelAnalysis
[i] dataset: py\dict
[t] |T.Call

[w] |W.Python3.10

[r] |ValidateData << dataset >> validated: py\dict

[f] |StatisticalAnalysis << validated >> stats: py\dict
[f] |MachineLearning << validated >> predictions: py\list
[f] |Visualization << validated >> charts: py\bytes

[j] |JoinAll

[r] |CombineResults 
[~] << stats 
[~] << predictions 
[~] << charts 
[~] >> report: py\dict

[o] >> report
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

[r] |U.Network.HttpGet << url >> response: py\dict

[~][!] !> pg\!NetworkError
[~][~][r] |U.Log << #LogLevel.Warning << "Network error, retrying"
[~][~][r] |U.Error.RetryWithBackoff << max_attempts: 3
[~][~][r] |U.Network.HttpGet << url >> response

[~][!] !> pg\!TimeoutError
[~][~][r] |U.Log << #LogLevel.Error << "Timeout, using cached data"
[~][~][r] |FetchFromCache << url >> response

[o] >> response
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
[~][r] |U.Log << "Status is pending"
[~][r] |HandlePending

[?] status ?> "complete"
[~][r] |U.Log << "Status is complete"
[~][r] |HandleComplete
[~][x] |Exit << 0

[?] status ?> "failed"
[~][r] |U.Log.Error << "Status is failed"
[~][r] |HandleFailure
[~][x] |Exit << 1

[r] |U.Log << "Unknown status"
[x]
```

### Complex Queue Configuration
```polyglot
[@] com.example>queued
[X]

[|] ResourceIntensiveTask
[i] large_data: py\dict
[t] |T.FileChanged << "data/*.csv"

[Q] |Q.ToQueue << #Queue.Default
[Q] |Q.Priority << 2
[Q] |Q.CpuAvailable << 70.0
[Q] |Q.MemoryAvailable << 8192
[Q] |Q.MaxAttempts << 5
[Q] |Q.RetryStrategy << #RetryStrategy.Exponential
[Q] |Q.Kill.CpuLimit << 90.0
[Q] |Q.Kill.MemoryLimit << 95.0
[Q] |Q.Kill.ExecutionTimeout << T"30:"

[w] |W.Python3.10

[\] |U.Log << "Starting intensive task"
[r] |ProcessLargeData << large_data >> result: py\dict
[/] |U.Log << "Completed intensive task"

[o] >> result
[x]
```

---

[Next: Type System →](03-type-system.md)