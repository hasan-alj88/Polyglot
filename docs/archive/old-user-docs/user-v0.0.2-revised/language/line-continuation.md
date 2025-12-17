# Line Continuation & String Operations

**Version:** 0.0.2
**Last Updated:** 2025-11-18
**Status:** Complete

---

## Overview

Polyglot provides explicit syntax for multi-line expressions and string operations through the line continuation block marker `[*]`, string concatenation operator `+"`, and string interpolation `{.variable}`.

**Key Principles:**
- Explicit over implicit
- Syntactic sugar for readability (NOT semantic)
- Type-safe string operations
- Clear scope boundaries

---

## Table of Contents

1. [Line Continuation Block](#line-continuation-block)
2. [String Concatenation Operator](#string-concatenation-operator)
3. [String Interpolation](#string-interpolation)
4. [Whitespace Handling](#whitespace-handling)
5. [Comment Stripping](#comment-stripping)
6. [Scope Rules](#scope-rules)
7. [Complete Examples](#complete-examples)
8. [Common Mistakes](#common-mistakes)

---

## Line Continuation Block

### `[*]` Marker

**Purpose:** Syntactic line continuation for readability

**Syntax:**
```polyglot
[*] <content>
[*] <more content>
[*] <even more content>
```

**Example:**
```polyglot
[r] .message: pg\string <<
[*] "This is a very long message "
[*]  +" that needs to span multiple lines "
[*]  +" for better code readability."
```

**Important:** `[*]` is NOT the XOR operator - that's `[^]`

---

### Syntactic vs Semantic

**`[*]` is purely syntactic:**
- Does NOT change program behavior
- Does NOT affect runtime
- Only affects source code layout
- Stripped during compilation

```polyglot
// These are IDENTICAL at runtime:

// Version 1: Single line
[r] .msg: pg\string << "Hello World"

// Version 2: With continuation
[*] [r] .msg: pg\string <<
[*] "Hello World"
```

---

## String Concatenation Operator

### `+"` Operator

**Purpose:** Explicit string literal concatenation

**Syntax:**
```polyglot
"string1"  +" "string2"  +" "string3"
```

**Type Restriction:** Literals only (NOT variables)

---

### Basic Usage

```polyglot
[r] .greeting: pg\string << "Hello"  +" ", "  +" "World!"
// Result: "Hello, World!"
```

---

### With Line Continuation

```polyglot
[*] "First part "
[*]  +" "second part "
[*]  +" "third part"
// Result: "First part second part third part"
```

---

### Concatenation is Explicit

**NOT allowed:**
```polyglot
// ✗ WRONG - No implicit concatenation
[r] .msg: pg\string << "Hello" "World"

// ✓ CORRECT - Explicit  +" operator
[r] .msg: pg\string << "Hello"  +" "  +" "World"
```

---

## String Interpolation

### `{.variable}` Syntax

**Purpose:** Embed variables in string literals

**Syntax:**
```polyglot
"text {.variable_name} more text"
```

**Example:**
```polyglot
[r] .name: pg\string << "Alice"
[r] .age: pg\int << 30

[r] .message: pg\string << "Hello, {.name}! You are {.age} years old."
// Result: "Hello, Alice! You are 30 years old."
```

---

### Interpolation vs Concatenation

**Use interpolation for variables:**
```polyglot
// ✓ GOOD - Interpolation
[r] .greeting: pg\string << "Hello, {.name}!"

// ✗ BAD - Concatenation for variables
[r] .greeting: pg\string << "Hello, "  +" .name  +" "!"  // Error!
```

**Use concatenation for literals:**
```polyglot
// ✓ GOOD - Concatenation
[r] .msg: pg\string << "Part1"  +" " "  +" "Part2"

// ✗ UNNECESSARY - Interpolation for literals
[r] .msg: pg\string << "Part1 {Part2}"  // No variable here
```

---

## Whitespace Handling

### Inside vs Outside Strings

**Inside strings:** Whitespace preserved exactly
**Outside strings:** Whitespace stripped

```polyglot
[*] "Hello    World"    // Multiple spaces inside preserved
[*]  +" "    Next"       // Leading spaces inside preserved
// Result: "Hello    World    Next"
```

---

### Stripping Rules

**Whitespace OUTSIDE quotes stripped:**
```polyglot
[*]     "text"     // Spaces outside stripped
[*]    +"  next"    // Spaces outside stripped, inside preserved
// Result: "text  next"
```

**Practical example:**
```polyglot
[*] "SELECT * FROM users "
[*]    +" "WHERE age > 18 "    // Indentation stripped
[*]      +" "ORDER BY name"     // Indentation stripped
// Result: "SELECT * FROM users WHERE age > 18 ORDER BY name"
```

---

## Comment Stripping

### Comments Removed Before Joining

**Comments are stripped BEFORE line continuation:**

```polyglot
[*] "First part "      // This comment removed
[*]  +" "second part "  // This comment removed
[*]  +" "third part"    // This comment removed
// Result: "First part second part third part"
```

---

### Comment Behavior

```polyglot
// Comments do NOT appear in result
[*] "Hello "  // Comment 1
[*]  +" "World"   // Comment 2
// Result: "Hello World" (no comments)
```

---

## Scope Rules

### Continuation Scope

**Scope ends at first non-`[*]` line:**

```polyglot
[*] "Line 1 "
[*]  +" "Line 2 "
[*]  +" "Line 3"
[r] |NextOperation  // Scope ends here

// Lines 1-3 joined together
// [r] starts new scope
```

---

### Explicit Scope Boundary

```polyglot
// Continuation block
[*] "Part A "
[*]  +" "Part B"

// New statement (scope ended)
[r] |ProcessData

// Another continuation block
[*] "Part C "
[*]  +" "Part D"
```

---

## Complete Examples

### Example 1: Long SQL Query

```polyglot
[r] .query: pg\string <<
[*] "SELECT users.id, users.name, orders.total "
[*]  +" "FROM users "
[*]  +" "INNER JOIN orders ON users.id = orders.user_id "
[*]  +" "WHERE users.active = true "
[*]  +" "AND orders.total > 100 "
[*]  +" "ORDER BY orders.total DESC "
[*]  +" "LIMIT 50"
```

---

### Example 2: Multi-line Log Message

```polyglot
[r] .error_count: pg\int << 5
[r] .timestamp: pg\dt << #DT.Now

[r] .log_message: pg\string <<
[*] "ERROR: System health check failed. "
[*]  +" "Found {.error_count} critical errors "
[*]  +" "at {.timestamp}. "
[*]  +" "Please investigate immediately."
```

---

### Example 3: HTML Template

```polyglot
[r] .user_name: pg\string << "Alice"
[r] .user_email: pg\string << "alice@example.com"

[r] .html_template: pg\string <<
[*] "<!DOCTYPE html><html><head +"
[*]  +" "<title>Welcome {.user_name}</title +"
[*]  +" "</head><body +"
[*]  +" "<h1>Hello, {.user_name}!</h1 +"
[*]  +" "<p>Your email: {.user_email}</p +"
[*]  +" "</body></html +"
```

---

### Example 4: API Request Body

```polyglot
[r] .user_id: pg\int << 42
[r] .action: pg\string << "update_profile"

[r] .request_body: pg\string <<
[*] "{"
[*]  +" "  \"user_id\": {.user_id},"
[*]  +" "  \"action\": \"{.action}\","
[*]  +" "  \"timestamp\": \"{#DT.Now}\","
[*]  +" "  \"source\": \"api_v2\""
[*]  +" "}"
```

---

### Example 5: Configuration String

```polyglot
[r] .db_host: pg\string << "localhost"
[r] .db_port: pg\int << 5432
[r] .db_name: pg\string << "production"

[r] .connection_string: pg\string <<
[*] "postgresql://{.db_host}:{.db_port}/"
[*]  +" "{.db_name}?sslmode=require"
[*]  +" "&connect_timeout=10"
[*]  +" "&application_name=polyglot_app"
```

---

### Example 6: Error Message with Context

```polyglot
[r] .file_path: pg\path << \\DataDir\\users.csv
[r] .line_number: pg\int << 142
[r] .column_name: pg\string << "email"

[r] .error_message: pg\string <<
[*] "Validation Error: "
[*]  +" "Invalid data in file {.file_path} "
[*]  +" "at line {.line_number}, "
[*]  +" "column '{.column_name}'. "
[*]  +" "Expected format: user@domain.com"
```

---

## Common Mistakes

### Mistake 1: Using `[*]` for Logic

```polyglot
// ✗ WRONG - [*] is NOT XOR
[*] .condition1
[*] .condition2

// ✓ CORRECT - Use [^] for XOR (boolean blocks close by context)
[^] .condition1
[^] .condition2
```

---

### Mistake 2: Concatenating Variables

```polyglot
// ✗ WRONG - Cannot concatenate variables with  +"
[r] .msg: pg\string << .var1  +" "  +" .var2

// ✓ CORRECT - Use interpolation
[r] .msg: pg\string << "{.var1} {.var2}"
```

---

### Mistake 3: Implicit Concatenation

```polyglot
// ✗ WRONG - No implicit concatenation
[r] .msg: pg\string << "Hello" "World"

// ✓ CORRECT - Explicit  +" operator
[r] .msg: pg\string << "Hello"  +" "  +" "World"
```

---

### Mistake 4: Forgetting Quotes

```polyglot
// ✗ WRONG - Missing quotes around literals
[*] Hello World

// ✓ CORRECT - Proper quoting
[*] "Hello World"
```

---

### Mistake 5: Mixed Continuation and Logic

```polyglot
// ✗ WRONG - Confusing syntax
[*] "text"
[r] |Operation   // Ends continuation scope
[*] "more text"  // New continuation (separate)

// ✓ CORRECT - Clear separation
[*] "text part 1 "
[*]  +" "text part 2"
// Continuation scope ends

[r] |Operation

[*] "separate text"
// New continuation scope
```

---

### Mistake 6: Expecting Semantic Behavior

```polyglot
// ✗ WRONG EXPECTATION - [*] doesn't change logic
[*] [r] |Operation1
[*] [r] |Operation2
// Does NOT make operations sequential - [*] is syntactic only!

// ✓ CORRECT - Sequential operations
[r] |Operation1
[r] |Operation2
```

---

## Design Rationale

### Why Explicit?

**Explicit over implicit:**
- Clear intent in source code
- No ambiguity about multi-line behavior
- Easy to identify continuation blocks
- Compiler can validate syntax

---

### Why `[*]` Symbol?

**Rationale:**
- Consistent with block marker pattern `[x]`
- Visually suggests "continuation" or "more"
- Available character not used elsewhere
- `[^]` reserved for XOR (boolean logic)

---

### Why `+"` for Concatenation?

**Rationale:**
- Visually suggests "append" or "continue"
- Explicit operator (no implicit concatenation)
- Consistent with Polyglot's explicit philosophy
- Cannot be confused with comparison `>`

---

## Performance Notes

**No runtime overhead:**
- `[*]` stripped during compilation
- String concatenation happens at compile-time for literals
- Interpolation compiled to efficient string formatting
- Zero performance cost for readability

---

## See Also

- [Block Markers](block-markers.md) - `[*]` line continuation block
- [Operators](operators.md) - `+"` concatenation operator
- [Type System](type-system.md) - `pg\string` type details
- [Comments](comments.md) - Comment stripping behavior

---

**Session Document:** `docs/project/agent-sessions/carson-2025-11-18-line-continuation-spec.md`
**Design Decisions:** Complete specification with whitespace and scope rules
**Status:** Production-ready specification

---

**End of Line Continuation Documentation**
