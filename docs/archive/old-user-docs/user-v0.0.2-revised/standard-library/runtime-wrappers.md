# Runtime Wrappers

**Version:** 0.0.2
**Last Updated:** 2025-11-11
**Status:** Complete

---

## Overview

Runtime wrappers (`|W.*`) allow Polyglot pipelines to execute code in other programming languages. This enables seamless integration with existing codebases and leveraging language-specific libraries.

**Supported Runtimes:**
- Python (multiple versions)
- Node.js (multiple versions)
- Rust
- Go
- Ruby
- Deno

**Key Features:**
- Fixed version wrappers
- Dynamic version wrappers
- Multiple runtimes in single pipeline
- Transparent integration
- Implementation uses `uv` (internal detail)

---

## Table of Contents

1. [Wrapper Basics](#wrapper-basics)
2. [Block Marker: `[W]`](#block-marker-w)
3. [Special Wrapper: `|W.Polyglot.Scope`](#special-wrapper-wpolyglotscope) ⭐ **Important!**
4. [Fixed Version Wrappers](#fixed-version-wrappers)
5. [Dynamic Version Wrappers](#dynamic-version-wrappers)
6. [Supported Runtimes](#supported-runtimes)
7. [Multiple Wrappers](#multiple-wrappers)
8. [Wrapper Scope](#wrapper-scope)
9. [Best Practices](#best-practices)
10. [Examples](#examples)

---

## Wrapper Basics

### What are Runtime Wrappers?

**Runtime wrappers** establish an execution environment for running code in languages other than Polyglot.

**Think of them as:**
- Context managers
- Environment activators
- Language bridges

---

### Why Use Wrappers?

**Integration:**
- Use existing Python/Node/Rust libraries
- Leverage language-specific tools
- Integrate with legacy code

**Specialization:**
- Use Python for data science
- Use Node for web APIs
- Use Rust for performance-critical code

**Flexibility:**
- Mix languages in single pipeline
- Choose best tool for each task
- Gradual migration paths

---

### Wrapper Syntax

**Fragment showing wrapper usage pattern:**
```polyglot
// ... (part of pipeline) ...
[W] |W.RuntimeName
[r] |YourOperation
// ... (rest of pipeline) ...
```

**Complete Example:**
```polyglot
[|] AnalyzeDataPipeline
[i] .data: pg\string
[t] |T.Call
[W] |W.Python3.11                    // Wrapper for Python runtime

[r] |AnalyzeData                     // Operation runs in Python context
[<] .input: py\str << .data
[>] .output: py\str >> .result

[o] .result: pg\string
[X]
```

---

## Block Marker: `[W]`

### Purpose

The `[W]` block marker establishes a wrapper context for subsequent operations.

---

### Syntax

**Fragment:**
```polyglot
// ... (part of pipeline) ...
[W] |W.Runtime
// ... (operations follow) ...
```

**Complete Example:**
```polyglot
[|] DataProcessing
[i] .data: pg\string
[t] |T.Call
[W] |W.Python3.11                    // Wrapper declaration

[r] |PythonOperation                 // Runs in Python context
[<] .input: py\str << .data
[>] .output: py\str >> .result

[o] .result: pg\string
[X]
```

---

### Scope

Operations after `[W]` run in that wrapper context until:
1. A new `[W]` is declared (changes context)
2. The pipeline ends

```polyglot
[|] MultiLanguage
[i] .data: pg\string
[t] |T.Call

// Native Polyglot operations first
[W] |W.Polyglot.Scope
[r] .initial: pg\string << .data

// Python context starts
[W] |W.Python3.11
[r] |PythonOp1
[<] .input: py\str << .initial
[>] .result1: py\str >> .py_result

[r] |PythonOp2
[<] .data: py\str << .py_result
[>] .result2: py\str >> .final_py

// Node context starts
[W] |W.Node20
[r] |NodeOp1
[<] .input: node\string << .final_py
[>] .result: node\string >> .node_result

[r] |NodeOp2
[<] .data: node\string << .node_result
[>] .final: node\string >> .final_result

[o] .final_result: pg\string
[X]
```

---

## Special Wrapper: `|W.Polyglot.Scope`

### The Scope Management Wrapper

**Critical Concept:** `|W.Polyglot.Scope` manages variable lifecycle and memory cleanup for all pipelines.

**Purpose:**
- Manages variable lifecycle
- Handles memory cleanup when variables go out of scope
- Marks the end of variable lifetime
- Ensures proper resource deallocation

**Behavior:**
- **IMPLICIT** when other wrappers are present (Python, Node, Rust, etc.)
- **EXPLICIT** when NO other wrappers are defined
- **CANNOT** leave `[W]` blank - must declare a wrapper

---

### When to Explicitly Use `|W.Polyglot.Scope`

**Use it explicitly when:** You don't need any language-specific runtime (Python, Node, Rust, etc.)

```polyglot
[|] MyPipeline
[i] .input: pg\string
[t] |T.Call
[W] |W.Polyglot.Scope    // EXPLICIT - required when no other wrappers

[r] .result: pg\string << "value"
[o] .result: pg\string
[X]
// When pipeline ends, |W.Polyglot.Scope cleans up:
// - .input goes out of scope
// - .result goes out of scope
// - Memory is freed
```

**Invalid - Missing Wrapper:**
```polyglot
[|] MyPipeline
[i] .input: pg\string
[t] |T.Call
// ✗ ERROR: No [W] wrapper declared! Must use |W.Polyglot.Scope

[r] .result: pg\string << "value"
[o] .result: pg\string
[X]
```

**Valid - Implicit with Other Wrapper:**
```polyglot
[|] PythonPipeline
[i] .input: pg\string
[t] |T.Call
[W] |W.Python3.11    // |W.Polyglot.Scope is IMPLICIT here

[r] |PythonOperation
[o] !NoError
[X]
// |W.Polyglot.Scope is automatically added when Python wrapper is used
```

---

### Variable Lifecycle and Scope

**Variables exist from declaration until pipeline ends:**

```polyglot
[|] ProcessData
[i] .data: pg\string
[t] |T.Call
[W] |W.Polyglot.Scope    // Explicit - no other wrappers needed

// LIFECYCLE START: .data declared
[r] .processed: pg\string << .data + " processed"
// LIFECYCLE START: .processed declared

[o] .processed: pg\string
[X]
// LIFECYCLE END: Pipeline ends
// |W.Polyglot.Scope cleanup:
//   1. .data marked for garbage collection
//   2. .processed marked for garbage collection
//   3. Memory freed
```

---

### Memory Cleanup Rules

**What gets cleaned up:**
1. **All input variables** (declared with `[i]`)
2. **All internal variables** (declared with `[r]`)
3. **All output variables** (declared with `[o]`)
4. **Temporary pipeline-scoped data**

**When cleanup happens:**
- **Immediately** after pipeline completes (success or failure)
- **Before** control returns to calling pipeline
- **Automatically** - no manual cleanup needed

---

### Example: Default Wrapper (Required)

**Correct - Explicit Polyglot.Scope:**
```polyglot
[|] SimpleOperation
[i] .input: pg\int
[t] |T.Call
[W] |W.Polyglot.Scope    // EXPLICIT - required when no other wrappers

[r] .doubled: pg\int << .input * 2
[o] .doubled: pg\int
[X]
// Cleanup happens automatically
```

**Correct - Implicit with Runtime Wrapper:**
```polyglot
[|] PythonOperation
[i] .input: pg\int
[t] |T.Call
[W] |W.Python3.11        // Polyglot.Scope IMPLICIT here

[r] |DoubleInPython
[o] .result: pg\int
[X]
// |W.Polyglot.Scope automatically added with Python wrapper
```

**Incorrect - Missing wrapper:**
```polyglot
[|] SimpleOperation
[i] .input: pg\int
[t] |T.Call
// ✗ ERROR: No [W] wrapper! Must declare one.

[r] .doubled: pg\int << .input * 2
[o] .doubled: pg\int
[X]
```

**Wrapper Declaration Rules:**
- `[W]` marker CANNOT be left blank
- If you don't need Python/Node/Rust → use `[W] |W.Polyglot.Scope` (EXPLICIT)
- If you DO use Python/Node/Rust → `|W.Polyglot.Scope` is IMPLICIT (automatically added)
- This ensures every pipeline has proper scope management

---

### Nested Pipeline Scopes

**Each pipeline has its own scope:**

```polyglot
[|] OuterPipeline
[i] .outer_data: pg\string
[t] |T.Call
[W] |W.Polyglot.Scope    // Outer scope

[r] |InnerPipeline
[<] .input: pg\string << .outer_data
[>] .result: pg\string >> .inner_result

[o] .inner_result: pg\string
[X]
// Outer scope cleanup: .outer_data, .inner_result freed

[|] InnerPipeline
[i] .input: pg\string
[t] |T.Call
[W] |W.Polyglot.Scope    // Inner scope (separate!)

[r] .result: pg\string << .input + " processed"
[o] .result: pg\string
[X]
// Inner scope cleanup: .input, .result freed
// This happens BEFORE returning to OuterPipeline
```

**Key Point:** Each pipeline creates a new scope. Variables in inner pipelines are cleaned up before returning to outer pipelines.

---

### Why This Matters

**Memory Safety:**
- No memory leaks
- Automatic resource management
- Clear variable lifetimes

**Performance:**
- Immediate cleanup after pipeline ends
- No garbage collection delays
- Predictable memory usage

**Simplicity:**
- No manual `free()` or `delete` calls
- No `try/finally` cleanup blocks
- Just write pipelines - cleanup is automatic

---

### Advanced: Implicit Scope with Language Wrappers

**When you use a language wrapper, `|W.Polyglot.Scope` is IMPLICITLY added:**

```polyglot
[|] PythonOperation
[i] .data: pg\string
[t] |T.Call
[W] |W.Python3.11    // |W.Polyglot.Scope IMPLICIT here

[r] |PythonCode
[<] .input: py\str << .data
[>] .output: py\str >> .result

[o] .result: pg\string
[X]
// |W.Polyglot.Scope automatically manages:
// - Polyglot variable cleanup (.data, .result)
// Python wrapper manages:
// - Python-specific resources (py\str conversions, etc.)
```

**This means:**
- You get scope management automatically
- No need to explicitly declare `|W.Polyglot.Scope` when using Python/Node/Rust
- Variable lifecycle is always managed properly

---

### Key Takeaways

1. **`|W.Polyglot.Scope` behavior depends on context:**
   - **EXPLICIT** when no other wrappers → Must write `[W] |W.Polyglot.Scope`
   - **IMPLICIT** when other wrappers present → Automatically added
2. **Variables are cleaned up automatically** - When pipeline ends
3. **Each pipeline has its own scope** - Nested pipelines = nested scopes
4. **Memory is freed immediately** - No delays, no leaks
5. **You write less code** - No manual cleanup needed
6. **[W] marker cannot be blank** - Every pipeline must declare a wrapper

---

## Fixed Version Wrappers

### What are Fixed Version Wrappers?

**Fixed version wrappers** specify an exact runtime version.

**Format:**
```
|W.Runtime.Version
```

**Example:**
```polyglot
[W] |W.Python3.11
[W] |W.Node20
[W] |W.Ruby3.2
```

---

### When to Use Fixed Versions

**Use fixed versions when:**
- Reproducibility is critical
- Code depends on specific version features
- Testing against specific version
- Production deployments

```polyglot
// ✓ GOOD - Reproducible
[W] |W.Python3.11
[r] |DataAnalysis
```

---

### Python Fixed Versions

Available Python versions (examples):

```polyglot
[W] |W.Python3.8
[W] |W.Python3.9
[W] |W.Python3.10
[W] |W.Python3.11
[W] |W.Python3.12
```

**Example:**
```polyglot
[|] PythonDataAnalysis
[i] .data: pg\string

[W] |W.Python3.11
[r] |RunAnalysis
[<] .script: pg\path << "analyze.py"
[<] .input: pg\string << .data
[>] .result: pg\string >> analysis_result

[X]
```

---

### Node.js Fixed Versions

Available Node versions (examples):

```polyglot
[W] |W.Node18
[W] |W.Node20
[W] |W.Node21
```

**Example:**
```polyglot
[|] NodeWebService
[i] .request: pg\serial

[W] |W.Node20
[r] |ProcessRequest
[<] .data: pg\serial << .request
[>] .response: pg\serial >> result

[X]
```

---

### Other Runtime Fixed Versions

**Rust:**
```polyglot
[W] |W.Rust
[r] |PerformanceTask
```

**Go:**
```polyglot
[W] |W.Go
[r] |ConcurrentTask
```

**Ruby:**
```polyglot
[W] |W.Ruby3.2
[r] |RubyScript
```

**Deno:**
```polyglot
[W] |W.Deno
[r] |DenoTask
```

---

## Dynamic Version Wrappers

### What are Dynamic Version Wrappers?

**Dynamic version wrappers** use the latest available version of a runtime.

**Format:**
```
|W.Runtime
```

**Example:**
```polyglot
[W] |W.Python
[W] |W.Node
[W] |W.Ruby
```

---

### When to Use Dynamic Versions

**Use dynamic versions when:**
- Prototyping and development
- Code is version-agnostic
- Always want latest features
- Local development

```polyglot
// ✓ GOOD for development
[W] |W.Python
[r] |TestScript
```

---

### Python Dynamic

```polyglot
[W] |W.Python  // Uses latest Python 3.x
[r] |RunScript
```

**Behavior:** Uses latest available Python 3.x version on the system.

---

### Node.js Dynamic

```polyglot
[W] |W.Node  // Uses latest Node.js
[r] |RunScript
```

**Behavior:** Uses latest available Node.js version on the system.

---

### Trade-offs

| Aspect | Fixed Version | Dynamic Version |
|--------|---------------|-----------------|
| **Reproducibility** | ✓ Guaranteed | ✗ May vary |
| **Maintenance** | Update manually | Auto updates |
| **Production** | ✓ Recommended | ⚠ Caution |
| **Development** | Optional | ✓ Convenient |
| **CI/CD** | ✓ Predictable | ⚠ May break |

---

## Supported Runtimes

### Python

**Namespace:** `|W.Python*`

**Versions:**
- `|W.Python` - Latest Python 3.x
- `|W.Python3.8`
- `|W.Python3.9`
- `|W.Python3.10`
- `|W.Python3.11`
- `|W.Python3.12`

**Use Cases:**
- Data analysis
- Machine learning
- Scientific computing
- Scripting

**Example:**
```polyglot
[W] |W.Python3.11
[r] |DataProcessing
[<] .script: pg\path << "process.py"
[<] .data_file: pg\path << \\DataDir\\input.csv
[>] .results: pg\string >> processed_data
```

---

### Node.js

**Namespace:** `|W.Node*`

**Versions:**
- `|W.Node` - Latest Node.js
- `|W.Node18`
- `|W.Node20`
- `|W.Node21`

**Use Cases:**
- Web APIs
- JSON processing
- Async I/O
- JavaScript libraries

**Example:**
```polyglot
[W] |W.Node20
[r] |APICall
[<] .endpoint: pg\string << "https://api.example.com"
[<] .params: pg\serial << request_params
[>] .response: pg\serial >> api_response
```

---

### Rust

**Namespace:** `|W.Rust`

**Versions:**
- `|W.Rust` - Latest stable Rust

**Use Cases:**
- Performance-critical code
- Systems programming
- Memory-safe operations
- Compiled performance

**Example:**
```polyglot
[W] |W.Rust
[r] |FastComputation
[<] .input: pg\array{pg\int} << large_dataset
[>] .result: pg\array{pg\int} >> processed_data
```

---

### Go

**Namespace:** `|W.Go`

**Versions:**
- `|W.Go` - Latest Go

**Use Cases:**
- Concurrent operations
- Network services
- Systems tools
- Fast compilation

**Example:**
```polyglot
[W] |W.Go
[r] |ConcurrentProcessor
[<] .workers: pg\int << 10
[<] .tasks: pg\array{pg\string} << task_list
[>] .results: pg\array{pg\string} >> processed_tasks
```

---

### Ruby

**Namespace:** `|W.Ruby*`

**Versions:**
- `|W.Ruby` - Latest Ruby
- `|W.Ruby3.2`
- `|W.Ruby3.3`

**Use Cases:**
- Text processing
- Scripting
- DSLs
- Rapid prototyping

**Example:**
```polyglot
[W] |W.Ruby3.2
[r] |TextProcessing
[<] .input: pg\string << text_data
[>] .result: pg\string >> processed_text
```

---

### Deno

**Namespace:** `|W.Deno`

**Versions:**
- `|W.Deno` - Latest Deno

**Use Cases:**
- Modern JavaScript/TypeScript
- Secure by default
- Web APIs
- Module system

**Example:**
```polyglot
[W] |W.Deno
[r] |TypeScriptTask
[<] .script: pg\path << "process.ts"
[>] .result: pg\string >> output
```

---

## Multiple Wrappers

### Using Multiple Runtimes

A single pipeline can use multiple runtime wrappers:

```polyglot
[|] MultiRuntimePipeline
[i] .data: pg\string

// Step 1: Python processing
[W] |W.Python3.11
[r] |PythonAnalyze
[<] .input: pg\string << .data
[>] .analysis: pg\string >> python_result

// Step 2: Node processing
[W] |W.Node20
[r] |NodeTransform
[<] .input: pg\string << python_result
[>] .transformed: pg\string >> node_result

// Step 3: Rust performance processing
[W] |W.Rust
[r] |RustOptimize
[<] .input: pg\string << node_result
[>] .optimized: pg\string >> final_result

[X]
```

---

### Wrapper Transitions

Each `[W]` declaration transitions to a new wrapper context:

```polyglot
[|] WrapperTransitions
// Context 1: Python
[W] |W.Python3.11
[r] |PythonOp1
[r] |PythonOp2

// Context 2: Node (Python context ends)
[W] |W.Node20
[r] |NodeOp1

// Context 3: Back to Python
[W] |W.Python3.11
[r] |PythonOp3

[X]
```

---

### Data Passing Between Wrappers

Data flows seamlessly between wrappers:

```polyglot
[|] DataFlow
[i] .input: pg\string

// Python: Generate data
[W] |W.Python3.11
[r] |GenerateData
[<] .seed: pg\string << .input
[>] .data: pg\string >> python_data

// Node: Process data
[W] |W.Node20
[r] |ProcessData
[<] .input: pg\string << python_data  // From Python
[>] .processed: pg\string >> node_data

// Rust: Optimize data
[W] |W.Rust
[r] |OptimizeData
[<] .input: pg\string << node_data  // From Node
[>] .optimized: pg\string >> final_data

[X]
```

**Key Point:** Polyglot handles serialization/deserialization between runtimes.

---

## Wrapper Scope

### Operations Within Wrapper Context

All operations after `[W]` run in that wrapper until changed:

```polyglot
[W] |W.Python3.11
[r] |Op1  // Runs in Python
[r] |Op2  // Runs in Python
[r] |Op3  // Runs in Python

[W] |W.Node20
[r] |Op4  // Runs in Node
[r] |Op5  // Runs in Node
```

---

### Nested Operations

Nested operations inherit wrapper context:

```polyglot
[W] |W.Python3.11
[p] |ParallelTask
[<] .input: pg\string << data
[~][r] |PythonOp1  // Runs in Python context
[~][<] .param: pg\string << .input
[>] .result >> output
```

---

### Wrapper Context Inheritance

**Question for consideration:** How do wrapper contexts interact with:
- Parallel blocks?
- Pipeline calls?
- Nested operations?

**Current assumption:** Wrapper context is inherited by child operations within the same pipeline.

---

## Best Practices

### 1. Use Fixed Versions in Production

```polyglot
// ✓ GOOD - Reproducible production code
[W] |W.Python3.11
[r] |CriticalOperation

// ✗ RISKY - May break with updates
[W] |W.Python
[r] |CriticalOperation
```

---

### 2. Choose Appropriate Runtime

```polyglot
// ✓ GOOD - Right tool for the job
[W] |W.Python3.11
[r] |DataScience  // Python for ML/data

[W] |W.Node20
[r] |WebAPI  // Node for web APIs

[W] |W.Rust
[r] |PerformanceCritical  // Rust for speed

// ✗ POOR - Wrong tool
[W] |W.Python3.11
[r] |PerformanceCritical  // Should use Rust
```

---

### 3. Minimize Wrapper Transitions

```polyglot
// ✓ GOOD - Batch operations per runtime
[W] |W.Python3.11
[r] |PythonOp1
[r] |PythonOp2
[r] |PythonOp3

[W] |W.Node20
[r] |NodeOp1
[r] |NodeOp2

// ✗ INEFFICIENT - Too many transitions
[W] |W.Python3.11
[r] |PythonOp1

[W] |W.Node20
[r] |NodeOp1

[W] |W.Python3.11  // Back to Python
[r] |PythonOp2
```

---

### 4. Handle Runtime Dependencies

```polyglot
// ✓ GOOD - Document dependencies
/*
 * Requires:
 * - Python 3.11+ with pandas, numpy
 * - Node 20+ with express, axios
 */
[|] DataPipeline
[W] |W.Python3.11
[r] |AnalyzeWithPandas

[W] |W.Node20
[r] |ServeWithExpress
[X]
```

---

### 5. Consider Performance Overhead

```polyglot
// ✓ GOOD - Use wrappers when necessary
[W] |W.Python3.11
[r] |ComplexDataAnalysis  // Needs Python libraries

// ✗ UNNECESSARY - Use native Polyglot
[W] |W.Python3.11
[r] |SimpleStringConcat  // Don't need Python for this
```

---

### 6. Test with Multiple Versions

```polyglot
// ✓ GOOD - Test compatibility
[|] TestPipeline

// Test with Python 3.10
[W] |W.Python3.10
[r] |RunTests

// Test with Python 3.11
[W] |W.Python3.11
[r] |RunTests

// Test with Python 3.12
[W] |W.Python3.12
[r] |RunTests

[X]
```

---

### 7. Document Wrapper Requirements

```polyglot
// ✓ GOOD - Clear requirements
[|] MLPipeline
// Requires: Python 3.11+ with tensorflow, scikit-learn
[W] |W.Python3.11
[r] |TrainModel
[X]
```

---

### 8. Use Dynamic Versions for Development

```polyglot
// ✓ GOOD - Flexible during development
[|] DevelopmentPipeline
[W] |W.Python  // Latest for dev
[r] |TestFeature
[X]

// ✓ GOOD - Fixed for production
[|] ProductionPipeline
[W] |W.Python3.11  // Specific for prod
[r] |RunFeature
[X]
```

---

## Examples

### Example 1: Python Data Analysis

```polyglot
[|] AnalyzeCustomerData
[i] .data_file: pg\path

[W] |W.Python3.11
[r] |LoadData
[<] .file: pg\path << .data_file
[>] .dataframe: pg\string >> df

[r] |AnalyzeData
[<] .df: pg\string << df
[>] .insights: pg\string >> analysis

[r] |GenerateReport
[<] .analysis: pg\string << analysis
[>] .report: pg\string >> final_report

[o] .report: pg\string
[X]
```

---

### Example 2: Node.js Web API

```polyglot
[|] FetchUserData
[i] .user_id: pg\string

[W] |W.Node20
[r] |CallAPI
[<] .endpoint: pg\string << "https://api.example.com/users"
[<] .id: pg\string << .user_id
[>] .response: pg\serial >> user_data

[r] |ParseResponse
[<] .data: pg\serial << user_data
[>] .parsed: pg\serial >> user_info

[o] .user: pg\serial
[X]
```

---

### Example 3: Multi-Runtime Pipeline

```polyglot
[|] CompleteWorkflow
[i] .input_file: pg\path

// Step 1: Python - Load and clean data
[W] |W.Python3.11
[r] |LoadCSV
[<] .file: pg\path << .input_file
[>] .data: pg\string >> raw_data

[r] |CleanData
[<] .input: pg\string << raw_data
[>] .cleaned: pg\string >> clean_data

// Step 2: Node - Transform to JSON
[W] |W.Node20
[r] |ConvertToJSON
[<] .data: pg\string << clean_data
[>] .json: pg\string >> json_data

// Step 3: Rust - Performance processing
[W] |W.Rust
[r] |OptimizeData
[<] .input: pg\string << json_data
[>] .optimized: pg\string >> final_data

[o] .result: pg\string
[X]
```

---

### Example 4: Parallel Multi-Runtime

```polyglot
[|] ParallelProcessing
[i] .data: pg\string

// Process in Python and Node in parallel
[p] |PythonProcessing
[W] |W.Python3.11
[r] |AnalyzeWithPython
[<] .input: pg\string << .data
[>] .result >> python_result

[p] |NodeProcessing
[W] |W.Node20
[r] |TransformWithNode
[<] .input: pg\string << .data
[>] .result >> node_result

// Join results
[Y] |Y.Join
[>] python_result
[>] node_result

// Combine with Rust
[W] |W.Rust
[r] |CombineResults
[<] .python: pg\string << python_result
[<] .node: pg\string << node_result
[>] .combined: pg\string >> final_result

[X]
```

---

## See Also

### Language Specification
- [Block Markers](../language/block-markers.md) - `[W]` marker details
- [Complete Syntax Reference](../language/syntax-complete.md) - Wrapper syntax

### Standard Library
- [Overview](overview.md) - Standard library organization

### Examples
- [Complete Workflows](../examples/complete-workflows.md) - Multi-runtime examples

### Planning
- [Decision Log](../decision-log.md) - Wrapper decisions (#21)

---

**End of Runtime Wrappers Reference**