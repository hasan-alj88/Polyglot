# Async-Centric Language

Polyglot is fundamentally different from traditional programming languages. Understanding this difference is crucial to writing effective Polyglot code.

**The Core Difference:** In most languages, you write synchronous code and add async manually. In Polyglot, **everything is async by default**, and the language handles synchronization automatically.

## The Mental Model Shift

### Traditional Synchronous Thinking

In traditional languages, code executes line by line, blocking at each step:

```python
# Python - Synchronous mental model
def process(:
    data = fetch_from_api(        # Wait here
    cleaned = clean_data(data     # Wait here
    result = analyze(cleaned      # Wait here
    save_to_db(result            # Wait here
    return result
```

Each line blocks until complete. The flow is simple and linear.

### Traditional Async (Manual Coordination

When you need async in traditional languages, you manually manage it:

```python
# Python - Manual async/await
async def process(:
    data = await fetch_from_api(        # Explicitly await
    cleaned = await clean_data(data     # Explicitly await
    result = await analyze(cleaned      # Explicitly await
    await save_to_db(result            # Explicitly await
    return result
```

```javascript
// JavaScript - Promises
async function process( {
    const data = await fetchFromAPI(;       // Explicitly await
    const cleaned = await cleanData(data;   // Explicitly await
    const result = await analyze(cleaned;   // Explicitly await
    await saveToDB(result;                 // Explicitly await
    return result;

```

You must:
- Mark functions as `async`
- Add `await` keywords everywhere
- Handle promises/futures explicitly
- Coordinate parallelism manually

### Polyglot Async (Automatic Coordination

In Polyglot, async is the default—no keywords needed:

```polyglot
[|Pipeline] .process
[o] .result: py\DataFrame

.data << py\fetch_from_api(      // Starts async, becomes Pending
.cleaned << py\clean_data(.data  // Waits automatically if needed
.result << py\analyze(.cleaned   // Waits automatically if needed
py\save_to_db(.result            // Waits automatically if needed
```

**No `async` keyword. No `await` keyword. No promises. No manual coordination.**

The language knows:
- When to start async operations
- When to wait for results
- When operations can run in parallel
- How to propagate errors

## How It Works: Variable States

The magic comes from **variable states**. Every variable in Polyglot is a state machine:

```
Declared → Pending → Ready
                  ↘ Faulted
```

### State 1: Declared

When you declare a variable, it exists but has no value:

```polyglot
[|Pipeline] .example
[i] .input: String
[o] .output: Integer

// At this point:
// .input is Declared (will be provided when pipeline runs
// .output is Declared (will be computed
```

### State 2: Pending

When you start a computation, the variable becomes Pending:

```polyglot
.result << py\expensive_computation(
// .result is now Pending (computation running in background

// Execution continues immediately - doesn't wait!
.other_work << do_something_else(
// This runs in parallel with expensive_computation
```

### State 3: Ready

When computation completes, the variable becomes Ready:

```polyglot
.result << py\expensive_computation(  // Pending
// ... computation finishes ...
// .result is now Ready

.value << .result + 10  // Can use it - value is available
```

### State 4: Faulted

If computation fails, the variable becomes Faulted:

```polyglot
.result << py\might_fail(  // Could fault

// If it faults, .result carries error information
// Using .result propagates the error automatically
.value << .result + 10  // .value also becomes Faulted
```

## The PUSH and PULL Operators

Two operators control async flow:

### PUSH `<<` - Start Computation

The PUSH operator starts an async operation and makes the variable Pending:

```polyglot
.data << py\requests.get("https://api.example.com"
// Returns immediately!
// .data is Pending while HTTP request runs
```

**Key Points:**
- Execution doesn't wait
- Variable becomes Pending immediately
- Computation runs in background
- Multiple PUSHes can run in parallel

### PULL `>>` - Wait if Needed

The PULL operator accesses a field/method and waits if the variable is Pending:

```polyglot
.response << py\requests.get("https://api.example.com"
// .response is Pending

.body << .response >> text
// PULL waits for .response to become Ready
// Then accesses .text field
```

**Key Points:**
- Waits automatically if Pending
- Returns immediately if Ready
- Propagates errors if Faulted
- No explicit await needed

## Automatic Waiting Examples

### Example 1: Sequential Dependencies

```polyglot
[|Pipeline] .sequential
[i] .url: String
[o] .result: Integer

// Step 1: Start fetch (Pending
.response << py\requests.get(.url

// Step 2: Wait for response, then parse (Pending
.data << py\json.loads(.response >> text

// Step 3: Wait for data, then count (Pending
.result << .data >> length

// Each step waits automatically for the previous step
// No await keywords needed
```

**Execution Flow:**
1. `.response << py\requests.get(.url` - Starts HTTP request, returns immediately
2. `.data << py\json.loads(.response >> text` - Waits for HTTP, then parses
3. `.result << .data >> length` - Waits for parsing, then gets length

### Example 2: Automatic Parallelism

```polyglot
[|Pipeline] .parallel
[i] .url1: String
[i] .url2: String
[o] .combined: String

// Start both fetches simultaneously (both Pending
.data1 << py\requests.get(.url1
.data2 << py\requests.get(.url2

// This waits for BOTH to complete
.combined << py\merge(.data1, .data2
```

**Execution Flow:**
1. `.data1 << py\requests.get(.url1` - Starts fetch 1, returns immediately
2. `.data2 << py\requests.get(.url2` - Starts fetch 2, returns immediately (parallel!
3. `.combined << py\merge(.data1, .data2` - Waits for both, then merges

**No manual parallelism code!** The language detects that `.data1` and `.data2` are independent and runs them in parallel automatically.

### Example 3: Conditional Waiting

```polyglot
[|Pipeline] .conditional
[i] .use_cache: Boolean
[i] .url: String
[o] .data: py\dict

// Only fetch if not using cache
.response << py\requests.get(.url

// Wait only if we fetched
.data << .use_cache ? py\cache.get( : .response >> json(
```

The language is smart enough to wait for `.response` only if the else branch is taken.

## Common Patterns

### Pattern 1: Fan-Out, Fan-In

Start multiple operations, wait for all:

```polyglot
[|Pipeline] .fan_out_fan_in
[i] .urls: Array
[o] .results: Array

// Fan-out: Start all fetches (all Pending
.responses << .urls | [~] .url -> py\requests.get(.url

// Fan-in: Wait for all, then process
.results << .responses | [~] .resp -> .resp >> json(
```

### Pattern 2: Pipeline Chaining

Chain operations that depend on each other:

```polyglot
[|Pipeline] .chain
[i] .input: String
[o] .output: String

.step1 << py\process_a(.input
.step2 << rs\process_b(.step1
.step3 << go\process_c(.step2
.output << js\process_d(.step3

// Each step waits for the previous automatically
```

### Pattern 3: Error Handling

Errors propagate through Faulted state:

```polyglot
[|Pipeline] .with_errors
[i] .url: String
[o] .result: py\dict

.response << py\requests.get(.url  // Might fault (404, timeout, etc.
.data << .response >> json(         // Propagates fault if .response faulted
.result << .data                     // Propagates fault if .data faulted

// If any step faults, .result is Faulted
// No try/catch needed for propagation
```

### Pattern 4: Partial Failure

Some operations can fail without failing the whole pipeline:

```polyglot
[|Pipeline] .partial_failure
[i] .primary_url: String
[i] .fallback_url: String
[o] .data: py\dict

.primary << py\requests.get(.primary_url
.fallback << py\requests.get(.fallback_url

// Use primary if Ready, otherwise fallback
.data << .primary !? .fallback >> json(

// If both fault, .data is Faulted
```

## No Await Keyword

You might wonder: "How do I know when to wait?"

**Answer: You don't need to know.** The language waits automatically when you use a Pending variable.

```polyglot
.result << expensive_operation(  // Starts async

// Any use of .result waits automatically:
.value1 << .result + 10           // Waits
.value2 << .result >> field       // Waits
.value3 << some_function(.result // Waits

// Even passing to another function waits:
py\print(.result  // Waits for result to be Ready
```

**Contrast with Python:**

```python
# Python - must remember to await
result = expensive_operation(  # Returns coroutine

value1 = await result + 10      # Must await
value2 = (await result.field   # Must await
value3 = some_function(await result  # Must await

print(await result  # Must await
```

Polyglot eliminates the mental burden of remembering to await.

## Automatic Error Propagation

When a variable is Faulted, the error propagates automatically:

```polyglot
[|Pipeline] .error_propagation
[i] .url: String
[o] .count: Integer

.response << py\requests.get(.url  // Might fault (network error
.data << .response >> json(         // If .response faulted, .data faults
.count << .data >> length            // If .data faulted, .count faults

// No try/catch needed - errors propagate through state
```

**To handle errors explicitly, use error handlers:**

```polyglot
[|Pipeline] .error_handling
[i] .url: String
[o] .count: Integer

[!] .network_error
[<] <error_type: !NetworkError

.response << py\requests.get(.url
.data << .response >> json(
.count << .data >> length

[b] .network_error
    // Handle network errors
    .count << 0
[/]
```

## Common Mistakes from Sync Programmers

### Mistake 1: Over-Sequencing

```polyglot
// BAD: Forces sequential execution
.data1 << py\fetch(.url1
.temp1 << .data1  // Unnecessary wait
.data2 << py\fetch(.url2
.temp2 << .data2  // Unnecessary wait
.result << py\merge(.temp1, .temp2
```

```polyglot
// GOOD: Allows parallel execution
.data1 << py\fetch(.url1
.data2 << py\fetch(.url2
.result << py\merge(.data1, .data2  // Waits for both
```

### Mistake 2: Forgetting PULL

```polyglot
// BAD: Might not wait properly
.response << py\requests.get(.url
.body << .response.text  // Should use >> not .
```

```polyglot
// GOOD: Explicit PULL
.response << py\requests.get(.url
.body << .response >> text
```

### Mistake 3: Manual Sleep

```polyglot
// BAD: Don't use sleep to wait
.result << py\async_operation(
py\time.sleep(5  // DON'T DO THIS
.value << .result
```

```polyglot
// GOOD: Let state system wait
.result << py\async_operation(
.value << .result  // Waits automatically
```

## Performance Implications

### Parallelism is Free

```polyglot
// These run in parallel automatically:
.a << py\fetch(.url1
.b << py\fetch(.url2
.c << py\fetch(.url3

// Wait for all three:
.result << py\combine(.a, .b, .c
```

**No manual thread pools, no async libraries, no coordination code.**

### Lazy Evaluation

Variables only compute when needed:

```polyglot
.expensive << py\very_expensive_operation(

// If .expensive is never used, it never runs
// (Unless marked with [!] eager evaluation
```

### Efficient Waiting

The runtime uses efficient async I/O, not busy-waiting or sleep loops.

## Thinking Async

To write effective Polyglot:

1. **Stop thinking about execution order** - Think about data dependencies
2. **Let independent operations run in parallel** - Don't force sequence
3. **Trust the state system** - Don't manually wait or coordinate
4. **Use PUSH for starting, PULL for accessing** - Clear intent
5. **Let errors propagate** - Don't over-handle errors

## Comparison Table

| Aspect | Traditional Sync | Traditional Async | Polyglot |
|--------|------------------|-------------------|----------|
| **Default** | Synchronous | Synchronous + opt-in async | Asynchronous |
| **Waiting** | Automatic (blocking | Manual (await/then | Automatic (non-blocking |
| **Parallelism** | Manual (threads | Manual (Promise.all/gather | Automatic |
| **Keywords** | None | async, await, then, catch | None |
| **Error Handling** | try/catch | try/catch + promise rejection | State propagation + handlers |
| **Mental Model** | Sequential | Sequential + async markers | Data flow |

## Next Steps

Now that you understand the async-centric model:

1. **Deep Dive:** Read [Variable State System](variable-state-system.md for complete state details
2. **Practice:** Try [Getting Started](getting-started.md examples with async in mind
3. **Advanced:** Explore [Parallel Execution](advanced/parallel-execution.md for explicit parallelism
4. **Real Code:** See [Cross-Language Integration](examples/cross-language-integration.md examples

The async-centric model is Polyglot's superpower. Master it, and orchestration becomes natural and effortless.
