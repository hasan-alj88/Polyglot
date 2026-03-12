---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: variables-lifecycle
shard: false

# --- Classification ---
type: spec
topic: Variables & Lifecycle
summary: Specification for Variables & Lifecycle
keywords:
  - types
  - type-system
  - specification

# --- BMAD Agent Routing ---
agents:
  - architect
  - developer
phase: solutioning
workflow: any
module: bmm
complexity: medium

# --- Dependency Chain ---
prereqs:
  - language-syntax
unlocks:
  - advanced-features

# --- Relationships ---
related:
  []
parent: language-types

# --- Metadata ---
status: stable
updated: 2025-12-16
version: 0.0.4
tags:
  - "#types"
  - "#spec"
---
# Variables & Lifecycle

**What You'll Learn:**
- The 5 variable states in Polyglot
- How variables transition between states
- Pull-based state transitions
- Variable scope and lifetime
- Common patterns and pitfalls

---

## The 5 Variable States

Every variable in Polyglot exists in exactly one of five states:

| State | Description | Can Read? | Can Write? | Can Exit Scope? |
|-------|-------------|-----------|------------|-----------------|
| **Pending** | Declared, awaiting value | ❌ No | ✅ Yes | ❌ No |
| **Default** | Has default value, can receive one more push | ✅ Yes | ✅ Yes (once) | ✅ Yes |
| **Final** | Has final value, immutable | ✅ Yes | ❌ No | ✅ Yes |
| **Faulted** | Error occurred during assignment | ❌ No | ❌ No | ✅ Yes |
| **Released** | Exited scope, no longer accessible | ❌ No | ❌ No | N/A |

---

## State Transition Diagram

```
┌─────────────┐
│ Enter Scope │
└──────┬──────┘
       │
       ▼
  ┌─────────┐
  │ Pending │◄────────────┐
  └────┬────┘             │
       │                  │
       ├──── Push (<<, >>)────────────┐
       │                              │
       ├──── Default Push (<~, ~>)────┤
       │                              ▼
       │                         ┌─────────┐
       │                         │ Default │
       │                         └────┬────┘
       │                              │
       │                              ├──── Push (<<, >>)────┐
       │                              │                       │
       ├──── Error ──────────────┐    │                       │
       │                         │    │                       │
       ▼                         ▼    ▼                       ▼
  ┌─────────┐              ┌─────────┐                  ┌───────┐
  │ Faulted │              │  Final  │                  │ Final │
  └────┬────┘              └────┬────┘                  └───┬───┘
       │                        │                           │
       │                        │                           │
       └──── Exit Scope ────────┴───────────────────────────┘
                                │
                                ▼
                          ┌──────────┐
                          │ Released │
                          └──────────┘
```

---

## State Transitions Explained

### 1. Enter Scope → Pending

Every variable starts in **Pending** state when declared:

```polyglot
[r] $name :string                  // State: Pending
[r] $age :int                      // State: Pending
[r] $email :string                 // State: Pending
```

**Rules:**
- **First declaration MUST have datatype stated**
- Cannot read from Pending variables
- Can write to Pending variables

### 2. Pending → Final (via Push)

Normal assignment with `<<` or `>>`:

```polyglot
[r] $name :string                  // State: Pending
[r] $name << "Alice"               // State: Final
```

**Operators that trigger this transition:**
- `<<` - Push from right
- `>>` - Push from left (in pipeline outputs)

**After transition:**
- Variable is **immutable**
- Can be read freely
- Cannot be written again

### 3. Pending → Default (via Default Push)

Assignment with default operators:

```polyglot
[r] $timeout :int                  // State: Pending
[r] $timeout <~ 30                 // State: Default (can receive 1 more push)
```

**Operators that trigger this transition:**
- `<~` - Default push from right
- `~>` - Default push from left

**Special property:**
- Variable can receive **one more push** to become Final
- If no additional push occurs, exits scope as Default

### 4. Default → Final (via Push)

Overriding a default value:

```polyglot
[|] <timeout :int
   [%] %default << "30"

// Later in pipeline call:
[r] |MyPipeline
[|] <timeout << 60                 // Pending → Default (from %default) → Final (from call)
```

**Common pattern:**
- Pipeline parameters with defaults start as Pending
- Receive default value → Default
- Receive caller value → Final

### 5. Pending → Faulted (via Error)

Error during assignment:

```polyglot
[r] $data :string                  // State: Pending
[z] $data << |FetchFromAPI         // If error occurs → Faulted
[z][!] *! ? "Error occurred"
```

**Triggers:**
- Exception in pipeline call
- Type mismatch
- Runtime error during assignment

**After transition:**
- Variable cannot be read
- Variable cannot be written
- Error handling can check for faulted state

### 6. Final/Faulted/Default → Released (Exit Scope)

Variables are released when scope ends:

```polyglot
{|} |ProcessUser
[|] <user_id :string

   [r] $user << |FetchUser <id << $user_id  // Final
   [r] $name << $user.name                  // Final

   // ... processing ...

{x}  // $user and $name → Released
```

**Rules:**
- **Pending variables CANNOT exit scope** (compilation error)
- Final, Default, and Faulted variables can exit scope
- Released variables are garbage collected

---

## Pull-Based State Transitions

**Critical Concept:** All state transitions occur from **PULL-ing from Final variables**.

### The PULL → PUSH Mechanism

```
Source Variable (Final) → PULL → PUSH → Destination Variable (Pending → Final)
```

**Example:**

```polyglot
[r] $source :string << "Alice"     // $source: Pending → Final

[r] $destination :string           // $destination: Pending
[r] $destination << $source        // PULL from $source (Final)
                                   // PUSH to $destination (Pending → Final)
```

### Pull Triggers State Transition

The **PULL operation** from a Final source triggers the **PUSH** to destination:

```
1. Evaluate right-hand side (RHS)
2. If RHS is a variable, PULL its value (must be Final or Default)
3. PUSH value to left-hand side (LHS)
4. LHS transitions: Pending → Final (or Default → Final)
```

### Cannot Pull from Pending

```polyglot
[r] $source :string                // State: Pending
[r] $destination :string           // State: Pending
[r] $destination << $source        // ❌ ERROR: Cannot PULL from Pending
```

**Rule:** Can only PULL from **Final** or **Default** variables.

### Pipeline Execution Depends on Final Inputs

Pipelines **only execute** when all inputs are **Final** or **Default**:

```polyglot
{|} |ProcessOrder
[|] <order_id :string              // Input parameter

[t] |T.Call                        // Implicit trigger: inputs must be Final/Default
[W] |W.Polyglot.Scope

   [r] $order << |Database.Orders.Find
   [|] <order_id << $order_id      // PULL from input (must be Final/Default)

{x}
```

**This makes input parameters implicit triggers:**
- Pipeline cannot run until inputs are Final/Default
- PULL from inputs drives execution

---

## Variable Declaration Rules

### Rule 1: First Declaration MUST Have Datatype

```polyglot
[r] $name :string << "Alice"       ✅ Correct
[r] $name << "Alice"               ❌ ERROR: Missing datatype

// After first declaration with type:
[r] $age :int                      ✅ First declaration with type
// Later references don't need type
```

**Why:** Parser needs to know type at first declaration.

### Rule 2: Cannot Reassign Final Variables

```polyglot
[r] $name :string << "Alice"       // State: Final
[r] $name << "Bob"                 // ❌ ERROR: Cannot write to Final variable
```

**Solution:** Use Default if you need one reassignment:

```polyglot
[r] $name :string <~ "Alice"       // State: Default
[r] $name << "Bob"                 // ✅ OK: Default → Final
[r] $name << "Charlie"             // ❌ ERROR: Final cannot be reassigned
```

### Rule 3: Pending Variables Cannot Exit Scope

```polyglot
{|} |BadPipeline
   [r] $result :string             // State: Pending
{x}  // ❌ ERROR: $result is still Pending
```

**Solution:** Ensure all variables reach Final, Default, or Faulted:

```polyglot
{|} |GoodPipeline
   [r] $result :string             // State: Pending
   [r] $result << "value"          // State: Final
{x}  // ✅ OK: $result is Final
```

---

## Common Patterns

### Pattern 1: Simple Assignment

```polyglot
[r] $name :string << "Alice"       // Pending → Final
```

**State Flow:** Pending → Final

### Pattern 2: Default with Override

```polyglot
[r] $timeout :int <~ 30            // Pending → Default
// Optional override:
[r] $timeout << 60                 // Default → Final (if called)
// Or exit as Default (if not overridden)
```

**State Flow:** Pending → Default → (maybe Final)

### Pattern 3: Pipeline Parameter with Default

```polyglot
{|} |ProcessOrder
[|] <priority :string
   [%] %default << "medium"        // Provides default value

[t] |T.Call
[W] |W.Polyglot.Scope

   // $priority starts Pending
   // Receives default → Default
   // May receive caller value → Final
   [r] $priority_value << $priority  // PULL (must be Final or Default)

{x}
```

**State Flow:** Pending → Default (from %default) → maybe Final (from caller)

### Pattern 4: Error Handling

```polyglot
[r] $data :string                  // State: Pending
[z] $data << |FetchFromAPI         // May fail
[z][!] !Network.* ? "Network error"
[z][!] *! ? "Unknown error"

// If success: $data is Final
// If error: $data is Faulted
```

**State Flow:** Pending → (Final OR Faulted)

### Pattern 5: Conditional Assignment

```polyglot
[r] $result :string                // State: Pending

[f] $age >=? 18
   [r] $result << "adult"          // Path 1: Pending → Final
[f] *?
   [r] $result << "minor"          // Path 2: Pending → Final

// Both paths must assign, or compilation error
```

**State Flow:** Pending → Final (via either branch)

---

## Variable Scope

### Scope Rules

Variables are scoped to their **containing block**:

```polyglot
{|} |OuterPipeline
   [r] $outer :string << "visible in outer"

   [f] #True
      [r] $inner :string << "only in this block"
      [r] $result << $outer       // ✅ Can access outer scope
   [f]

   [r] $x << $inner                // ❌ ERROR: $inner not in scope
{x}
```

**Rules:**
- Inner scopes can access outer scope variables
- Outer scopes cannot access inner scope variables
- Variables must reach Final/Default/Faulted before exiting scope

### Nested Scopes

```polyglot
{|} |Level1
   [r] $var1 :string << "level 1"

   [f] #True
      [r] $var2 :string << "level 2"

      [f] #True
         [r] $var3 :string << "level 3"
         [r] $all << $var1       // ✅ Access level 1
         [r] $all << $var2       // ✅ Access level 2
         [r] $all << $var3       // ✅ Access level 3
      [f]

      [r] $x << $var3            // ❌ ERROR: var3 exited scope
   [f]
{x}
```

---

## Loop Variable Lifecycle

### Unpack Creates Iteration Scope Variables

```polyglot
[p] ~ForEach.Array
[~] <array << $items               // PULL from $items (Final)
[~] >item >> $element              // PUSH to $element (Pending → Final in iteration)

   // Iteration scope:
   [r] $processed << |Transform <input << $element  // PULL from $element (Final)

   [v] *Into.Array
   [*] <item << $processed         // PULL from $processed (Final)
   [*] >array >> $results          // PUSH to $results (in main scope)
```

**Variable Lifecycles:**
- `$items` - Main scope (Final before loop)
- `$element` - Iteration scope (Pending → Final in each iteration, Released after)
- `$processed` - Iteration scope (Pending → Final in each iteration, Released after)
- `$results` - Main scope (Pending → Final after loop completes)

### Iteration Variables Are Recreated Each Loop

```polyglot
[p] ~ForEach.Array
[~] <array << [1, 2, 3]
[~] >item >> $item

   // Iteration 1: $item = 1 (Pending → Final → Released)
   // Iteration 2: $item = 2 (Pending → Final → Released)
   // Iteration 3: $item = 3 (Pending → Final → Released)
```

Each iteration creates **new** variables in iteration scope.

---

## Common Pitfalls

### Pitfall 1: Reading Before Assignment

```polyglot
[r] $name :string                  // State: Pending
[r] $result << $name               // ❌ ERROR: Cannot PULL from Pending
```

**Fix:** Ensure variable is Final before reading:

```polyglot
[r] $name :string << "Alice"       // State: Final
[r] $result << $name               // ✅ OK
```

### Pitfall 2: Forgetting to Assign in All Branches

```polyglot
[r] $result :string                // State: Pending

[f] $age >=? 18
   [r] $result << "adult"          // Path 1: Pending → Final
[f] *?
   // Path 2: $result still Pending!

{x}  // ❌ ERROR: $result may be Pending
```

**Fix:** Assign in all branches:

```polyglot
[r] $result :string

[f] $age >=? 18
   [r] $result << "adult"
[f] *?
   [r] $result << "minor"          // ✅ Both paths assign

{x}  // ✅ OK: $result is definitely Final
```

### Pitfall 3: Too Many Pushes to Default

```polyglot
[r] $value :int <~ 10              // State: Default
[r] $value << 20                   // State: Final
[r] $value << 30                   // ❌ ERROR: Final cannot be reassigned
```

**Remember:** Default allows **one** additional push, then becomes Final.

### Pitfall 4: Missing Datatype on First Declaration

```polyglot
[r] $name << "Alice"               // ❌ ERROR: Missing datatype
```

**Fix:**

```polyglot
[r] $name :string << "Alice"       // ✅ OK
```

---

## State Checking (Future Feature)

**Planned for future versions:**

```polyglot
[f] $variable.state =? :pg.state.final
   // Logic for Final variables
[f] $variable.state =? :pg.state.faulted
   // Logic for Faulted variables
```

**Current Version (v0.0.4):** No direct state checking available. Use error handling patterns instead.

---

## Summary

### 5 States
1. **Pending** - Declared, no value
2. **Default** - Has default, accepts one more push
3. **Final** - Immutable value
4. **Faulted** - Error occurred
5. **Released** - Exited scope

### Key Transitions
- **Pending → Final:** Normal assignment (`<<`, `>>`)
- **Pending → Default:** Default assignment (`<~`, `~>`)
- **Default → Final:** Override default
- **Pending → Faulted:** Error during assignment
- **{Final, Default, Faulted} → Released:** Exit scope

### Critical Rules
1. First declaration MUST have datatype
2. Can only PULL from Final or Default variables
3. All state transitions driven by PULL operations
4. Pending variables CANNOT exit scope
5. Final variables are immutable
6. Default variables accept ONE more push

---

## Related Documentation

- [Markers Reference](./markers.md) - `[r]`, `[p]`, execution markers
- [I/O Operators](./io-operators.md) - `<<`, `>>`, `<~` operators
- [Pipeline Structure](./pipeline-structure.md) - Input parameters as implicit triggers
- [Loop System](../User/language/advanced/loop-system.md) - Iteration scope lifecycle
- [Error Handling](../features/error-handling/error-handling.md) - Faulted state handling

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../README.md)
