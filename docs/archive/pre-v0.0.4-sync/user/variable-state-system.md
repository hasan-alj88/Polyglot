---
archived: true
archived_date: 2025-12-17
archived_reason: "Superseded by v0.0.4 sync"
original_path: user/variable-state-system.md
---

<!-- ARCHIVED: 2025-12-17 | Reason: Superseded by v0.0.4 sync -->

# Variable State System

The variable state system is Polyglot's secret weapon for automatic async orchestration. Understanding how variables transition through states is essential to mastering the language.

## The Complete Variable Life-cycle

Every variable in Polyglot goes through a complete life-cycle from birth (declaration to death (scope cleanup:

```
DECLARATION
     │
     ├─ No value          → Pending (awaiting push
     ├─ Default <~ or ~>  → Default (can push once more
     ├─ Push << (sync    → Final (No more push allowed
     └─ Push >> (async   → Pending → Final/Faulted

USAGE & EXECUTION
     │
     └─ State: Pending/Default/Final/Faulted

PIPELINE ENDS [X]
     │
     └─ All variables → Cleared (memory freed
```

---

## The Five Core States

Every variable exists in exactly one of six states at any given time:

1. **Pending** - Declared without value (awaiting push, OR async operation in progress
2. **Default** - Has default push, can be overridden once with push
3. **Final** - Value available and immutable (no more pushes
4. **Faulted** - Operation failed, carries error info
5. **Cleared** - Scope ended, memory freed (terminal state

Let's explore each state in detail.

## State 1: Pending

A variable is **Pending** when it is declared without a value (awaiting push OR when an async push operation is in progress.

### How Variables Become Pending

**1. Declaration Without Value (Awaiting Push:**

```polyglot
[|] |Example
[i] .input_data:pg.string  // Pending (awaiting push
[i] .count:pg.int          // Pending (awaiting push
[t] |T.Manual
```

When a variable is declared without push, it starts in **Pending** state (awaiting push.

**2. Async Push Operations:**

```polyglot
[r] |ExpensiveProcess
[>] >result:pg.string >> .data
// .result is now Pending (async push operation in progress
```

**3. Variables Awaiting Push:**

```polyglot
[r] .temp:pg.int  // Pending (awaiting push
[r] .temp << 42    // Normal push: Pending → Final
```

### Pending State Rules

- **Cannot be used** directly until receives push or async push completes
- **Awaited automatically** when used in expressions or passed to functions
- **Can receive push** via `<<` or `>>` operators (normal push
- **Type is known** but value is not yet available
- **Waiting for push** - either normal push or async push completion

**Example:**

```polyglot
[|] |Example
[i] .value:pg.int  // Input - provided by CALLER
[t] |T.Call

// Pipeline WAITS until .value is Final (provided by caller
// Then executes body
[r] .result:pg.string << U.Int.ToString"{.value"

// CALLER provides the input:
[r] |Example
[<] <value:pg.int << 42  // Input provided → pipeline triggers
[>] >result:pg.string >> .output
```

### Async Pending vs Declaration Pending

**Declaration Pending:**
```polyglot
[r] .var:pg.int  // Pending, awaiting push
```

**Async Pending:**
```polyglot
[r] |AsyncOperation
[>] >var:pg.int >> .result  // Pending, computation running
```

Both are in Pending state, but for different reasons. The runtime treats them identically - both wait for Final or Faulted.

## State 2: Default

A variable is **Default** when it has a default value that can be overridden **exactly once**.

### How Variables Become Default

**Using Default PUSH `<~`:**

```polyglot
[i] .timeout:pg.int <~ 30         // Default with value 30
[i] .retries:pg.int <~ 3          // Default with value 3
```

**Using Default PULL `~>`:**

```polyglot
[i] .defaultConfig: #Config <~ .config  // Default from source
```

### Default State Rules

- **Has a value** that can be used immediately
- **Allows ONE override** during assignment
- **After override** or first use → transitions to Final (immutable
- **Acts like Final** for reading/pulling
- **Different from Final** in that it accepts one more push

**Override-Once Behavior:**

```polyglot
[|] |ProcessData
[i] .timeout:pg.int <~ 30  // Default (default: 30
[i] .my_url:pg.url << url"http://MyWebsite.com"
[t] |T.Manual

// Case 1: Use default (no override - explicitly pass .timeout
[r] |FetchWithTimeout
[<] <url:pg.string << .my_url
[<] <timeout:pg.int << .timeout  // Passes default value (30
[>] >result:pg.string >> .data
// .timeout = 30, now transitioned to Final

// Case 2: Override once, then use
[r] .timeout << 60  // Override: Default → Final (value: 60
[r] .timeout << 90  // ERROR: Already overridden, now Final (immutable

[r] |FetchWithTimeout
[<] <url:pg.string << .my_url
[<] <timeout:pg.int << .timeout  // Passes overridden value (60
[>] >result:pg.string >> .data
```

### Use Cases

**Pipeline Input Parameters with Defaults:**
```polyglot
[|] |ProcessBatch
[i] .items:pg.array              // Required input
[i] .batch_size:pg.int <~ 100    // Optional: default batch size
[i] .parallel:pg.bool <~ #Boolean.False  // Optional: parallel mode
[t] |T.Call
```

**Optional Behavior Flags:**
```polyglot
[|] |FetchData
[i] .url:pg.string               // Required input
[i] .retry_on_failure:pg.bool <~ #Boolean.True  // Optional: retry behavior
[i] .max_retries:pg.int <~ 3     // Optional: retry limit
[t] |T.Call
```

**⚠️ Configuration/Settings: Use `[#]` Enumeration Instead:**

For application configuration (especially sensitive data, use enumerations with serial file loading:

```polyglot
[#] #Config
[s] .file :pg.serial <<YAML.Load"config.yaml"   // Serial file loader
[<] .max_retries:pg.int <~ 3
[<] .timeout_ms:pg.int <~ 5000
[<] .cache_enabled:pg.bool <~ #Boolean.True
[<] .log_level:pg.string <~ "INFO"
[<] .api_key:pg.string << .file.api_key         // Load from file
[<] .database_url:pg.url << .file.db_url        // Load from file
[X]
```

## State 3: Final

A variable is **Final** when its value is available and can be used.

### How Variables Become Final

**1. Successful Computation:**

```polyglot
[r] |ProcessData
[>] >result:pg.string >> .data  // Pending
// ... computation completes successfully ...
// .result is now Final
```

**2. Direct Assignment:**

```polyglot
[r] .value:pg.int << 42  // Immediately Final (literal
[r] .name:pg.string << "Alice"  // Immediately Final (literal
[r] .flag:pg.bool << #True  // Immediately Final (literal
```

**3. Pure Transformations:**

```polyglot
[r] .a:pg.int << 10  // Final
[r] .b:pg.int << 20  // Final
[r] .sum:pg.int << U.Int.Add"{.a, .b"  // Final (both operands Final
```

### Using Final Variables

Once Final, variables can be used freely:

```polyglot
[r] |FetchData
[>] >data:pg.string >> .result
// |FetchData.Outputs.data pushed into .result
// Thus .result Pending → Final (when |FetchData completes

// All of these work once .result is Final:
[r] .length:pg.int << U.String.Length"{.result"
[r] .first:pg.string << U.String.Char.First"{.result"

[r] |Transform
[<] <input:pg.string << .result
[>] >processed:pg.string >> .output
```

### Final State is Permanent

Once a variable becomes Final, it stays Final:

```polyglot
[r] .value:pg.int << 42  // Final

// Can use .value many times
[r] .double:pg.int << U.Int.Multiply"{.value, 2"
[r] .triple:pg.int << U.Int.Multiply"{.value, 3"
[r] .str:pg.string << U.Int.ToString"{.value"

// .value is still Final - doesn't change
```

**No re-execution!** Variables compute once and cache the result.

## State 4: Faulted

A variable is **Faulted** when a computation fails and produces an error.

### How Variables Become Faulted

**1. Pipeline Operation Failure:**

```polyglot
[r] |HttpGet
[<] <url:pg.string << "https://invalid-url.example"
[>] >data:pg.string >> .response
// Network error → .data becomes Faulted
```

**2. Type Errors:**

```polyglot
[r] |ProcessInt
[<] <value:pg.string << "not an int"  // Wrong type passed
[>] >result:pg.int >> .output
// Type error → .result becomes Faulted
```

**3. Pipeline Errors:**

```polyglot
[r] |FailingPipeline
[<] <input:pg.string
[>] >output:pg.string >> .result
// Pipeline fails → .output becomes Faulted
```

**4. Explicit Error Creation:**

```polyglot
[r] .error: !NetworkError << !NetworkError(
    .message: "Connection timeout",
    .code: 504

// .error is Faulted (carries error
```

### Error Propagation

Faulted variables propagate their error automatically:

```polyglot
[r] |MightFail
[>] >a:pg.int >> .result  // Becomes Faulted

[r] .b:pg.int << U.Int.Add"{.a, 10"  // .b also becomes Faulted (propagates error
[r] |ProcessValue
[<] <input:pg.int << .b
[>] >c:pg.int >> .output  // .output also becomes Faulted
[r] .d:pg.int << U.Int.Add"{.output, 5"  // .d also becomes Faulted

// Error propagates through entire chain
```

**No try/catch needed for propagation!**

### Error Information

Faulted variables carry error information:

```polyglot
[r] |RiskyOperation
[>] >result:pg.string >> .data
// If faults, .result carries:
// - .message: Error description
// - .code: Error code (if applicable
// - .trace: Stack trace
```

### Handling Faulted Variables

**1. Error Handler Blocks:**

```polyglot
[!] .network_error
[<] <error_type: !NetworkError

[r] |HttpGet
[<] <url:pg.string
[>] >data:pg.string >> .response

[b] .network_error
    // Handle network errors
    [r] |CacheGet
    [<] <url:pg.string
    [>] >data:pg.string >> .cached
[/]
```

**2. Error Coalescing:**

```polyglot
// Use .primary if Final, otherwise .fallback
[r] .result:pg.string << .primary !? .fallback
```

**3. Error Checking:**

```polyglot
// Check if variable is Faulted
[r] .is_error:pg.bool << .result !?
```

## State 5: Cleared

A variable is **Cleared** when the pipeline scope ends and all variables are freed from memory. This is the **terminal state** - no transitions are possible after Cleared.

### How Variables Become Cleared

**All variables automatically transition to Cleared when the pipeline ends:**

```polyglot
[|] |ProcessData
[i] .input:pg.string
[t] |T.Manual

[r] .temp1:pg.string << .input + "A"   // temp1 created
[r] .temp2:pg.string << .temp1 + "B"   // temp2 created
[r] .result:pg.string << .temp2 + "C"  // result created

[o] .result:pg.string
[X]
// Pipeline ends - ALL variables → Cleared:
// - .input → Cleared (freed
// - .temp1 → Cleared (freed
// - .temp2 → Cleared (freed
// - .result → Cleared (freed
```

### The |W.Polyglot.Scope Wrapper

Every pipeline has an **implicit** `|W.Polyglot.Scope` wrapper that manages variable lifecycle:

```polyglot
[|] |Example
[i] .input:pg.string
[t] |T.Manual
// [W] |W.Polyglot.Scope ← IMPLICIT! Always present

[r] .var1:pg.int << 10     // var1 born
[r] .var2:pg.int << 20     // var2 born

[o] .sum:pg.int << U.Int.Add"{.var1, .var2"
[X]
// Scope cleanup triggered by |W.Polyglot.Scope:
// All variables (.input, .var1, .var2, .sum → Cleared
```

### Cleared State Rules

- **Terminal state** - no transitions possible from Cleared
- **Memory freed** - variable data is deallocated
- **Cannot be accessed** - any access after Cleared is a runtime error
- **Happens automatically** - no manual cleanup needed
- **Applies to all states** - Final, Faulted, Pending all → Cleared

**Accessing Cleared Variables:**

```polyglot
[|] |Example
[i] .value:pg.int << 42
[t] |T.Manual
[o] .value:pg.int
[X]
// After [X], .value is Cleared

// This would be an ERROR if attempted:
// .result << U.Int.Add"{.value, 10"  // RuntimeError: Variable accessed after scope cleanup
```

### Nested Block Scopes

Variables created within conditional blocks `[~]` are scoped to those blocks and transition to Cleared when exiting:

```polyglot
[|] |AccessControl
[i] .age:pg.uint
[o] .message:pg.string
[t] |T.Call
[W] |W.Polyglot.Scope  // Explicit intent: no [\] and [/] blocks needed

[?] .age <<? 18  // If age less than 18
[~][r] .msg1:pg.string << "Minor: Not allowed access"
[~][r] U.Log.Info"{.msg1"
[~][o] .message:pg.string << .msg1
// .msg1 now out of scope → Cleared state

[?] .age >>=? 18  // If age greater than or equal to 18
[~][r] .msg2:pg.string << "Adult: Access granted"
[~][r] U.Log.Info"{.msg2"
[~][o] .message:pg.string << .msg2
// .msg2 now out of scope → Cleared state

[X]
// After pipeline ends, .age and .message → Cleared
```

**Key Points:**
- `[W] |W.Polyglot.Scope` explicitly declares that `[\]` and `[/]` are intentionally omitted
- Variables created in `[~]` blocks exist only within that block
- When exiting `[~]`, those variables transition to **Cleared** state
- Input and output variables persist until pipeline `[X]`

## The Push/Pull Operator `<<`

The `<<` operator is a **bidirectional push/pull operator** that pulls data from the source (right and pushes it to the destination (left.

**Operation:** `destination << source`
- **Pull:** Data is pulled from the source (right side
- **Push:** Data is pushed to the destination (left side

### Basic Push/Pull Left `<<`

```polyglot
[r] .result: TypeName << expression
```

**Behavior:**
1. **Pull:** Evaluates the source expression (right-hand side
2. **Push:** Pushes result to destination variable (left-hand side
3. Variable transitions: Pending → Final (or stays Pending if async
4. Returns immediately without waiting

### Push/Pull Left `<<` Examples

**Literals (Immediately Final:**

```polyglot
[r] .number:pg.int << 42           // Pull 42, push to .number → Final
[r] .text:pg.string << "Hello"     // Pull "Hello", push to .text → Final
[r] .flag:pg.bool << #True         // Pull #True, push to .flag → Final
```

**Expressions (Pull from computation, push to variable:**

```polyglot
[r] .a:pg.int << 10                // Pull 10, push to .a → Final
[r] .b:pg.int << 20                // Pull 20, push to .b → Final
[r] .sum:pg.int << U.Int.Add"{.a, .b"  // Pull sum result, push to .sum → Final

[r] |AsyncOperation
[>] >c:pg.int >> .result       // Async: .result → Pending
[r] .d:pg.int << U.Int.Add"{.c, 10"  // Pull .c (waits, push to .d
```

**Variable to Variable:**

```polyglot
[r] .source:pg.string << "data"    // Pull "data", push to .source
[r] .copy:pg.string << .source     // Pull from .source, push to .copy
```

### Push/Pull Left `<<` Never Waits

The `<<` operator returns immediately, even if source becomes Pending:

```polyglot
[r] |VerySlowProcess
[>] >slow:pg.string >> .data   // Returns instantly, .data → Pending
// Execution continues while pipeline runs

[r] |OtherWork
[>] >other_work:pg.string >> .result  // Runs in parallel

// Waiting only happens when Pending variable is used:
[r] .result:pg.string << .slow + "10"  // Pulls .slow (waits if Pending, pushes to .result
```

## The Push/Pull Operator `>>`

The `>>` operator is a **bidirectional push/pull operator** that pulls data from the source (left and pushes it to the destination (right.

**Operation:** `source >> destination`
- **Pull:** Data is pulled from the source (left side
- **Push:** Data is pushed to the destination (right side

**Primary Use:** Pipeline outputs to caller scope variables

### Basic Push/Pull Right `>>`

```polyglot
[>] >pipeline_output: TypeName >> .target_variable
```

**Behavior:**
1. **Pull:** Pipeline output `>pipeline_output` provides data (source
2. **Push:** Data pushed to `.target_variable` in caller scope (destination
3. If pipeline is async, `.target_variable` becomes Pending
4. When pipeline completes, `.target_variable` becomes Final or Faulted
5. Automatic waiting when `.target_variable` is used

### Push/Pull Right `>>` Examples

**Pipeline Output (Pull from pipeline, push to caller:**

```polyglot
[r] |HttpGet
[<] <url:pg.string << "https://api.example.com"
[>] >response:pg.dict >> .http_response
// Pull from >response (pipeline output, push to .http_response (caller
// .http_response is Pending until |HttpGet completes
```

**Accessing Data:**

```polyglot
[r] |HttpGet
[<] <url:pg.string
[>] >response:pg.dict >> .http_data
// Pull from pipeline output, push to .http_data

// Use stdlib utilities - pull from .http_data, push to destination
[r] .status:pg.int << U.Dict.Get"{.http_data, 'status_code'"
[r] .body:pg.string << U.Dict.Get"{.http_data, 'body'"
```

**Pipeline Chaining (Multiple push/pull operations:**

```polyglot
[r] |HttpGet
[<] <url:pg.string
[>] >response:pg.dict >> .http_response
// Pull from >response, push to .http_response

[r] |ParseJson
[<] <json_string:pg.string << .http_response
// Pull from .http_response, push to <json_string
[>] >parsed:pg.dict >> .json_data
// Pull from >parsed, push to .json_data

[r] |ExtractUser
[<] <data:pg.dict << .json_data
// Pull from .json_data, push to <data
[>] >user:pg.dict >> .user_data
// Pull from >user, push to .user_data
```

### Push/Pull Right `>>` Waits Efficiently

Pipeline outputs wait efficiently using async I/O:

```polyglot
[r] |ExpensiveOperation
[>] >result:pg.string >> .data
// Pull from >result (pipeline, push to .data (caller → Pending

// ... other code ...

[r] .value:pg.string << .data + " processed"
// Pull from .data (waits if Pending, push to .value
// Efficient async wait (not busy loop
```

The runtime uses async I/O primitives, not sleep loops.

## The DEFAULT Push/Pull Operators `<~` and `~>`

The DEFAULT operators are **bidirectional push/pull operators with default values** that can be overridden once.

### Default Push/Pull Left `<~`

**Operation:** `destination <~ source`
- **Pull:** Default value pulled from source (right side
- **Push:** Default pushed to destination (left side
- **Override:** Can be overridden once before becoming Final

```polyglot
.variable: type <~ default_value
```

Creates variable in **Default** state with a default value that can be overridden once.

**Example:**

```polyglot
[|] |Example
[i] .timeout:pg.int <~ 30
// Pull 30 (default, push to .timeout → Default state
[t] |T.Manual

[r] .timeout << 60
// Override: Pull 60, push to .timeout → Default → Final (value: 60

[r] |Fetch
[<] <url:pg.string
[<] <timeout:pg.int << .timeout
// Pull from .timeout (60, push to <timeout
[>] >result:pg.string >> .data
```

### Default Push/Pull Right `~>`

**Operation:** `source ~> destination`
- **Pull:** Default value pulled from source (left side
- **Push:** Default pushed to destination (right side
- **Override:** Can be overridden once before becoming Final

```polyglot
[>] >output: type ~> .fallback
```

Creates pipeline output with default fallback value pulled from `.fallback`.

**Example:**

```polyglot
[|] |MaybeProcess
[i] .data:pg.string
[o] .result:pg.string
[t] |T.Manual
[W] |W.Polyglot.Scope

[?] .data ==? ""
[~][o] .result:pg.string ~> .default_message
// Pull from .default_message, push to >result with default
// If caller doesn't provide, uses default

[?] *?
[~][o] .result:pg.string << .data
// Pull from .data, push to >result

[X]
```

## State Transitions Summary

| Transition              | How                       | When                                |
| ----------------------- | ------------------------- | ----------------------------------- |
| **→ Pending**           | Declaration without value | Variable declared: `.var: type`     |
| **→ Default**           | Default assignment        | `<~` or `~>` operators              |
| **→ Final**             | Direct assignment         | `<<` with literal/sync value        |
| **Pending → Final**     | Assignment or computation | `<<` operator or async completes    |
| **Pending → Faulted**   | Computation fails         | Error occurs during async operation |
| **Default → Final**     | Override or first use     | `<<` override or used in expression |
| **Final → Cleared**     | Pipeline ends             | Scope cleanup via `[X]`             |
| **Faulted → Cleared**   | Pipeline ends             | Scope cleanup via `[X]`             |
| **Pending → Cleared**   | Pipeline ends             | Scope cleanup via `[X]`             |
| **Any State → Cleared** | Pipeline ends `[X]`       | Automatic scope cleanup             |

## Automatic Waiting Rules

The runtime automatically waits when:

1. **Using a variable:**
   ```polyglot
   .result:pg.int << U.Int.Add"{.pending_var, 10"  // Waits for pending_var
   ```

2. **Using Pending variables:**
   ```polyglot
   .combined:pg.string << .pending_var + " suffix"  // Waits
   ```

3. **Pipeline inputs:**
   ```polyglot
   [r] |ProcessData
   [<] <input:pg.string << .pending_var  // Waits
   [>] >output:pg.string >> .result
   ```

4. **Pipeline calls:**
   ```polyglot
   [r] |SomePipeline
   [<] <data:pg.string << .pending_var  // Waits
   [>] >result:pg.string >> .output
   ```

The runtime **never waits** when:

1. **PUSH operator:**
   ```polyglot
   [r] |AsyncOperation
   [>] >var:pg.int >> .result  // Doesn't wait
   ```

2. **Variable declaration:**
   ```polyglot
   .var:pg.int  // Doesn't wait
   ```

## Error Propagation Example

```polyglot
[|] |ErrorChain
[i] .url:pg.string
[o] .result:pg.int
[t] |T.Manual

// Step 1: Might fault (network error
[r] |HttpGet
[<] <url:pg.string
[>] >response:pg.dict >> .http_response  // Could become Faulted

// Step 2: Parse JSON from response
[r] |ParseJson
[<] <input:pg.dict << .http_response
[>] >parsed:pg.dict >> .json
// If .http_response is Faulted, .json becomes Faulted

// Step 3: Extract data field
[r] .data:pg.dict << U.Dict.Get"{.json, 'data'"
// If .json is Faulted, .data becomes Faulted

// Step 4: Get length
[r] .length:pg.int << U.Dict.Count"{.data"
// If .data is Faulted, .length becomes Faulted

// Step 5: Assign to result
[r] .result:pg.int << .length
// If .length is Faulted, .result becomes Faulted

[X]
// Entire chain faults if any step fails!
```

**To handle errors:**

```polyglot
[|] |ErrorChainWithHandling
[i] .url:pg.string
[o] .result:pg.int
[t] |T.Manual

[!] .handle_network
[<] <error_type: !NetworkError

[r] |HttpGet
[<] <url:pg.string
[>] >response:pg.dict >> .http_response

[r] |ParseJson
[<] <input:pg.dict << .http_response
[>] >parsed:pg.dict >> .json

[r] .data:pg.dict << U.Dict.Get"{.json, 'data'"
[r] .length:pg.int << U.Dict.Count"{.data"
[r] .result:pg.int << .length

[b] .handle_network
    // Error occurred - use fallback
    .result:pg.int << 0
[/]

[X]
```

## Performance Implications

### Lazy Evaluation

Variables only compute when needed:

```polyglot
[r] |VeryExpensiveOperation
[>] >expensive:pg.string >> .result

// If condition is false, .expensive never runs
[r] .result:pg.string << .use_expensive ? .expensive : .cheap
```

### Parallel Execution

Independent Pending variables run in parallel:

```polyglot
// All run simultaneously
[r] |Fetch
[<] <url:pg.string << .url1
[>] >a:pg.string >> .result_a

[r] |Fetch
[<] <url:pg.string << .url2
[>] >b:pg.string >> .result_b

[r] |Fetch
[<] <url:pg.string << .url3
[>] >c:pg.string >> .result_c

// Wait for all
[r] .combined:pg.array << [.a, .b, .c]
```

### Caching

Final variables cache their value:

```polyglot
[r] |ExpensiveComputation
[>] >value:pg.int >> .computed

// Computation runs once, value is cached in .computed
[r] .use1:pg.int << U.Int.Add"{.computed, 10"
[r] .use2:pg.int << U.Int.Multiply"{.computed, 2"
[r] .use3:pg.string << U.Int.ToString"{.computed"
```

## Common Patterns

### Pattern 1: Parallel Fan-Out

```polyglot
// Start many operations in parallel through unpack
[p] ~ForEach
[<] .urls
[>] .url
[~][r] |Fetch
[~][<] <url:pg.string
[~][>] >result:pg.string >> .fetched
[~]
// All fetches run in parallel, collected at join
```

### Pattern 2: Sequential Pipeline

```polyglot
[r] |Process
[<] <input:pg.string
[>] >step1:pg.string >> .processed

[r] |Transform
[<] <data:pg.string << .step1
[>] >step2:pg.string >> .transformed

[r] |Finalize
[<] <data:pg.string << .step2
[>] >step3:pg.string >> .final
// Each step waits for previous
```

### Pattern 3: Conditional Computation

```polyglot
[r] .use_cache:pg.bool << #True
[r] .result:pg.string << .use_cache
    ? .cached_value
    : .fetched_value
// Only runs the selected branch
```

### Pattern 4: Error Fallback

```polyglot
[r] |FetchPrimary
[>] >primary:pg.string >> .primary_result

[r] |FetchFallback
[>] >fallback:pg.string >> .fallback_result

[r] .result:pg.string << .primary !? .fallback
// Use primary if Final, fallback if Faulted
```

## Debugging State Transitions

### State Inspection (Development

```polyglot
// Check state at runtime (development mode
[r] .state:pg.string << .variable@state  // Returns "Declared" | "Pending" | "Final" | "Faulted"
```

### Logging State Changes

```polyglot
// Log when variable becomes Final
[r] |Operation
[>] >value:pg.string >> .result

[r] |LogInfo
[<] <message:pg.string << "Value computed: {.value"  // Waits for .value
```

## Best Practices

1. **Don't Over-Sequence:** Let independent operations run in parallel
   ```polyglot
   // Good - parallel
   [r] |Fetch
   [<] <url:pg.string << .url1
   [>] >a:pg.string >> .result_a

   [r] |Fetch
   [<] <url:pg.string << .url2
   [>] >b:pg.string >> .result_b

   // Bad - sequential
   [r] |Fetch
   [<] <url:pg.string << .url1
   [>] >a:pg.string >> .result_a
   .temp:pg.string << .a  // Unnecessary wait
   [r] |Fetch
   [<] <url:pg.string << .url2
   [>] >b:pg.string >> .result_b
   ```

2. **Trust Automatic Waiting:** Don't manually synchronize
   ```polyglot
   // Good - automatic
   [r] |AsyncOperation
   [>] >result:pg.int >> .computed
   .value:pg.int << U.Int.Add"{.result, 10"

   // Bad - manual (don't do this
   [r] |AsyncOperation
   [>] >result:pg.int >> .computed
   [r] |Sleep
   [<] <seconds:pg.int << 5  // DON'T DO THIS
   .value:pg.int << U.Int.Add"{.result, 10"
   ```

3. **Use stdlib utilities for data access:** Be explicit
   ```polyglot
   // Good - use stdlib utilities
   .status:pg.int << U.Dict.Get"{.response, 'status_code'"

   // Avoid - dot notation doesn't work for complex access
   // .status:pg.int << .response.status_code  // NOT VALID
   ```

4. **Handle Expected Errors:** Use error handlers for recoverable errors
   ```polyglot
   [!] .handle_404
   [<] <error_type: !HTTPError
   [<] <status: 404

   [r] |Fetch
   [<] <url:pg.string
   [>] >data:pg.string >> .result

   [b] .handle_404
       [r] |UseDefaultData
       [>] >data:pg.string >> .default_result
   [/]
   ```

## Next Steps

- **Practice:** Try [Getting Started](getting-started.md examples to see states in action
- **Patterns:** Read [Async-Centric Language](async-centric-language.md for high-level patterns
- **Advanced:** Explore [Parallel Execution](advanced/parallel-execution.md for explicit parallelism
- **Errors:** Learn [Error Handling](syntax/error-handling.md for comprehensive error patterns

The state system is Polyglot's foundation. Master it, and async orchestration becomes second nature.
