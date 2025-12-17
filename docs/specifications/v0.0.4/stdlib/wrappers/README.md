---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: readme
shard: false

# --- Classification ---
type: api
topic: Wrappers (`|W.*`)
summary: API reference: Wrappers (`|W.*`)
keywords:
  - stdlib
  - api
  - reference

# --- BMAD Agent Routing ---
agents:
  - developer
phase: implementation
workflow: any
module: bmm
complexity: low

# --- Dependency Chain ---
prereqs:
  - type-system
unlocks:
  []

# --- Relationships ---
related:
  []

# --- Metadata ---
status: stable
updated: 2025-12-16
version: 0.0.4
tags:
  - "#stdlib"
  - "#api"
---
# Wrappers (`|W.*`)

**Runtime integration and execution environment management**

---

## Complete Wrappers Tree

```
|W.* (Wrappers)
│
├── |W.Polyglot.Scope
│
├── |W.RT.* (Runtime Wrappers)
│   ├── |W.RT.Python3.12
│   ├── |W.RT.Rust
│   ├── |W.RT.JavaScript
│   ├── |W.RT.Go
│   └── |W.RT.Java
│
├── |W.DB.* (Database Wrappers)
│   └── |W.DB.Transaction
│
└── |W.HTTP.* (HTTP Wrappers)
    └── |W.HTTP.Client
```

**Total: ~9 wrapper pipelines**

---

## What Are Wrappers?

**Wrappers** provide the **execution environment** for pipeline logic with:

1. **Setup** - Initialize runtime, allocate resources
2. **Execution Environment** - Provide context for pipeline logic
3. **Cleanup** - Tear down runtime, release resources (guaranteed, even on error)

**Required:** Every pipeline MUST have exactly one `[W]` wrapper.

**See:** [Pipeline Structure - Wrapper](../../language/control-flow/pipeline-structure.md#wrapper-w---required)

---

## Polyglot Wrapper

### `|W.Polyglot.Scope`

**Purpose:** Pure Polyglot execution environment (no external runtime)

**Use when:**
- Pipeline uses only Polyglot constructs
- No external runtime needed
- Most common wrapper

**Example:**
```polyglot
{|} \|ProcessOrder
[|] <order_id :pg.string
[|] >result :pg.string

[t] \|T.Call
[W] \|W.Polyglot.Scope              // Pure Polyglot

   [r] $order << \|Database.Orders.Find <id << $order_id
   [r] $status :pg.string << $order.status

   [|] >result << $status

{x}
```

**Setup:**
- Create isolated Polyglot scope
- Initialize variable tracking

**Cleanup:**
- Release variables
- Clear scope

---

## Runtime Wrappers (`|W.RT.*`)

**Purpose:** Integrate external programming language runtimes

**Pattern:** `|W.RT.Language[Version]`

### `|W.RT.Python3.12`

**Purpose:** Python 3.12 runtime with standard library and packages

**Use when:**
- Need Python libraries (NumPy, Pandas, SciKit-Learn, etc.)
- Data science, machine learning
- Python-specific operations

**Example:**
```polyglot
{|} \|AnalyzeData
[|] <dataset :pg.array.pg.float
[|] >analysis :pg.serial

[t] \|T.Call
[W] \|W.RT.Python3.12

   [r] $result :pg.serial << \|Python.NumPy.Analyze
   [|] <data << $dataset

   [|] >analysis << $result

{x}
```

**Setup:**
- Initialize Python 3.12 interpreter
- Load standard library
- Import common packages (NumPy, Pandas, etc.)
- Set up Polyglot-Python bridge

**Cleanup:**
- Release Python objects
- Clear interpreter state
- Shutdown interpreter

**Available Packages:**
- NumPy, Pandas, SciKit-Learn
- Matplotlib (plotting)
- Requests (HTTP)
- And more...

---

### `|W.RT.Rust`

**Purpose:** Rust runtime for high-performance operations

**Use when:**
- Performance-critical operations
- Memory safety guarantees
- Low-level systems programming

**Example:**
```polyglot
{|} \|HighPerformanceCompute
[|] <matrix :pg.array.pg.array.pg.float
[|] >result :pg.array.pg.array.pg.float

[t] \|T.Call
[W] \|W.RT.Rust

   [r] $computed :pg.array.pg.array.pg.float << \|Rust.MatrixMultiply
   [|] <input << $matrix

   [|] >result << $computed

{x}
```

**Setup:**
- Initialize Rust runtime
- Allocate memory
- Set up FFI bridge

**Cleanup:**
- Free allocated memory
- Drop Rust objects
- Shutdown runtime

---

### `|W.RT.JavaScript`

**Purpose:** JavaScript/Node.js runtime

**Use when:**
- Frontend processing
- Node.js ecosystem
- Async/event-driven operations

**Example:**
```polyglot
{|} \|ProcessWithNode
[|] <data :pg.serial
[|] >result :pg.serial

[t] \|T.Call
[W] \|W.RT.JavaScript

   [r] $processed :pg.serial << \|Node.ProcessData
   [|] <input << $data

   [|] >result << $processed

{x}
```

---

### `|W.RT.Go`

**Purpose:** Go runtime for concurrent operations

**Use when:**
- Concurrent/parallel processing
- Network services
- Go ecosystem

---

### `|W.RT.Java`

**Purpose:** Java runtime (JVM)

**Use when:**
- Enterprise Java applications
- JVM ecosystem
- Legacy Java integration

---

## Database Wrappers (`|W.DB.*`)

### `|W.DB.Transaction`

**Purpose:** Database transaction management

**Use when:**
- Multiple database operations must be atomic
- Need ACID guarantees
- Rollback on error

**Example:**
```polyglot
{|} \|UpdateUserAndLog
[|] <user_id :pg.string
[|] <new_email :pg.string
[|] >success :pg.bool

[t] \|T.Call
[W] \|W.DB.Transaction
   [.] .connection << "main_db"

   // Both operations in single transaction
   [r] \|Database.Users.Update
   [|] <id << $user_id
   [|] <email << $new_email

   [r] \|Database.AuditLog.Insert
   [|] <user_id << $user_id
   [|] <action << "email_updated"

   [|] >success << #True

{x}  // COMMIT if success, ROLLBACK if error
```

**Setup:**
- Connect to database
- BEGIN TRANSACTION

**Cleanup (success):**
- COMMIT TRANSACTION
- Close connection

**Cleanup (error):**
- ROLLBACK TRANSACTION
- Close connection

---

## HTTP Wrappers (`|W.HTTP.*`)

### `|W.HTTP.Client`

**Purpose:** HTTP client wrapper for making requests

**Use when:**
- Making HTTP requests
- API integration
- Web scraping

**Example:**
```polyglot
{|} \|FetchFromAPI
[|] <url :pg.string
[|] >data :pg.serial

[t] \|T.Call
[W] \|W.HTTP.Client
   [.] .timeout << 30
   [.] .retry << 3

   [r] $response :pg.serial << \|HTTP.Get
   [|] <url << $url

   [|] >data << $response.body

{x}
```

**Setup:**
- Initialize HTTP client
- Configure timeout, retries
- Set headers

**Cleanup:**
- Close connections
- Release resources

---

## Wrapper Comparison

| Wrapper | Setup Cost | Use Case | Cleanup |
|---------|------------|----------|---------|
| `\|W.Polyglot.Scope` | Low | Pure Polyglot | Simple |
| `\|W.RT.Python3.12` | High | Data science, ML | Complex |
| `\|W.RT.Rust` | Medium | Performance | Simple |
| `\|W.RT.JavaScript` | Medium | Node.js ecosystem | Medium |
| `\|W.DB.Transaction` | Low | Atomic DB operations | ACID guaranteed |
| `\|W.HTTP.Client` | Low | HTTP requests | Connection pooling |

---

## Common Patterns

### Pattern 1: Pure Polyglot Pipeline

```polyglot
{|} \|SimpleTransform
[t] \|T.Call
[W] \|W.Polyglot.Scope              // Most common

   [r] $result << \|Transform <input << $data

{x}
```

### Pattern 2: Python Data Science

```polyglot
{|} \|MLPredict
[t] \|T.Call
[W] \|W.RT.Python3.12

   [r] $prediction << \|Python.Model.Predict
   [|] <model << $trained_model
   [|] <features << $input_features

{x}
```

### Pattern 3: High-Performance Rust

```polyglot
{|} \|FastCompute
[t] \|T.Call
[W] \|W.RT.Rust

   [r] $result << \|Rust.ComputeIntensive
   [|] <data << $large_dataset

{x}
```

### Pattern 4: Database Transaction

```polyglot
{|} \|AtomicUpdate
[t] \|T.Call
[W] \|W.DB.Transaction

   [r] \|DB.Table1.Update <data << $data1
   [r] \|DB.Table2.Update <data << $data2
   // Both commit or both rollback

{x}
```

---

## Wrapper Selection Guide

**Choose wrapper based on:**

1. **Operation type**
   - Pure orchestration → `|W.Polyglot.Scope`
   - Computation → Runtime wrapper
   - Database → `|W.DB.Transaction`
   - HTTP → `|W.HTTP.Client`

2. **Performance requirements**
   - Low latency → `|W.Polyglot.Scope` or `|W.RT.Rust`
   - High throughput → `|W.RT.Rust` or `|W.RT.Go`
   - Not critical → Any wrapper

3. **Ecosystem needs**
   - Python libraries → `|W.RT.Python3.12`
   - JVM libraries → `|W.RT.Java`
   - Node packages → `|W.RT.JavaScript`
   - None → `|W.Polyglot.Scope`

4. **Transaction requirements**
   - ACID needed → `|W.DB.Transaction`
   - No transactions → Other wrappers

---

## Wrapper Configuration

**Some wrappers accept configuration:**

```polyglot
[W] \|W.DB.Transaction
   [.] .connection << "main_db"
   [.] .isolation << "READ_COMMITTED"
   [.] .timeout << 30

[W] \|W.HTTP.Client
   [.] .timeout << 60
   [.] .retry << 3
   [.] .headers << {{"User-Agent", "Polyglot/1.0"}}
```

---

## Error Handling with Wrappers

**Cleanup is guaranteed even on error:**

```polyglot
{|} \|ProcessWithCleanup
[t] \|T.Call
[W] \|W.DB.Transaction

   [z] $data << \|Database.Query
   [z][!] !Database.* >> $db_error :!
   [z][!] *! >> $other_error :!

   [y] $db_error.state =? :pg.state.faulted
      // Transaction will ROLLBACK (cleanup)
      [|] >error << $db_error

{x}  // Cleanup runs here (ROLLBACK or COMMIT)
```

**Wrapper cleanup happens:**
- After successful completion
- After error (even if unhandled)
- Before pipeline returns

---

## Custom Wrappers

**Future feature:** Users will be able to define custom wrappers.

**Pattern:**
```polyglot
{|} \|W.Custom.MyWrapper
// Wrapper definition
{x}

// Usage:
{|} \|MyPipeline
[W] \|W.Custom.MyWrapper
{x}
```

---

## Related Documentation

- [Pipeline Structure](../../language/control-flow/pipeline-structure.md) - Wrapper usage
- [Standard Library Overview](../README.md) - Complete package tree
- [Loop System](../../language/advanced/loop-system.md) - Wrappers in loops

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../README.md)
