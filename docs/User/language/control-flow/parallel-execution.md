# Parallel Execution Guide

**Version:** v0.0.4
**Category:** Language Guide
**Audience:** Intermediate users
**Prerequisites:** Basic pipeline syntax, sequential execution

---

## Overview

Polyglot supports **parallel execution** of pipeline operations using the `[p]` marker, allowing multiple operations to run simultaneously for improved performance.

**Key difference:**
- `[r]` - Sequential (one after another)
- `[p]` - Parallel (all at once)

---

## Basic Parallel Syntax

### Parallel Marker

```polyglot
[p] operation1
[p] operation2
[p] operation3
```

**What happens:**
- All three operations start simultaneously
- Execution continues when all complete
- Order of completion is non-deterministic

---

## Sequential vs Parallel

### Sequential Execution `[r]`

```polyglot
[r] $a :pg.int << |U.SlowOperation1""    // Waits to complete
[r] $b :pg.int << |U.SlowOperation2""    // Then starts this
[r] $c :pg.int << |U.SlowOperation3""    // Then starts this

// Total time = Time1 + Time2 + Time3
```

**Timeline:**
```
Op1: [████████████████]
Op2:                  [████████████████]
Op3:                                   [████████████████]
```

### Parallel Execution `[p]`

```polyglot
[p] $a :pg.int << |U.SlowOperation1""    // Starts immediately
[p] $b :pg.int << |U.SlowOperation2""    // Starts immediately
[p] $c :pg.int << |U.SlowOperation3""    // Starts immediately

// Total time = max(Time1, Time2, Time3)
```

**Timeline:**
```
Op1: [████████████████]
Op2: [████████████████]
Op3: [████████████████]
     └── All start together
```

---

## Complete Example

### Parallel Multi-Language Hello World

```polyglot
{@} @Local::Examples.ParallelHello:1.0.0.0
{x}



{|} |ParallelHello
[%] %Doc "Multi-language Hello World with PARALLEL execution"

[t] |T.CLI"parallel"

[|] <log_path :pg.path <~ \\FileDir\\parallel_hello.log
[|] >error <~ !NoError

[w] |W.RT.Python3.9
(|) <requirements:pg.path << \\NoPath\\
(|) >env:pg.serial >> $py

[w] |W.RT.Rust
(|) <dependencies:pg.path << \\NoPath\\
(|) >env:pg.serial >> $rust

[w] |W.RT.JS
(|) <packages:pg.path << \\NoPath\\
(|) >env:pg.serial >> $js

[r] $timestamp :pg.dt << |DT.Now"iso8601"
[r] $header :pg.string << "=== Parallel Execution - Started {$timestamp} ===\n"

[r] |U.File.Text.Append
(|) <file:pg.path << $log_path
(|) <content:pg.string << $header

[p] |U.RT.Python.Code
(|) <env:pg.serial << $py
(|) <kwargs.file:py.str << $log_path
(|) <code:pg.string << ""
[+] +"import time"
[+] +"def log(file):"
[+] -"    time.sleep(0.1)"
[+] -"    with open(file, 'a') as f:"
[+] -"        f.write('[Python] Hello at ' + str(time.time()) + '\\n')"
[+] -""
[+] +"log(file)"

[p] |U.RT.Rust.Code
(|) <env:pg.serial << $rust
(|) <kwargs.file:rust.String << $log_path
(|) <code:pg.string << ""
[+] +"use std::fs::OpenOptions;"
[+] +"use std::io::Write;"
[+] +"use std::thread;"
[+] +"use std::time::Duration;"
[+] -""
[+] +"fn main() {"
[+] -"    thread::sleep(Duration::from_millis(100));"
[+] -"    let mut file = OpenOptions::new()"
[+] -"        .create(true)"
[+] -"        .append(true)"
[+] -"        .open(file)"
[+] -"        .expect(\"Failed to open file\");"
[+] -""
[+] -"    let now = std::time::SystemTime::now();"
[+] -"    writeln!(file, \"[Rust] Hello at {:?}\", now)"
[+] -"        .expect(\"Failed to write\");"
[+] +"}"

[p] |U.RT.JS.Code
(|) <env:pg.serial << $js
(|) <kwargs.file:js.string << $log_path
(|) <code:pg.string << ""
[+] +"const fs = require('fs');"
[+] -""
[+] +"setTimeout(() => {"
[+] -"    const now = Date.now();"
[+] -"    fs.appendFileSync(file, `[JavaScript] Hello at ${now}\\n`);"
[+] +"}, 100);"

[r] $footer :pg.string << "=== All runtimes completed ===\n"

[r] |U.File.Text.Append
(|) <file:pg.path << $log_path
(|) <content:pg.string << $footer

{x}
```

**Usage:**
```bash
polyglot run parallel
```

**Expected output (order varies!):**
```
=== Parallel Execution - Started 2025-12-26T17:30:00Z ===
[JavaScript] Hello at 1735234200123
[Python] Hello at 1735234200.124
[Rust] Hello at SystemTime { tv_sec: 1735234200, tv_nsec: 125000000 }
=== All runtimes completed ===
```

---

## Race Conditions

### Understanding Non-Determinism

When operations run in parallel, **order of completion is unpredictable**:

**Run 1:**
```
[Python] Hello at 1.0
[Rust] Hello at 1.1
[JavaScript] Hello at 1.2
```

**Run 2:**
```
[JavaScript] Hello at 2.0
[Python] Hello at 2.1
[Rust] Hello at 2.2
```

**Run 3:**
```
[Rust] Hello at 3.0
[JavaScript] Hello at 3.1
[Python] Hello at 3.2
```

### File Write Conflicts

**Warning:** Parallel file writes can cause race conditions!

```polyglot
// ⚠️ UNSAFE: Multiple parallel writes to same file
[p] |U.File.Text.Append
(|) <file:pg.path << $log
(|) <content:pg.string << "Line 1"

[p] |U.File.Text.Append
(|) <file:pg.path << $log
(|) <content:pg.string << "Line 2"

[p] |U.File.Text.Append
(|) <file:pg.path << $log
(|) <content:pg.string << "Line 3"

// Result: Lines may interleave or corrupt!
```

**Safe alternative:**
```polyglot
// ✅ SAFE: Collect results, then write sequentially
[p] $result1 :pg.string << |U.Process.Data1""
[p] $result2 :pg.string << |U.Process.Data2""
[p] $result3 :pg.string << |U.Process.Data3""

[r] $combined :pg.string << "{$result1}\n{$result2}\n{$result3}"

[r] |U.File.Text.Append
(|) <file:pg.path << $log
(|) <content:pg.string << $combined
```

---

## When to Use Parallel Execution

### ✅ Good Use Cases

**1. Independent computations**
```polyglot
[p] $cpu_data :pg.serial << |U.System.GetCPU""
[p] $memory_data :pg.serial << |U.System.GetMemory""
[p] $disk_data :pg.serial << |U.System.GetDisk""
```

**2. Multiple API calls**
```polyglot
[p] $user :pg.serial << |U.API.GetUser"{$userId}"
[p] $posts :pg.array.pg.serial << |U.API.GetPosts"{$userId}"
[p] $followers :pg.array.pg.serial << |U.API.GetFollowers"{$userId}"
```

**3. Batch processing different data**
```polyglot
[p] $processed_a :pg.serial << |U.Transform.DataA"{$dataA}"
[p] $processed_b :pg.serial << |U.Transform.DataB"{$dataB}"
[p] $processed_c :pg.serial << |U.Transform.DataC"{$dataC}"
```

**4. Multi-language operations** (if no shared resources)
```polyglot
[p] |U.RT.Python.Code
(|) <env:pg.serial << $py
(|) <code:pg.string << "..."

[p] |U.RT.Rust.Code
(|) <env:pg.serial << $rust
(|) <code:pg.string << "..."
```

### ❌ Avoid Parallel For

**1. Shared resource writes**
```polyglot
// ❌ BAD: Same file
[p] |U.File.Text.Append"..."
[p] |U.File.Text.Append"..."
```

**2. Dependent operations**
```polyglot
// ❌ BAD: Step2 needs Step1 result
[p] $step1 :pg.int << |U.Process.Step1""
[p] $step2 :pg.int << |U.Process.Step2"{$step1}"  // Race!
```

**3. Ordered sequences**
```polyglot
// ❌ BAD: Must process in order
[p] |U.Database.Insert"{$record1}"
[p] |U.Database.Insert"{$record2}"
[p] |U.Database.Insert"{$record3}"
// Records may insert out of order!
```

---

## Mixing Sequential and Parallel

### Pattern: Parallel Groups

```polyglot
// Group 1: Parallel initialization
[p] $py :pg.serial << |W.RT.Python3.9
(|) <requirements:pg.path << \\NoPath\\

[p] $rust :pg.serial << |W.RT.Rust
(|) <dependencies:pg.path << \\NoPath\\

// Group 2: Sequential processing
[r] $data :pg.serial << |U.LoadData""

// Group 3: Parallel processing
[p] $python_result :pg.serial << |U.RT.Python.Process"{$py}, {$data}"
[p] $rust_result :pg.serial << |U.RT.Rust.Process"{$rust}, {$data}"

// Group 4: Sequential finalization
[r] $combined :pg.serial << |U.Merge"{$python_result}, {$rust_result}"
[r] |U.SaveResult"{$combined}"
```

**Timeline:**
```
Group 1 (parallel):
  Python init: [████]
  Rust init:   [████]

Group 2 (sequential):
  Load data:          [████]

Group 3 (parallel):
  Python proc:             [████████]
  Rust proc:               [████████]

Group 4 (sequential):
  Merge:                            [██]
  Save:                                [██]
```

---

## Performance Considerations

### Speedup Calculation

**Sequential time:** T₁ + T₂ + T₃
**Parallel time:** max(T₁, T₂, T₃) + overhead

**Example:**
- Operation 1: 100ms
- Operation 2: 150ms
- Operation 3: 120ms

**Sequential:** 100 + 150 + 120 = 370ms
**Parallel:** max(100, 150, 120) = 150ms + overhead (~160ms)

**Speedup:** ~2.3x faster

### Overhead Costs

Parallel execution has overhead:
- Thread/process creation
- Context switching
- Synchronization
- Result collection

**Rule of thumb:**
- Use parallel if operations take > 10ms each
- Don't use for trivial operations (< 1ms)

---

## Error Handling in Parallel

### All Must Succeed

```polyglot
[p] |U.RT.Python.Code"..."
(|) >result:pg.bool >> $py_ok

[p] |U.RT.Rust.Code"..."
(|) >result:pg.bool >> $rust_ok

[p] |U.RT.JS.Code"..."
(|) >result:pg.bool >> $js_ok

[f] $py_ok =? #False
   [r] >error << !RuntimeError.Python
{x}

[f] $rust_ok =? #False
   [r] >error << !RuntimeError.Rust
{x}

[f] $js_ok =? #False
   [r] >error << !RuntimeError.JS
{x}

[f] *?
   [r] >error << !NoError
{x}
```

**Note:** All parallel operations complete even if one fails. Error checking happens after all finish.

---

## Common Patterns

### Pattern 1: Fan-Out, Fan-In

```polyglot
// Fan-out: Start multiple operations
[p] $result1 :pg.int << |U.Process1""
[p] $result2 :pg.int << |U.Process2""
[p] $result3 :pg.int << |U.Process3""

// Fan-in: Combine results
[r] $total :pg.int << |U.Math.Add"{$result1}, {$result2}"
[r] $total :pg.int << |U.Math.Add"{$total}, {$result3}"
```

### Pattern 2: Parallel with Timeout

**Note:** Syntax for timeout not verified, showing concept:

```polyglot
[p] $result :pg.serial << |U.SlowOperation""
[p] $timeout :pg.bool << |U.Sleep.Seconds"5"

// Check which finished first (conceptual)
```

### Pattern 3: Resource Pool

```polyglot
// Process multiple files in parallel
[p] $file1_result :pg.serial << |U.Process.File"file1.txt"
[p] $file2_result :pg.serial << |U.Process.File"file2.txt"
[p] $file3_result :pg.serial << |U.Process.File"file3.txt"
[p] $file4_result :pg.serial << |U.Process.File"file4.txt"
```

---

## Best Practices

### ✅ 1. Keep Operations Independent

```polyglot
// ✅ GOOD: No dependencies
[p] $a :pg.int << |U.Calc.A""
[p] $b :pg.int << |U.Calc.B""

// ❌ BAD: B depends on A
[p] $a :pg.int << |U.Calc.A""
[p] $b :pg.int << |U.Calc.B"{$a}"  // Race condition!
```

### ✅ 2. Avoid Shared Mutable State

```polyglot
// ✅ GOOD: Separate outputs
[p] $python_data :pg.serial << |U.RT.Python.Process""
[p] $rust_data :pg.serial << |U.RT.Rust.Process""

// ❌ AVOID: Same file
[p] |U.File.Append"{$file}, {$python_data}"
[p] |U.File.Append"{$file}, {$rust_data}"
```

### ✅ 3. Use for I/O-Bound Tasks

Parallel execution shines for:
- Network calls
- File operations
- Database queries
- External process execution

Not as useful for:
- Pure computation (CPU-bound)
- Very fast operations

### ✅ 4. Document Parallel Sections

```polyglot
// Parallel fetch from multiple APIs (no dependencies)
[p] $user_data :pg.serial << |U.API.Users"{$id}"
[p] $order_data :pg.serial << |U.API.Orders"{$id}"
[p] $shipping_data :pg.serial << |U.API.Shipping"{$id}"
```

---

## Quick Reference

```
┌──────────────────────────────────────────────┐
│ PARALLEL EXECUTION                           │
├──────────────────────────────────────────────┤
│                                              │
│  MARKER                                      │
│  [p]  Parallel execution                     │
│  [r]  Sequential execution                   │
│                                              │
│  CHARACTERISTICS                             │
│  - All [p] operations start together         │
│  - Non-deterministic completion order        │
│  - Continues when all complete               │
│  - Has overhead cost                         │
│                                              │
│  USE FOR                                     │
│  ✅ Independent operations                   │
│  ✅ I/O-bound tasks                          │
│  ✅ API calls                                │
│  ✅ Multi-file processing                    │
│                                              │
│  AVOID FOR                                   │
│  ❌ Dependent operations                     │
│  ❌ Shared resource writes                   │
│  ❌ Ordered sequences                        │
│  ❌ Trivial operations (< 1ms)               │
│                                              │
└──────────────────────────────────────────────┘
```

---

## See Also

- [Hello World Tutorial](./hello-world-tutorial.md) - Sequential execution basics
- [Error Handling](./error-handling-guide.md) - Handling parallel errors
- [Performance Guide](./performance-guide.md) - Optimization strategies (if exists)

---

**Generated by:** Polly (Polyglot Language Expert)
**For:** User Documentation
**Version:** v0.0.4
**Last Updated:** 2025-12-26
**Confidence:** 🟢 Confident (based on verified parallel example, but timeout and some advanced patterns not fully verified)
