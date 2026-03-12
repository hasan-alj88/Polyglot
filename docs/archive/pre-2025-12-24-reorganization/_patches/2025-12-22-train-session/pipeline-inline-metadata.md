# %Pipeline.Inline Metadata System
**Feature Type:** Language Feature - Metadata
**Marker:** `[%]`
**Since:** v0.0.4
**Status:** ✅ Verified in Training (L-2025-12-22-007)

---

## Overview

The `%Pipeline.Inline` metadata system allows pipelines to accept formatted string arguments and automatically parse them into structured inputs using a dedicated parser pipeline.

**Key Features:**
- Parse inline string arguments (e.g., `"Database:config.yml"`) into structured data
- Automatic invocation of parser pipeline
- Access to special `%Formatted_string` variable
- Direct wiring from parser outputs to main pipeline inputs

**Common Use Cases:**
- Config validators that accept `"type:value"` format
- Email parsers from `"user@domain"` format
- URL parsers from `"protocol://host:port/path"` format
- Any domain-specific inline DSL parsing

---

## Syntax

### Basic Structure

```polyglot
{|} |MainPipeline
[%] %Pipeline.Inline
   [%] |ParserPipeline
   (|) <formatted_string:pg.string << %Formatted_string
   (|) >parsed_output:type >> <input_param

[<] <input_param:type
[>] >output:type

// Main pipeline logic using parsed inputs
{x}
```

### Components

1. **`[%] %Pipeline.Inline`** - Metadata block marker
2. **`[%] |ParserPipeline`** - Invokes the parser pipeline
3. **`%Formatted_string`** - Special variable containing the inline string
4. **`>output >> <input`** - Wires parser outputs to main inputs

---

## %Formatted_string Special Variable

The `%Formatted_string` is a **built-in special variable** available only within `%Pipeline.Inline` metadata blocks.

**Properties:**
- **Type:** `pg.string`
- **Scope:** Only accessible in %Pipeline.Inline metadata
- **Purpose:** Captures the string argument passed to the inline pipeline call
- **Binding:** `<formatted_string:pg.string << %Formatted_string`

**Example:**
```polyglot
[%] %Pipeline.Inline
   [%] |ParseEmail
   (|) <formatted_string:pg.string << %Formatted_string  // ← Captures "user@domain.com"
   (|) >user :pg.string >> <user
   (|) >domain :pg.string >> <domain
```

---

## Complete Example: Config Validator

This example demonstrates all %Pipeline.Inline patterns:

```polyglot
{#} #;MyApp;Config;Type
[A] #ConfigType
[.] .Database
[.] .API
[.] .Cache
{x}


{|} |ParseValidateConfig
[t] |T.Call

[<] <formatted_string:pg.string
[>] >config_type :#ConfigType
[>] >value :pg.string

[w] |W.Runtime.Python3.9
(|) <requirements:pg.path << \\NoPath\\
(|) >env:pg.serial >> $py

[r] |RT.Python.Run.Code
(|) <env:pg.serial << $py
(|) <kwargs.formatted_string:pg.string << $formatted_string
(|) >config_type :#ConfigType >> >config_type
(|) >value :pg.string >> >value
(|) <code:pg.string << |U.String.Python""
[+] +"def parse(formatted_string:str)->dict:"
[+] -"   config_type, value = formatted_string.split(':')"
[+] -"   return dict(config_type=config_type, value=value)"
{x}


{|} |ValidateConfig
[%] %Pipeline.Inline
   [%] |ParseValidateConfig
   (|) <formatted_string:pg.string << %Formatted_string
   (|) >config_type :#ConfigType >> <config_type
   (|) >value :pg.string >> <value

[<] <config_type :#ConfigType
[<] <value :pg.string
[>] >is_valid :pg.bool <~ #False

[W] |W.Polyglot.Scope
[m] $result << $config_type
   [?] #ConfigType.Database ? "db_valid"
   [?] #ConfigType.API ? "api_valid"
   [?] #ConfigType.Cache ? "cache_valid"
   [?] * ? "unknown"

[f] $result =? "db_valid"
   [f] $value =? ""
      [r] >is_valid << #False
   [f] *?
      [r] >is_valid << #True
{x}
```

**Usage:**
```polyglot
[r] |ValidateConfig "Database:config.yml"
(|) >is_valid >> $valid
```

The string `"Database:config.yml"` is:
1. Captured by `%Formatted_string`
2. Passed to `|ParseValidateConfig`
3. Parsed into `config_type="Database"` and `value="config.yml"`
4. Wired to main pipeline inputs
5. Validated by main logic

---

## Parser Pipeline Pattern

The parser pipeline is a **separate, complete pipeline** that extracts structured data from the formatted string.

### Parser Requirements

```polyglot
{|} |ParserPipeline
[t] |T.Call  // Must be callable

[<] <formatted_string:pg.string  // Input from %Formatted_string
[>] >field1:type                  // Outputs wire to main inputs
[>] >field2:type

// Parsing logic (often uses runtime wrappers)
{x}
```

### Parser Output → Main Input Wiring

```polyglot
[%] %Pipeline.Inline
   [%] |ParserPipeline
   (|) <formatted_string:pg.string << %Formatted_string
   (|) >parsed_field :type >> <main_input  // ← Direct wiring
```

**Key Pattern:** Parser outputs bind directly to main pipeline inputs using `>output >> <input` syntax.

---

## Additional Examples

### Email Parser

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
   (|) <formatted_string:pg.string << %Formatted_string
   (|) >user :pg.string >> <user
   (|) >domain :pg.string >> <domain

[<] <user :pg.string
[<] <domain :pg.string
[>] >is_valid :pg.bool <~ #False

// Email validation logic
{x}
```

**Usage:**
```polyglot
[r] |ValidateEmail "admin@example.com"
(|) >is_valid >> $valid
```

---

## Best Practices

### 1. Parser Pipeline Separation

Always define the parser as a **separate pipeline**, not inline:

```polyglot
✅ Good - Separate parser:
{|} |Parser
[t] |T.Call
// ...
{x}

{|} |Main
[%] %Pipeline.Inline
   [%] |Parser
   // ...
{x}

❌ Bad - Trying to inline parser logic:
{|} |Main
[%] %Pipeline.Inline
   // Cannot define pipeline logic here
{x}
```

### 2. Use Runtime Wrappers for Parsing

For complex parsing, use Python/JavaScript runtime wrappers:

```polyglot
✅ Recommended:
[w] |W.Runtime.Python3.11
[r] |RT.Python.Run.Code
(|) <code << |U.String.Python""
[+] +"def parse(s): ..."

❌ Avoid: Manual string manipulation in Polyglot
```

### 3. Always Bind %Formatted_string

The parser must receive the formatted string:

```polyglot
✅ Required:
[%] %Pipeline.Inline
   [%] |Parser
   (|) <formatted_string:pg.string << %Formatted_string

❌ Missing binding - parser won't receive input:
[%] %Pipeline.Inline
   [%] |Parser
```

### 4. Wire All Parser Outputs

Ensure all parser outputs wire to main inputs:

```polyglot
✅ Complete wiring:
(|) >field1 :type >> <field1
(|) >field2 :type >> <field2

❌ Missing wiring - data lost:
(|) >field1 :type  // Not wired to main input
```

---

## Technical Notes

### Execution Order

1. User invokes main pipeline with inline string
2. `%Pipeline.Inline` metadata triggers
3. `%Formatted_string` captures inline string argument
4. Parser pipeline executes with formatted string as input
5. Parser outputs wire to main pipeline inputs
6. Main pipeline executes with parsed inputs

### Type Safety

- Parser output types **must match** main input types
- Type checking occurs at compile time
- Mismatched types produce compilation errors

### Performance

- Parser pipeline executes **once** per inline call
- No caching between calls
- For repeated parsing, consider caching parsed results

---

## Common Patterns

### Pattern 1: Simple Key-Value Parser

```polyglot
{|} |ParseKeyValue
[t] |T.Call
[<] <formatted_string:pg.string
[>] >key :pg.string
[>] >value :pg.string

// Parse "key:value" format
{x}
```

### Pattern 2: Multi-Field Parser

```polyglot
{|} |ParseURL
[t] |T.Call
[<] <formatted_string:pg.string
[>] >protocol :pg.string
[>] >host :pg.string
[>] >port :pg.int
[>] >path :pg.string

// Parse "protocol://host:port/path" format
{x}
```

### Pattern 3: Enum-Based Parser

```polyglot
{|} |ParseCommand
[t] |T.Call
[<] <formatted_string:pg.string
[>] >action :#ActionType
[>] >target :pg.string

// Parse "action:target" into enum + string
{x}
```

---

## See Also

- [Special Variables](/docs/User/language/types/special-variables.md) - %Formatted_string documentation
- [Runtime Wrappers](/docs/User/stdlib/wrappers/runtime-wrappers.md) - Python/Rust/JS runtime integration
- [Metadata System](/docs/User/language/advanced/metadata.md) - All metadata types
- [Complete Examples](/docs/User/examples/inline-pipeline-parser.md) - More working examples

---

**Added:** 2025-12-22 (Training Session)
**Verified:** Human review and correction in training session
**Source:** L-2025-12-22-007
**Target Path:** `docs/User/language/advanced/pipeline-inline-metadata.md`
