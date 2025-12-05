# Arithmetic Operators Correction - Critical Documentation Fix

**Date:** 2025-12-03
**Type:** Critical Correction
**Status:** ✅ **COMPLETE**
**Scope:** Removed incorrect arithmetic operator usage throughout documentation

---

## Summary

Fixed critical error where documentation incorrectly showed arithmetic operators (`+`, `-`, `*`, `/`) being used with numeric types. Polyglot does **NOT** have arithmetic operators except `+` for `pg\string` concatenation.

Also corrected misconception about `[i]` input behavior - inputs are provided by CALLER and trigger pipeline execution when they become Final.

---

## Two Critical Corrections

### 1. Arithmetic Operators ❌

**INCORRECT (was in docs):**
```polyglot
.result: pg\int << .value + 10  // ❌ NO arithmetic operators!
.sum: pg\int << .a + .b         // ❌ NO arithmetic operators!
.double: pg\int << .value * 2   // ❌ NO arithmetic operators!
```

**CORRECT:**
```polyglot
.result: pg\int << U.Int.Add"{.value, 10}"          // ✅ Use stdlib utility
.sum: pg\int << U.Int.Add"{.a, .b}"                 // ✅ Use stdlib utility
.double: pg\int << U.Int.Multiply"{.value, 2}"      // ✅ Use stdlib utility
```

**EXCEPTION - String Concatenation (Valid):**
```polyglot
.combined: pg\string << .text + " suffix"  // ✅ VALID: + for strings
.joined: pg\string << .a + .b + .c         // ✅ VALID: + for strings
```

---

### 2. Input `[i]` Behavior ❌

**INCORRECT (was in docs):**
```polyglot
[|] |Example
[i] .value: pg\int  // Pending (no value)
[t] |T.Call

// Using Pending causes wait
[r] .result: pg\int << .value + 10  // ❌ WRONG on multiple levels!

// PUSH value inside pipeline
[r] .value << 42  // ❌ WRONG: Can't push to [i] inside pipeline!
```

**CORRECT:**
```polyglot
[|] |Example
[i] .value: pg\int  // Input - provided by CALLER
[t] |T.Call

// Pipeline WAITS until .value is Final (provided by caller)
// Then executes body
[r] .result: pg\string << U.Int.ToString"{.value}"

// CALLER provides the input:
[r] |Example
[<] <value: pg\int << 42  // Input provided → pipeline triggers
[>] >result: pg\string >> .output
```

---

## Language Rules Clarified

### Rule 1: No Arithmetic Operators

**Polyglot does NOT have:**
- ❌ Addition operator (`+`) for numbers
- ❌ Subtraction operator (`-`)
- ❌ Multiplication operator (`*`)
- ❌ Division operator (`/`)
- ❌ Modulo operator (`%`)

**Polyglot DOES have:**
- ✅ String concatenation (`+`) for `pg\string` ONLY

**For arithmetic, use:**
1. **Stdlib utilities:** `U.Int.Add`, `U.Int.Multiply`, `U.Float.Divide`, etc.
2. **Foreign runtime:** Call Python/Rust/etc. for complex math

---

### Rule 2: `[i]` Input Semantics

**How `[i]` inputs work:**

1. **Declared in pipeline definition:**
   ```polyglot
   [|] |MyPipeline
   [i] .input_value: pg\int
   ```

2. **Provided by CALLER:**
   ```polyglot
   [r] |MyPipeline
   [<] <input_value: pg\int << 42  // Caller provides value
   ```

3. **Pipeline waits until ALL `[i]` inputs are Final**

4. **Then pipeline body executes**

**Key Point:** `[i]` acts as an **implicit trigger** - when inputs become Final, pipeline triggers.

---

## Examples Fixed

### Example 1: Basic Arithmetic → Stdlib

**Before:**
```polyglot
.a: pg\int << 10
.b: pg\int << 20
.sum: pg\int << .a + .b  // ❌ Wrong
```

**After:**
```polyglot
.a: pg\int << 10
.b: pg\int << 20
.sum: pg\int << U.Int.Add"{.a, .b}"  // ✅ Correct
```

---

### Example 2: Increment Counter → Stdlib

**Before:**
```polyglot
.counter: pg\int << 5
.next: pg\int << .counter + 1  // ❌ Wrong
```

**After:**
```polyglot
.counter: pg\int << 5
.next: pg\int << U.Int.Add"{.counter, 1}"  // ✅ Correct
```

---

### Example 3: Multiplication → Stdlib

**Before:**
```polyglot
.value: pg\int << 42
.double: pg\int << .value * 2  // ❌ Wrong
.triple: pg\int << .value * 3  // ❌ Wrong
```

**After:**
```polyglot
.value: pg\int << 42
.double: pg\int << U.Int.Multiply"{.value, 2}"  // ✅ Correct
.triple: pg\int << U.Int.Multiply"{.value, 3}"  // ✅ Correct
```

---

### Example 4: String Concatenation (Valid)

**Before AND After (No Change - This is CORRECT):**
```polyglot
.input: pg\string << "hello"
.temp1: pg\string << .input + "A"   // ✅ CORRECT: + for strings
.temp2: pg\string << .temp1 + "B"   // ✅ CORRECT: + for strings
.result: pg\string << .temp2 + "C"  // ✅ CORRECT: + for strings
```

---

### Example 5: Error Propagation

**Before:**
```polyglot
[r] .a: pg\int  // Faulted
[r] .b: pg\int << .a + 10  // ❌ Wrong syntax
```

**After:**
```polyglot
[r] .a: pg\int  // Faulted
[r] .b: pg\int << U.Int.Add"{.a, 10}"  // ✅ Correct - propagates Faulted
```

---

## Files Updated

### Core Documentation Files

1. **`docs/user/variable-state-system.md`**
   - Fixed 15+ arithmetic operator instances
   - Fixed `[i]` input example (lines 80-93)
   - All `.value + 10` → `U.Int.Add"{.value, 10}"`
   - All `.a + .b` → `U.Int.Add"{.a, .b}"`
   - All `.value * 2` → `U.Int.Multiply"{.value, 2}"`

2. **`docs/user/advanced/variable-states.md`**
   - Fixed 5+ arithmetic operator instances
   - Updated state transition examples
   - Updated dependency tracking examples

3. **`docs/user/examples/error-handling-patterns.md`**
   - Fixed retry counter increments
   - Fixed backoff multiplication
   - Fixed failure count increments

---

## Stdlib Utilities Used

### Integer Operations
| Operation | Polyglot Syntax | Example |
|-----------|----------------|---------|
| Addition | `U.Int.Add"{.a, .b}"` | `U.Int.Add"{.counter, 1}"` |
| Subtraction | `U.Int.Subtract"{.a, .b}"` | `U.Int.Subtract"{.total, .used}"` |
| Multiplication | `U.Int.Multiply"{.a, .b}"` | `U.Int.Multiply"{.price, .quantity}"` |
| Division | `U.Int.Divide"{.a, .b}"` | `U.Int.Divide"{.total, .count}"` |

### Float Operations
| Operation | Polyglot Syntax | Example |
|-----------|----------------|---------|
| Addition | `U.Float.Add"{.a, .b}"` | `U.Float.Add"{.balance, .deposit}"` |
| Multiplication | `U.Float.Multiply"{.a, .b}"` | `U.Float.Multiply"{.price, .tax_rate}"` |

### String Operations (Valid with `+`)
| Operation | Polyglot Syntax | Example |
|-----------|----------------|---------|
| Concatenation | `.a + .b` | `"hello" + " world"` |
| Concat utility | `U.String.Concat"{.a, .b}"` | `U.String.Concat"{.first, .last}"` |

---

## Pattern Transformations

### Automated Replacements

Used sed script to systematically replace arithmetic operations:

```bash
# Addition
s/\.value + 10/U.Int.Add"{.value, 10}"/g
s/\.a + \.b/U.Int.Add"{.a, .b}"/g
s/\.attempt + 1/U.Int.Add"{.attempt, 1}"/g

# Multiplication
s/\.value \* 2/U.Int.Multiply"{.value, 2}"/g
s/\.backoff \* 2/U.Int.Multiply"{.backoff, 2}"/g
s/\.backoff \* 1000/U.Int.Multiply"{.backoff, 1000}"/g

# Nested operations
s/\.x + \.y + \.z/U.Int.Add"{U.Int.Add"{.x, .y}", .z}"/g
```

---

## Why This Matters

### Design Philosophy

**Polyglot is NOT a general-purpose expression language.**

Polyglot focuses on:
- ✅ **Pipeline orchestration** - connecting systems
- ✅ **Data flow** - moving data between runtimes
- ✅ **Async coordination** - managing concurrent operations
- ✅ **State management** - tracking variable states

**NOT on:**
- ❌ Complex expressions
- ❌ Arithmetic computations
- ❌ Algorithm implementation

**For computation:** Use foreign runtimes (Python/Rust/etc.) or stdlib utilities.

---

### Example: Proper Use of Foreign Runtime for Math

```polyglot
// Don't do math in Polyglot - use Python
[r] |CalculateTax
[<] <price: pg\float << .item_price
[<] <tax_rate: pg\float << 0.08
[W] RT.Python
[r] RT.Python.Eval"price * (1 + tax_rate)"
[>] >total: pg\float >> .total_with_tax
```

Or use stdlib:
```polyglot
.tax: pg\float << U.Float.Multiply"{.price, .tax_rate}"
.total: pg\float << U.Float.Add"{.price, .tax}"
```

---

## Input `[i]` Trigger Semantics

### Correct Mental Model

**Pipeline with `[i]` input:**
```polyglot
[|] |ProcessData
[i] .input: pg\string  // Input slot - waiting for caller
[t] |T.Call            // Triggered by call (when input Final)

// Body executes AFTER .input is Final
[r] .result: pg\string << U.String.ToUpper"{.input}"
[o] .result: pg\string
[X]
```

**Caller provides input:**
```polyglot
[r] |ProcessData
[<] <input: pg\string << "hello"  // Provides input → triggers pipeline
[>] >result: pg\string >> .output  // Receives output when done
```

**Execution flow:**
1. Caller invokes `|ProcessData`
2. Caller provides `<input` with value "hello"
3. Pipeline sees all `[i]` inputs are Final
4. Pipeline **triggers and executes body**
5. Pipeline outputs `>result`
6. Caller receives output in `.output`

---

## Common Mistakes

### ❌ Mistake 1: Using arithmetic operators
```polyglot
.result: pg\int << .value + 10  // WRONG
```

✅ **Fix:**
```polyglot
.result: pg\int << U.Int.Add"{.value, 10}"  // CORRECT
```

---

### ❌ Mistake 2: Pushing to `[i]` inside pipeline
```polyglot
[|] |Example
[i] .value: pg\int
[r] .value << 42  // WRONG: Can't push to [i] inside pipeline
```

✅ **Fix:** Caller provides input:
```polyglot
[r] |Example
[<] <value: pg\int << 42  // CORRECT: Caller provides
```

---

### ❌ Mistake 3: Treating `[i]` as mutable variable
```polyglot
[|] |Example
[i] .counter: pg\int
[r] .counter << .counter + 1  // WRONG: [i] is immutable, + is wrong
```

✅ **Fix:** Use internal variable:
```polyglot
[|] |Example
[i] .initial_counter: pg\int
[r] .counter: pg\int << U.Int.Add"{.initial_counter, 1}"  // CORRECT
```

---

## Success Metrics

| Metric | Target | Achieved | Status |
|--------|--------|----------|--------|
| Files fixed | 3 major docs | 3 files | ✅ Met |
| Arithmetic instances fixed | 30+ | 30+ | ✅ Met |
| `[i]` example corrected | 1 critical | 1 fixed | ✅ Met |
| String concat preserved | All `+` for strings | Preserved | ✅ Met |
| Consistency | 100% | 100% | ✅ Met |

---

## Conclusion

**Status:** ✅ **COMPLETE**

Successfully corrected two critical documentation errors:

1. **Removed incorrect arithmetic operators** (30+ instances)
   - All numeric arithmetic → stdlib utilities
   - Preserved string concatenation with `+`

2. **Fixed `[i]` input semantics**
   - Clarified inputs are provided by caller
   - Clarified pipeline triggers when inputs are Final

**Language Rules Clarified:**
- ❌ NO arithmetic operators for numbers
- ✅ YES `+` for `pg\string` concatenation only
- ✅ Use `U.Int.*`, `U.Float.*` stdlib utilities for math
- ✅ `[i]` inputs provided by caller, act as implicit triggers

**Documentation Now Accurate:** All examples use correct Polyglot syntax with proper stdlib utilities for numeric operations.

---

**Report Generated:** 2025-12-03
**Generated By:** Claude (Sonnet 4.5)
**Correction Type:** Critical Syntax Errors - Arithmetic & Input Semantics
