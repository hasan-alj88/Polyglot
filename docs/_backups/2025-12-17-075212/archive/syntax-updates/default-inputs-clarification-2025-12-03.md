# Default Inputs Clarification - Usage Must Be Explicit

**Date:** 2025-12-03
**Type:** Conceptual Clarification
**Status:** ✅ **COMPLETE**
**Scope:** Clarified that default `[i]` inputs must be explicitly used/passed

---

## Summary

Corrected misconception that default inputs are "automatically used." Even when an `[i]` input has a default value (`<~`), it must still be **explicitly passed** or **used** in the pipeline body.

---

## The Misconception

### ❌ INCORRECT (implied automatic usage):

```polyglot
[|] |ProcessData
[i] .timeout: pg\int <~ 30  // Default value 30
[t] |T.Manual

// "No override - default is used"
[r] |FetchWithTimeout
[<] <url: pg\string
[<] <timeout: pg\int  // ❌ Not providing timeout!
[>] >result: pg\string >> .data
// "default is used" - but WHERE? HOW?
```

**Problem:** Said "default is used" but didn't show the default value being passed to `<timeout`. This implies automatic usage, which is WRONG.

---

## The Correct Understanding

### ✅ CORRECT: Default Must Be Explicitly Passed

**Pattern 1: Use default without override**

```polyglot
[|] |ProcessData
[i] .timeout: pg\int <~ 30  // Default value 30
[i] .my_url: pg\url << url"http://MyWebsite.com"
[t] |T.Manual

// Use default - explicitly pass .timeout
[r] |FetchWithTimeout
[<] <url: pg\string << .my_url
[<] <timeout: pg\int << .timeout  // ✅ Explicitly passes default (30)
[>] >result: pg\string >> .data
// .timeout = 30, transitions to Final
```

**Pattern 2: Override default, then use**

```polyglot
[|] |ProcessData
[i] .timeout: pg\int <~ 30  // Default value 30
[i] .my_url: pg\url << url"http://MyWebsite.com"
[t] |T.Manual

// Override the default first
[r] .timeout << 60  // Override: Default → Final (60)
[r] .timeout << 90  // ❌ ERROR: Already Final!

// Use the overridden value
[r] |FetchWithTimeout
[<] <url: pg\string << .my_url
[<] <timeout: pg\int << .timeout  // ✅ Explicitly passes overridden (60)
[>] >result: pg\string >> .data
```

---

## What Defaults Actually Mean

### Default Inputs: Caller Perspective

When a pipeline has default inputs:

```polyglot
[|] |ProcessData
[i] .timeout: pg\int <~ 30  // Default value 30
[t] |T.Call
```

**Caller has two options:**

**Option 1: Provide explicit value (override default)**
```polyglot
[r] |ProcessData
[<] <timeout: pg\int << 60  // Override: caller provides 60
```

**Option 2: Don't provide value (use default)**
```polyglot
[r] |ProcessData
// <timeout not provided → uses default 30
```

**Key Point:** Default means "fallback value if caller doesn't provide one," NOT "automatically used everywhere."

---

### Default Inputs: Inside Pipeline

Inside the pipeline body, the input is **just a variable** that must be explicitly used:

```polyglot
[|] |ProcessData
[i] .timeout: pg\int <~ 30
[t] |T.Call

// Must explicitly use .timeout in expressions
[r] .double: pg\int << U.Int.Multiply"{.timeout, 2}"

// Must explicitly pass .timeout to other pipelines
[r] |FetchWithTimeout
[<] <timeout: pg\int << .timeout  // ✅ Explicit usage
```

**No automatic insertion!** The pipeline doesn't magically know to use `.timeout` for every `<timeout` argument.

---

## Correct Mental Model

### Think of Default as "Optional Parameter with Fallback"

```polyglot
// Like a function with optional parameter
[|] |ProcessData(timeout: int = 30)  // Pseudo-code analogy
```

**Inside function:**
```python
def process_data(timeout=30):
    # timeout is just a variable
    fetch_with_timeout(url, timeout)  # Must explicitly pass it
```

**In Polyglot:**
```polyglot
[|] |ProcessData
[i] .timeout: pg\int <~ 30

[r] |FetchWithTimeout
[<] <timeout: pg\int << .timeout  // Must explicitly pass it
```

---

## Examples

### Example 1: Configuration with Defaults

```polyglot
[|] |WebServer
[i] .port: pg\uint <~ 8080         // Default port
[i] .host: pg\string <~ "localhost" // Default host
[t] |T.Call

// Must explicitly use inputs
[r] |StartServer
[<] <port: pg\uint << .port        // ✅ Explicit
[<] <host: pg\string << .host      // ✅ Explicit
```

---

### Example 2: Retry Logic with Default Attempts

```polyglot
[|] |RetryOperation
[i] .max_retries: pg\int <~ 3      // Default 3 retries
[i] .operation: #Operation
[t] |T.Call

// Use default in logic
[r] .counter: pg\int << 0

[?] .counter <? .max_retries       // ✅ Explicitly using .max_retries
[~][r] |ExecuteOperation
[~][<] <op: #Operation << .operation
```

---

### Example 3: Override Default Configuration

```polyglot
[|] |DatabaseConnection
[i] .timeout: pg\int <~ 30         // Default 30 seconds
[t] |T.Call

// Override for this specific connection
[r] .timeout << 120                // Override to 120 seconds

// Use overridden value
[r] |ConnectDB
[<] <timeout: pg\int << .timeout   // ✅ Uses 120, not 30
```

---

## Common Mistakes

### ❌ Mistake 1: Assuming automatic usage
```polyglot
[i] .timeout: pg\int <~ 30

[r] |SomePipeline
// Forgot to provide <timeout - assumes default is automatic
[<] <url: pg\string << .url
// ❌ Missing: [<] <timeout: pg\int << .timeout
```

---

### ❌ Mistake 2: Saying "default is used" without showing usage
```polyglot
// Documentation mistake:
// "If no override, default is used"
[r] |FetchData
[<] <url: pg\string << .url
// ❌ Where/how is default used? Not shown!
```

---

### ❌ Mistake 3: Thinking defaults propagate automatically
```polyglot
[i] .config: #Config <~ .defaultConfig

// Assuming every pipeline call gets .config automatically
[r] |Pipeline1  // ❌ Doesn't automatically get .config
[r] |Pipeline2  // ❌ Doesn't automatically get .config

// Must explicitly pass it
[r] |Pipeline1
[<] <config: #Config << .config  // ✅ Explicit
```

---

## Rule Summary

### The Rule

**Default inputs (`[i] .var: Type <~ value`) provide fallback values for callers, but inside the pipeline body, they must be explicitly used like any other variable.**

**Default means:**
- ✅ Caller can omit input (uses fallback)
- ✅ Caller can provide input (overrides fallback)
- ✅ Inside pipeline, variable has a value to work with

**Default does NOT mean:**
- ❌ Variable automatically inserted everywhere
- ❌ Implicit usage in expressions
- ❌ Automatic propagation to nested pipelines

---

## Files Updated

1. **`docs/user/variable-state-system.md`**
   - Fixed "Override-Once Behavior" example
   - Added explicit `.timeout` passing in both cases
   - Added `.my_url` input for completeness
   - Clarified Case 1 and Case 2

---

## Before/After Comparison

### Before (Incorrect):
```polyglot
// Case 1: No override - default is used
[r] |FetchWithTimeout
[<] <url: pg\string
[<] <timeout: pg\int  // ❌ Not showing where value comes from!
```

### After (Correct):
```polyglot
// Case 1: Use default (no override) - explicitly pass .timeout
[r] |FetchWithTimeout
[<] <url: pg\string << .my_url
[<] <timeout: pg\int << .timeout  // ✅ Explicitly passes default (30)
```

---

## Conclusion

**Status:** ✅ **COMPLETE**

Successfully clarified that default `[i]` inputs must be explicitly used:

**Key Clarification:**
- Default inputs provide fallback values for callers
- Inside pipeline body, defaults are regular variables
- All usage must be explicit - no automatic insertion

**Misconception Corrected:**
- "Default is used" → "Default is explicitly passed/used"

**Example Updated:**
- Added explicit `<< .timeout` in pipeline calls
- Showed both non-override and override patterns clearly

This clarification prevents confusion about automatic behavior and reinforces Polyglot's explicit data flow philosophy.

---

**Report Generated:** 2025-12-03
**Generated By:** Claude (Sonnet 4.5)
**Clarification Type:** Default Input Usage Semantics
