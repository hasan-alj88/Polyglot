# Reserved Enumerations

**Version:** 0.0.2
**Last Updated:** 2025-11-16
**Status:** Partial - #Boolean Complete, Others Deferred

---

## Overview

Reserved enumerations are system-provided enumerations that replace keywords and provide type-safe values for common language constructs. They use the `#` prefix and are part of Polyglot's zero-keyword philosophy.

**Key Properties:**
- System-provided (built-in to the language)
- Most are **non-extendable** (closed set of values)
- Some allow user extension (e.g., `#Path.Identifiers.*`)
- Type-safe at compile-time
- Support exhaustive pattern matching

---

## Boolean System Enumeration

### `#Boolean`

**Status:** ✓ COMPLETE
**Extendable:** NO (System-provided, closed)
**Purpose:** Polyglot's fundamental boolean type. Replaces `True` and `False` keywords.

#### Variants

```polyglot
#Boolean.True      // Boolean true value
#Boolean.False     // Boolean false value
```

#### Aliases

```polyglot
#True    // Shorthand for #Boolean.True
#False   // Shorthand for #Boolean.False
```

#### Type Declaration

```polyglot
// Variable with boolean type
[r] .is_active: #Boolean << #True
[r] .is_valid: #Boolean << #False

// Input parameter
[i] .enable_feature: #Boolean

// Output parameter
[o] .success: #Boolean
```

#### Conditional Switching

**Exhaustive matching required** - both branches must be present:

```polyglot
[|] ProcessFlag
[i] .flag: #Boolean
[t] |T.Call
[w] |W.NoSetup.NoCleanup

[?] .flag =? #True
[~][r] |HandleTrue
[~][o] .result: pg\string << "Enabled"

[?] .flag =? #False
[~][r] |HandleFalse
[~][o] .result: pg\string << "Disabled"

[X]
```

#### Compiler Rules

1. **Exhaustive Boolean Switch:** When switching on `#Boolean`, BOTH `#True` and `#False` branches required
2. **Type Safety:** Cannot assign non-boolean values to `#Boolean` variables
3. **No Extension:** Users cannot add variants (e.g., no `#Boolean.Maybe`)
4. **Alias Consistency:** `#True`/`#False` always resolve to `#Boolean.True`/`#Boolean.False`

#### Boolean Operations

Boolean operations use utility pipelines:

```polyglot
// NOT operation
[r] |U.Boolean.Not
[<] .value: #Boolean << #True
[>] .result: #Boolean >> .negated     // → #False

// AND operation
[r] |U.Boolean.And
[<] .a: #Boolean << #True
[<] .b: #Boolean << #False
[>] .result: #Boolean >> .and_result  // → #False

// OR operation
[r] |U.Boolean.Or
[<] .a: #Boolean << #True
[<] .b: #Boolean << #False
[>] .result: #Boolean >> .or_result   // → #True

// XOR operation
[r] |U.Boolean.Xor
[<] .a: #Boolean << #True
[<] .b: #Boolean << #True
[>] .result: #Boolean >> .xor_result  // → #False
```

#### Migration from Keywords

| v0.0.1 (Keywords) | v0.0.2 (Enumeration) |
|-------------------|----------------------|
| `True` | `#True` or `#Boolean.True` |
| `False` | `#False` or `#Boolean.False` |
| `[?] .var ?> True` | `[?] .var =? #True` |
| `pg\bool` (if existed) | `#Boolean` |

---

## Other Reserved Enumerations

**Status:** Documentation Deferred

The following reserved enumerations are planned but deferred to a later documentation phase:

### Confirmed Enumerations

- **`#Path.Identifiers.*`** - Cross-platform path identifiers (Extendable)
- **`#Comparison`** - Comparison result values (Non-extendable)
- **`#None`** - Special "no value" singleton (Non-extendable)

### Pending Definition

- **`#Queues.*`** - Custom queue definitions (Extendable)
- **`#DT.Business.Week.*`** - Business week definitions (Extendable)
- **`#DT.*`** - Other datetime-related enumerations
- **`#Status.*`** - System status values (Non-extendable)

### Deprecated

- **`#Errors.*`** - Replaced by `!Error` syntax in v0.0.2

---

## Design Rationale

### Why Enumerations Instead of Keywords?

1. **Zero-Keyword Philosophy:** Maintains Polyglot's goal of having no keywords
2. **Exhaustive Pattern Matching:** Enables compile-time verification of all cases
3. **Type Safety:** Enumerations are types, providing stronger type checking
4. **Consistency:** All special values use the same `#` prefix notation
5. **Extensibility:** Some enumerations allow user extension while others are closed

### Why Exhaustive Matching?

1. **Prevents Logic Errors:** Forces handling of all possible cases
2. **Maintainability:** Makes code explicit about all branches
3. **Refactoring Safety:** Adding/removing variants causes compile errors in incomplete matches
4. **Documentation:** Code self-documents all possible states

---

## See Also

- **[Reserved Enumeration Schema Decisions](../audit/reserved-enumeration-schema-decisions.md)** - Complete schema documentation
- **[Quick Language Reference](../audit/quick-language-reference.md)** - Fast syntax reference
- **[Type System](../language/02-type-system.md)** - Type system documentation
- **[Operators](../language/05-operators.md)** - Operator semantics

---

**Note:** Detailed schemas for all reserved enumerations are documented in `../audit/reserved-enumeration-schema-decisions.md`. This file provides user-facing documentation for confirmed enumerations.