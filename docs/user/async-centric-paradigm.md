# The Async-Centric Paradigm

## Overview

Polyglot's async-centric paradigm fundamentally reimagines how programs handle asynchronous operations. Instead of treating async as a special case requiring keywords and explicit handling, Polyglot makes **all operations potentially asynchronous by default**, with the runtime automatically managing execution order based on data dependencies.

## Core Principles

### 1. Everything is Potentially Async

In traditional languages, you must explicitly mark async operations:

```javascript
// JavaScript - explicit async
async function fetchData( {
  const a = await fetch('/api/a';
  const b = await fetch('/api/b';
  return combine(a, b;

```

In Polyglot, operations are async by default:

```polyglot
// Polyglot - implicit async
[r] .a:pg.string << |U.HTTP.Get"/api/a"  // Starts immediately
[r] .b:pg.string << |U.HTTP.Get"/api/b"  // Starts immediately
[r] .result:pg.serial << |combine(.a, .b  // Waits for both
```

### 2. Automatic Dependency Tracking

The Polyglot runtime analyzes your code to understand dependencies and executes operations as soon as their inputs are ready.

```polyglot
[@] @Local::ParallelExample:1.0.0.0
[X]

[|] |ParallelDataProcessing
[i] .db_connection:pg.string << #MyDB.ConnectionString
[t] |T.Call
[o] .final:pg.array.pg.serial

[W] |W.DB.Postgresql
[<] <connection:pg.string << .db_connection
[>] >session:pg.db >> .db

// All three start in parallel
[p] |U.DB.Postgresql.Query.SQL
[<] <sql:pg.string << |SQL"SELECT * FROM table_a"
[>] >result:pg.array.pg.serial >> .data_a

[p] |U.DB.Postgresql.Query.SQL
[<] <sql:pg.string << |SQL"SELECT * FROM table_b"
[>] >result:pg.array.pg.serial >> .data_b

[p] |U.File.Text.ReadAll
[<] <file:pg.path << \\FileDir\\file.txt
[>] >content:pg.string >> .data_c

// Join first two results
[Y] |Y.JoinAll
[<] .data_a
[<] .data_b

// These two run in parallel (each waits only for its input
[r] @MyLib|process
[<] <data1:pg.array.pg.serial << .data_a
[<] <data2:pg.array.pg.serial << .data_b
[>] >processed1:pg.array.pg.serial >> .processed_a
[>] >processed2:pg.array.pg.serial >> .processed_b

// Wait for file data
[Y] |Y.JoinAll
[<] .data_c

[r] |U.String.Into.Array
[<] <delimiter:pg.string << ","
[>] >array:pg.array.pg.string >> .processed_c

// This waits for all dependencies
[r] |U.Array.Concat
[<] <arrays.*:pg.array.pg.serial << {
[*] .processed_a, .processed_b, .processed_c 
[>] >array:pg.array.pg.serial >> .final

[o] .final:pg.array.pg.serial
[X]
```

**Execution Flow:**
1. `[p]` blocks start all three fetch operations immediately in parallel
2. When `.data_a` is Ready, first `@MyLib|process` starts
3. When `.data_b` is Ready, second `@MyLib|process` starts
4. When `.data_c` is Ready, string-to-array conversion starts
5. When `.processed_a`, `.processed_b`, and `.processed_c` are all Ready, `|U.Array.Concat` executes

### 3. State-Based Execution Model

Every variable exists in one of three states:

| State | Description | Can Use? | Real-World Analogy |
|-------|-------------|----------|--------------------|
| **Declared** | Variable exists but has no value yet | ✗ | 📦 Order placed, tracking number assigned |
| **Pending** | Async operation in progress | ✗ | 🚚 Package in transit |
| **Ready** | Value available | ✓ | 📬 Package delivered, ready to open |

The runtime tracks these states automatically:

```polyglot
x = compute(  # x is Declared, then immediately Pending

# Runtime blocks here if x is not Ready yet
y = x + 10

# Now x must be Ready, so y becomes Pending, then Ready
```

## How It Works

### Dependency Graph Construction

**Metaphor:** 🍳 *Think of the runtime as a restaurant kitchen. The head chef (runtime) looks at the recipe (your code) and figures out which prep tasks can happen in parallel and which must wait for ingredients.*

When your Polyglot program is compiled, the compiler builds a dependency graph:

```polyglot
[r] .a:pg.serial << |fetch_a
[r] .b:pg.serial << |fetch_b
[r] .c:pg.serial << |process(.a
[r] .d:pg.serial << |process(.b
[r] .result:pg.serial << |combine(.c, .d
```

**Dependency Graph:**
```
|fetch_a ──> .a ──> |process(.a ──> .c ──┐
                                           ├──> |combine(.c,.d ──> .result
|fetch_b ──> .b ──> |process(.b ──> .d ──┘
```

**Kitchen Analogy:**
- `|fetch_a` and `|fetch_b` = Two cooks fetching ingredients from the pantry (can happen in parallel)
- `|process(.a)` and `|process(.b)` = Two cooks chopping ingredients (each waits for their ingredients to arrive, but chop independently)
- `|combine(.c, .d)` = Head chef combines both prepped ingredients into final dish (must wait for both cooks to finish)

### Runtime Execution

The runtime uses this graph to:
1. Start all operations with no dependencies immediately
2. Monitor state transitions (Declared → Pending → Ready
3. Execute operations as soon as their inputs are Ready
4. Propagate errors through the dependency chain

### PUSH vs PULL Operators

Polyglot provides explicit control when needed:

#### PUSH Operator (Fire and Forget
```polyglot
# Start operation but don't wait for result
log_message( PUSH

# Continue immediately
next_operation(
```

#### PULL Operator (Force Wait
```polyglot
# Force immediate synchronous execution
config = load_config( PULL

# config is guaranteed Ready here
validate(config
```

## Benefits

### 1. Implicit Parallelism

You get parallel execution without thinking about it:

```polyglot
# Traditional sequential (JavaScript
# async function process( {
#   const a = await fetch_a(;
#   const b = await fetch_b(;  // Waits for a!
#   return combine(a, b;
# 

# Polyglot automatic parallel
a = fetch_a(
b = fetch_b(  # Doesn't wait for a
result = combine(a, b
```

### 2. Simplified Code

No async/await keywords cluttering your code:

```polyglot
# Clean and simple
data = fetch_data(
processed = transform(data
result = save(processed
```

Compare with TypeScript:
```typescript
// Verbose
const data = await fetch_data(;
const processed = await transform(data;
const result = await save(processed;
```

### 3. Automatic Optimization

The runtime optimizes execution automatically:

```polyglot
# Compiler detects these are independent
result_1 = expensive_computation_1(
result_2 = expensive_computation_2(
result_3 = expensive_computation_3(

# They all run in parallel automatically
combined = merge(result_1, result_2, result_3
```

### 4. Natural Error Propagation

Errors flow through the dependency graph automatically:

```polyglot
data = fetch_data(  # Might fail
processed = transform(data  # Won't run if fetch failed
result = save(processed  # Won't run if transform failed

# Check final result
match result {
  Ok(value => success(value,
  Err(e => handle_error(e  # Gets error from any step

```

## Pipeline Integration

Pipelines are the primary way to structure async workflows:

```polyglot
&#124;Pipeline DataProcessor
  input: DataSource
  output: Result<Report, Error>

  # All of these can run in parallel if independent
  raw_data = input.read(
  validated = validate(raw_data
  transformed = transform(validated
  aggregated = aggregate(transformed

  return Report{aggregated
!
```

The Pipeline block:
- Defines clear input/output boundaries
- Automatically manages state for all internal variables
- Handles error propagation
- Can be tested and deployed independently

## Cross-Language Async

Wrapper blocks handle async operations across languages:

```polyglot
&#124;Wrapper rust_processor
  lang: Rust
  function: "process_data"
  input: RawData
  output: ProcessedData
!

# Call is async but looks synchronous
data = load_data(
result = rust_processor(data  # Calls Rust, automatically async
save(result
```

The runtime:
- Marshals data to Rust
- Executes Rust function (potentially in parallel with other ops
- Unmarshals result when ready
- Continues execution

## State Transitions in Detail

Think of variables like **package deliveries**. Each state represents where your package is in the delivery process.

### Declared State
**Metaphor:** 📦 *You've placed an order and received a tracking number, but the package hasn't shipped yet.*

```polyglot
# Variable declared, no value yet
x: Int
# x is Declared
```

The variable exists (you have a tracking number), but there's no value yet (no package in transit). You can't use it until it's assigned.

### Pending State
**Metaphor:** 🚚 *Package is in transit - you can see it's on the way, but you can't use what's inside yet.*

```polyglot
# Assignment starts async operation
x = fetch_data(
# x is now Pending (operation running
```

The operation is running (truck is driving), but the result isn't available yet. The runtime knows work is happening, but you must wait before using the value.

**Key Insight:** You can't open a package that's still on the truck!

### Ready State
**Metaphor:** 📬 *Package delivered to your door - you can open it and use the contents.*

```polyglot
# When operation completes, x becomes Ready
# Runtime automatically unblocks any code waiting for x
y = x + 10  # This line waits for x to be Ready
```

The operation finished (package delivered). Now you can use the value. If another operation was waiting for this value, it automatically starts (you can now unbox and use the contents).

**Key Insight:** The runtime waits for delivery automatically - no explicit `await` needed!

### Error State
**Metaphor:** ❌ *Package was lost or damaged during shipping - delivery failed.*

```polyglot
# If operation fails, x becomes Ready with Err
x = might_fail(

match x {
  Ok(val => use(val,
  Err(e => handle(e  # x is Ready(Err

```

The operation failed (delivery failed). The variable is Ready, but instead of a value, it carries error information (damaged package report). You can handle the error explicitly or let it propagate.

**Key Insight:** Errors are just another kind of "delivery" - they flow through your pipeline automatically.

## Comparison with Traditional Async

### Traditional (JavaScript
```javascript
async function process( {
  // Must think about await placement
  const a = await fetch_a(;  // Sequential
  const b = await fetch_b(;  // Sequential (bad!

  // Or use Promise.all (manual parallelism
  const [a, b] = await Promise.all([
    fetch_a(,
    fetch_b(
  ];

  return combine(a, b;

```

### Polyglot
```polyglot
# Automatic parallelism, no keywords
a = fetch_a(
b = fetch_b(
result = combine(a, b
```

### Traditional (Rust
```rust
async fn process( -> Result<Data, Error> {
    // Manual async handling
    let a = fetch_a(.await?;
    let b = fetch_b(.await?;

    // Or use tokio::join for parallelism
    let (a, b = tokio::join!(
        fetch_a(,
        fetch_b(
    ;

    combine(a?, b?

```

### Polyglot
```polyglot
# Same clean syntax
a = fetch_a(
b = fetch_b(
result = combine(a, b
```

## Best Practices

### 1. Trust the Runtime
Don't overthink parallelism. Write code naturally and let the runtime optimize:

```polyglot
# Just write what you mean
data = fetch(
processed = transform(data
saved = save(processed
```

### 2. Use PUSH for Fire-and-Forget
When you don't need the result:

```polyglot
log_event(event PUSH
update_metrics(count PUSH

# Continue without waiting
continue_processing(
```

### 3. Use PULL for Critical Ordering
When execution order matters:

```polyglot
config = load_config( PULL  # Must have config
initialize(config
start_server(
```

### 4. Structure with Pipelines
Organize async workflows into Pipeline blocks:

```polyglot
&#124;Pipeline ETL
  input: Source
  output: Result<Report, Error>

  extracted = extract(input
  transformed = transform(extracted
  loaded = load(transformed

  return Report{loaded
!
```

## Summary

The async-centric paradigm is Polyglot's core innovation:

- **All operations are async by default** - no keywords needed
- **Automatic dependency tracking** - runtime executes when ready
- **State-based execution** - Declared → Pending → Ready lifecycle
- **Implicit parallelism** - get concurrency without thinking about it
- **Natural error flow** - errors propagate through dependencies
- **Cross-language async** - same model works across all languages

This paradigm makes async programming as natural as writing synchronous code, while achieving better performance through automatic parallelism.

**Next**: Learn about [Variable States](advanced/variable-states.md in detail, or explore [Parallel Execution](advanced/parallel-execution.md patterns.
