---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: core-principles
shard: false

# --- Classification ---
type: guide
topic: Core Principles
summary: Introduction to Core Principles
keywords:
  - beginner
  - getting-started
  - introduction

# --- BMAD Agent Routing ---
agents:
  - developer
  - tech-writer
phase: any
workflow: greenfield
module: bmm
complexity: low

# --- Dependency Chain ---
prereqs:
  []
unlocks:
  - language-syntax

# --- Relationships ---
related:
  []

# --- Metadata ---
status: stable
updated: 2025-12-16
version: 0.0.4
tags:
  - "#getting-started"
  - "#beginner"
---
# Core Principles

**What You'll Learn:**
- The 6 foundational principles of Polyglot
- Why Polyglot was designed this way
- How these principles guide all language features

---

## The 6 Core Principles

### 1. No Keywords

**Polyglot has zero keywords.** Instead, it uses:
- **Markers** - Square-bracketed symbols: `[r]`, `[|]`, `{|}`
- **Operators** - Prefix symbols: `$`, `:`, `#`, `|`, `!`, `@`, `%`

**Why?**
- **Extensibility:** No reserved words means no naming conflicts
- **Clarity:** Syntax is visual and consistent
- **Simplicity:** Learn markers and operators, not hundreds of keywords

**Example:**

```polyglot
[r] $name :string << "Alice"       // No "let", "var", "const"
[f] $age >? 18                     // No "if"
[m] $result << $status             // No "switch", "match"
```

**Compare to other languages:**

```javascript
// JavaScript - many keywords
let name = "Alice";
if (age > 18) { ... }
switch (status) { ... }
```

```polyglot
// Polyglot - zero keywords
[r] $name :string << "Alice"
[f] $age >? 18 ...
[m] $result << $status ...
```

---

### 2. One Line = One Marker + One Expression

**Each line must have:**
1. Exactly **one marker** (`[r]`, `[|]`, `[f]`, etc.)
2. Exactly **one expression** (assignment, operation, etc.)

**Why?**
- **Readability:** Easy to scan and understand
- **Predictability:** No surprises, no hidden complexity
- **Tooling:** Simple to parse and format

**Valid:**

```polyglot
[r] $variable << value             ✅ One marker, one expression
[f] $age >? 18                     ✅ One marker, one expression
[|] <input :string                 ✅ One marker, one expression
```

**Invalid:**

```polyglot
[r] $a << 1; $b << 2               ❌ Two expressions on one line
$variable << value                 ❌ No marker
[r] [r] $x << 1                    ❌ Two markers
```

**Exception: Multi-line strings**

```polyglot
[r] $query << |SQL""
[+] +"SELECT * FROM orders"        // [+] continues previous line
[+] +"WHERE id = 123"
```

Each `[+]` line is a **continuation** of the original `[r]` line.

**Critical: Pipeline Chaining**

When chaining pipelines with `|>`, each chain segment must be on its own line:

```polyglot
// ❌ INCORRECT - Multiple pipeline calls on one line
[r] |Step1 |> |Step2 |> |Step3

// ✅ CORRECT - Each chain segment on its own line
[r] |Step1 |> |Step2                      // Chain Step1 → Step2
[|] <input << $value                     // Input to Step1
[|] >output1 >> <input2                  // Step1 output → Step2 input
[|] |> |Step3                             // Chain Step2 → Step3
[|] >output2 >> <input3                  // Step2 output → Step3 input
[|] |>                                    // End chain
[|] >final >> $result                    // Capture Step3 output
```

**Why?** When multiple pipelines chain on one line, the compiler cannot determine which output connects to which input, especially when parameter names might be identical across pipelines. Separating each chain segment eliminates this ambiguity.

---

### 3. Indentation for Nesting

**Polyglot uses 3-space indentation** to denote nested scopes.

**Why 3 spaces?**
- **Visual clarity:** More visible than 2 spaces
- **Comfortable:** Less deep nesting than 4 spaces
- **Distinctive:** Not tabs, not 2, not 4 - uniquely Polyglot

**Example:**

```polyglot
{|} |OuterPipeline
[|] <input :string

[t] |T.Call
[W] |W.Polyglot.Scope

   [f] $input =? "test"            // 3-space indent (nested in wrapper)
      [r] $result << "matched"     // 6-space indent (nested in conditional)
   [f] *?
      [r] $result << "not matched"

   [|] >output << $result

{x}
```

**Nesting levels:**
- 0 spaces: Pipeline definition level
- 3 spaces: Inside wrapper
- 6 spaces: Inside conditional
- 9 spaces: Inside loop inside conditional

**No `~\` markers** (these were used in v0.0.2, removed in v0.0.3+):

```polyglot
// v0.0.2 (OLD):
[f] $condition
\~\ [r] $result << "value"         // Explicit nesting marker

// v0.0.4 (NEW):
[f] $condition
   [r] $result << "value"          // Indentation only
```

---

### 4. Universal Hierarchy: `PREFIX.identifier.path`

**All constructs use dot `.` for hierarchy navigation.**

**Pattern:**
```
PREFIX.namespace.identifier.field
```

**Examples:**

| Construct | Example | Hierarchy |
|-----------|---------|-----------|
| **Variable with field** | `$user.profile.name` | Variable + field path |
| **Type** | `:pg.array.string` | Namespace + type |
| **Enum value** | `#OrderStatus.Processing` | Type + value |
| **Pipeline** | `\|Database.Users.Find` | Namespace + pipeline |
| **Error** | `!Network.HTTP.Timeout` | Category + domain + error |
| **Registry** | `@Local::MyApp:1.0.0.0` | Registry::name:version |

**Why universal hierarchy?**
- **Consistency:** Same pattern everywhere
- **Predictability:** If you know one, you know all
- **Greppable:** Easy to search for identifiers
- **Namespacing:** Natural organization

**Accessing nested fields:**

```polyglot
[r] $user << #User
   [.] .profile
      [.] .name << "Alice"
      [.] .email << "alice@example.com"

[r] $name :string << $user.profile.name
[r] $email :string << $user.profile.email
```

**Nested types:**

```polyglot
[r] $matrix :array.array.int << {{1, 2}, {3, 4}}
//           ^     ^      ^
//           array of array of int
```

---

### 5. Explicit Over Implicit

**Polyglot favors explicitness** - no magic behavior, no hidden assumptions.

**Why?**
- **Clarity:** Code does exactly what it says
- **Maintainability:** No surprising side effects
- **AI-friendly:** Behavior is predictable and documented

**Examples of explicitness:**

#### Explicit Types on First Declaration

```polyglot
[r] $name :string << "Alice"       ✅ Type stated
[r] $name << "Alice"               ❌ Type inferred (not allowed)
```

#### Explicit Metadata

```polyglot
{|} |ProcessOrder
[%] %Doc << "Processes customer orders"  // Explicit documentation
[%] %Author
   [.] .name << "Alice"                  // Explicit author

[|] <order_id :string
   [%] %description << "Order identifier"  // Explicit parameter docs
{x}
```

#### Explicit Error Handling

```polyglot
[z] $data << |FetchFromAPI""
[z][!] !Network.* ? "Network error"      // Explicit error handling
[z][!] *! ? "Unknown error"
```

No implicit `try-catch` - you must specify error handlers.

#### Explicit Defaults

```polyglot
{|} |ProcessOrder
[|] <priority :string
   [%] %default << "medium"              // Explicit default value
{x}
```

#### Explicit Triggers and Wrappers

```polyglot
{|} |MyPipeline
[t] |T.Call                              // Explicit trigger (required)
[W] |W.Polyglot.Scope                    // Explicit wrapper (required)
{x}
```

**Contrast with implicit:**

```python
# Python - implicit type, implicit error handling
name = "Alice"                  # Type inferred
data = fetch_from_api()         # May throw, no explicit handling
```

```polyglot
# Polyglot - explicit type, explicit error handling
[r] $name :string << "Alice"           # Type stated
[z] $data << |FetchFromAPI             # Explicit try block
[z][!] *! ? "Error occurred"
```

---

### 6. Variable Prefix: `$` (Not `,`)

**All variables and parameters use `$` prefix.**

**Why `$` instead of `,`?**

**Problem with comma:**
```polyglot
[f] ,age ?[,min_age, ,max_age]     ❌ AMBIGUOUS!
//      ^  ^         ^
//      comma separator or variable prefix?
```

Is `,min_age` a variable or part of the range syntax?

**Solution with `$`:**
```polyglot
[f] $age ?[$min_age, $max_age]     ✅ CLEAR!
//      ^  ^         ^
//      comma is separator, $ is prefix
```

**Benefits of `$`:**
1. **No ambiguity** - Comma `,` is exclusively a separator
2. **Familiar** - Shell, PHP, Perl, PowerShell use `$`
3. **Clear visual distinction** - Variables stand out
4. **Greppable** - Search for `grep "\$user"` to find variable usage
5. **Consistent** - Same prefix for variables and parameters

**Examples:**

```polyglot
[r] $name :string << "Alice"              // Variable
[|] <input_param :string                  // Parameter (uses $ in calls)
[r] $result << |MyPipeline <input << $name  // $name as argument
```

---

## How Principles Work Together

### Example: Complete Pipeline

```polyglot
{|} |ProcessOrder                  // Principle 1: No keywords
[%] %Doc << "Process order"        // Principle 5: Explicit metadata

[|] <order_id :string              // Principle 6: $ prefix (in calls)
[|] >result :string                // Principle 2: One marker, one expression

[t] |T.Call                        // Principle 5: Explicit trigger
[W] |W.Polyglot.Scope              // Principle 5: Explicit wrapper

[r] $order << |Database.Orders.Find  // Principle 4: Hierarchy (.Orders.Find)
[|] <order_id << $order_id           // Principle 6: $ prefix ($order_id)

[f] $order.status =? "pending"       // Principle 4: Hierarchy ($order.status)
   [r] $result << "Processing"       // Principle 3: Indentation (3 spaces, nested in conditional)
[f] *?
   [r] $result << "Already processed"

[|] >result << $result

{x}
```

**Every line follows the principles:**
- No keywords anywhere
- Each line: one marker + one expression
- Indentation shows nesting (3 spaces for conditional branches)
- Hierarchy with dots (`.Orders.Find`, `$order.status`)
- Explicit metadata, trigger, wrapper
- Variables use `$` prefix
- Wrapper `[W]` doesn't create nesting - pipeline logic at same level

---

## Consequences of Principles

### Extensibility

**No keywords → No conflicts:**

```polyglot
[r] $if :string << "valid variable name"      ✅ "if" is not reserved
[r] $class :string << "also valid"            ✅ "class" is not reserved
[r] $function :string << "no problem"         ✅ "function" is not reserved
```

You can use any identifier without worrying about future keyword additions.

### Consistency

**Universal hierarchy → Predictable syntax:**

If you know how to access `$user.profile.name`, you automatically know:
- `$config.database.host` (variable fields)
- `:pg.array.string` (type namespace)
- `#OrderStatus.Processing` (enum values)
- `|Database.Users.Find` (pipeline namespace)

Same pattern, different contexts.

### Simplicity

**One line = one marker + one expression → Easy to read:**

```polyglot
[r] $name << "Alice"
[r] $age << 30
[r] $email << "alice@example.com"
```

Scan down the `[r]` markers - you instantly see three sequential assignments.

### Tooling-Friendly

**Explicit syntax → Simple parsing:**

```
[marker] $variable :type << value
   ^        ^       ^      ^
 Marker  Variable  Type  Operator + Expression
```

Every line has predictable structure - easy for:
- Syntax highlighters
- Formatters
- Linters
- AI assistants
- IDEs

---

## Design Philosophy

### Orchestration Language

**Polyglot is an orchestrator, not a general-purpose language.**

**What this means:**
- **No binary operations:** No `$x * 2`, `$a + $b`
- **No mutables:** Variables are immutable after becoming Final
- **Calls external runtimes:** Python, Rust, JavaScript, etc. do computation
- **Focuses on coordination:** Connecting pipelines, managing data flow

**Example:**

```polyglot
// ❌ Not valid Polyglot (no binary operations):
[r] $doubled :int << $num * 2

// ✅ Valid Polyglot (inline pipeline call with formatted string):
[r] $doubled :int << |U.Math.Double"{$num}"
```

**Note:** `"{$num}"` is a **formatted string template**, not an argument. The variable `$num` is converted to its string representation, then passed to the pipeline's formatter which parses it back to the required type.

**Why?**
- Polyglot coordinates **between** languages/systems
- Each runtime does what it's best at (Python for ML, Rust for performance)
- Polyglot provides **unified interface** and **data flow control**

### Pipeline-Centric

**Everything is a pipeline.**

Even simple operations use **inline pipeline calls**:

```polyglot
[r] $timestamp :string << |DT.Now""                    // No parameters (empty string)
[r] $doubled :float << |U.Math.Double"{5.0}"           // Literal value formatted
[r] $uppercase :string << |String.Upper"{$name}"       // Variable formatted
[r] $sum :int << |U.Math.Add"{$x}, {$y}"               // Multiple variables
```

**Inline syntax** (`|Pipeline"{template}"`) is the most common feature in Polyglot:
- `{$var}` - Variable with default string representation
- `{$var:format}` - Variable with specific format (e.g., `:hex`, `:json`)
- Literal text preserved as-is in the template

**Behind the scenes - Three-Phase Execution:**

When you write `|U.Math.Add"{$x}, {$y}"`:

1. **Phase 1:** Variables converted to strings (parallel execution)
   - `$x` → `|U.String.Polyglot.Int.Default` → `"5"`
   - `$y` → `|U.String.Polyglot.Int.Default` → `"3"`

2. **Phase 2:** String substitution into template
   - Template: `"{$x}, {$y}"`
   - Result: `"5, 3"`

3. **Phase 3:** Formatter pipeline parses the string
   - Special variable `%Inline.FormattedString` contains `"5, 3"`
   - Formatter parses and outputs to main pipeline's input parameters
   - Main pipeline executes with parsed values

**Key Point:** This is NOT simple argument passing—it's a sophisticated template system that enables flexible string-based parameter formatting while maintaining type safety.

**See:** [Inline Pipelines](../User/language/advanced/inline-pipelines.md) for complete details on `%Inline.FormattedString` and formatter pipelines

**Why?**
- **Consistency:** Same calling pattern for all operations
- **Composability:** Pipelines can chain naturally
- **Testability:** Every operation is a pipeline → easy to test/mock
- **Convenience:** Single-line calls for common operations

---

## Comparison with Other Languages

### Python
```python
# Keywords: def, if, for, class, import, try, except, etc.
def process_order(order_id, priority="medium"):
    if priority == "high":
        result = "express"
    else:
        result = "standard"
    return result
```

### Polyglot
```polyglot
# Zero keywords, markers instead
{|} |ProcessOrder
[|] <order_id :string
[|] <priority :string
   [%] %default << "medium"

[t] |T.Call
[W] |W.Polyglot.Scope

   [m] $result << $priority
      [?] "high" ? "express"
      [?] * ? "standard"

   [|] >result << $result
{x}
```

---

## Summary

### The 6 Principles

1. **No Keywords** - Only markers and operators
2. **One Line = One Marker + One Expression** - Consistent structure
3. **Indentation for Nesting** - 3-space indentation
4. **Universal Hierarchy** - `PREFIX.identifier.path` everywhere
5. **Explicit Over Implicit** - No magic behavior
6. **Variable Prefix: `$`** - Clear, unambiguous, familiar

### Why These Principles?

- **Clarity:** Code is easy to read and understand
- **Consistency:** Same patterns across all features
- **Extensibility:** No reserved words, infinite namespacing
- **Tooling-Friendly:** Simple, predictable syntax
- **AI-Optimized:** Explicit structure, no hidden behavior

### Result

A language that is:
- **Greppable:** Easy to search (`grep "\$user"`, `grep "\\|Database"`)
- **Scannable:** Visual markers guide the eye
- **Predictable:** Few rules, applied universally
- **Maintainable:** Explicit intent, no surprises

---

## Related Documentation

- [Prefix System](./prefix-system.md) - All prefixes (`$`, `:`, `#`, etc.)
- [Markers Reference](./markers.md) - All markers (`[r]`, `[|]`, etc.)
- [Variables & Lifecycle](./variables-lifecycle.md) - Variable prefix and states
- [Pipeline Structure](./pipeline-structure.md) - Pipeline orchestration

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../README.md)
