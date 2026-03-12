# Special Variables
**Feature Type:** Language Feature - Built-in Variables
**Since:** v0.0.4
**Status:** ✅ Verified in Training

---

## Overview

Special variables in Polyglot are **built-in variables** with the `%` prefix that provide access to runtime context and metadata within specific scopes.

**Characteristics:**
- Prefix: `%` (percent sign)
- Scope-limited availability
- Read-only (cannot be reassigned)
- Type-safe bindings required

---

## %Formatted_string

### Purpose

Captures the inline string argument passed to a pipeline with `%Pipeline.Inline` metadata.

### Properties

| Property | Value |
|----------|-------|
| **Name** | `%Formatted_string` |
| **Type** | `pg.string` |
| **Scope** | Only within `%Pipeline.Inline` metadata blocks |
| **Mutability** | Read-only |
| **Availability** | v0.0.4+ |

### Usage

```polyglot
{|} |MainPipeline
[%] %Pipeline.Inline
   [%] |ParserPipeline
   (|) <formatted_string:pg.string << %Formatted_string  // ← Binding required
   (|) >parsed_output >> <input

[<] <input:type
// ...
{x}
```

### Example

```polyglot
{|} |ParseEmail
[t] |T.Call
[<] <formatted_string:pg.string
[>] >user :pg.string
[>] >domain :pg.string

[w] |W.Runtime.Python3.11
(|) <requirements:pg.path << \\NoPath\\
(|) >env:pg.serial >> $py

[r] |RT.Python.Run.Code
(|) <env:pg.serial << $py
(|) <kwargs.email:pg.string << $formatted_string
(|) >user :pg.string >> >user
(|) >domain :pg.string >> >domain
(|) <code:pg.string << |U.String.Python""
[+] +"def parse(email:str)->dict:"
[+] -"   user, domain = email.split('@')"
[+] -"   return dict(user=user, domain=domain)"
{x}


{|} |ValidateEmail
[%] %Pipeline.Inline
   [%] |ParseEmail
   (|) <formatted_string:pg.string << %Formatted_string  // ← Captures "user@domain.com"
   (|) >user :pg.string >> <user
   (|) >domain :pg.string >> <domain

[<] <user :pg.string
[<] <domain :pg.string
[>] >is_valid :pg.bool

// Validation logic
{x}
```

**Invocation:**
```polyglot
[r] |ValidateEmail "admin@example.com"  // ← This string becomes %Formatted_string
```

### Behavior

1. **Inline Call**: User invokes pipeline with string argument
2. **Capture**: `%Formatted_string` receives the string value
3. **Binding**: String bound to parser pipeline input
4. **Processing**: Parser extracts structured data
5. **Wiring**: Parser outputs wire to main pipeline inputs

### Scope Restrictions

```polyglot
✅ Valid - Inside %Pipeline.Inline:
[%] %Pipeline.Inline
   [%] |Parser
   (|) <str:pg.string << %Formatted_string

❌ Invalid - Outside metadata block:
{|} |Pipeline
[<] <input << %Formatted_string  // Compilation error
{x}

❌ Invalid - Wrong metadata context:
[%] %SomeOtherMetadata
   (|) <str << %Formatted_string  // Compilation error
{x}
```

### Type Requirements

```polyglot
✅ Correct type:
<formatted_string:pg.string << %Formatted_string

❌ Wrong type:
<formatted_string:pg.int << %Formatted_string  // Type mismatch error
```

---

## Other Special Variables

*(This section will be expanded as more special variables are discovered/documented)*

### Potential Future Variables

The `%` prefix is reserved for special variables. Future additions may include:

- `%Pipeline.Name` - Name of current pipeline
- `%Execution.Context` - Runtime execution context
- `%Trigger.Source` - Source that triggered execution
- `%Environment.Vars` - Access to environment variables

*Note: These are speculative and not yet implemented.*

---

## Naming Convention

### Pattern

```
%Category.SpecificName
```

**Examples:**
- `%Formatted_string` - Single-level name (historical)
- `%Pipeline.Inline` - Metadata (not a variable)
- Future: `%Execution.StartTime`

### Rules

1. **Prefix:** Always `%`
2. **Case:** PascalCase for segments
3. **Scope:** Limited to specific contexts
4. **Immutability:** Always read-only
5. **Type:** Explicit type binding required

---

## Best Practices

### 1. Always Bind to Typed Parameter

```polyglot
✅ Good:
<formatted_string:pg.string << %Formatted_string

❌ Bad:
$str << %Formatted_string  // Direct to variable may not work
```

### 2. Check Scope Availability

Before using a special variable, verify it's available in your context:

```polyglot
✅ %Formatted_string in %Pipeline.Inline:
[%] %Pipeline.Inline
   (|) <str << %Formatted_string

❌ %Formatted_string elsewhere:
[<] <input << %Formatted_string  // Error
```

### 3. Document Special Variable Usage

When using special variables, add comments:

```polyglot
[%] %Pipeline.Inline
   [%] |Parser
   (|) <formatted_string:pg.string << %Formatted_string  // ← Input from inline string
   (|) >output >> <input
```

---

## Technical Notes

### Compile-Time vs Runtime

- **Scope checking:** Compile-time
- **Type checking:** Compile-time
- **Value binding:** Runtime

### Performance

- No performance overhead
- Direct value passing (no copying)
- Same efficiency as regular parameter binding

### Error Messages

Common errors when using special variables:

```
Error: %Formatted_string not available in this context
  → Use %Formatted_string only within %Pipeline.Inline metadata

Error: Type mismatch for %Formatted_string
  → Expected: pg.string, Got: pg.int

Error: %Unknown_variable not defined
  → Check special variable name and scope
```

---

## See Also

- [%Pipeline.Inline Metadata](/docs/User/language/advanced/pipeline-inline-metadata.md) - Primary use of %Formatted_string
- [Variable System](/docs/User/language/syntax/variables.md) - Regular variables ($ prefix)
- [Metadata System](/docs/User/language/advanced/metadata.md) - All metadata types
- [Type System](/docs/User/language/types/overview.md) - Type binding rules

---

**Added:** 2025-12-22 (Training Session)
**Verified:** Human review and correction in training session
**Source:** L-2025-12-22-007
**Target Path:** `docs/User/language/types/special-variables.md`
