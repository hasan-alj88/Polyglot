# Polyglot Type Definition System

**Date:** 2025-12-11
**Status:** 🔧 DESIGN REFINEMENT

---

## Philosophy

> **"All types are bytes manipulated with tools. Types add constraints and semantics."**

Polyglot types are built on primitives (`pg.int`, `pg.string`, etc.) with:
- Constraints (validation rules)
- Violation handlers (what to do when violated)
- Transformations (how to convert)

---

## New Block: `{:}` Type Definition

Similar to:
- `{#}` - Enum/struct definition
- `{!}` - Error definition
- `{|}` - Pipeline definition
- `{:}` - **Type definition**

---

## Syntax Refinement

### Basic Constrained Type

```polyglot
{:} :data.age
[<] i<value:pg.int                     // Input: underlying type

[%] %Constraint
   [.] .min:pg.int << 0
   [.] .max:pg.int << 120
   [.] .violation:#pg.violation
      [.] .error << !Warning.Integer.Outbound
      [.] .msg << "Age must be between 0 and 120"
      [.] .action << #Violation.Action.Clip
         [.] .pipeline << |U.Int.Clip
            <value << :data.age<value   // Access input
            <min << 0
            <max << 120
            >clipped                    // Return marker (see below)
{x}
```

### Email String Type

```polyglot
{:} :string.email
[<] i<value:pg.string

[%] %Constraint
   [.] .pattern << "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
   [.] .violation:#pg.violation
      [.] .error << !String.Email.Invalid
      [.] .msg << "Invalid email format"
      [.] .action << #Violation.Action.Raise
{x}
```

### URL Type with Sanitization

```polyglot
{:} :data.url.safe
[<] i<value:pg.string

[%] %Constraint
   [.] .pattern << "^https?://.*"
   [.] .violation:#pg.violation
      [.] .error << !String.URL.Invalid
      [.] .msg << "URL must start with http:// or https://"
      [.] .action << #Violation.Action.Transform
         [.] .pipeline << |U.String.PrependHTTPS
            <value << :data.url.safe<value
            >sanitized

[%] %Constraint
   [.] .max_length << 2048
   [.] .violation:#pg.violation
      [.] .error << !String.URL.TooLong
      [.] .action << #Violation.Action.Raise
{x}
```

---

## Return Marker Syntax

**Problem:** Need syntax for "this output is the return value of the type"

### Proposal A: `[>!]` Return Marker
```polyglot
{:} :data.age
[<] i<value:pg.int

[%] %Constraint
   [.] .violation:#pg.violation
      [.] .action << #Violation.Action.Clip
         [.] .pipeline << |U.Int.Clip
            <value << :data.age<value
            <min << 0
            <max << 120
            >clipped >> [>!]           // Return this output
{x}
```

### Proposal B: `o>!` Return Output Operator
```polyglot
{:} :data.age
[<] i<value:pg.int

[%] %Constraint
   [.] .violation:#pg.violation
      [.] .action << #Violation.Action.Clip
         [.] .pipeline << |U.Int.Clip
            <value << :data.age<value
            <min << 0
            <max << 120
            o>! clipped                // Return output marker
{x}
```

### Proposal C: Implicit Single Output
```polyglot
{:} :data.age
[<] i<value:pg.int

[%] %Constraint
   [.] .violation:#pg.violation
      [.] .action << #Violation.Action.Clip
         [.] .pipeline << |U.Int.Clip
            <value << :data.age<value
            <min << 0
            <max << 120
            >clipped                   // Single output = implicit return
{x}
```

**Recommendation:** **Proposal C (Implicit)** - Single output pipelines implicitly return.

---

## Violation Actions (Standard Enum)

```polyglot
{#} #Violation.Action
[.] .Raise                             // Throw error immediately
[.] .Clip                              // Clip to valid range
[.] .Transform                         // Transform to valid value
[.] .Default                           // Use default value
[.] .Warn                              // Log warning, continue
[.] .Ignore                            // Silently ignore
{x}
```

---

## Cross-Language Types

**Polyglot is polyglot!** Define types from other languages:

### Python Types
```polyglot
{:} :py.str
[<] i<value:pg.string
[%] %Backend << "python"
[%] %Native << "str"
{x}

{:} :py.dict
[<] i<value:pg.serial
[%] %Backend << "python"
[%] %Native << "dict"
{x}
```

### Rust Types
```polyglot
{:} :rust.i32
[<] i<value:pg.int
[%] %Backend << "rust"
[%] %Native << "i32"
[%] %Constraint
   [.] .min << -2147483648
   [.] .max << 2147483647
{x}

{:} :rust.u64
[<] i<value:pg.int
[%] %Backend << "rust"
[%] %Native << "u64"
[%] %Constraint
   [.] .min << 0
   [.] .max << 18446744073709551615
{x}
```

### JavaScript Types
```polyglot
{:} :js.number
[<] i<value:pg.int
[%] %Backend << "javascript"
[%] %Native << "Number"
{x}
```

---

## Type Composition

### Constrained Collections
```polyglot
{:} :data.adult_ages
[<] i<value:pg.array.pg.int

[%] %Constraint
   [.] .item_constraint << :data.age   // Each item must be valid age
   [.] .min_length << 1
   [.] .max_length << 1000
{x}
```

### Optional Types
```polyglot
{:} :data.optional_email
[<] i<value:pg.optional.pg.string

[%] %Constraint
   [.] .when_some << :string.email     // When value present, validate as email
   [.] .allow_none << #Boolean.True
{x}
```

---

## Usage in Pipelines

```polyglot
{|} |RegisterUser
[<] i<email:string.email               // Type validates automatically
[<] i<age:data.age                     // Type validates automatically

[t] |T.Call
[W] |W.Polyglot.Scope

// Type system ensures:
// - email is valid email format
// - age is 0-120 (clipped if out of range)

[r] $user << #User
   [.] .email                          // Already validated
   [.] .age                            // Already validated/clipped

[>] o>user << $user:User
{x}
```

---

## Type Conversions

```polyglot
{:} :data.age_string
[<] i<value:pg.string

[%] %Convert << :data.age              // Can convert to :data.age
   [.] .pipeline << |U.String.ParseInt
      <value << :data.age_string<value
      >parsed

[%] %Constraint
   [.] .pattern << "^[0-9]+$"
{x}

// Usage
[r] $age_str:data.age_string << "25"
[r] $age:data.age << $age_str          // Auto-converts via pipeline
```

---

## Standard Violation Patterns

### Pattern 1: Clip to Range
```polyglot
[%] %Constraint
   [.] .min << 0
   [.] .max << 100
   [.] .violation:#pg.violation
      [.] .action << #Violation.Action.Clip
         [.] .pipeline << |U.Int.Clip
            <value << :type<value
            <min << 0
            <max << 100
            >clipped
```

### Pattern 2: Raise Error
```polyglot
[%] %Constraint
   [.] .pattern << "regex"
   [.] .violation:#pg.violation
      [.] .action << #Violation.Action.Raise
      [.] .error << !InvalidFormat
```

### Pattern 3: Use Default
```polyglot
[%] %Constraint
   [.] .min_length << 1
   [.] .violation:#pg.violation
      [.] .action << #Violation.Action.Default
      [.] .value << "default_value"
```

### Pattern 4: Transform
```polyglot
[%] %Constraint
   [.] .violation:#pg.violation
      [.] .action << #Violation.Action.Transform
         [.] .pipeline << |U.String.Sanitize
            <value << :type<value
            >sanitized
```

---

## Type Hierarchy Organization

```
:pg.*                                  // Polyglot primitives
:pg.int, :pg.string, :pg.serial, etc.

:data.*                                // Application data types
:data.age, :data.email, :data.url

:string.*                              // String specializations
:string.email, :string.url, :string.uuid

:int.*                                 // Integer specializations
:int.positive, :int.percent, :int.age

:py.*                                  // Python types
:py.str, :py.dict, :py.list

:rust.*                                // Rust types
:rust.i32, :rust.u64, :rust.String

:js.*                                  // JavaScript types
:js.number, :js.string, :js.object
```

---

## Benefits

1. **Self-Documenting**
   - Type name shows intent: `:data.age` vs `:pg.int`
   - Constraints are visible in definition

2. **Compile-Time Validation**
   - Compiler generates validation code
   - Zero runtime overhead (inlined)

3. **Cross-Language**
   - Define types from any backend
   - Automatic conversions

4. **Composable**
   - Build complex types from simple ones
   - Reuse constraints

5. **Safe**
   - Cannot bypass constraints
   - Type system enforces

---

**Status:** ✅ Refined design - Ready for your feedback
