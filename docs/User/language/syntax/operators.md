# Assignment Operators Reference

**Version:** v0.0.4
**Category:** Language Reference
**Audience:** All users

---

## Overview

Polyglot uses arrow operators to control data flow and variable mutability. Understanding these operators is crucial for writing correct Polyglot code.

---

## The Four Assignment Operators

| Operator | Name | Direction | State | Description |
|----------|------|-----------|-------|-------------|
| `<<` | Pull Final | ← | Final | Pull value from right, make immutable |
| `<~` | Pull Default | ← | Default | Pull value from right, allow one override |
| `>>` | Push Final | → | Final | Push value to right, make immutable |
| `~>` | Push Default | → | Default | Push value to right, allow one override |

---

## Arrow Direction

The arrow shows **data flow direction**:

```
Source   →→   Destination     (Push with >>)
Source   ~>   Destination     (Push default with ~>)

Destination   ←←   Source     (Pull with <<)
Destination   <~   Source     (Pull default with <~)
```

**Visual rule:** Follow the arrow to see where data flows!

---

## Final vs Default State

### Final State (`<<` and `>>`)

**Characteristics:**
- Value is **immutable** after assignment
- **No more pushes allowed**
- Unlimited pulls allowed
- Use when value won't change

**Example:**
```polyglot
[r] $name :pg.string << "Alice"
// $name is now FINAL
// Cannot be changed

[r] $name :pg.string << "Bob"    // ❌ ERROR: Already final
```

### Default State (`<~` and `~>`)

**Characteristics:**
- Value can be **overridden once**
- **One more push allowed**, then becomes final
- Use for parameters with defaults

**Example:**
```polyglot
[|] <timeout :pg.int <~ 30
// $timeout is DEFAULT with value 30
// Can be overridden once

[r] $timeout :pg.int << 60      // ✅ Override allowed
// Now $timeout is FINAL at 60

[r] $timeout :pg.int << 90      // ❌ ERROR: Already final
```

---

## Pull Operators (`<<` and `<~`)

### Pull Final `<<`

**Syntax:**
```
(destination) << (source)
```

**Data flows:** Right to left (←)
**Result:** Destination becomes **final**

**Examples:**
```polyglot
// Variable assignment
[r] $age :pg.int << 25

// From pipeline call
[r] $result :pg.string << |U.Process.Data"input"

// From another variable
[r] $copy :pg.string << $original
```

### Pull Default `<~`

**Syntax:**
```
(destination) <~ (source)
```

**Data flows:** Right to left (←)
**Result:** Destination becomes **default**

**Examples:**
```polyglot
// Input parameter with default
[|] <timeout :pg.int <~ 30

// Variable with default
[r] $status :pg.string <~ "pending"

// Output parameter with default
[|] >error <~ !NoError
```

---

## Push Operators (`>>` and `~>`)

### Push Final `>>`

**Syntax:**
```
(source) >> (destination)
```

**Data flows:** Left to right (→)
**Result:** Destination becomes **final**

**Examples:**
```polyglot
// Push literal to variable
[r] "Alice" >> $name

// Push to output parameter
[r] !NoError >> >error

// Equivalent to pull:
[r] $name :pg.string << "Alice"    // Pull
[r] "Alice" >> $name                // Push (same result!)
```

### Push Default `~>`

**Syntax:**
```
(source) ~> (destination)
```

**Data flows:** Left to right (→)
**Result:** Destination becomes **default**

**Examples:**
```polyglot
// Push default to parameter
[|] 30 ~> <timeout

// Equivalent to pull:
[|] <timeout :pg.int <~ 30    // Pull
[|] 30 ~> <timeout             // Push (same result!)
```

---

## When to Use Each Operator

### Use `<<` (Pull Final) When:
- ✅ Assigning constants
- ✅ Capturing pipeline outputs
- ✅ Variables that won't change
- ✅ Most variable assignments

**Example:**
```polyglot
[r] $pi :pg.float << 3.14159
[r] $timestamp :pg.dt << |DT.Now"iso8601"
```

### Use `<~` (Pull Default) When:
- ✅ Input parameters with defaults
- ✅ Output parameters (can be updated in pipeline)
- ✅ Variables that might be overridden once

**Example:**
```polyglot
[|] <retries :pg.int <~ 3
[|] >error <~ !NoError
```

### Use `>>` (Push Final) When:
- ✅ Pushing to outputs
- ✅ Preferring left-to-right reading
- ✅ Same cases as `<<` but different visual style

**Example:**
```polyglot
[r] "Completed" >> >status
[r] #True >> $success
```

### Use `~>` (Push Default) When:
- ✅ Same cases as `<~` but different visual style
- ✅ Less common in practice

---

## Variable Lifecycle

```
┌─────────┐
│ Declared│  Type assigned, no value
└────┬────┘
     │
     ├── << or >> ──→ ┌───────┐
     │                 │ Final │  Immutable
     │                 └───┬───┘
     │                     │
     └── <~ or ~> ──→ ┌─────────┐
                       │ Default │  One more push allowed
                       └────┬────┘
                            │
                            └── push ──→ ┌───────┐
                                          │ Final │
                                          └───────┘
```

**States:**
1. **Declared** - Type set, no value
2. **Default** - Value set with `<~`/`~>`, can override once
3. **Final** - Value set with `<<`/`>>` or after default override, immutable

---

## Common Patterns

### Input Parameter with Default

```polyglot
[|] <timeout :pg.int <~ 30
// Caller can override:
// polyglot run cmd --timeout=60
```

### Output Parameter (Default → Final)

```polyglot
// Definition (default)
[|] >error <~ !NoError

// Update in pipeline (becomes final)
[r] >error << !RuntimeError
```

### Variable Reassignment (Default → Final)

```polyglot
[r] $status :pg.string <~ "pending"
// ... do work ...
[r] $status :pg.string << "completed"  // One override allowed
```

### Immutable Constants

```polyglot
[r] $MAX_RETRIES :pg.int << 5
[r] $API_URL :pg.string << "https://api.example.com"
```

---

## Parameter Direction vs Assignment

**Common confusion:** `<` in parameter vs `<<` in assignment

```polyglot
[|] <input :pg.string <~ "default"
    └─┬─┘               └─┬─┘
  INPUT direction    Assignment operator
  (not related!)
```

- `<input` - The `<` means **INPUT parameter direction**
- `<~ "default"` - The `<~` is **default assignment operator**

**These are different concepts!**

---

## Examples Comparison

### Pull vs Push (Same Result)

```polyglot
// Pull syntax (common)
[r] $name :pg.string << "Alice"

// Push syntax (less common)
[r] "Alice" >> $name

// Both create final variable $name with value "Alice"
```

### Final vs Default

```polyglot
// Final - immutable
[r] $config :pg.string << "production"
[r] $config :pg.string << "staging"    // ❌ ERROR

// Default - one override allowed
[r] $mode :pg.string <~ "default"
[r] $mode :pg.string << "custom"       // ✅ OK (one override)
[r] $mode :pg.string << "another"      // ❌ ERROR (now final)
```

---

## Quick Reference Card

```
┌──────────────────────────────────────────────┐
│ ASSIGNMENT OPERATORS                         │
├──────────────────────────────────────────────┤
│                                              │
│  PULL (from right)                           │
│  ─────────────────                           │
│  dest << source    Final (immutable)         │
│  dest <~ source    Default (one override)    │
│                                              │
│  PUSH (to right)                             │
│  ────────────────                            │
│  source >> dest    Final (immutable)         │
│  source ~> dest    Default (one override)    │
│                                              │
│  STATES                                      │
│  ──────                                      │
│  Declared  →  Default  →  Final              │
│           ↘             ↗                    │
│             ──────────→                      │
│                                              │
└──────────────────────────────────────────────┘
```

---

## See Also

- [Variable Lifecycle Guide](./variable-lifecycle.md) - Detailed state explanations
- [Type System](./type-system.md) - Type annotations
- [Error Handling](./error-handling.md) - Faulted state
- [Hello World Tutorial](./hello-world-tutorial.md) - Practical examples

---

**Generated by:** Polly (Polyglot Language Expert)
**For:** User Documentation
**Version:** v0.0.4
**Last Updated:** 2025-12-26
