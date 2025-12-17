# Variable States (Advanced

## Overview

Polyglot's state system is fundamental to its async-centric paradigm. Every variable progresses through a complete lifecycle from declaration to scope cleanup: **Pending → Default/Final → Faulted (on error → Cleared (scope end**. Understanding these states is key to mastering Polyglot's automatic parallelism and dependency tracking.

## The Six Core States

### 1. Pending State

A variable is declared without a value OR an async operation is in progress.

```polyglot
[r] .x:pg.int  // Pending (no value yet

[r] |FetchData
[>] >data:pg.string >> .y  // Pending (async operation running
```

**Characteristics:**
- Declaration without value, OR async operation in progress
- No value available yet
- Using the variable blocks until Final or Faulted
- Multiple Pending variables run in parallel

### 2. Default State

A variable has a default value that can be overridden once.

```polyglot
[i] .timeout:pg.int <~ 30  // Default with value 30
[i] .defaultConfig: #Config <~ .config  // Default from source
```

**Characteristics:**
- Has a usable value (the default
- Can be overridden exactly once
- After override or first use → transitions to Final
- Acts like Final for reading

### 3. Final State

The value is available and immutable.

```polyglot
[r] .x:pg.int << 42  // Final immediately (sync

[r] |FetchData
[>] >data:pg.string >> .y  // Pending → Final (async

[r] .result:pg.int << U.Int.Add"{.x, .y"  // Waits for .y to be Final
```

**Characteristics:**
- Value is available
- Immutable - no more pushes accepted
- Can be pulled/read unlimited times
- Remains Final until scope ends

### 4. Faulted State

An operation failed and the variable carries error information.

```polyglot
[r] |HttpGet
[<] <url:pg.string << .invalid_url
[>] >response:pg.string >> .data  // Becomes Faulted if request fails

[r] .result:pg.string << .data  // Error propagates
```

**Characteristics:**
- Operation failed with error
- Contains error info (.message, .code, .trace
- Error propagates through dependent variables
- Cannot be used directly (use error handling

### 5. Cleared State

The pipeline scope ended and all variables are freed from memory. **Terminal state**.

```polyglot
[|] |Example
[i] .input:pg.string
[t] |T.Manual

[r] .temp:pg.int << 42
[o] .result:pg.int << U.Int.Add"{.temp, 10"
[X]
// All variables → Cleared (memory freed
// Cannot access .input, .temp, or .result after this point
```

**Characteristics:**
- Pipeline ended (reached `[X]`
- All variables freed via `|W.Polyglot.Scope` wrapper
- Terminal state - no further transitions
- Accessing Cleared variable is runtime error

## State Transitions

### Complete Lifecycle

```polyglot
[|] |Example
[i] .input:pg.string  // Born: Pending (no value
[t] |T.Manual

// Path 1: Pending → Final
[r] .x:pg.int  // Pending
[r] .x << 42    // Pending → Final

// Path 2: Default → Final
[r] .y:pg.int <~ 30  // Default
[r] .y << 60          // Default → Final (override

// Path 3: Direct Final
[r] .z:pg.int << 100  // Pending → Final (immediate

// Path 4: Async Pending → Final/Faulted
[r] |Fetch
[>] >result:pg.string >> .data  // Pending → Final/Faulted

[o] .result:pg.int << U.Int.Add"{.x, .y" + .z
[X]
// Death: All variables → Cleared (scope cleanup
```

**Complete State Diagram:**
```
DECLARATION
     ├─ No value          → Pending
     ├─ Default <~ or ~>  → Default
     ├─ Push << (sync    → Final
     └─ Push >> (async   → Pending

USAGE & TRANSITIONS
Pending      ──(assignment──> Final
Pending      ──(async fail──> Faulted
Default ──(override────> Final
Default ──(first use──> Final

SCOPE END
Any State    ──([X] cleanup──> Cleared (terminal
```

### Synchronous vs Async Operations

**Synchronous (immediate Final:**

```polyglot
[r] .x:pg.int << 42  // Pending → Final (instant
[r] .y:pg.string << "hello"  // Pending → Final (instant
[r] .sum:pg.int << U.Int.Add"{.x, 10"  // Final immediately (both operands Final
```

**Async (Pending → Final:**

```polyglot
[r] |FetchData
[>] >result:pg.string >> .data  // Pending (async running
// ... execution continues ...
[r] .length:pg.int << U.String.Length"{.data"  // Waits for .data to be Final
```

### PULL Auto-Await

```polyglot
[r] |HttpGet
[<] <url:pg.string
[>] >http_response:pg.dict >> .response  // Pending

[r] .status:pg.int << U.Dict.Get"{.response, 'status_code'"  // Auto-awaits .response

// Using .response waits until it's Final or Faulted
```

## Dependency Tracking

The runtime automatically tracks which variables depend on which:

### Simple Dependency

```polyglot
[r] |FetchA
[>] >result:pg.int >> .a  // .a is Pending

[r] .b:pg.int << .a + 10      // Waits for .a to be Final, .b becomes Pending
[r] .c:pg.int << .b * 2       // Waits for .b to be Final, .c becomes Pending
```

**Execution:**
1. `|FetchA` starts (.a is Pending
2. Runtime sees .b depends on .a
3. When .a is Final, compute .b (.b is Pending
4. When .b is Final, compute .c (.c is Pending
5. When .c is Final, can be used

### Parallel Dependencies

```polyglot
// All three start immediately (parallel
[r] |FetchA
[>] >result_a:pg.int >> .a  // .a is Pending

[r] |FetchB
[>] >result_b:pg.int >> .b  // .b is Pending

[r] |FetchC
[>] >result_c:pg.int >> .c  // .c is Pending

// This waits for all three
[r] |Combine
[<] <input_a:pg.int << .a
[<] <input_b:pg.int << .b
[<] <input_c:pg.int << .c
[>] >combined:pg.int >> .result
```

**Execution:**
1. `|FetchA`, `|FetchB`, `|FetchC` all start
2. All three operations run in parallel
3. Runtime waits for all to be Final
4. When all Final, `|Combine` executes

### Complex Dependency Graph

```polyglot
// Multiple async operations
[r] |FetchDataA
[>] >fetched_a:pg.dict >> .data_a     // Pending

[r] |FetchDataB
[>] >fetched_b:pg.dict >> .data_b     // Pending (parallel with .data_a

// These depend on their inputs
[r] |Process
[<] <input:pg.dict << .data_a
[>] >output:pg.dict >> .processed_a  // Waits for .data_a

[r] |Process
[<] <input:pg.dict << .data_b
[>] >output:pg.dict >> .processed_b  // Waits for .data_b

// This waits for both processed results
[r] |Merge
[<] <left:pg.dict << .processed_a
[<] <right:pg.dict << .processed_b
[>] >merged:pg.dict >> .final
```

**Dependency Graph:**
```
|FetchDataA ──> .data_a ──> |Process ──> .processed_a ──┐
                                                          ├──> |Merge ──> .final
|FetchDataB ──> .data_b ──> |Process ──> .processed_b ──┘
```

**Execution:**
1. `|FetchDataA` and `|FetchDataB` start in parallel
2. When `.data_a` is Final, first `|Process` starts
3. When `.data_b` is Final, second `|Process` starts
4. When both `.processed_a` and `.processed_b` are Final, `|Merge` executes

## Working with States

### Checking State (Advanced

Normally you don't check states explicitly, but for debugging:

```polyglot
# Check if value is ready (advanced feature
if x.is_ready( {
  use(x
 else {
  # x is still Pending
  wait_or_do_something_else(

```

### Forcing State Transitions

#### PULL Operator

```polyglot
# Force synchronous execution
config = load_config( PULL

# config is guaranteed Final here
# Execution waits until operation completes
initialize(config
```

#### PUSH Operator

```polyglot
# Fire and forget (don't wait
log_event(data PUSH

# Continues immediately
# log_event runs in background
continue_processing(
```

## State in Pipelines

Pipelines automatically manage states:

```polyglot
&#124;Pipeline ProcessData
  input: Source
  output: Result

  # Each line creates new states
  raw = fetch(input        # raw is Pending
  cleaned = clean(raw      # Waits for raw, cleaned is Pending
  validated = validate(cleaned  # Waits for cleaned, validated is Pending

  return validated
!
```

**State Flow:**
1. `input` is Final (provided by caller
2. `raw` becomes Pending
3. When `raw` is Final, `cleaned` becomes Pending
4. When `cleaned` is Final, `validated` becomes Pending
5. When `validated` is Final, pipeline returns

### Parallel Processing in Pipelines

```polyglot
&#124;Pipeline ProcessBatch
  input: List<Item>
  output: List<Result>

  # Expand creates parallel Pending states
  ...input => {
    processed = process(item
    validated = validate(processed
    return save(validated
  
!
```

**State Flow:**
- Each item gets its own state tracking
- All items process in parallel
- Pipeline completes when all items are Final

## State with Error Handling

### Result States

```polyglot
# Operation might fail
result = risky_operation(  # result is Pending

# When Final, result is either Ok or Err
match result {
  Result.Ok(value => use(value,   # value is Final
  Result.Err(e => handle_error(e  # e is Final

```

### Error Propagation

```polyglot
fn process( -> Result<Data, Error> {
  # If any step returns Err, function returns early
  a = step1(?  # Pending → Final(Ok or returns Err
  b = step2(a? # Waits for a, then Pending → Final(Ok or returns Err
  c = step3(b? # Waits for b, then Pending → Final(Ok or returns Err

  return Result.Ok(c

```

**State Flow with Errors:**
1. `step1(` executes, result becomes Final
2. If Final(Err, function returns immediately
3. If Final(Ok, value extracted, `step2(` starts
4. Pattern repeats for each step

## Advanced State Patterns

### Conditional States

```polyglot
# State depends on condition
if use_cache {
  data = get_from_cache(  # Fast path
 else {
  data = fetch_from_api(  # Slow path (Pending


# data might be Final or Pending here
# Runtime handles both cases
result = process(data
```

### State Aggregation

```polyglot
# Collect multiple async results
tasks = [task1(, task2(, task3(]

# All tasks are Pending
# Wait for all to be Final
results = tasks.collect(

# results is Final when all tasks complete
```

### Lazy Evaluation

```polyglot
# Computation doesn't start until value is used
lazy_value = expensive_computation(

# If we never use lazy_value, it might not execute
if condition {
  use(lazy_value  # Now it executes (if not already

```

## State Debugging

### Visualizing State Flow

```polyglot
# Enable state tracking (debugging feature
@trace_states
fn complex_workflow( {
  a = fetch_a(
  b = fetch_b(
  c = process(a, b
  return c


# Output shows state transitions:
# [0ms] a: Declared → Pending (fetch_a started
# [0ms] b: Declared → Pending (fetch_b started
# [100ms] a: Pending → Final (fetch_a completed
# [150ms] b: Pending → Final (fetch_b completed
# [150ms] c: Declared → Pending (process started
# [200ms] c: Pending → Final (process completed
```

## Best Practices

### 1. Let the Runtime Manage States

```polyglot
# Good: Natural code, runtime manages states
a = fetch(
b = process(a
c = save(b

# Don't: Manual state management
a = fetch(
wait_for(a  # Unnecessary
b = process(a
wait_for(b  # Unnecessary
c = save(b
```

### 2. Declare Variables Close to Use

```polyglot
# Good: Declare when needed
fn process( {
  # ... other work ...

  data = fetch_data(  # Starts when needed
  result = transform(data
  return result


# Less optimal: Early declaration
fn process( {
  data = fetch_data(  # Starts immediately

  # ... other work that doesn't use data ...
  # data might be Final but sitting idle

  result = transform(data
  return result

```

### 3. Use PULL Only When Necessary

```polyglot
# Good: PULL for critical ordering
config = load_config( PULL
initialize(config  # Must have config first
start_server(

# Don't: Unnecessary PULL
data = fetch_data( PULL  # Unnecessary if next line uses data
result = process(data    # Would wait automatically
```

### 4. Leverage Parallel States

```polyglot
# Good: Start all operations immediately
users = fetch_users(
products = fetch_products(
orders = fetch_orders(

# All three run in parallel
report = generate_report(users, products, orders

# Don't: Sequential when could be parallel
users = fetch_users(
report1 = generate_user_report(users
products = fetch_products(  # Could have started earlier
report2 = generate_product_report(products
```

### 5. Handle Errors at State Boundaries

```polyglot
# Good: Clear error handling
result = risky_operation(

match result {
  Result.Ok(value => process(value,
  Result.Err(e => handle_error(e


# Or use ?
fn process( -> Result<Data, Error> {
  value = risky_operation(?  # Propagate error
  return Result.Ok(transform(value

```

## State System Summary

| State | Description | Can Use? | Operations |
|-------|-------------|----------|------------|
| **Declared** | Variable exists, no value | ✗ | Type checking only |
| **Pending** | Async operation running | ✗ (waits | State can be queried |
| **Final** | Value available | ✓ | All operations allowed |

## State Transition Rules

1. **Declared → Pending**: Assignment to async operation
2. **Pending → Final**: Operation completes (success or error
3. **Declared → Final**: Synchronous operation (instant
4. **No backwards transitions**: Once Final, stays Final

## Performance Implications

### Parallel Execution

```polyglot
# Efficient: 3 operations in parallel
a = fetch_a(  # 100ms
b = fetch_b(  # 100ms
c = fetch_c(  # 100ms
result = combine(a, b, c
# Total time: ~100ms (parallel
```

### Sequential Execution

```polyglot
# Less efficient: Sequential execution
a = fetch_a( PULL  # 100ms
b = fetch_b( PULL  # 100ms (waits for a
c = fetch_c( PULL  # 100ms (waits for b
result = combine(a, b, c
# Total time: ~300ms (sequential
```

## Next Steps

- **Parallel Execution**: Deep dive into [parallelism patterns](parallel-execution.md
- **Pipeline Lifecycle**: Understand [how pipelines manage states](pipeline-lifecycle.md
- **Async Paradigm**: Review [async-centric design](../async-centric-paradigm.md
- **Examples**: See [variable states in action](../examples/data-processing.md

---

**See also**: [Async-Centric Paradigm](../async-centric-paradigm.md | [Operators](../syntax/operators.md | [Block Markers](../syntax/block-markers.md
