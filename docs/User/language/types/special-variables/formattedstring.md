# %Formatted_string

## Purpose

Captures the inline string argument passed to a pipeline with `%Pipeline.Inline` metadata.

## Properties

| Property | Value |
|----------|-------|
| **Name** | `%Formatted_string` |
| **Type** | `pg.string` |
| **Scope** | Only within `%Pipeline.Inline` metadata blocks |
| **Mutability** | Read-only |
| **Availability** | v0.0.4+ |

## Usage

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

## Example

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

## Behavior

1. **Inline Call**: User invokes pipeline with string argument
2. **Capture**: `%Formatted_string` receives the string value
3. **Binding**: String bound to parser pipeline input
4. **Processing**: Parser extracts structured data
5. **Wiring**: Parser outputs wire to main pipeline inputs

## Scope Restrictions

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

## Type Requirements

```polyglot
✅ Correct type:
<formatted_string:pg.string << %Formatted_string

❌ Wrong type:
<formatted_string:pg.int << %Formatted_string  // Type mismatch error
```

---
