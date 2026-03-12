---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: io-operators
shard: false

# --- Classification ---
type: reference
topic: I/O Operators
summary: Reference for I/O Operators
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
# I/O Operators

**What You'll Learn:**
- Critical distinction between definition and call contexts
- All I/O operators: `<`, `>`, `<<`, `>>`, `<~`, `~>`
- Input/output parameter syntax
- Default value mechanisms
- Common patterns and pitfalls

---

## Critical Distinction: Definition vs Call

**The same markers mean different things in different contexts!**

### Context 1: Pipeline Definition (Signature)

**Declare** what inputs/outputs a pipeline has:

```polyglot
{|} |MyPipeline
[|] <input_param :string           // Define input parameter
[|] >output_param :string          // Define output parameter
{x}
```

**Operators in definition:**
- `[|] <param` - Input parameter definition
- `[|] >param` - Output parameter definition

### Context 2: Pipeline Call (Usage)

**Provide** values for inputs, **capture** values from outputs:

```polyglot
[r] |MyPipeline
[|] <input_param << $my_value      // Pass input value
[|] >output_param >> $result       // Capture output value
```

**Operators in call:**
- `<param <<` - Pass input value
- `>param >>` - Capture output value

**Key Point:** Same `[|]` marker, different operators!

---

## The 6 I/O Operators

| Operator | Context | Purpose | Example |
|----------|---------|---------|---------|
| `<` | Definition | Input parameter | `[\|] <param :type` |
| `>` | Definition | Output parameter | `[\|] >param :type` |
| `<<` | Call | Pass input | `[\|] <param << $value` |
| `>>` | Call | Capture output | `[\|] >param >> $variable` |
| `<~` | Definition | Default input | `[\|] <param <~ value` |
| `~>` | Definition | Default output | `[\|] >param ~> value` |

---

## Input Operators

### `<` - Input Parameter Definition

**Context:** Pipeline definition

**Purpose:** Declare an input parameter

**Syntax:**
```polyglot
[|] <parameter_name :type
```

**Example:**
```polyglot
{|} |ProcessOrder
[|] <order_id :string              // Input parameter
[|] <priority :string              // Input parameter
{x}
```

**This means:**
- Pipeline `|ProcessOrder` expects two inputs
- `order_id` must be type `:string`
- `priority` must be type `:string`

### `<<` - Push Input Value (PUSH →)

**Context:** Pipeline call

**Purpose:** **PUSH** a value **into** an input parameter (data flows from right to left →)

**Direction:** `value` → `<<` → `<parameter`

**Syntax:**
```polyglot
[|] <parameter_name << value
```

**Example:**
```polyglot
[r] |ProcessOrder
[|] <order_id << "ORD-123"         // PUSH "ORD-123" → <order_id
[|] <priority << "high"            // PUSH "high" → <priority
```

**This means:**
- Calling `|ProcessOrder`
- **PUSHING** `"ORD-123"` **INTO** `<order_id` parameter (→)
- **PUSHING** `"high"` **INTO** `<priority` parameter (→)

### Complete Input Example

```polyglot
// DEFINITION:
{|} |CalculateTotal
[|] <price :float                  // Input definition
[|] <quantity :int                 // Input definition
[|] >total :float                  // Output definition

[t] |T.Call
[W] |W.Polyglot.Scope

   [r] $result :float << |U.Math.Multiply
   [|] <x << $price                // PULL from input parameter
   [|] <y << $quantity

   [|] >total << $result           // Push to output parameter

{x}

// CALL:
[r] $price :float << 19.99
[r] $quantity :int << 3

[r] |CalculateTotal
[|] <price << $price               // Pass input
[|] <quantity << $quantity         // Pass input
[|] >total >> $order_total         // Capture output
```

---

## Output Operators

### `>` - Output Parameter Definition

**Context:** Pipeline definition

**Purpose:** Declare an output parameter

**Syntax:**
```polyglot
[|] >parameter_name :type
```

**Example:**
```polyglot
{|} |FetchUser
[|] <user_id :string               // Input
[|] >user_data :User               // Output
[|] >status_code :int              // Output
{x}
```

**This means:**
- Pipeline `|FetchUser` returns two outputs
- `user_data` will be type `:User`
- `status_code` will be type `:int`

### `>>` - Pull Output Value (PULL →)

**Context:** Pipeline call

**Purpose:** **PULL** the value of an output parameter **into** a variable (data flows from left to right →)

**Direction:** `>parameter` → `>>` → `$variable`

**Syntax:**
```polyglot
[|] >parameter_name >> $variable
```

**Example:**
```polyglot
[r] |FetchUser
[|] <user_id << "USER-123"
[|] >user_data >> $user            // PULL >user_data → $user
[|] >status_code >> $status        // PULL >status_code → $status
```

**This means:**
- Calling `|FetchUser`
- **PULLING** `>user_data` output **INTO** variable `$user` (→)
- **PULLING** `>status_code` output **INTO** variable `$status` (→)

### Complete Output Example

```polyglot
// DEFINITION:
{|} |ParseJSON
[|] <json_string :string           // Input
[|] >parsed_data :pg.serial        // Output
[|] >success :bool                 // Output

[t] |T.Call
[W] |W.Polyglot.Scope

   [z] $data :pg.serial << |JSON.Parse <input << $json_string
   [z][!] *! >> $error :!

   [f] $error.state =? :pg.state.faulted
      [|] >parsed_data << #EmptyData
      [|] >success << #False
   [f] *?
      [|] >parsed_data << $data
      [|] >success << #True

{x}

// CALL:
[r] $json :string << "{\"name\": \"Alice\"}"

[r] |ParseJSON
[|] <json_string << $json          // Pass input
[|] >parsed_data >> $data          // Capture output
[|] >success >> $ok                // Capture output

[f] $ok =? #True
   [r] $name :string << $data.name
```

---

## Default Value Operators

### `<~` - Default Input Value

**Context:** Pipeline definition

**Purpose:** Provide a default value for an input parameter

**Syntax:**
```polyglot
[|] <parameter_name :type <~ default_value
```

**Example:**
```polyglot
{|} |ProcessOrder
[|] <order_id :string              // Required (no default)
[|] <priority :string <~ "medium"  // Optional (has default)
[|] <timeout :int <~ 30            // Optional (has default)

[t] |T.Call
[W] |W.Polyglot.Scope

   // $priority starts as Default (can be overridden to Final)
   // $timeout starts as Default (can be overridden to Final)

{x}
```

**Variable lifecycle:**
1. Parameter declared → **Pending**
2. Default applied → **Default** (can receive one more push)
3. Caller provides value → **Final**
4. OR no caller value → remains **Default**

**Calling with defaults:**

```polyglot
// Call 1: Use all defaults
[r] |ProcessOrder
[|] <order_id << "ORD-123"
// priority = "medium" (default), timeout = 30 (default)

// Call 2: Override priority
[r] |ProcessOrder
[|] <order_id << "ORD-123"
[|] <priority << "high"            // Override default
// priority = "high", timeout = 30 (default)

// Call 3: Override both
[r] |ProcessOrder
[|] <order_id << "ORD-123"
[|] <priority << "high"
[|] <timeout << 60
// priority = "high", timeout = 60
```

### `~>` - Default Output Value

**Context:** Pipeline definition

**Purpose:** Provide a default value for an output parameter

**Syntax:**
```polyglot
[|] >parameter_name :type ~> default_value
```

**Example:**
```polyglot
{|} |FetchOptionalData
[|] <key :string
[|] >value :string ~> ""           // Default to empty string
[|] >found :bool ~> #False  // Default to false

[t] |T.Call
[W] |W.Polyglot.Scope

   [r] $data :string << |Database.Get <key << $key

   [f] $data.state =? :pg.state.final
      [|] >value << $data          // Override default
      [|] >found << #True // Override default
   // If not found, defaults remain

{x}

// CALL:
[r] |FetchOptionalData
[|] <key << "missing_key"
[|] >value >> $result
[|] >found >> $exists

// If key not found: $result = "", $exists = false (defaults)
```

**When to use `~>`:**
- Output may not always be produced
- Provide sensible fallback value
- Avoid leaving output Pending

---

## Assignment Operators

### `<<` - Push From Right (Assignment)

**Purpose:** Assign value from right side to left side

**Contexts:**
- Variable assignment
- Pipeline input
- Pipeline output assignment (internal)

**Examples:**

**Variable assignment:**
```polyglot
[r] $name :string << "Alice"
[r] $value :int << 42
```

**Pipeline input:**
```polyglot
[r] |MyPipeline
[|] <input << $my_value            // Push $my_value to <input
```

**Internal output assignment:**
```polyglot
{|} |MyPipeline
[|] >result :string

[t] |T.Call
[W] |W.Polyglot.Scope

   [r] $temp :string << "value"
   [|] >result << $temp            // Push $temp to >result

{x}
```

### `>>` - Push From Left (Capture)

**Purpose:** Capture value from left side to right side

**Contexts:**
- Pipeline output capture

**Examples:**

**Capture output:**
```polyglot
[r] |MyPipeline
[|] >result >> $my_variable        // Capture >result into $my_variable
```

**Multiple outputs:**
```polyglot
[r] |ComplexPipeline
[|] >output1 >> $result1
[|] >output2 >> $result2
[|] >output3 >> $result3
```

---

## Direction Summary

### Input Flow: Caller → Pipeline

```
Caller:     $value  →  << <param  →  Pipeline
           (source)              (destination)

Pipeline sees:  $param (Final/Default)
```

### Output Flow: Pipeline → Caller

```
Pipeline:  >param  →  >> $variable  →  Caller
          (source)                 (destination)

Caller sees:  $variable (Final)
```

### Visual Arrows

**`<<` points left:**
```polyglot
[r] $result << value               // Value flows INTO $result (from right)
[|] <input << $value               // $value flows INTO <input (from right)
```

**`>>` points right:**
```polyglot
[|] >output >> $result             // Output flows INTO $result (to right)
```

---

## Complete Examples

### Example 1: Simple Pipeline

```polyglot
// DEFINITION:
{|} |Greet
[|] <name :string                  // Input definition
[|] >greeting :string              // Output definition

[t] |T.Call
[W] |W.Polyglot.Scope

   [r] $message :string << |String.Concat
   [|] <parts << {"Hello, ", $name, "!"}

   [|] >greeting << $message       // Output assignment

{x}

// CALL:
[r] $user_name :string << "Alice"

[r] |Greet
[|] <name << $user_name            // Pass input: $user_name → <name
[|] >greeting >> $welcome_msg      // Capture output: >greeting → $welcome_msg

// $welcome_msg = "Hello, Alice!"
```

### Example 2: With Defaults

```polyglot
// DEFINITION:
{|} |SendEmail
[|] <recipient :string             // Required
[|] <subject :string               // Required
[|] <priority :string <~ "normal"  // Optional (default)
[|] <retry_count :int <~ 3         // Optional (default)

[|] >sent :bool                    // Output

[t] |T.Call
[W] |W.Polyglot.Scope

   [r] $result :bool << |Email.Send
   [|] <to << $recipient
   [|] <subject << $subject
   [|] <priority << $priority       // Uses default or override
   [|] <retries << $retry_count     // Uses default or override

   [|] >sent << $result

{x}

// CALL 1: Use defaults
[r] |SendEmail
[|] <recipient << "alice@example.com"
[|] <subject << "Hello"
// priority = "normal" (default), retry_count = 3 (default)
[|] >sent >> $success

// CALL 2: Override defaults
[r] |SendEmail
[|] <recipient << "bob@example.com"
[|] <subject << "Urgent"
[|] <priority << "high"            // Override
[|] <retry_count << 5              // Override
[|] >sent >> $success
```

### Example 3: Multiple Outputs

```polyglot
// DEFINITION:
{|} |DivideWithRemainder
[|] <dividend :int
[|] <divisor :int
[|] >quotient :int
[|] >remainder :int

[t] |T.Call
[W] |W.Polyglot.Scope

   [r] $q :int << |U.Math.Divide
   [|] <x << $dividend
   [|] <y << $divisor

   [r] $r :int << |U.Math.Modulo
   [|] <x << $dividend
   [|] <y << $divisor

   [|] >quotient << $q
   [|] >remainder << $r

{x}

// CALL:
[r] |DivideWithRemainder
[|] <dividend << 17
[|] <divisor << 5
[|] >quotient >> $q                // Captures 3
[|] >remainder >> $r               // Captures 2
```

---

## Common Patterns

### Pattern 1: Transform Input to Output

```polyglot
{|} |Transform
[|] <input :string
[|] >output :string

[t] |T.Call
[W] |W.Polyglot.Scope

   [r] $result :string << |ProcessInput <data << $input
   [|] >output << $result

{x}
```

### Pattern 2: Conditional Output

```polyglot
{|} |ValidateInput
[|] <data :string
[|] >valid :bool
[|] >error_message :string ~> ""   // Default empty

[t] |T.Call
[W] |W.Polyglot.Scope

   [f] $data =? ""
      [|] >valid << #False
      [|] >error_message << "Input cannot be empty"
   [f] *?
      [|] >valid << #True
      // error_message remains default ""

{x}
```

### Pattern 3: Chaining Pipelines (Sequential)

**Without composition (verbose):**
```polyglot
[r] |Step1
[|] <input:pg.string << $initial_value
[|] >output:pg.int >> $step1_result

[r] |Step2
[|] <input:pg.int << $step1_result        // Output from Step1 → Input to Step2
[|] >output:pg.float >> $step2_result

[r] |Step3
[|] <input:pg.float << $step2_result      // Output from Step2 → Input to Step3
[|] >output:pg.string >> $final_result
```

**With composition operator `|>` (recommended):**
```polyglot
[r] |Step1 |> |Step2                      // Chain Step1 → Step2
[|] <input:pg.string << $initial_value    // Input to Step1
[|] >output:pg.int >> <input              // Step1 output → Step2 input
[|] |> |Step3                              // Chain Step2 → Step3
[|] >output:pg.float >> <input            // Step2 output → Step3 input
[|] |>                                     // End chain
[|] >output:pg.string >> $final_result    // Capture Step3 output
```

**Critical:** Each `|>` must be on its own line (one marker + one expression rule). This allows the compiler to clearly map which outputs connect to which inputs.

### Pattern 4: Variadic Input (via Metadata)

```polyglot
{|} |ProcessMultiple
[|] <items :array.string
   [%] %variadic << "true"         // Accept multiple values
[|] >count :int

[t] |T.Call
[W] |W.Polyglot.Scope

   [r] $count :int << |Array.Length <array << $items
   [|] >count << $count

{x}

// CALL:
[r] |ProcessMultiple
[|] <items <<< {"item1", "item2", "item3"}  // Variadic input
[|] >count >> $total
```

---

## Common Pitfalls

### Pitfall 1: Mixing Definition and Call Syntax

```polyglot
// ❌ WRONG:
{|} |MyPipeline
[|] <input :string << "default"    // Mixing definition with <<

// ✅ CORRECT:
{|} |MyPipeline
[|] <input :string <~ "default"    // Use <~ for default
```

### Pitfall 2: Forgetting to Assign Outputs

```polyglot
// ❌ WRONG:
{|} |MyPipeline
[|] >result :string

[t] |T.Call
[W] |W.Polyglot.Scope

   [r] $temp :string << "value"
   // Forgot to assign >result!

{x}  // ERROR: >result is Pending

// ✅ CORRECT:
{|} |MyPipeline
[|] >result :string

[t] |T.Call
[W] |W.Polyglot.Scope

   [r] $temp :string << "value"
   [|] >result << $temp            // Assigned

{x}
```

### Pitfall 3: Wrong Direction

```polyglot
// ❌ WRONG:
[r] |MyPipeline
[|] >output << $variable           // Should be >>

// ✅ CORRECT:
[r] |MyPipeline
[|] >output >> $variable
```

### Pitfall 4: Missing Input Value

```polyglot
{|} |RequiresInput
[|] <required_param :string        // No default!
{x}

// ❌ WRONG:
[r] |RequiresInput
// Missing <required_param!

// ✅ CORRECT:
[r] |RequiresInput
[|] <required_param << "value"
```

---

## Metadata for I/O Parameters

### Parameter Documentation

```polyglot
{|} |ProcessOrder
[|] <order_id :string
   [%] %description << "Unique order identifier"
   [%] %example << "ORD-12345"
   [%] %format << "ORD-{number}"

[|] <priority :string <~ "medium"
   [%] %description << "Processing priority level"
   [%] %valid_values << {"low", "medium", "high"}

[|] >status :int
   [%] %description << "HTTP status code"

{x}
```

### Variadic Parameters

```polyglot
{|} |ConcatStrings
[|] <strings :array.string
   [%] %variadic << "true"         // Accepts multiple values with <<<
   [%] %min_items << "1"

[|] >result :string

{x}

// CALL:
[r] |ConcatStrings
[|] <strings <<< {"Hello", " ", "World"}  // Variadic input (<<<)
[|] >result >> $greeting
```

---

## Summary

### The 6 Operators

| Operator | Context | Direction | Purpose |
|----------|---------|-----------|---------|
| `<` | Definition | - | Declare input parameter |
| `>` | Definition | - | Declare output parameter |
| `<<` | Call/Assignment | → | Push value from right |
| `>>` | Call/Capture | → | Capture value to right |
| `<~` | Definition | → | Default input value |
| `~>` | Definition | → | Default output value |

### Key Distinctions

**Definition vs Call:**
- `[|] <param :type` - Define input
- `[|] <param << value` - Pass input

**Input vs Output:**
- `<param <<` - Pass input to pipeline
- `>param >>` - Capture output from pipeline

**Assignment vs Default:**
- `<param << value` - Pass value (call context)
- `<param <~ value` - Set default (definition context)

### Visual Mnemonics

- `<` - Points to parameter name (input flows in)
- `>` - Points away from parameter name (output flows out)
- `<<` - Double arrow = assign/push value
- `>>` - Double arrow = capture value
- `~` - Wavy line = "maybe" (default)

---

## Related Documentation

- [Pipeline Structure](./pipeline-structure.md) - Pipeline execution order
- [Variables & Lifecycle](./variables-lifecycle.md) - Variable states
- [Markers Reference](./markers.md) - `[|]` marker
- [Operators Reference](./operators.md) - All operators

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../README.md)
