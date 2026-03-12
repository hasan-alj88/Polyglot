---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: markers
shard: false

# --- Classification ---
type: reference
topic: Markers Reference
summary: Reference for Markers Reference
keywords:
  - syntax
  - reference
  - language

# --- BMAD Agent Routing ---
agents:
  - developer
  - architect
phase: planning
workflow: any
module: bmm
complexity: medium

# --- Dependency Chain ---
prereqs:
  - core-principles
unlocks:
  - control-flow
  - type-system

# --- Relationships ---
related:
  []
parent: language-syntax

# --- Metadata ---
status: stable
updated: 2025-12-16
version: 0.0.4
tags:
  - "#syntax"
  - "#reference"
---
# Markers Reference

**What You'll Learn:**
- Complete reference of all Polyglot markers
- Distinction between markers and operators
- Usage context for each marker
- Common patterns and examples

---

## Markers vs Operators

### Critical Distinction

**Markers** are the **square-bracketed symbols** that denote execution context and structure:
- Examples: `[r]`, `[p]`, `[|]`, `[~]`, `[*]`, `[s]`, `{|}`

**Operators** are the **prefix symbols** used in identifiers:
- Examples: `|Pipeline`, `#Enum`, `!Error`, `~ForEach`, `*Into.Array`, `$variable`

```polyglot
[r] $result << |ProcessData        // [r] is MARKER, | is OPERATOR
[p] ~ForEach.Array                 // [p] is MARKER, ~ is OPERATOR
[*] <item << $value                // [*] is MARKER
```

**Remember:** If it's in square brackets `[ ]` or curly braces `{ }`, it's a marker!

### Documentation Conventions

**Placeholders in examples:**
- `:datatype` - Represents any type (`:string`, `:int`, etc.)
- `$variable` - Represents any variable name
- `|SomePipeline` - Represents any pipeline name

**NOT placeholders:**
- `[z]` - Specifically means "try block", not "any marker"
- Use actual markers (`[r]`, `[p]`, `[b]`, `[s]`, `[v]`, `[t]`, `[W]`) when demonstrating execution

---

## Block Delimiters

Markers that define and close scope blocks.

### `{@}` - Registry Definition Start
Opens a registry (package) definition block.

```polyglot
{@} @Local::OrderProcessing:1.0.0.0
[A] @OrderProc
[<] @PgTypes
{x}
```

### `{|}` - Pipeline Definition Start
Opens a pipeline definition block.

```polyglot
{|} |ProcessOrder
[|] <order_id :string
[|] >result :string
{x}
```

### `{#}` - Enum/Struct Definition Start
Opens an enum or struct definition block.

```polyglot
{#} #OrderStatus
[.] .pending
[.] .processing
[.] .completed
{x}
```

### `{!}` - Error Definition Start
Opens an error definition block.

```polyglot
{!} !InvalidOrder
[.] .message :string
[.] .code :int
{x}
```

### `{A}` - Alias Definition Block
Opens an alias definition block (rare usage).

```polyglot
{A}
[A] @MyAlias << @Full::Package::Name:1.0.0.0
{x}
```

### `{x}` - Close Block
Closes any block opened with `{@}`, `{|}`, `{#}`, `{!}`, or `{A}`.

**Universal closer:** Always lowercase `x`, works for all block types.

---

## I/O Markers

Markers for pipeline input/output and data flow.

### `[|]` - Universal I/O Marker

Used in **two contexts:**

#### Context 1: Pipeline Definition (Parameters)
```polyglot
{|} |MyPipeline
[|] <input_param :string          // Input parameter definition
[|] >output_param :string         // Output parameter definition
{x}
```

#### Context 2: Pipeline Call (Arguments)
```polyglot
[r] |MyPipeline
[|] <input_param << $my_value     // Pass input value
[|] >output_param >> $result      // Capture output value
```

**Key Point:** Same `[|]` marker, different operators (`<param` vs `<param <<`)

### `[~]` - Unpack Marker
Transfers data from main scope to iteration scope in loops.

```polyglot
[p] ~ForEach.Array
[~] <array << $items              // Unpack: main → iteration
[~] >item >> $element             // Output to iteration scope
   // ... iteration logic ...
```

**Direction:** Main scope **→** Iteration scope

### `[*]` - Pack Marker
Transfers data from iteration scope back to main scope (or next iteration).

```polyglot
   // ... iteration logic ...
   [v] *Into.Array                // Pack aggregator
   [*] <item << $processed        // Pack: iteration → main
   [*] >array >> $results         // Output to main scope
```

**Direction:** Iteration scope **→** Main scope (or next iteration)

**See Also:** [I/O Operators - Variadic Parameters](./io-operators.md#variadic-parameters) for `<<<` and `>>>` variadic operators.

---

## Execution Markers

Markers that control how code executes.

### `[r]` - Sequential Execution (Run)
Executes statement sequentially in order.

```polyglot
[r] $name :string << "Alice"
[r] $email :string << "alice@example.com"
[r] $user << #User
   [.] .name
   [.] .email
```

**Context:**
- Variable declarations
- Sequential loops (when used with `~ForEach`)
- Ordered execution

### `[p]` - Parallel Execution
Executes statement in parallel (when possible).

```polyglot
[p] $result1 :string << |SlowOperation1
[p] $result2 :string << |SlowOperation2
// Both execute concurrently
```

**Context:**
- Independent operations
- Parallel loops (when used with `~ForEach`)
- Non-blocking calls

### `[b]` - Background Execution
Executes statement in background (fire-and-forget).

```polyglot
[b] |LogToFile <message << "Processing started"
// Continues immediately without waiting
```

**Context:**
- Logging
- Fire-and-forget loops
- Non-critical operations

### `[f]` - Fork / Conditional
Creates a forked execution path (conditional branch).

```polyglot
[f] $age >? 18
   [r] $status << "adult"
[f] *?
   [r] $status << "minor"
```

**Mnemonic:** `y` looks like a visual **fork** in the road
**Context:** Conditionals, if-then-else logic

### `[v]` - Join
Joins/synchronizes execution paths or aggregates loop results.

```polyglot
[p] ~ForEach.Array
[~] <array << $items
[~] >item >> $element
   [r] $processed << |Transform <input << $element
   [v] *Into.Array                // Join/aggregate results
   [*] <item << $processed
   [*] >array >> $results
```

**Mnemonic:** `V` looks like a visual **join** (funnel)
**Context:**
- Loop aggregation
- Synchronization points
- Collection operations

**IMPORTANT:** `[v]` is **JOIN**, not "vacuum"!

---

## Control Flow Markers

### `[m]` - Match Expression
Opens a match expression (like switch/case).

```polyglot
[m] $result << $status
   [?] "pending" ? "⏳ Waiting"
   [?] "processing" ? "⚙️ In Progress"
   [?] "completed" ? "✅ Done"
   [?] * ? "❓ Unknown"           // Exhaustive match required
```

### `[?]` - Match Case
Defines a case within a match expression.

```polyglot
[m] $priority_level << $priority
   [?] "high" ? 1
   [?] "medium" ? 2
   [?] "low" ? 3
   [?] * ? 2                      // Default case
```

**Pattern:** `[?] pattern ? result`

### `[z]` - Try Block
Opens an error handling try block.

```polyglot
[z] $data << |FetchFromAPI""
[z][!] !Network.* ? "Network error occurred"
[z][!] *! ? "Unknown error"
```

**IMPORTANT:** `[z]` is NOT a placeholder marker. It specifically means "try block" for error handling. When examples show generic pipeline execution, use actual execution markers like `[r]`, `[p]`, or `[b]`.

---

## Pipeline Structure Markers

### `[t]` - Trigger
Defines trigger conditions for pipeline execution.

```polyglot
{|} |ProcessOrder
[|] <order_id :string

[t] |T.Call                        // Trigger type
[W] |W.Polyglot.Scope              // Wrapper

// ... pipeline logic ...
{x}
```

**REQUIRED** in pipeline definitions
**Position:** After inputs, before queue

### `[Q]` - Queue Control
Defines queue behavior for pipeline instances.

```polyglot
{|} |ProcessOrder
[|] <order_id :string

[t] |T.Call
[Q] |Q.Serial                      // Queue configuration
[W] |W.Polyglot.Scope

// ... pipeline logic ...
{x}
```

**Optional** (has default)
**Position:** After trigger, before wrapper
**Purpose:** Controls concurrent pipeline instances

### `[W]` - Wrapper
Defines runtime wrapper (setup + cleanup).

```polyglot
{|} |ProcessOrder
[|] <order_id :string

[t] |T.Call
[Q] |Q.Serial
[W] |W.Polyglot.Scope              // Contains setup + cleanup

// ... pipeline logic ...
{x}
```

**REQUIRED** in pipeline definitions
**Contains:** Setup code + cleanup code
**Examples:** `|W.Polyglot.Scope`, `|W.RT.Python3.12`

---

## Data Structure Markers

### `[.]` - Field Definition
Defines a field in struct/enum or accesses a field in instances.

#### Context 1: Definition
```polyglot
{#} #User
[.] .name :string
[.] .email :string
[.] .age :int
{x}
```

#### Context 2: Instance Creation
```polyglot
[r] $user << #User
   [.] .name << "Alice"
   [.] .email << "alice@example.com"
   [.] .age << 30
```

#### Context 3: Shorthand
```polyglot
[r] $name << "Alice"
[r] $email << "alice@example.com"

[r] $user << #User
   [.] .name                       // << $name (implicit)
   [.] .email                      // << $email (implicit)
```

### `[s]` - Serial Load Block ⭐ NEW
Loads serial data (files) with parallel execution and error handling.

#### Context 1: Struct/Enum Field Mapping
```polyglot
{#} #Config
[s] .yaml_file :pg.file.yaml
   [.] .database :string
   [.] .port :int
   [.] .host :string
[s][!] *! ? #Config.error
{x}
```

#### Context 2: Pipeline Execution (Load Entire Content)
```polyglot
[r] $file_path :string << "/etc/config.yaml"
[s] |YAML.Load
[s] <file << $file_path
[s] >content >> $yaml_content :pg.serial
   [.] << *                        // Load entire content
[s][!] *! >> $load_error :!
```

**Key Features:**
- All `[s]` blocks at same level run **in parallel**
- Single error handler `[s][!] *!` applies to all (forall/each)
- Auto-collection of results

**See Also:**
- [Serial Load Block Feature](../User/language/advanced/serial-load-block.md) - Complete feature documentation
- [Enums & Serial Data - Serial Load Blocks](./enums-serial.md#serial-load-blocks) - Usage in enum definitions

---

## Metadata Markers

### `[%]` - Metadata
Defines metadata properties.

```polyglot
{|} |ProcessOrder
[%] %Doc << "Processes customer orders"
[%] %Author
   [.] .name << "Alice <alice@example.com>"
   [.] .since << "1.0.0"

[|] <order_id :string
   [%] %description << "Unique order identifier"
{x}
```

**Common Metadata:**
- `%Doc` - Documentation
- `%Author` - Author information
- `%Deprecated` - Deprecation notices
- `%Reserved` - Reserved indication (privileged)

### Metadata Naming Convention

**PascalCase for global/structural metadata:**
- `%Doc` - Documentation strings
- `%Author` - Author information
- `%Deprecated` - Deprecation notices
- `%Reserved` - Reserved indication
- `%Version` - Version information

**lowercase for parameter-specific metadata:**
- `%description` - Parameter description
- `%example` - Example value
- `%format` - Expected format
- `%default` - Default value
- `%variadic` - Variadic flag
- `%min_items` - Minimum items (variadic)
- `%max_items` - Maximum items (variadic)
- `%valid_values` - Valid value constraints
- `%range` - Numeric range

**Pattern:**
- **PascalCase** (`%Name`) - Applies to entire construct (pipeline, enum, struct)
- **lowercase** (`%name`) - Applies to specific parameter or field

**Example:**
```polyglot
{|} |ProcessOrder
[%] %Doc << "Processes customer orders"          // PascalCase: pipeline-level
[%] %Author                                        // PascalCase: pipeline-level
   [.] .name << "Alice"

[|] <order_id :string
   [%] %description << "Unique order identifier"  // lowercase: parameter-level
   [%] %example << "ORD-12345"                    // lowercase: parameter-level

{x}
```

---

## String Handling Markers

### `[+]` - Multi-line Continuation
Continues a multi-line string or expression.

```polyglot
[r] $query << |SQL""
[+] +"SELECT * FROM orders"
[+] +"WHERE order_id = "
[+] +|SQL"{$order_id}"
```

**Purpose:** Safeguard against silent concatenation bugs

**Pattern:** Each line prefixed with `+`

---

## Special Purpose Markers

### `[A]` - Alias/Attribute
Defines aliases or attributes.

```polyglot
{@} @Local::OrderProcessing:1.0.0.0
[A] @OrderProc                     // Package alias
[<] @PgTypes
{x}
```

**Context:** Package aliases, attribute definitions

### `[<]` - Import
Imports another registry (package).

```polyglot
{@} @Local::MyApp:1.0.0.0
[<] @PgTypes
[<] @Local::DatabaseOps:1.0.0.0
{x}
```

### `[&]` - AND Condition
Logical AND in complex conditions.

```polyglot
[f] $age >=? 18
[&] $has_license =? #True
   [r] $can_drive << #True
```

### `[^]` - XOR Condition
Logical XOR in complex conditions.

```polyglot
[f] $is_admin =? #True
[^] $is_owner =? #True
   [r] $has_edit_rights << #True
```

---

## Boolean Grouping & Indentation

### Understanding Boolean Marker Chains

**Boolean markers create logical chains:**
- `[f]` - Initial condition (fork)
- `[&]` - AND continuation (both conditions must be true)
- `[^]` - XOR continuation (exactly one condition must be true)

**Indentation shows scope:**
- Code indented under boolean markers executes when the condition chain is satisfied
- All code at the same indentation level is part of the same conditional block

### Basic AND Chain

```polyglot
[f] $age >=? 18
[&] $has_license =? #True
[&] $has_insurance =? #True
   [r] $can_drive << #True
   [r] $status << "Authorized driver"
```

**Logic:** Execute only if ALL conditions are true (age >= 18 AND has_license AND has_insurance)

### Basic XOR Chain

```polyglot
[f] $payment_method =? "credit_card"
[^] $payment_method =? "debit_card"
   [r] $processor << "CardProcessor"
   [r] $requires_cvv << #True
```

**Logic:** Execute if EXACTLY ONE is true (credit OR debit, but not both, not neither)

### Multiple Conditional Branches

```polyglot
// Branch 1: Premium user
[f] $user_tier =? "premium"
[&] $subscription_active =? #True
   [r] $max_storage << 1000
   [r] $support_level << "priority"

// Branch 2: Standard user
[f] $user_tier =? "standard"
[&] $subscription_active =? #True
   [r] $max_storage << 100
   [r] $support_level << "regular"

// Branch 3: Free tier (default)
[f] $user_tier =? "free"
   [r] $max_storage << 10
   [r] $support_level << "community"
```

**Indentation:** Each separate `[f]` starts a new independent conditional chain

### Nested Conditions with Indentation

```polyglot
[f] $role =? "admin"
   // Executes only if role == admin
   [r] $base_permissions << ["read", "write", "delete"]

   // Nested condition within admin block
   [f] $department =? "security"
   [&] $clearance_level >=? 3
      [r] $extra_permissions << ["audit", "user_mgmt"]
      [r] |GrantAccess <permissions << $extra_permissions
```

**Indentation levels:**
1. `[f] $role =? "admin"` - Top level condition
2. Code indented once - Executes when admin
3. Nested `[f]` indented once - Sub-condition within admin block
4. Code indented twice - Executes when admin AND (security AND clearance >= 3)

### Complex Multi-Condition Logic

```polyglot
// Authorization logic with multiple checks
[f] $user_authenticated =? #True
[&] $session_valid =? #True
   // User is authenticated with valid session

   [f] $resource_owner =? $current_user_id
      // Owner has full access
      [r] $permissions << ["read", "write", "delete", "share"]

   [f] $is_admin =? #True
   [^] $is_moderator =? #True
      // Admin XOR Moderator (not both) has elevated access
      [r] $permissions << ["read", "write", "delete"]

   [f] $has_viewer_role =? #True
      // Regular viewer has read-only
      [r] $permissions << ["read"]

// Separate branch: Not authenticated
[f] $user_authenticated =? #False
   [r] $permissions << []
   [r] !Unauthorized << "Authentication required"
```

**Complex logic breakdown:**
1. First check: authenticated AND session valid
2. Within authenticated scope, three separate branches:
   - Owner path (most access)
   - Admin XOR Moderator path (elevated access)
   - Viewer path (read-only)
3. Separate top-level branch for unauthenticated users

### XOR for Mutually Exclusive States

```polyglot
{#} #PaymentStatus
[.] .Pending
[.] .Processing
[.] .Completed
[.] .Failed
{x}

// Handle terminal states (exactly one should be true)
[f] $status =? #PaymentStatus.Completed
   [r] $message << "Payment successful"
   [r] |SendReceipt <user << $customer

[f] $status =? #PaymentStatus.Failed
   [r] $message << "Payment failed"
   [r] |SendErrorNotification <user << $customer

// Handle active state
[f] $status =? #PaymentStatus.Processing
   [r] $message << "Processing payment..."
   [r] |UpdateUI <status << $message
```

**Note:** Each `[f]` without continuations (`[&]`, `[^]`) starts an independent conditional branch

### Indentation Best Practices

**✓ GOOD - Clear hierarchy:**
```polyglot
[f] $level >? 10
[&] $score >? 100
   [r] $rank << "expert"
   [r] $bonus << 500

   [f] $streak >? 7
      [r] $extra_bonus << 200
```

**✗ BAD - Inconsistent indentation:**
```polyglot
[f] $level >? 10
[&] $score >? 100
[r] $rank << "expert"        // Wrong: not indented
      [r] $bonus << 500      // Wrong: over-indented
```

**✓ GOOD - Separated branches:**
```polyglot
[f] $option =? "A"
   [r] $result << 1

[f] $option =? "B"
   [r] $result << 2
```

**✗ BAD - Ambiguous continuation:**
```polyglot
[f] $option =? "A"
   [r] $result << 1
[f] $option =? "B"           // Unclear if independent or related
   [r] $result << 2
```

---

## Error Propagation Patterns

### How Errors Flow Through Markers

**Error propagation with `[r]`, `[v]`, and `[^]`:**

Different markers handle error states (Faulted variables) in specific ways:

| Marker | Error Behavior | Example Use Case |
|--------|----------------|------------------|
| `[r]` | Stops execution if variable enters Faulted state | Sequential operations that depend on success |
| `[v]` | Aggregates errors from parallel paths | Join point after parallel execution |
| `[^]` | XOR condition can check Faulted vs non-Faulted | Success XOR failure branching |

---

### Pattern 1: Sequential with Error Check

**Using `[r]` for sequential operations with `[^]` for error branching:**

```polyglot
[r] $data << |FetchData""

// Check if Faulted (failed) or not (success)
[f] $data.state =? :pg.state.faulted
   [r] $message << "Data fetch failed"
   [r] $status << 500

[^] $data.state =? :pg.state.final
   [r] $message << "Data fetch succeeded"
   [r] $status << 200
```

**How it works:**
1. `[r]` attempts operation (may enter Faulted state)
2. `[f]` checks if Faulted
3. `[^]` XOR checks if Final (exactly one must be true)

---

### Pattern 2: Parallel Execution with Join

**Using `[p]` parallel execution with `[v]` join to aggregate errors:**

```polyglot
[p] $result1 << |Operation1
[p] $result2 << |Operation2
[p] $result3 << |Operation3

// Join point - wait for all to complete
[v] *WaitAll

// Check which operations failed
[f] $result1.state =? :pg.state.faulted
   [r] $errors << "Operation1 failed"

[f] $result2.state =? :pg.state.faulted
   [r] $errors << "Operation2 failed"

[f] $result3.state =? :pg.state.faulted
   [r] $errors << "Operation3 failed"
```

**How it works:**
1. `[p]` runs operations in parallel (each may fail independently)
2. `[v]` synchronizes/joins execution (waits for all)
3. Error checks determine which failed

---

### Pattern 3: Try-Catch with Sequential Recovery

**Using `[z]` try block with `[r]` sequential fallback:**

```polyglot
[z] $data << |PrimarySource""
[z][!] *! >> $primary_error

// If primary failed, try fallback
[f] $primary_error.state =? :pg.state.faulted
   [r] $data << |FallbackSource""
   [z][!] *! >> $fallback_error

   // If both failed, use default
   [f] $fallback_error.state =? :pg.state.faulted
      [r] $data << #DefaultData
```

**How it works:**
1. `[z]` tries primary source (may fail → Faulted + error object)
2. `[f]` checks if primary failed
3. `[r]` sequentially tries fallback
4. Nested `[f]` checks if fallback also failed

---

### Pattern 4: Conditional Execution Based on Error State

**Using `[^]` to branch on success vs failure:**

```polyglot
[r] $result << |RiskyOperation""

[f] $result.state =? :pg.state.faulted
   [r] $log_level << "ERROR"
   [r] |Logger.Log <level << $log_level <message << "Operation failed"

[^] $result.state =? :pg.state.final
   [r] $log_level << "INFO"
   [r] |Logger.Log <level << $log_level <message << "Operation succeeded"
```

**Why `[^]` XOR?**
- Variable state is **either** Faulted **or** Final, never both
- `[^]` enforces this mutual exclusivity
- Compiler ensures both paths are handled (exhaustive)

---

### Error State Flow Summary

**Variable states and transitions:**
```
Pending → [r] operation → {Final | Faulted}
                           ↓         ↓
                        Success   Error
```

**Marker behavior with Faulted variables:**
- **`[r]`** - Stops propagating Faulted (doesn't execute if input is Faulted)
- **`[v]`** - Waits for all paths (including Faulted ones) before proceeding
- **`[^]`** - Can check XOR conditions on state (Faulted XOR Final)
- **`[z][!]`** - Catches errors and extracts error object

**Pattern:** Use `[r]` for sequential dependency, `[v]` for parallel aggregation, `[^]` for XOR state branching.

---

## Marker Categories Summary

### Block Structure
- `{@}`, `{|}`, `{#}`, `{!}`, `{A}` - Open blocks
- `{x}` - Close block

### I/O & Data Flow
- `[|]` - Pipeline I/O
- `[~]` - Unpack (main → iteration)
- `[*]` - Pack (iteration → main)
- `[s]` - Serial load ⭐ NEW

### Execution Control
- `[r]` - Sequential
- `[p]` - Parallel
- `[b]` - Background
- `[f]` - Fork
- `[v]` - Join

### Pipeline Structure
- `[t]` - Trigger (REQUIRED)
- `[Q]` - Queue (optional)
- `[W]` - Wrapper (REQUIRED)

### Control Flow
- `[m]` - Match expression
- `[?]` - Match case
- `[z]` - Try block

### Data Structures
- `[.]` - Field definition/access
- `[s]` - Serial load block

### Other
- `[%]` - Metadata
- `[+]` - Multi-line continuation
- `[A]` - Alias
- `[<]` - Import
- `[&]` - AND
- `[^]` - XOR

---

## Common Patterns

### Variable Declaration
```polyglot
[r] $name :string << "value"
```

### Pipeline Definition
```polyglot
{|} |PipelineName
[|] <input :type
[|] >output :type
[t] |T.Call
[W] |W.Polyglot.Scope
   // logic
{x}
```

### Loop Pattern
```polyglot
[p] ~ForEach.Array
[~] <array << $items
[~] >item >> $element
   [r] $processed << |Process <input << $element
   [v] *Into.Array
   [*] <item << $processed
   [*] >array >> $results
```

### Conditional Pattern
```polyglot
[f] $value >? 10
   [r] $result << "high"
[f] *?
   [r] $result << "low"
```

### Match Pattern
```polyglot
[m] $result << $value
   [?] "case1" ? "result1"
   [?] "case2" ? "result2"
   [?] * ? "default"
```

---

## Related Documentation

- [I/O Operators](./io-operators.md) - `<<`, `>>`, `<~` operators
- [Operators Reference](./operators.md) - All operators
- [Pipeline Structure](./pipeline-structure.md) - Pipeline execution order
- [Variables & Lifecycle](./variables-lifecycle.md) - Variable states
- [Loop System](../User/language/advanced/loop-system.md) - `[~]` and `[*]` in detail
- [Serial Load Block](../User/language/advanced/serial-load-block.md) - `[s]` in detail

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../README.md)
