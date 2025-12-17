# Runtime Execution Model

**Version:** 0.0.2
**Last Updated:** 2025-11-19
**Status:** Complete

---

## Overview

Polyglot's runtime execution model supports both sequential and parallel execution patterns, with automatic synchronization and error handling. This document focuses on execution patterns, particularly the parallel-first architecture of Serial Load Blocks.

---

## Table of Contents

1. [Execution Block Types](#execution-block-types)
2. [Sequential Execution](#sequential-execution)
3. [Parallel Execution](#parallel-execution)
4. [Serial Load Block Execution](#serial-load-block-execution)
5. [Synchronization Points](#synchronization-points)
6. [Error Handling During Execution](#error-handling-during-execution)

---

## Execution Block Types

### Execution Order

Block execution follows a strict hierarchy:

```
[t]           Triggers (determine IF pipeline executes)
  ↓
[i], [i]      Inputs and Constants (define parameters)
  ↓
[Q]           Queue Assignment (route to execution queue)
  ↓
[\]           Setup (allocate resources, initialize state)
  ↓
[r],[p],[s],[b],[Y]   Execution Blocks (perform work)
  ↓
[/]           Cleanup (release resources, finalize state)
  ↓
[o]           Output (return results)
  ↓
[X]           End (close pipeline scope)
```

**Execution blocks** perform the actual work:
- `[r]` - Run Sequential (one at a time)
- `[p]` - Parallel Execution (concurrent, manual synchronization)
- `[s]` - Serial Load (parallel with automatic join)
- `[b]` - Broadcast (fan-out pattern)
- `[Y]` - Join (synchronization point)

---

## Sequential Execution

### `[r]` - Run Sequential

**Purpose:** Execute operations one at a time, in order

**Execution Model:**
```
[r] Operation1  →  wait for completion
[r] Operation2  →  wait for completion
[r] Operation3  →  wait for completion
```

**Example:**
```polyglot
[r] |LoadData
[>] .data >> dataset

[r] |ProcessData
[<] .input: pg\serial << dataset
[>] .processed >> result

[r] |SaveData
[<] .output: pg\serial << result
```

**Characteristics:**
- Operations execute in strict order
- Each operation completes before the next begins
- Errors halt execution flow
- Simplest execution pattern

---

## Parallel Execution

### `[p]` - Parallel Execution

**Purpose:** Execute operations concurrently with manual synchronization

**Execution Model:**
```
[p] Operation1  ╮
[p] Operation2  ├─→ execute concurrently
[p] Operation3  ╯
[Y] |Y.Join     →  synchronization point
```

**Example:**
```polyglot
// Initialize result variables
[r] .result1: pg\string << ""
[r] .result2: pg\string << ""

// Parallel operations
[p] |ProcessPartA
[<] .data: pg\string << .input
[>] .output >> result1

[p] |ProcessPartB
[<] .data: pg\string << .input
[>] .output >> result2

// Manual synchronization
[Y] |Y.Join
[>] result1
[>] result2

// Continue after join
[r] |CombineResults
[<] .a: pg\string << result1
[<] .b: pg\string << result2
```

**Characteristics:**
- Operations start concurrently
- Requires manual `[Y]` join for synchronization
- Variables must be declared before parallel blocks
- More complex but offers maximum control

---

## Serial Load Block Execution

### `[s]` - Parallel-First Architecture

**Purpose:** Load files in parallel with automatic synchronization

**Key Innovation:** Automatic join eliminates need for manual synchronization

---

### Three-Step Execution Process

**Step 1: Collect Paths**
```polyglot
[s] .config1 << JSON"app.json"
[s] .config2 << JSON"db.json"
[s] .configs << JSON"*.json"  // Wildcard expansion
```

**Runtime Actions:**
- Resolve all file paths
- Expand wildcards (e.g., `*.json` → multiple paths)
- Apply filters (e.g., ExcludeFileName)
- Validate paths exist (defer errors to step 2)

---

**Step 2: Load in Parallel**
```
Thread 1: Load app.json    ╮
Thread 2: Load db.json     ├─→ Parallel I/O
Thread 3: Load config1.json │
Thread 4: Load config2.json ╯
    ↓
Automatic Join (wait for all)
```

**Runtime Actions:**
- Create thread/task for each file
- Load files concurrently (I/O bound operations)
- Parse serialized data (JSON, YAML, TOML, XML)
- Capture errors per-file (no global failure)
- Wait for ALL loads to complete (automatic join)

---

**Step 3: Assign to Variables**
```polyglot
.config1 = loaded data OR #None.ErrorState
.config2 = loaded data OR #None.ErrorState
.configs = merged/combined results
```

**Runtime Actions:**
- Assign loaded data to variables (success case)
- Assign `#None.ErrorState` to variables (error case)
- Set `.error` field on each variable
- Validate reserved enumeration types (if applicable)
- Continue execution (partial success model)

---

### Automatic Join Behavior

**Before Next Operation:**
```polyglot
[s] .db_config << JSON"db.json"       // Load starts
[s] .api_config << JSON"api.json"     // Load starts
[s] .cache_config << JSON"cache.json" // Load starts

// Implicit join happens HERE (wait for all loads)

[r] |SetupDatabase   // Executes only after ALL loads complete
[<] .config: pg\serial << .db_config
```

**Join Guarantees:**
- All `[s]` blocks at the same scope complete before next operation
- Variables are ready for use
- Errors are available for handling
- No manual `[Y]` join required

---

### Parallel Execution with Error Handling

**Partial Success Model:**

```polyglot
[s] .config1 << JSON"exists.json"      // ✓ Loads successfully
[s] .config2 << JSON"missing.json"     // ✗ Fails with !File.NotFound
[s] .config3 << JSON"invalid.json"     // ✗ Fails with !JSON.ParseError

// Automatic join (all loads attempted)

[s][!] !File.NotFound
[r] |LogMissing

[s][!] !JSON.ParseError
[r] |LogInvalid

// config1 has data, config2 and config3 have #None.ErrorState
[r] |ProcessConfig1
[<] .data: pg\serial << .config1  // Uses successful load
```

**Execution Behavior:**
- All three loads start in parallel
- Each load completes independently
- Successful loads assign data
- Failed loads assign `#None.ErrorState`
- Errors trigger scope-level `[s][!]` handlers
- Execution continues after error handling

---

### Performance Characteristics

**Sequential Loading (Hypothetical):**
```
File 1: 100ms  →  File 2: 100ms  →  File 3: 100ms
Total: 300ms
```

**Parallel Loading (Actual [s] Behavior):**
```
File 1: 100ms  ╮
File 2: 100ms  ├─→ Concurrent I/O
File 3: 100ms  ╯
Total: ~100ms (plus join overhead)
```

**Speedup:** Near-linear for I/O-bound file loading (3x faster for 3 files)

---

## Synchronization Points

### Automatic Synchronization

**`[s]` blocks automatically synchronize:**
```polyglot
[s] .data1 << JSON"file1.json"
[s] .data2 << JSON"file2.json"
// Implicit join point
[r] |NextOperation  // Waits for both loads
```

---

### Manual Synchronization

**`[Y]` join for `[p]` blocks:**
```polyglot
[p] |Operation1
[p] |Operation2
// No automatic join - must use [Y]
[Y] |Y.Join
[>] result1
[>] result2
// Explicit join point
[r] |NextOperation  // Waits for join
```

---

### Scope-Based Synchronization

**Different scopes have separate synchronization:**
```polyglot
// Scope 1
[s] .config1 << JSON"file1.json"
[s] .config2 << JSON"file2.json"
// Join happens here

[?] .env =? #Production
// Scope 2 (nested)
[~] [s] .prod_config << JSON"prod.json"
// Separate join for this scope
[~] [r] |UseProdConfig

[?] .env =? #Development
// Scope 3 (nested)
[~] [s] .dev_config << JSON"dev.json"
// Separate join for this scope
[~] [r] |UseDevConfig
```

---

## Error Handling During Execution

### Error-Carrying Variables

**All variables from `[s]` blocks carry error state:**

```polyglot
[s] .config << JSON"config.json"

// Success state:
.config = loaded data
.config.error = !NoError

// Error state:
.config = #None.ErrorState
.config.error = !File.NotFound
```

**Runtime Behavior:**
- Variable assignment always succeeds (no exceptions thrown)
- Value is either data or `#None.ErrorState`
- Error field always populated
- Execution continues (no halt)

---

### Error Precedence

**Specific errors override general errors:**

```polyglot
[s] .critical << JSON"critical.json"
[s] .optional << JSON"optional.json"

[~][!] !File.NotFound       // Handles critical.json specifically
[r] |FailPipeline

[s][!] !File.NotFound       // Handles all other serial blocks
[r] |LogWarning

// Runtime: [~][!] takes precedence over [s][!] for critical.json
```

---

### Implicit Error Notification

**No handler → automatic notification:**

```polyglot
[s] .config << JSON"config.json"
// No [s][!] handler defined
```

**Runtime Behavior:**
- If load fails: automatic logging/console output
- Error not silently ignored
- Critical for debugging automated pipelines
- Context-aware (console vs. log file)

---

## Execution Lifecycle Example

**Complete execution flow:**

```polyglot
[@] MyApp.Loader
[#] 001

[|] LoadConfigs
[i] .env: #Environment
[t] |T.Call                    // ← TRIGGER: Decide IF to execute

[\] |Setup                     // ← SETUP: Initialize resources
[<] .log_path: pg\path << "\\Logs\\loader.log"

[s] .base << JSON"base.json"   // ← EXECUTE (Step 1: Collect)
[s] .env << JSON"{.env}.json"  // ← EXECUTE (Step 1: Collect)
[s] .secrets << JSON"*.secret" // ← EXECUTE (Step 1: Collect)
                               // Step 2: Load in parallel
                               // Step 3: Assign + auto join

[s][!] !File.NotFound          // ← ERROR HANDLING
[r] |U.Log.Warn
[<] .msg: pg\string << "Config missing"

[r] |MergeConfigs              // ← EXECUTE (Sequential)
[<] .b: pg\serial << .base
[<] .e: pg\serial << .env
[>] .merged >> final_config

[/] |Cleanup                   // ← CLEANUP: Release resources

[o] .final_config: pg\serial   // ← OUTPUT: Return result
[X]                            // ← END: Close scope
```

**Execution Order:**
1. Trigger evaluation (`[t]`)
2. Setup phase (`[\]`)
3. Serial load collection (step 1)
4. Parallel loading (step 2)
5. Automatic join
6. Variable assignment (step 3)
7. Error handling (`[s][!]`)
8. Sequential processing (`[r]`)
9. Cleanup phase (`[/]`)
10. Output (`[o]`)
11. End (`[X]`)

---

## Best Practices

### When to Use Each Pattern

**Use `[r]` Sequential when:**
- Operations must execute in strict order
- Each operation depends on the previous result
- Simplicity is more important than performance

**Use `[p]` Parallel when:**
- Operations are independent
- Manual control over synchronization is needed
- Complex fan-out/fan-in patterns required

**Use `[s]` Serial Load when:**
- Loading multiple files
- I/O-bound operations benefit from parallelism
- Automatic synchronization preferred
- Partial success model desired

---

### Performance Optimization

**Parallel Loading:**
```polyglot
// Good: Parallel I/O
[s] .file1 << JSON"large1.json"
[s] .file2 << JSON"large2.json"
[s] .file3 << JSON"large3.json"
// ~3x faster than sequential
```

**Avoid Unnecessary Parallelism:**
```polyglot
// Bad: Single file doesn't benefit
[s] .single << JSON"file.json"
// No speedup (only one file)

// Good: Use simple load
[r] |U.JSON.Load
[<] .path: pg\path << "file.json"
[>] .data >> single
```

---

### Error Handling Strategy

**Always handle expected errors:**
```polyglot
[s] .config << JSON"config.json"

[s][!] !File.NotFound
[r] |UseDefaultConfig

[s][!] !JSON.ParseError
[r] |FailWithClearError
```

**Use variable-level checks for critical data:**
```polyglot
[s] .critical_config << JSON"critical.json"

[!] .critical_config.error =? !File.NotFound
[o] !ConfigurationError  // Fail pipeline immediately
```

---

**Last Updated:** 2025-11-19
**Related Documents:**
- [Block Markers](../language/block-markers.md) - Complete block marker reference
- [Error Handling](../language/error-handling.md) - Error handling patterns
- [Parallel Execution](../language/parallel-execution.md) - Advanced parallel patterns
