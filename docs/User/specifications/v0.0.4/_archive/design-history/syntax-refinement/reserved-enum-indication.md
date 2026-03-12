<!-- ARCHIVED: 2025-12-16 | Reason: Historical context, not current specification | Superseded by: Current specifications in /language/ and /features/ -->

# Reserved Enumeration Indication - Design Proposals

**Date:** 2025-12-11
**Status:** 💡 DESIGN PROPOSALS

---

## The Problem

**Question:** How do we distinguish reserved (language-level) enums from user-defined enums?

### Why This Matters

1. **IDE Intelligence**
   - Syntax highlighting
   - Autocomplete suggestions
   - Error detection

2. **Compiler Behavior**
   - Reserved enums have special semantics
   - Cannot be redefined or shadowed
   - May have compiler-generated implementations

3. **Developer Understanding**
   - Quick visual recognition
   - Prevents naming conflicts
   - Clear documentation

### Examples of Reserved Enums

```polyglot
#Boolean.True / #Boolean.False         // Language primitives
#Violation.Action.Raise                // Type system actions
#IO.Stream.TriplePull                  // I/O operations
#OrderStatus.Processing                // User-defined (NOT reserved)
```

---

## Proposal 1: Naming Convention (Recommended)

**Use `pg.*` namespace prefix for all reserved enums**

### Syntax

```polyglot
// Reserved (language-level)
#pg.Boolean.True
#pg.Boolean.False
#pg.Violation.Action.Raise
#pg.IO.Stream.TriplePull
#pg.Optional.None
#pg.Optional.Some

// User-defined (application-level)
#OrderStatus.Processing
#PaymentMethod.CreditCard
#UserRole.Admin
```

### Benefits

✅ **Clear visual distinction** - `pg.` prefix immediately identifies reserved
✅ **Greppable** - Easy to search: `grep "#pg\\."`
✅ **No syntax changes** - Uses existing hierarchy pattern
✅ **Consistent with type system** - `:pg.int`, `:pg.string`, `#pg.Boolean`
✅ **Scalable** - Can add more reserved enums without syntax changes
✅ **IDE-friendly** - Simple pattern matching for syntax highlighting

### Standard Reserved Enums

```polyglot
// Core primitives
#pg.Boolean.True
#pg.Boolean.False

// Optional/Result types
#pg.Optional.None
#pg.Optional.Some

#pg.Result.Ok
#pg.Result.Err

// Violation actions
#pg.Violation.Action.Raise
#pg.Violation.Action.Clip
#pg.Violation.Action.Transform
#pg.Violation.Action.Default
#pg.Violation.Action.Warn
#pg.Violation.Action.Ignore

// I/O streams
#pg.IO.Stream.SinglePush              // <
#pg.IO.Stream.SinglePull              // >
#pg.IO.Stream.DoublePush              // <<
#pg.IO.Stream.DoublePull              // >>
#pg.IO.Stream.TriplePush              // <<<
#pg.IO.Stream.TriplePull              // >>>

#pg.IO.Stream.Keys.numeric            // Array indexing
#pg.IO.Stream.Keys.named              // Map keys

// Comparison results
#pg.Ordering.Less
#pg.Ordering.Equal
#pg.Ordering.Greater
```

### Migration Path

**Alias for backward compatibility:**

```polyglot
{A}
// Old short forms (deprecated)
[A] #pg.Boolean.True >> #True
[A] #pg.Boolean.False >> #False
[A] #pg.Optional.None >> #None
[A] #pg.Optional.Some >> #Some

// Both forms work:
[f] $flag =? #pg.Boolean.True          // Explicit
[f] $flag =? #True                     // Aliased (requires import)
{x}
```

**In code:**

```polyglot
{@} @Local::MyApp:1.0.0.0
[<] @PgTypes                           // Import standard aliases

// Can use short forms
[f] $active =? #True
[f] $optional =? #None

// Or explicit forms
[f] $active =? #pg.Boolean.True
[f] $optional =? #pg.Optional.None
{x}
```

### IDE Configuration

Simple pattern matching rules:

```json
{
  "syntax_highlighting": {
    "reserved_enum": {
      "pattern": "#pg\\.[A-Z][A-Za-z0-9]*(\\.[A-Z][A-Za-z0-9]*)*",
      "color": "keyword",
      "style": "bold"
    },
    "user_enum": {
      "pattern": "#[A-Z][A-Za-z0-9]*(\\.[A-Z][A-Za-z0-9]*)*",
      "color": "type",
      "style": "normal"
    }
  },
  "autocomplete": {
    "reserved_enums": {
      "prefix": "#pg.",
      "source": "language_server"
    }
  }
}
```

---

## Proposal 2: Metadata Marker

**Use `%Reserved` metadata on enum definitions**

### Syntax

```polyglot
{#} #Boolean
[%] %Reserved                          // Marks as reserved

[.] .True
[.] .False
{x}

{#} #OrderStatus                       // No %Reserved = user-defined
[.] .Pending
[.] .Processing
{x}
```

### Benefits

✅ **Explicit declaration** - Clear in definition
✅ **Compiler enforced** - Prevents redefinition
✅ **Documentation** - Self-documenting code

### Drawbacks

❌ **Not visible at use-site** - Need to check definition
❌ **IDE complexity** - Requires semantic analysis
❌ **Not greppable at call-site** - Can't distinguish `#Boolean.True` visually

### IDE Requirements

```json
{
  "semantic_tokens": {
    "enum_usage": {
      "check_definition": true,
      "if_has_metadata": "%Reserved",
      "then_style": "keyword"
    }
  }
}
```

---

## Proposal 3: Registry-Based

**Reserved enums come from standard registry**

### Syntax

```polyglot
// Standard library defines reserved enums
{@} @Polyglot::Core:0.0.4.0
{x}

{#} #Boolean
[.] .True
[.] .False
{x}

// User imports standard registry
{@} @Local::MyApp:1.0.0.0
[<] @Polyglot::Core                    // Import reserved types
{x}

// Usage
[f] $flag =? @Polyglot::Core#Boolean.True
```

### With Alias

```polyglot
{@} @Local::MyApp:1.0.0.0
[<] @Core << @Polyglot::Core

[f] $flag =? @Core#Boolean.True

// Or with enum aliases
{A}
[A] @Core#Boolean.True >> #True
{x}

[f] $flag =? #True
```

### Benefits

✅ **Clear origin** - Registry prefix shows source
✅ **Versioned** - Standard library has version
✅ **No naming conflicts** - Different registries isolated

### Drawbacks

❌ **Verbose** - `@Polyglot::Core#Boolean.True` is long
❌ **Requires aliases** - Need alias imports for ergonomics
❌ **Complex for beginners** - Registry system adds cognitive load

---

## Proposal 4: Capitalization Convention

**Reserved enums use SCREAMING_SNAKE_CASE**

### Syntax

```polyglot
// Reserved (all caps)
#BOOLEAN.TRUE
#BOOLEAN.FALSE
#VIOLATION_ACTION.RAISE
#IO_STREAM.TRIPLE_PULL

// User-defined (PascalCase)
#OrderStatus.Processing
#PaymentMethod.CreditCard
```

### Benefits

✅ **Visually distinct** - Impossible to miss
✅ **No syntax changes** - Pure convention
✅ **IDE-friendly** - Simple regex patterns

### Drawbacks

❌ **Breaks naming consistency** - Everything else is PascalCase
❌ **Ugly** - Screaming case is visually noisy
❌ **Not greppable by reservation status** - Would need to maintain list

---

## Proposal 5: Special Prefix Character

**Use `$` prefix for reserved enums (not recommended - conflicts with variables)**

### Syntax

```polyglot
// Reserved ($ prefix)
#$Boolean.True
#$Violation.Action.Raise

// User-defined (no prefix)
#OrderStatus.Processing
```

### Drawbacks

❌ **Syntax ambiguity** - `$` already means variables
❌ **Requires parser changes** - New token rules
❌ **Visual confusion** - Looks like variable reference

---

## Comparison Table

| Proposal | Visual Distinction | IDE Support | Greppable | Syntax Change | Recommended |
|----------|-------------------|-------------|-----------|---------------|-------------|
| 1. `pg.*` namespace | ✅ Clear | ✅ Simple | ✅ Yes | ❌ No | ⭐ **YES** |
| 2. `%Reserved` metadata | ⚠️ At definition only | ⚠️ Complex | ❌ No | ❌ No | ⚠️ Maybe |
| 3. Registry-based | ✅ Very clear | ✅ Medium | ✅ Yes | ❌ No | ⚠️ Maybe |
| 4. SCREAMING_CASE | ✅ Very clear | ✅ Simple | ❌ No | ❌ No | ❌ No |
| 5. Special prefix | ✅ Clear | ⚠️ Complex | ⚠️ Confusing | ✅ Yes | ❌ No |

---

## Recommendation: Proposal 1 (`pg.*` Namespace)

### Implementation

**1. Standard reserved enums use `pg.*` prefix:**

```polyglot
// File: @Polyglot::Core/enums/boolean.pg
{@} @Polyglot::Core:0.0.4.0
{x}

{#} #pg.Boolean
[.] .True
[.] .False
{x}
```

**2. Provide convenient aliases in standard import:**

```polyglot
// File: @Polyglot::Core/aliases.pg
{@} @Polyglot::Core:0.0.4.0
[A] @PgTypes
{x}

{A}
// Boolean aliases
[A] #pg.Boolean.True >> #True
[A] #pg.Boolean.False >> #False

// Optional aliases
[A] #pg.Optional.None >> #None
[A] #pg.Optional.Some >> #Some

// Result aliases
[A] #pg.Result.Ok >> #Ok
[A] #pg.Result.Err >> #Err

// Type aliases
[A] :pg.int >> :int
[A] :pg.string >> :string
[A] :pg.bool >> :bool
{x}
```

**3. User imports standard aliases:**

```polyglot
{@} @Local::MyApp:1.0.0.0
[<] @PgTypes                           // Import standard aliases

// Short form (with alias)
[f] $active =? #True
[r] $count:int << 42

// Explicit form (always works)
[f] $active =? #pg.Boolean.True
[r] $count:pg.int << 42
{x}
```

**4. IDE configuration (simple pattern matching):**

```yaml
syntax_highlighting:
  reserved_enum:
    pattern: '#pg\.[A-Z][A-Za-z0-9]*(\.[A-Z][A-Za-z0-9]*)*'
    style: keyword_color

  reserved_type:
    pattern: ':pg\.[a-z][a-z0-9]*(\.[a-z][a-z0-9]*)*'
    style: keyword_color

  user_enum:
    pattern: '#[A-Z][A-Za-z0-9]*(\.[A-Z][A-Za-z0-9]*)*'
    style: type_color

  user_type:
    pattern: ':[a-z][a-z0-9]*(\.[a-z][a-z0-9]*)*'
    style: type_color

autocomplete:
  reserved:
    triggers: ['#pg.', ':pg.']
    source: language_server
    priority: high
```

### Benefits Summary

1. **Consistency** - Matches type system (`:pg.int`, `:pg.string`)
2. **Simplicity** - No new syntax, pure naming convention
3. **Flexibility** - Can use short aliases or explicit forms
4. **Clarity** - Immediately obvious which enums are reserved
5. **Tooling** - Simple patterns for IDEs and linters
6. **Scalability** - Easy to add new reserved enums
7. **Greppability** - Search for all reserved: `grep "#pg\."`
8. **Backward compatible** - Aliases provide ergonomic short forms

---

## Migration Guide

### For Existing Code

**Before (hypothetical v0.0.3):**
```polyglot
[f] $flag =? #Boolean.True
[r] $result =? #None
```

**After (v0.0.4):**
```polyglot
// Option 1: Use aliases (recommended for ergonomics)
{@} @Local::MyApp:1.0.0.0
[<] @PgTypes

[f] $flag =? #True                     // Short form via alias
[r] $result =? #None                   // Short form via alias
```

```polyglot
// Option 2: Use explicit forms (always works, no import needed)
[f] $flag =? #pg.Boolean.True          // Explicit form
[r] $result =? #pg.Optional.None       // Explicit form
```

### For New Code

**Recommended pattern:**

```polyglot
{@} @Local::MyApp:1.0.0.0
[<] @PgTypes                           // Import standard aliases

// Use short forms for brevity
[f] $active =? #True
[f] $count:int >? 0
[r] $result:optional.string << #None

// Use explicit forms when clarity matters or in libraries
[f] $active =? #pg.Boolean.True
[f] $count:pg.int >? 0
[r] $result:pg.optional.pg.string << #pg.Optional.None
```

---

## Standard Library Organization

```
@Polyglot::Core:0.0.4.0
├── types/
│   ├── primitives.pg          (:pg.int, :pg.string, :pg.bool, etc.)
│   ├── collections.pg         (:pg.array, :pg.map, :pg.set)
│   ├── optional.pg            (:pg.optional)
│   └── result.pg              (:pg.result)
│
├── enums/
│   ├── boolean.pg             (#pg.Boolean)
│   ├── optional.pg            (#pg.Optional)
│   ├── result.pg              (#pg.Result)
│   ├── violation.pg           (#pg.Violation)
│   ├── io.pg                  (#pg.IO)
│   └── ordering.pg            (#pg.Ordering)
│
├── aliases.pg                 (@PgTypes export)
│
└── metadata.pg                (Standard metadata attributes)
```

---

## Examples in Practice

### Example 1: Web Application

```polyglot
{@} @Local::WebApp:1.0.0.0
[<] @PgTypes

// User-defined enums (no pg. prefix)
{#} #HttpStatus
[.] .Ok
[.] .NotFound
[.] .ServerError
{x}

{#} #UserRole
[.] .Admin
[.] .User
[.] .Guest
{x}

{|} |HandleRequest
[<] i<authorized:bool

[t] |T.Call
[W] |W.Polyglot.Scope

// Mix of reserved (with alias) and user-defined
[f] $authorized =? #True               // Reserved (via alias)
   [r] $status << #HttpStatus.Ok       // User-defined
[f] *?
   [r] $status << #HttpStatus.NotFound // User-defined

[>] o>status << $status:HttpStatus
{x}
```

### Example 2: Data Processing

```polyglot
{@} @Local::DataProcessor:1.0.0.0
[<] @PgTypes

{|} |ProcessRecord
[<] i<record:serial

[t] |T.Call
[W] |W.Polyglot.Scope

// Explicit form for library code
[r] $validation << |ValidateRecord
   <record << $record
   >result >> $result

// Pattern match on reserved Result enum
[m] $status << $result
   [?] #pg.Result.Ok ? "success"       // Explicit form
   [?] #pg.Result.Err ? "failed"       // Explicit form
   [?] * ? "unknown"

[>] o>status << $status:string
{x}
```

### Example 3: Type Constraints

```polyglot
{@} @Local::Types:1.0.0.0

{:} :data.age
[<] i<value:pg.int

[%] %Constraint
   [.] .min << 0
   [.] .max << 120
   [.] .violation:#pg.violation
      [.] .error << !Warning.Integer.Outbound
      [.] .msg << "Age must be between 0 and 120"
      [.] .action << #pg.Violation.Action.Clip  // Reserved enum (explicit)
         [.] .pipeline << |U.Int.Clip
            <value << :data.age<value
            <min << 0
            <max << 120
            >clipped
{x}
```

---

**Status:** ✅ Recommendation clear - `pg.*` namespace prefix

**Key Achievement:** Simple, consistent, greppable solution that leverages existing hierarchy pattern and alias system for best of both worlds (clarity + ergonomics).
