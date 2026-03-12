<!-- ARCHIVED: 2025-12-16 | Reason: Superseded by reorganized v0.0.4 specification | Superseded by: See /language/ and /stdlib/ -->

# Polyglot v0.0.4 - Complete Language Specification

**Version:** v0.0.4
**Status:** ✅ Finalized - December 2025
**Implementation Target:** Q2 2026 (Epic 1 implements v0.0.3)
**Last Updated:** 2025-12-14

---

## Quick Navigation

**🚀 Getting Started:**
- [Core Principles](#core-principles) - Language philosophy
- [Variable Prefix: `$`](#variable-prefix) - Why `$` not `,`
- [I/O Operators](#io-operators) - Critical distinction

**📖 Major Features:**
- [Loop System](./language/advanced/loop-system.md) - Unpack/pack with `[~]` and `[*]`
- [Reserved Indication](./language/advanced/reserved-indication.md) - Semicolon `;` system
- [Metadata System](./language/advanced/metadata-system.md) - `%` prefix annotations

**📚 Core Syntax:**
- [Prefix System](#prefix-system) - All prefixes (`$`, `:`, `#`, `|`, `!`, `@`, `%`)
- [Markers](#markers) - All bracket markers
- [Operators](#operators) - Complete operator reference

**🔗 Related Documentation:**
- [Migration from v0.0.3](./migrations/v0.0.3-to-v0.0.4.md)
- [Examples](../../project/examples/)
- [User Guide](../../user/)

---

## Core Principles

### 1. No Keywords
Only markers (`[r]`, `[|]`, `{|}`) and operators (`<<`, `>>`, `?`).

### 2. One Line = One Marker + One Expression
```polyglot
[r] $variable << value             ✅ One marker, one expression
[r] $a << 1; $b << 2               ❌ Multiple expressions
```

### 3. Indentation for Nesting
3 spaces for nested scopes (no `\~\` markers from v0.0.2).

### 4. Universal Hierarchy: `PREFIX.identifier.path`
All constructs use dot `.` for hierarchy navigation.

### 5. Explicit Over Implicit
Metadata makes intent clear, no magic behavior.

### 6. Variable Prefix: `$`
Dollar sign for all variables and parameters (not `,` due to separator ambiguity).

---

## Variable Prefix

### Why `$` Not `,`

**Problem with comma:**
```polyglot
[f] ,age ?[,min_age, ,max_age]     ❌ AMBIGUOUS!
//      ^  ^         ^
//      | comma separator or prefix?
```

**Solution with dollar:**
```polyglot
[f] $age ?[$min_age, $max_age]     ✅ CLEAR!
//      ^  ^         ^
//      comma is separator, $ is prefix
```

**Benefits:**
- No ambiguity with comma separator
- Familiar from shell, PHP, Perl, PowerShell
- Clear visual distinction from hierarchy (`.`)
- Greppable: `grep "\$user"`

---

## Prefix System

Complete prefix reference:

| Prefix | Purpose | Example | Hierarchy |
|--------|---------|---------|-----------|
| `$` | Variable/parameter | `$user`, `$user.profile.name` | Variable + field access |
| `:` | Type path | `:pg.string`, `:array.int` | Namespace.type |
| `#` | Enum/struct | `#OrderStatus.Processing` | Type.value |
| `\|` | Pipeline | `\|Database.Users.Find` | Namespace.pipeline |
| `!` | Error | `!Network.HTTP.Timeout` | Category.domain.error |
| `@` | Registry | `@Local::MyApp:1.0.0.0` | Registry::name:version |
| `%` | Metadata | `%Doc`, `%Author` | Property |

**Pattern:** Every construct has unique prefix + dot-separated hierarchy.

**See:** [Prefix System Details](./language/syntax/prefix-system.md) *(to be created)*

---

## I/O Operators

### Critical Distinction

**Two contexts for I/O:**

#### 1. Pipeline DEFINITION (Signature)

```polyglot
{|} |MyPipeline
[|] <input_param :string           // Define input
[|] >output_param :string          // Define output
{x}
```

**Operators:**
- `[|] <label` - Input parameter definition
- `[|] >label` - Output parameter definition

#### 2. Pipeline CALL (Usage)

```polyglot
[r] |MyPipeline
[|] <input_param << $my_value      // Pass input
[|] >output_param >> $result       // Capture output
```

**Operators:**
- `<label <<` - Pass input value
- `>label >>` - Capture output value

### Default Values

```polyglot
[|] <timeout :int <~ 30            // Default value operator
```

**Operator `<~`:** Assigns default when parameter not provided.

**See:** [I/O Operators Guide](./language/syntax/io-operators.md) *(to be created)*

---

## Markers

Complete marker reference:

### Block Delimiters
- `{@}` - Registry definition
- `{|}` - Pipeline definition
- `{#}` - Enum/struct definition
- `{!}` - Error definition
- `{A}` - Alias definition block
- `{x}` - Close block

### I/O Markers
- `[|]` - Universal I/O marker (definition and call)
- `[~]` - Unpack (loop: main → iteration)
- `[*]` - Pack (loop: iteration → main/next)

### Execution Markers
- `[r]` - Runtime variable / Sequential loop
- `[p]` - Parallel execution / Parallel loop
- `[b]` - Background / Fire-and-forget loop
- `[f]` - Conditional (yes)
- `[m]` - Match expression
- `[?]` - Match case
- `[z]` - Try block

### Other Markers
- `[t]` - Trigger
- `[W]` - Wrapper
- `[A]` - Alias/attribute
- `[.]` - Field
- `[+]` - Multi-line continuation
- `[%]` - Metadata
- `[v]` - Vacuum/aggregation
- `[Q]` - Queue
- `[&]` - AND condition
- `[^]` - XOR condition

**See:** [Markers Reference](./language/syntax/markers.md) *(to be created)*

---

## Operators

### Assignment & Flow
- `<<` - Assignment/input
- `>>` - Output/capture
- `<~` - Default value
- `<<<` - Variadic input (via metadata)
- `>>>` - Variadic output (via metadata)

### Conditionals
- `=?` / `!=?` - Equality / Not equal
- `>?` / `!>?` - Greater / Not greater (<=)
- `<?` / `!<?` - Less / Not less (>=)
- `>=?` / `!>=?` - Greater-equal / Not
- `<=?` / `!<=?` - Less-equal / Not

### Range
- `?[min, max]` - Inclusive both
- `?(min, max]` - Exclusive min, inclusive max
- `?[min, max)` - Inclusive min, exclusive max
- `?(min, max)` - Exclusive both

### Collection
- `in?` / `!in?` - In collection / Not in
- `re?` / `!re?` - Regex match / Not match

### Composition
- `|>` - Pipeline composition

### Hierarchy
- `.` - Universal hierarchy separator

### Match
- `?` - Match separator (`[?] pattern ? result`)

**See:** [Operators Reference](./language/syntax/operators.md) *(to be created)*

---

## Major Features

### Loop System

**Full specification:** [Loop System](./language/advanced/loop-system.md)

**Quick example:**
```polyglot
[p] ~ForEach.Array
[~] <array << $items               // Unpack from main
[~] >item >> $element              // Output to iteration
   [r] $processed << |Transform <input << $element
   [v] *Into.Array                 // Pack back to main
   [*] <item << $processed
   [*] >array >> $results
```

**Key operators:**
- `[~]` - Unpack (main → iteration)
- `[*]` - Pack (iteration → main or next)
- `[v]` - Vacuum/aggregation operation

### Reserved Indication

**Full specification:** [Reserved Indication System](./language/advanced/reserved-indication.md)

**Quick example:**
```polyglot
#;Boolean;True                     // Fully reserved
#OrderStatus.Processing            // Fully user-defined
#;DT;Business.FiscalYear.Q1       // Mixed: reserved;user
```

**Key syntax:**
- `;` - Reserved segment (core devs only)
- `.` - User-defined segment
- Mix to extend reserved namespaces

### Metadata System

**Full specification:** [Metadata System](./language/advanced/metadata-system.md)

**Quick example:**
```polyglot
{|} |ProcessOrder
[%] %Doc << "Processes order with validation"
[%] %Author
   [.] .name << "Alice <alice@example.com>"
   [.] .since << "1.0.0"

[|] <order_id:string
   [%] %description << "Unique identifier"
{x}
```

**Key metadata:**
- `%Doc` - Documentation
- `%Author` - Author tracking
- `%Deprecated` - Deprecation with migration
- `%Reserved` - Reserved indication (privileged)

---

## Additional Features

### Inline Pipelines

```polyglot
[r] $timestamp << |DT.Now""
[r] $doubled << |Double"{$value}"
[r] $safe_sql << |SQL"{$input}"
```

**Types:** SQL, HTML, JSON, URL, Regex, Shell escaping.

### Multi-line Strings

```polyglot
[r] $query << |SQL""
[+] +"SELECT * FROM orders"
[+] +"WHERE order_id = "
[+] +|SQL"{$order_id}"
```

**Safeguard against silent concatenation bugs.**

### Collection Literals

```polyglot
[r] $array:array.int << {1, 2, 3, 4, 5}
[r] $set:set.int << {1, 2, 3, 4, 5}
```

### Match Expressions

```polyglot
[m] $result << $value
   [?] "case1" ? "result1"
   [?] "case2" ? "result2"
   [?] * ? "default"               // Exhaustive (required)
```

### Early Return

```polyglot
[f] $input =? ""
   [|] >error << !InvalidInput"Empty input"
```

### Struct Shorthand

```polyglot
[r] $name << "Alice"
[r] $email << "alice@example.com"

[r] $user << #User
   [.] .name                       // << $name (implicit)
   [.] .email                      // << $email (implicit)
```

### Enum with Value Fields

```polyglot
{#} #Result
[.] .success
   [.] .data:serial
[.] .error
   [.] .code:int
{#}
```

---

## Modular Documentation

### Core Syntax
- [Prefix System](./language/syntax/prefix-system.md) *(to be created)*
- [I/O Operators](./language/syntax/io-operators.md) *(to be created)*
- [Markers Reference](./language/syntax/markers.md) *(to be created)*
- [Operators Reference](./language/syntax/operators.md) *(to be created)*
- [Enums & Structs](./core-syntax/enums.md) *(to be created)*
- [Variables & Types](./core-syntax/variables-types.md) *(to be created)*

### Features
- ✅ [Loop System](./language/advanced/loop-system.md)
- ✅ [Reserved Indication](./language/advanced/reserved-indication.md)
- ✅ [Metadata System](./language/advanced/metadata-system.md)
- [Error Handling](./features/error-handling.md) *(to be created)*
- [Inline Pipelines](./features/inline-pipelines.md) *(to be created)*
- [Variadic Input](./features/variadic-input.md) *(to be created)*
- [Pipeline Composition](./features/pipeline-composition.md) *(to be created)*

### Syntax Refinement (Archive)
- [Archived Specifications](./syntax-refinement/archive/) - Historical design docs

---

## Complete Example

**Comprehensive pipeline showcasing major features:**

```polyglot
{@} @Local::OrderProcessing:1.0.0.0
[A] @OrderProc
[<] @PgTypes
[<] @OrderData << @Local::OrderDBOps:1.0.0.0

[%] %Doc <<
[+] +"Order processing module"

[%] %Author
   [.] .name << "Alice <alice@example.com>"
   [.] .since << "1.0.0"
{x}

// File-scope enum
{#} #OrderResult
[.] .success
   [.] .order_id:string
[.] .error
   [.] .code:int
{#}

// Main pipeline
{|} |ProcessOrder
[%] %PublicTo << {@OrderProc}

[|] <order_id:string
   [%] %description << "Unique identifier"
[|] <priority:string
   [%] %description << "Priority level"
   [%] %default << "medium"

[t] |T.Call
[W] |W.Polyglot.Scope

// Multi-line SQL
[r] $query << |SQL""
[+] +"SELECT * FROM orders"
[+] +"WHERE order_id = "
[+] +|SQL"{$order_id}"

// Collection literal
[r] $valid_priorities << {"high", "medium", "low"}

// Validation with early return
[f] $priority in? $valid_priorities
   [r] $valid << #;Boolean;True
[f] *?
   [|] >error << !InvalidOrder"Invalid priority"

// Match expression
[m] $processing << $priority
   [?] "high" ? "express"
   [?] "medium" ? "standard"
   [?] "low" ? "economy"
   [?] * ? "standard"

// Loop with collection
[p] ~ForEach.Array
[~] <array << $order_items
[~] >item >> $item
   [r] $enriched << |EnrichItem <item << $item
   [v] *Into.Array
   [*] <item << $enriched
   [*] >array >> $enriched_items

// Struct with shorthand
[r] $order_id << "ORD-123"
[r] $created << |DT.Now""

[r] $result << #OrderResult.success
   [.] .order_id                  // Shorthand
   [.] .created << $created

[|] >result << $result
{x}
```

---

## Migration from v0.0.3

**Major changes:**
1. **Variables:** `,name` → `$name`
2. **Blocks:** `[|]...[X]` → `{|}...{x}`
3. **IO:** `[i]`/`[o]` → `[|] <param`/`[|] >param`
4. **Types:** `pg\string` → `:pg.string`
5. **Reserved:** `#True` → `#;Boolean;True`
6. **Loops:** New unpack/pack system with `[~]`/`[*]`

**See:** [Migration Guide](./migrations/v0.0.3-to-v0.0.4.md) *(to be created)*

---

## Implementation Status

**Current:** Epic 1 implements v0.0.3
**Target:** v0.0.4 implementation Q2 2026
**PRD:** [Product Requirements](../../project/prd.md)
**Epics:** [Epic Breakdown](../../project/epics.md)

---

## Version History

**See:** [Version Index](../VERSION-INDEX.md) for complete version matrix

**v0.0.4 highlights:**
- Variable prefix: `$`
- Loop system: `[~]`/`[*]` operators
- Reserved indication: `;` semicolon
- Metadata enhancements
- I/O operator distinction
- Match exhaustiveness
- Inline pipelines

---

**Last Updated:** 2025-12-14
**Maintained By:** Documentation Team
**For Questions:** See [User Guide](../../user/) or [GitHub Issues](https://github.com/polyglot/issues)
