---
status: deprecated
archived: 2026-03-22
source_branch: main
note: Superseded by docs/user/ and docs/technical/ in v0.0.4 documentation rewrite
replaced_by: docs/user/pglib/pipelines/W/INDEX.md
---
<!-- @d:docs/user/pglib/pipelines/W/INDEX.md -->
> **Deprecated:** This document is superseded. See the current spec for up-to-date content.

# Runtime Wrappers

**Version:** 0.0.2
**Last Updated:** 2025-11-11
**Status:** Complete

---

## Overview

<!-- @c:reference/glossary#pipeline -->
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
2. [Block Marker: `[w]`](#block-marker-w)
3. [Fixed Version Wrappers](#fixed-version-wrappers)
4. [Dynamic Version Wrappers](#dynamic-version-wrappers)
5. [Supported Runtimes](#supported-runtimes)
6. [Multiple Wrappers](#multiple-wrappers)
7. [Wrapper Scope](#wrapper-scope)
8. [Best Practices](#best-practices)
9. [Examples](#examples)

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

```polyglot
[w] |W.RuntimeName
[r] |YourOperation
[X]
```

**Example:**
```polyglot
[w] |W.Python3.11
[r] |AnalyzeData
[<] .input: pg\string << data
[X]
```

---

## Block Marker: `[w]`

### Purpose

<!-- @u:user/syntax/blocks#block-markers -->
The `[w]` block marker establishes a wrapper context for subsequent operations.

---

### Syntax

```polyglot
[w] |W.Runtime
```

**Example:**
```polyglot
[w] |W.Python3.11
[r] |PythonOperation
```

---

### Scope

Operations after `[w]` run in that wrapper context until:
1. A new `[w]` is declared (changes context)
2. The pipeline ends

```polyglot
[|] MultiLanguage
// No wrapper - native Polyglot
[r] |PolyglotOperation

// Python context starts
[w] |W.Python3.11
[r] |PythonOp1
[r] |PythonOp2

// Node context starts
[w] |W.Node20
[r] |NodeOp1
[r] |NodeOp2

// Back to native Polyglot? (TBD - may need explicit)
[X]
```

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
[w] |W.Python3.11
[w] |W.Node20
[w] |W.Ruby3.2
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
[w] |W.Python3.11
[r] |DataAnalysis
```

---

### Python Fixed Versions

Available Python versions (examples):

```polyglot
[w] |W.Python3.8
[w] |W.Python3.9
[w] |W.Python3.10
[w] |W.Python3.11
[w] |W.Python3.12
```

**Example:**
```polyglot
[|] PythonDataAnalysis
[i] .data: pg\string

[w] |W.Python3.11
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
[w] |W.Node18
[w] |W.Node20
[w] |W.Node21
```

**Example:**
```polyglot
[|] NodeWebService
[i] .request: pg\serial

[w] |W.Node20
[r] |ProcessRequest
[<] .data: pg\serial << .request
[>] .response: pg\serial >> result

[X]
```

---

### Other Runtime Fixed Versions

**Rust:**
```polyglot
[w] |W.Rust
[r] |PerformanceTask
```

**Go:**
```polyglot
[w] |W.Go
[r] |ConcurrentTask
```

**Ruby:**
```polyglot
[w] |W.Ruby3.2
[r] |RubyScript
```

**Deno:**
```polyglot
[w] |W.Deno
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
[w] |W.Python
[w] |W.Node
[w] |W.Ruby
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
[w] |W.Python
[r] |TestScript
```

---

### Python Dynamic

```polyglot
[w] |W.Python  // Uses latest Python 3.x
[r] |RunScript
```

**Behavior:** Uses latest available Python 3.x version on the system.

---

### Node.js Dynamic

```polyglot
[w] |W.Node  // Uses latest Node.js
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
[w] |W.Python3.11
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
[w] |W.Node20
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
[w] |W.Rust
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
[w] |W.Go
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
[w] |W.Ruby3.2
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
[w] |W.Deno
[r] |TypeScriptTask
[<] .script: pg\path << "process.ts"
[>] .result: pg\string >> output
```

---

## Multiple Wrappers

### Using Multiple Runtimes

<!-- @u:user/concepts/pipelines#structure -->
A single pipeline can use multiple runtime wrappers:

```polyglot
[|] MultiRuntimePipeline
[i] .data: pg\string

// Step 1: Python processing
[w] |W.Python3.11
[r] |PythonAnalyze
[<] .input: pg\string << .data
[>] .analysis: pg\string >> python_result

// Step 2: Node processing
[w] |W.Node20
[r] |NodeTransform
[<] .input: pg\string << python_result
[>] .transformed: pg\string >> node_result

// Step 3: Rust performance processing
[w] |W.Rust
[r] |RustOptimize
[<] .input: pg\string << node_result
[>] .optimized: pg\string >> final_result

[X]
```

---

### Wrapper Transitions

Each `[w]` declaration transitions to a new wrapper context:

```polyglot
[|] WrapperTransitions
// Context 1: Python
[w] |W.Python3.11
[r] |PythonOp1
[r] |PythonOp2

// Context 2: Node (Python context ends)
[w] |W.Node20
[r] |NodeOp1

// Context 3: Back to Python
[w] |W.Python3.11
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
[w] |W.Python3.11
[r] |GenerateData
[<] .seed: pg\string << .input
[>] .data: pg\string >> python_data

// Node: Process data
[w] |W.Node20
[r] |ProcessData
[<] .input: pg\string << python_data  // From Python
[>] .processed: pg\string >> node_data

// Rust: Optimize data
[w] |W.Rust
[r] |OptimizeData
[<] .input: pg\string << node_data  // From Node
[>] .optimized: pg\string >> final_data

[X]
```

**Key Point:** Polyglot handles serialization/deserialization between runtimes.

---

## Wrapper Scope

### Operations Within Wrapper Context

All operations after `[w]` run in that wrapper until changed:

```polyglot
[w] |W.Python3.11
[r] |Op1  // Runs in Python
[r] |Op2  // Runs in Python
[r] |Op3  // Runs in Python

[w] |W.Node20
[r] |Op4  // Runs in Node
[r] |Op5  // Runs in Node
```

---

### Nested Operations

Nested operations inherit wrapper context:

```polyglot
[w] |W.Python3.11
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
[w] |W.Python3.11
[r] |CriticalOperation

// ✗ RISKY - May break with updates
[w] |W.Python
[r] |CriticalOperation
```

---

### 2. Choose Appropriate Runtime

```polyglot
// ✓ GOOD - Right tool for the job
[w] |W.Python3.11
[r] |DataScience  // Python for ML/data

[w] |W.Node20
[r] |WebAPI  // Node for web APIs

[w] |W.Rust
[r] |PerformanceCritical  // Rust for speed

// ✗ POOR - Wrong tool
[w] |W.Python3.11
[r] |PerformanceCritical  // Should use Rust
```

---

### 3. Minimize Wrapper Transitions

```polyglot
// ✓ GOOD - Batch operations per runtime
[w] |W.Python3.11
[r] |PythonOp1
[r] |PythonOp2
[r] |PythonOp3

[w] |W.Node20
[r] |NodeOp1
[r] |NodeOp2

// ✗ INEFFICIENT - Too many transitions
[w] |W.Python3.11
[r] |PythonOp1

[w] |W.Node20
[r] |NodeOp1

[w] |W.Python3.11  // Back to Python
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
[w] |W.Python3.11
[r] |AnalyzeWithPandas

[w] |W.Node20
[r] |ServeWithExpress
[X]
```

---

### 5. Consider Performance Overhead

```polyglot
// ✓ GOOD - Use wrappers when necessary
[w] |W.Python3.11
[r] |ComplexDataAnalysis  // Needs Python libraries

// ✗ UNNECESSARY - Use native Polyglot
[w] |W.Python3.11
[r] |SimpleStringConcat  // Don't need Python for this
```

---

### 6. Test with Multiple Versions

```polyglot
// ✓ GOOD - Test compatibility
[|] TestPipeline

// Test with Python 3.10
[w] |W.Python3.10
[r] |RunTests

// Test with Python 3.11
[w] |W.Python3.11
[r] |RunTests

// Test with Python 3.12
[w] |W.Python3.12
[r] |RunTests

[X]
```

---

### 7. Document Wrapper Requirements

```polyglot
// ✓ GOOD - Clear requirements
[|] MLPipeline
// Requires: Python 3.11+ with tensorflow, scikit-learn
[w] |W.Python3.11
[r] |TrainModel
[X]
```

---

### 8. Use Dynamic Versions for Development

```polyglot
// ✓ GOOD - Flexible during development
[|] DevelopmentPipeline
[w] |W.Python  // Latest for dev
[r] |TestFeature
[X]

// ✓ GOOD - Fixed for production
[|] ProductionPipeline
[w] |W.Python3.11  // Specific for prod
[r] |RunFeature
[X]
```

---

## Examples

### Example 1: Python Data Analysis

```polyglot
[|] AnalyzeCustomerData
[i] .data_file: pg\path

[w] |W.Python3.11
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

[w] |W.Node20
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
[w] |W.Python3.11
[r] |LoadCSV
[<] .file: pg\path << .input_file
[>] .data: pg\string >> raw_data

[r] |CleanData
[<] .input: pg\string << raw_data
[>] .cleaned: pg\string >> clean_data

// Step 2: Node - Transform to JSON
[w] |W.Node20
[r] |ConvertToJSON
[<] .data: pg\string << clean_data
[>] .json: pg\string >> json_data

// Step 3: Rust - Performance processing
[w] |W.Rust
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
[w] |W.Python3.11
[r] |AnalyzeWithPython
[<] .input: pg\string << .data
[>] .result >> python_result

[p] |NodeProcessing
[w] |W.Node20
[r] |TransformWithNode
[<] .input: pg\string << .data
[>] .result >> node_result

// Join results
[Y] |Y.Join
[>] python_result
[>] node_result

// Combine with Rust
[w] |W.Rust
[r] |CombineResults
[<] .python: pg\string << python_result
[<] .node: pg\string << node_result
[>] .combined: pg\string >> final_result

[X]
```

---

## See Also

### Language Specification
- [Block Markers](../language/06-block-markers.md) - `[w]` marker details
- [Complete Syntax Reference](../language/01-syntax-complete.md) - Wrapper syntax

### Standard Library
- [Overview](00-overview.md) - Standard library organization

### Examples
- [Complete Workflows](../examples/complete-workflows.md) - Multi-runtime examples

### Planning
- [Decision Log](../decision-log.md) - Wrapper decisions (#21)

---

**End of Runtime Wrappers Reference**