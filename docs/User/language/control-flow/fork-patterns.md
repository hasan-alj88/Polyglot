# Fork Patterns and Exhaustiveness Guide

**Version:** v0.0.4
**Category:** Language Reference
**Audience:** Intermediate users
**Prerequisites:** Understanding of operators and basic syntax

---

## Overview

Fork patterns (`[f]` marker) provide conditional execution in Polyglot. Unlike traditional `if/else` statements, Polyglot's fork system enforces **exhaustiveness at compile time**, ensuring all possible cases are handled.

---

## Basic Fork Syntax

### Simple Comparison

```polyglot
[f] $variable =? value
   [r] // Execute if condition matches
{x}
```

**Pattern:**
- `[f]` - Fork marker
- `$variable` - Variable to check
- `=?` - Equality comparison operator
- `value` - Value to compare against

**Example:**
```polyglot
[f] $status =? #True
   [r] $message :pg.string << "Active"
{x}
```

---

## Comparison Operators

### Equality `=?`

```polyglot
[f] $flag =? #True
   [r] // When flag equals true
{x}

[f] $count =? 0
   [r] // When count equals zero
{x}

[f] $name =? "Alice"
   [r] // When name equals "Alice"
{x}
```

### Inequality (Negation)

```polyglot
// Check if NOT equal by handling other case
[f] $flag =? #True
   [r] // Handle true
{x}

[f] $flag =? #False
   [r] // Handle false (not true)
{x}
```

**Note:** Direct `!=` operator not shown in training. Use exhaustive patterns instead.

---

## Logical AND Operator

### Combining Conditions with `[&]`

```polyglot
[f] $conditionA =? #True
[&] $conditionB =? #True
   [r] // Execute only if BOTH are true
{x}
```

**How it works:**
- `[f]` starts the condition
- `[&]` continues with AND logic
- Both conditions must match for block to execute

**Example:**
```polyglot
[f] $python_success =? #True
[&] $rust_success =? #True
   [r] $message :pg.string << "Both succeeded!"
{x}
```

### Multiple AND Conditions

```polyglot
[f] $a =? #True
[&] $b =? #True
[&] $c =? #True
   [r] // All three must be true
{x}
```

---

## Exhaustiveness Requirement

### The Critical Rule

**All fork conditions must be exhaustive or you get a compile error.**

❌ **This will NOT compile:**
```polyglot
[f] $flag =? #True
   [r] // Handle true case
{x}

// ❌ COMPILE ERROR: What happens when $flag is #False?
```

✅ **This WILL compile:**
```polyglot
[f] $flag =? #True
   [r] // Handle true case
{x}

[f] $flag =? #False
   [r] // Handle false case
{x}

// ✅ Compiles - all cases covered
```

---

## Wildcard Fork `[f] *?`

### Purpose

The wildcard `[f] *?` handles "all other cases" to ensure exhaustiveness.

**It's NOT just convenience - it's a safety mechanism!**

### Usage

```polyglot
[f] $status =? #Status.Success
   [r] // Handle success
{x}

[f] $status =? #Status.Failure
   [r] // Handle failure
{x}

[f] *?
   [r] // Handle any other status value
{x}
```

### When Wildcard is Required

**Case 1: Unknown enum values**
```polyglot
[f] $result =? #True
   [r] // ...
{x}

[f] $result =? #False
   [r] // ...
{x}

// If compiler can't prove these are only values, need:
[f] *?
   [r] |U.Do.Nothing""
{x}
```

**Case 2: Large value space**
```polyglot
[f] $count =? 0
   [r] // Handle zero
{x}

[f] $count =? 1
   [r] // Handle one
{x}

[f] *?
   [r] // Handle all other numbers
{x}
```

---

## Common Patterns

### Boolean Fork

```polyglot
[f] $success =? #True
   [r] "Operation succeeded" >> >message
{x}

[f] $success =? #False
   [r] "Operation failed" >> >message
{x}
```

### Cascading Success Checks

```polyglot
[f] $step1 =? #True
   [r] $step2 :pg.bool << |U.Process.Step2""
   [f] $step2 =? #True
      [r] $step3 :pg.bool << |U.Process.Step3""
      [f] $step3 =? #True
         [r] >result << "All steps succeeded"
      {x}
   {x}
{x}

[f] *?
   [r] >result << "A step failed"
{x}
```

### Error Handling Pattern

```polyglot
[f] $python_result =? #False
[&] $rust_result =? #True
   [r] >error << !RuntimeError.Python
{x}

[f] $python_result =? #True
[&] $rust_result =? #False
   [r] >error << !RuntimeError.Rust
{x}

[f] $python_result =? #False
[&] $rust_result =? #False
   [r] >error << !RuntimeErrors.Both
{x}

[f] *?
   [r] >error << !NoError
{x}
```

### State Machine Pattern

```polyglot
[f] $state =? #State.Pending
   [r] $state :pg.serial << |U.Process.Pending""
{x}

[f] $state =? #State.Processing
   [r] $state :pg.serial << |U.Process.Next""
{x}

[f] $state =? #State.Complete
   [r] |U.Finalize""
{x}

[f] *?
   [r] >error << !UnknownState
{x}
```

---

## No-Op Pattern

### Using `|U.Do.Nothing`

When a case requires no action, use explicit no-op:

```polyglot
[f] $flag =? #True
   [r] |U.Process.Something""
{x}

[f] $flag =? #False
   [r] |U.Do.Nothing""    // Explicit: do nothing
{x}
```

**Why explicit?**
- Makes intent clear
- Satisfies exhaustiveness
- Self-documenting code

---

## Advanced Patterns

### Nested Forks

```polyglot
[f] $category =? #Category.A
   [f] $priority =? #High
      [r] |U.Process.HighPriorityA""
   {x}
   [f] $priority =? #Low
      [r] |U.Process.LowPriorityA""
   {x}
   [f] *?
      [r] |U.Do.Nothing""
   {x}
{x}

[f] $category =? #Category.B
   // ... similar structure
{x}

[f] *?
   [r] |U.Do.Nothing""
{x}
```

### Multiple Variable Checks (AND)

```polyglot
[f] $enabled =? #True
[&] $authenticated =? #True
[&] $authorized =? #True
   [r] |U.Process.Request""
{x}

[f] *?
   [r] >error << !AccessDenied
{x}
```

---

## Exhaustiveness Checking

### How the Compiler Checks

The Polyglot compiler analyzes:
1. **Type of variable** being forked on
2. **All possible values** for that type
3. **Cases you've covered** with `[f]` blocks
4. **Whether a wildcard `[f] *?` exists**

### Boolean Type (Fully Known)

```polyglot
// Compiler knows :pg.bool has exactly 2 values

[f] $flag =? #True
   // ...
{x}

[f] $flag =? #False
   // ...
{x}

// ✅ Exhaustive - no wildcard needed
```

### Integer Type (Infinite Values)

```polyglot
// Compiler knows :pg.int has infinite values

[f] $count =? 0
   // ...
{x}

// ❌ NOT exhaustive - missing all other integers!

[f] *?
   // ...
{x}

// ✅ Now exhaustive
```

### Custom Enums

```polyglot
// If enum is closed (compiler knows all values)
[f] $status =? #Status.Success
{x}
[f] $status =? #Status.Failure
{x}
// ✅ May be exhaustive if only 2 values

// If enum is open (could have more values)
[f] $status =? #Status.Success
{x}
[f] $status =? #Status.Failure
{x}
[f] *?
{x}
// ✅ Exhaustive with wildcard
```

---

## Common Mistakes

### ❌ Mistake 1: Missing Wildcard

```polyglot
[f] $value =? 0
   [r] // ...
{x}

[f] $value =? 1
   [r] // ...
{x}

// ❌ COMPILE ERROR: Not exhaustive
```

**Fix:**
```polyglot
[f] $value =? 0
   [r] // ...
{x}

[f] $value =? 1
   [r] // ...
{x}

[f] *?
   [r] |U.Do.Nothing""
{x}
```

### ❌ Mistake 2: Unreachable Wildcard

```polyglot
[f] $flag =? #True
   // ...
{x}

[f] $flag =? #False
   // ...
{x}

[f] *?
   // ⚠️ WARNING: Unreachable (boolean already exhaustive)
{x}
```

**Fix:** Remove unnecessary wildcard for boolean types.

### ❌ Mistake 3: Forgetting AND Continuation

```polyglot
[f] $a =? #True
   [f] $b =? #True    // ❌ Nested fork, not AND!
      // ...
   {x}
{x}
```

**Fix:**
```polyglot
[f] $a =? #True
[&] $b =? #True    // ✅ AND continuation
   // ...
{x}
```

---

## Best Practices

### ✅ 1. Use Wildcards for Safety

Even if you think you've covered all cases:
```polyglot
[f] $status =? #Expected.Value1
{x}
[f] $status =? #Expected.Value2
{x}
[f] *?
   [r] >error << !UnexpectedValue    // Safety net!
{x}
```

### ✅ 2. Make No-Op Explicit

```polyglot
[f] *?
   [r] |U.Do.Nothing""    // ✅ Clear intent
{x}

// Not:
[f] *?
{x}    // ❌ Empty block - unclear
```

### ✅ 3. Document Complex Conditions

```polyglot
// Check: Both services must be ready
[f] $serviceA =? #Ready
[&] $serviceB =? #Ready
   [r] |U.StartProcessing""
{x}
```

### ✅ 4. Order Forks Logically

```polyglot
// Success first, then errors
[f] $result =? #Success
   // ...
{x}

[f] $result =? #PartialSuccess
   // ...
{x}

[f] $result =? #Failure
   // ...
{x}

[f] *?
   // ...
{x}
```

---

## Comparison with Other Languages

### vs. If/Else

**Other languages:**
```python
if condition:
    # ...
# Forgot else - no compile error!
```

**Polyglot:**
```polyglot
[f] $condition =? #True
   [r] // ...
{x}

// ❌ COMPILE ERROR: Must handle #False case too!
```

**Benefit:** Exhaustiveness prevents bugs at compile time.

### vs. Pattern Matching

**Polyglot forks** are similar to pattern matching in Rust/ML but simpler:
- Only equality comparisons (no destructuring)
- AND combinations with `[&]`
- Mandatory exhaustiveness
- Wildcard `*?` for catch-all

---

## Quick Reference

```
┌──────────────────────────────────────────────┐
│ FORK PATTERNS                                │
├──────────────────────────────────────────────┤
│                                              │
│  SIMPLE FORK                                 │
│  [f] $var =? value                           │
│     [r] // action                            │
│  {x}                                         │
│                                              │
│  AND FORK                                    │
│  [f] $a =? value1                            │
│  [&] $b =? value2                            │
│     [r] // both must match                   │
│  {x}                                         │
│                                              │
│  WILDCARD                                    │
│  [f] *?                                      │
│     [r] // all other cases                   │
│  {x}                                         │
│                                              │
│  EXHAUSTIVENESS                              │
│  All cases must be covered or:               │
│  ❌ COMPILE ERROR                            │
│                                              │
└──────────────────────────────────────────────┘
```

---

## See Also

- [Error Handling Guide](./error-handling-guide.md) - Using forks for error cases
- [Enum Syntax](./enum-syntax-guide.md) - Enum values in comparisons
- [Operators Reference](./operators-reference.md) - Assignment in fork blocks
- [Hello World Tutorial](./hello-world-tutorial.md) - Fork examples

---

**Generated by:** Polly (Polyglot Language Expert)
**For:** User Documentation
**Version:** v0.0.4
**Last Updated:** 2025-12-26
**Confidence:** ✅ Verified (exhaustiveness and wildcard patterns confirmed by human trainer)
