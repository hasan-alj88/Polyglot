# Error Handling Basics

**Version:** v0.0.4
**Category:** Language Guide
**Audience:** Intermediate users
**Status:** ⚠️ Partial (some features not fully verified)

---

## Overview

Polyglot provides multiple mechanisms for handling errors:
1. **Variable faulted state** - Detected via `.var_state`
2. **Error output parameters** - Using `!ErrorType`
3. **Fork patterns** - Conditional error handling
4. **Error blocks** `[!]` - ⚠️ Syntax not fully verified

---

## Error Types

### Error Type Prefix `!`

```polyglot
!NoError
!RuntimeError.Python
!RuntimeError.Rust
!RuntimeErrors.Both
!NetworkError
!FileNotFound
```

**Pattern:**
- `!` prefix indicates error type
- Dot notation for namespacing
- Can be custom or reserved

---

## Variable Faulted State

### The Five States

Every variable has one of five states:
1. **Declared** - Type set, no value
2. **Default** - Value with `<~`/`~>`, can override once
3. **Final** - Value with `<<`/`>>`, immutable
4. **Faulted** - Pull source encountered error
5. **Released** - Out of scope

### Detecting Faulted State

**Using reserved `.var_state` field:**

```polyglot
[f] $result.var_state =? #True
   [r] // Variable is faulted
   [r] >error << !OperationFailed
{x}
```

**How it works:**
- All variables have `.var_state` reserved field
- Comparison `=? #True` checks if faulted
- Used for explicit state checking

**Note:** Direct state checking is possible but **not recommended**. Prefer `[!]` error blocks (see below).

---

## Output Error Parameters

### Declaration

```polyglot
[|] >error <~ !NoError
```

**Pattern:**
- `>error` - Output parameter (the `>` means output)
- `<~` - Default state (can be updated)
- `!NoError` - Default error type (no error)

### Updating Error Output

```polyglot
[r] >error << !RuntimeError.Python
```

**Pattern:**
- `>error` - Output parameter name
- `<<` - Final assignment
- `!RuntimeError.Python` - Specific error type

### Example Pattern

```polyglot
{|} |ProcessData
[|] <input :pg.string
[|] >result :pg.string
[|] >error <~ !NoError

[r] $processed :pg.string << |U.Process"{$input}"

[f] $processed.var_state =? #True
   [r] >error << !ProcessingFailed
{x}

[f] *?
   [r] >result << $processed
   [r] >error << !NoError
{x}
{x}
```

---

## Fork-Based Error Handling

### Pattern: Check Success/Failure

```polyglot
[r] $success :pg.bool << |U.Operation""

[f] $success =? #False
   [r] >error << !OperationFailed
   [r] >result << "Operation failed"
{x}

[f] $success =? #True
   [r] >error << !NoError
   [r] >result << "Operation succeeded"
{x}
```

### Pattern: Multiple Error Cases

```polyglot
[f] $python_ok =? #False
[&] $rust_ok =? #True
   [r] >error << !RuntimeError.Python
{x}

[f] $python_ok =? #True
[&] $rust_ok =? #False
   [r] >error << !RuntimeError.Rust
{x}

[f] $python_ok =? #False
[&] $rust_ok =? #False
   [r] >error << !RuntimeErrors.Both
{x}

[f] *?
   [r] >error << !NoError
{x}
```

### Pattern: Cascading Error Checks

```polyglot
[r] $step1 :pg.bool << |U.Step1""

[f] $step1 =? #False
   [r] >error << !Step1Failed
{x}

[f] $step1 =? #True
   [r] $step2 :pg.bool << |U.Step2""
   [f] $step2 =? #False
      [r] >error << !Step2Failed
   {x}
   [f] $step2 =? #True
      [r] $step3 :pg.bool << |U.Step3""
      [f] $step3 =? #False
         [r] >error << !Step3Failed
      {x}
      [f] *?
         [r] >error << !NoError
      {x}
   {x}
{x}
```

---

## Error Blocks `[!]` ⚠️

### Recommended Approach

According to training, the **recommended** way to handle errors is using `[!]` error blocks:

```polyglot
[!] !Error.To.Handle
   [r] // Handle specific error
{x}
```

**⚠️ Note:** Full syntax for `[!]` blocks not yet verified by human trainer. The above is based on mention during training. Use with caution until syntax confirmed.

### Conceptual Usage

```polyglot
// Conceptual example (syntax needs verification)
[r] $result :pg.serial << |U.RiskyOperation""

[!] !NetworkError
   [r] // Handle network error specifically
   [r] >error << !NetworkError
{x}

[!] !TimeoutError
   [r] // Handle timeout specifically
   [r] >error << !TimeoutError
{x}

[!] *?
   [r] // Handle any other error
   [r] >error << !UnknownError
{x}
```

**Status:** ⚠️ This section requires verification with human trainer for exact syntax.

---

## Complete Error Handling Example

### Multi-Runtime with Error Tracking

```polyglot
{@} @Local::Examples.ErrorHandling:1.0.0.0
{x}



{|} |ErrorHandlingExample
[%] %Doc "Demonstrates comprehensive error handling"

[t] |T.CLI"errortest"

[|] <simulate_error :pg.bool <~ #False
[|] >error <~ !NoError
[|] >success_count :pg.int

[w] |W.RT.Python3.9
(|) <requirements:pg.path << \\NoPath\\
(|) >env:pg.serial >> $py

[w] |W.RT.Rust
(|) <dependencies:pg.path << \\NoPath\\
(|) >env:pg.serial >> $rust

[r] |U.RT.Python.Code
(|) <env:pg.serial << $py
(|) <kwargs.should_fail:py.bool << $simulate_error
(|) <code:pg.string << ""
[+] +"def test(should_fail):"
[+] -"    if should_fail:"
[+] -"        raise Exception('Python error')"
[+] -"    return True"
[+] -""
[+] +"try:"
[+] -"    test(should_fail)"
[+] +"except:"
[+] -"    pass"
(|) >result:pg.bool >> $py_ok

[r] |U.RT.Rust.Code
(|) <env:pg.serial << $rust
(|) <kwargs.should_fail:rust.bool << $simulate_error
(|) <code:pg.string << ""
[+] +"fn main() {"
[+] -"    if should_fail {"
[+] -"        panic!(\"Rust error\");"
[+] -"    }"
[+] +"}"
(|) >result:pg.bool >> $rust_ok

[r] $results:pg.array.pg.bool <<{
[+]    $py_ok,
[+]    $rust_ok,
[+] }

[r] |U.Math.Sum.Bool
(|) <array:pg.array.pg.bool << $results
(|) >sum >> >success_count

[f] $py_ok =? #False
[&] $rust_ok =? #True
   [r] >error << !RuntimeError.Python
{x}

[f] $py_ok =? #True
[&] $rust_ok =? #False
   [r] >error << !RuntimeError.Rust
{x}

[f] $py_ok =? #False
[&] $rust_ok =? #False
   [r] >error << !RuntimeErrors.Both
{x}

[f] *?
   [r] >error << !NoError
{x}
{x}
```

**Usage:**
```bash
# All succeed
polyglot run errortest

# Simulate errors
polyglot run errortest --simulate_error=true
```

---

## Error Type Conventions

### Naming Patterns

**Reserved errors:**
```polyglot
!NoError              // Success, no error
!UnknownError         // Catch-all error
```

**Runtime errors:**
```polyglot
!RuntimeError.Python
!RuntimeError.Rust
!RuntimeError.JS
!RuntimeErrors.Both   // Multiple failed
```

**System errors:**
```polyglot
!FileNotFound
!PermissionDenied
!NetworkError
!TimeoutError
```

**Custom errors:**
```polyglot
!MyApp.ValidationError
!MyApp.AuthenticationError
!MyApp.DatabaseError
```

---

## Best Practices

### ✅ 1. Always Provide Error Outputs

```polyglot
[|] >error <~ !NoError    // ✅ GOOD: Error output available
```

### ✅ 2. Use Specific Error Types

```polyglot
// ✅ GOOD: Specific
[r] >error << !RuntimeError.Python

// ❌ AVOID: Generic
[r] >error << !Error
```

### ✅ 3. Set Default to NoError

```polyglot
[|] >error <~ !NoError    // ✅ Default: success

// Not:
[|] >error <~ !UnknownError  // ❌ Default: failure?
```

### ✅ 4. Handle All Error Cases

```polyglot
[f] $result =? #True
   [r] >error << !NoError
{x}

[f] $result =? #False
   [r] >error << !Failed
{x}

// ✅ Exhaustive
```

### ✅ 5. Prefer Error Blocks Over State Checks

```polyglot
// ✅ RECOMMENDED: Use [!] blocks
[!] !NetworkError
   // Handle error
{x}

// ❌ AVOID: Direct state checking
[f] $var.var_state =? #True
   // Handle faulted
{x}
```

**Rationale:** Error blocks are cleaner and more idiomatic (once syntax verified).

---

## Common Patterns

### Pattern 1: Try-Continue

```polyglot
[r] $result :pg.bool << |U.TrySomething""

[f] $result =? #False
   // Log but continue
   [r] |U.Log"Operation failed, continuing..."
{x}

[f] *?
   // Success path
{x}
```

### Pattern 2: Try-Abort

```polyglot
[r] $result :pg.bool << |U.TrySomething""

[f] $result =? #False
   [r] >error << !CriticalError
   // Pipeline ends with error
{x}

[f] *?
   // Continue processing
{x}
```

### Pattern 3: Retry Logic (Conceptual)

```polyglot
// Note: Loop syntax not yet verified
// This is conceptual

[r] $attempts :pg.int << 0
[r] $success :pg.bool << #False

// While not success and attempts < 3
[r] $result :pg.bool << |U.TrySomething""
[f] $result =? #False
   [r] $attempts :pg.int << |U.Math.Add"{$attempts}, 1"
{x}
[f] $result =? #True
   [r] $success :pg.bool << #True
{x}
```

---

## Checking Error Outputs

### From Calling Pipeline

```polyglot
{|} |Caller
[r] $result :pg.string
[r] $error :pg.error

[r] |ProcessData
(|) <input:pg.string << "data"
(|) >result:pg.string >> $result
(|) >error >> $error

[f] $error =? !NoError
   [r] // Success
{x}

[f] *?
   [r] // Handle error
   [r] |U.Log"Error occurred: {$error}"
{x}
{x}
```

---

## Limitations & Unknowns

### ⚠️ Not Yet Verified

The following features mentioned but not fully verified:

1. **`[!]` Error Block Syntax**
   - Exact syntax unclear
   - Wildcard `[!] *?` pattern unclear
   - How errors propagate unclear

2. **Error Propagation**
   - Auto-propagation behavior unclear
   - Nested pipeline error bubbling unclear

3. **Try/Catch Equivalent**
   - No verified try/catch pattern yet

4. **Error Recovery**
   - Transitioning from faulted state unclear

### 📋 To Be Documented

- Complete `[!]` block syntax
- Error propagation rules
- Nested error handling
- Error recovery mechanisms
- Stack traces / debugging

---

## Quick Reference

```
┌──────────────────────────────────────────────┐
│ ERROR HANDLING                               │
├──────────────────────────────────────────────┤
│                                              │
│  ERROR TYPES                                 │
│  !NoError              Success               │
│  !ErrorType            Specific error        │
│  !Namespace.Error      Namespaced           │
│                                              │
│  OUTPUT PARAMETERS                           │
│  [|] >error <~ !NoError                      │
│  [r] >error << !SpecificError                │
│                                              │
│  FAULTED STATE CHECK                         │
│  [f] $var.var_state =? #True                 │
│     // Variable is faulted                   │
│  {x}                                         │
│                                              │
│  RECOMMENDED (syntax TBD)                    │
│  [!] !ErrorType                              │
│     // Handle error                          │
│  {x}                                         │
│                                              │
└──────────────────────────────────────────────┘
```

---

## See Also

- [Fork Patterns](./fork-patterns-guide.md) - Conditional error handling
- [Variable Lifecycle](./operators-reference.md) - Faulted state details
- [Operators Reference](./operators-reference.md) - Assignment operators

---

**Generated by:** Polly (Polyglot Language Expert)
**For:** User Documentation
**Version:** v0.0.4
**Last Updated:** 2025-12-26
**Confidence:** 🟡 Learning (faulted states verified, `[!]` blocks mentioned but not detailed)
**Status:** ⚠️ Partial - Needs verification of `[!]` syntax and error propagation
