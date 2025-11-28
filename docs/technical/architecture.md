# Architecture

## Executive Summary

Polyglot is implemented as a Rust workspace with three backend services (Trigger Monitor, Queue Manager, Runner) that communicate via PostgreSQL and Redis. The architecture prioritizes async performance, runtime type resolution, and AI agent consistency through explicit patterns and conventions.

---

## What is Polyglot? Unlearning Traditional Programming

### If You're Coming From Python, JavaScript, Rust, Java, C++... Read This First

**Polyglot is NOT a traditional programming language.** If you approach Polyglot with traditional programming assumptions, you will be confused. This section exists to help you **unlearn** those patterns and understand what Polyglot actually is.

---

### Traditional Programming vs Polyglot Automation

#### **Traditional Languages: Imperative Execution**

In traditional languages, you write **imperative code that executes immediately**:

```python
# Traditional: Synchronous, immediate execution
user = fetch_user(user_id)      # Function call, returns now
email = user.email              # Access field now
send_notification(email)        # Execute now
```

**Mental Model:**
- Write functions/procedures
- Call them synchronously (or explicitly add async/await)
- Data exists in memory
- Control flow is IF/ELSE/FOR/WHILE

---

#### **Polyglot: Automation Jobs with Boxes and Triggers**

In Polyglot, you write **automated jobs** (called **pipelines**) that are **triggered by events** and execute **asynchronously**:

```polyglot
// Polyglot: Async-centric automation
[|] SendUserNotification          # Define automation job (pipeline)

[t] |T.OnUserCreated              # TRIGGER: When user created event fires
[<] .user_id: pg\string           # INPUT: User ID from trigger event

[r] |FetchUser                    # BOX 1: Call async operation
[<] .id: pg\string << .user_id    # Input to box
[>] .user: #UserProfile >> .profile  # Output from box

[r] |SendEmail                    # BOX 2: Call another async operation
[<] .email: pg\string << .profile.email  # Input pulled from box 1 output
[X]                               # End pipeline
```

**Mental Model:**
- Write automation jobs that respond to triggers
- Pipelines are boxes connected by I/O
- Everything is asynchronous by default
- Control flow is conditional triggers + exhaustive switch blocks

---

### The Core Difference: Automation Jobs, Not Scripts

| Aspect | Traditional Languages | Polyglot |
|--------|----------------------|----------|
| **Primary Purpose** | Write application logic | **Write automated jobs** |
| **Execution Model** | Synchronous (or explicit async) | **Async-centric (automatic waiting)** |
| **Structure** | Functions/Classes/Modules | **Pipelines (boxes with I/O)** |
| **Control Flow** | IF/ELSE/FOR/WHILE | **Triggers + exhaustive switch blocks** |
| **When it runs** | When you call it | **When trigger fires (events, schedules, conditions)** |
| **Data flow** | Variables in memory | **Variables transition through states (Pending → Ready)** |

---

### What Does "Automation Programming" Mean?

Polyglot is the **first language designed specifically for automation workflows**. Think of it as:

- **Cron on steroids** - Schedule jobs, but with full programming power
- **Event-driven orchestration** - Respond to file changes, API webhooks, database events
- **Multi-step workflows** - Chain operations with automatic error handling
- **Priority queues** - Control which jobs run first when resources are limited
- **Cross-language glue** - Coordinate Python, Rust, Go, Node.js in one workflow

**Real-world automation examples:**
- **Daily reports:** Every morning at 2 AM, gather stats (Rust), format report (Python), send to LLM
- **File watchers:** When CSV uploaded, validate (Rust), transform (Python), load to database (Go)
- **API webhooks:** GitHub push → run tests → deploy if green → notify Slack
- **Resource-aware processing:** Low-priority batch jobs that pause when high-priority work arrives

---

### Pipelines Are Boxes with I/O, Not Functions

**Traditional function:**
```python
def process_data(input_data):
    validated = validate(input_data)
    transformed = transform(validated)
    return save_to_db(transformed)
```
- Functions execute **now**
- Returns values **immediately** (or awaits explicitly)
- Control flow is function calls

---

**Polyglot pipeline (boxes with I/O):**
```polyglot
[|] ProcessData                      # Pipeline definition

[i] .input_data: pg\serial           # INPUT box

[r] |Validate                        # BOX 1 (async operation)
[<] .data: pg\serial << .input_data  # Pull input
[>] .validated: pg\serial >> .valid  # Push output

[r] |Transform                       # BOX 2 (async operation)
[<] .data: pg\serial << .valid       # Pull from BOX 1
[>] .result: pg\serial >> .transformed  # Push output

[r] |SaveToDB                        # BOX 3 (async operation)
[<] .data: pg\serial << .transformed # Pull from BOX 2
[X]
```

**Key Differences:**
- Pipelines are **triggered by events**, not called like functions
- Each `[r]` is a **box** (async operation that runs independently)
- Boxes connect via **I/O** (`[<]` inputs, `[>]` outputs)
- Data flows **asynchronously** between boxes
- If `|Validate` is still running, `|Transform` **waits automatically** (no explicit await)

---

### Conditional Triggers, Not IF-ELSE

**Traditional IF-ELSE:**
```javascript
if (user.role === "admin") {
    sendAdminNotification(user);
} else if (user.role === "user") {
    sendUserNotification(user);
}
// Else is optional - might not handle all cases
```

---

**Polyglot Conditional Triggers (exhaustive):**
```polyglot
[|] SendNotification

[?] .user.role =? #Roles.Admin       # TRIGGER: If admin
[~][r] |SendAdminNotification        # Execute this box
[~]

[?] .user.role =? #Roles.User        # TRIGGER: If user
[~][r] |SendUserNotification         # Execute this box
[~]

[?] *?                               # TRIGGER: Catch-all (REQUIRED!)
[~][r] |SendDefaultNotification      # Execute this box
[~]
```

**Key Differences:**
- `[?]` blocks are **conditional triggers** (not IF statements)
- **Exhaustive conditions REQUIRED** - must handle all cases or compile error
- Designed for async-centric execution (no indefinite waiting)
- `[~]` marks the body that executes when condition true

---

### Variables Don't Have Values Immediately - They Have States

**Traditional variables:**
```javascript
const name = "Alice";     // Available NOW
let count = 0;            // Can change NOW
```
- Variables have values **immediately** after assignment
- Mutability is about whether value can change
- No async coordination needed

---

**Polyglot variables (state-aware):**
```polyglot
[r] |FetchUser                        # Async operation
[>] .name: pg\string >> .user_name    # Variable is Pending (waiting for pipeline)

// .user_name transitions through states:
// 1. Declared (schema only)
// 2. Pending (async operation in progress)
// 3. Ready (value available) OR Faulted (operation failed)

[r] |ProcessName                      # This box runs later
[<] .name: pg\string << .user_name    # AUTO-AWAIT: Waits until .user_name is Ready

[?] .user_name.pgvar.state =? #PgVar.States.Ready    # Check state explicitly
[~][<] .final << .user_name.*                        # Access value
```

**Key Differences:**
- Variables **don't have values immediately** - they transition through **states**
- `Pending` → `Ready` or `Faulted`
- When you "pull from" a variable (`<<`, `>>`, `=?`), **auto-await occurs**
- Reserved namespace `.*.pgvar.*` tracks state (always Ready, database-backed)

---

### No Keywords - Only Operator Prefixes

**Traditional languages:**
```javascript
if (condition) { ... }       // "if" is a keyword
function myFunc() { ... }    // "function" is a keyword
class MyClass { ... }        // "class" is a keyword
for (let i = 0; ...) { ... } // "for", "let" are keywords
```
- Keywords are reserved words without prefixes
- `if`, `for`, `while`, `return`, `class`, `function`, etc.

---

**Polyglot has ZERO keywords:**
```polyglot
[|] MyPipeline               # | prefix = pipeline
[<] .myvar: #MySchema        # . prefix = variable, # prefix = schema
[r] |OtherPipeline           # | prefix = pipeline call
[!] !MyError                 # ! prefix = error type
```

**Rule:** EVERY identifier MUST have an operator prefix:
- `.` = Variable (defines and navigates hierarchy)
- `#` = Enumeration/Schema
- `|` = Pipeline
- `!` = Error type

**Why?**
- **Clarity:** No ambiguity - `.user` is definitely a variable, `#User` is definitely a schema
- **Serialization-centric:** Maps directly to hierarchical data tree
- **Cross-language integration:** Unambiguous serialization/deserialization

---

### ALL Data is Hierarchical Tree Structure

**Traditional languages:**
- Variables are memory addresses
- Objects/structs are memory layouts
- Serialization is an afterthought (add JSON.stringify, serde, etc.)

---

**Polyglot:**
- **ALL data is hierarchical serialized tree structure**
- Variables are **nodes** in the tree
- `.` operator **navigates** the tree
- Reserved namespaces (`.*.pgvar.*`) are **subtrees**

```polyglot
.config                      # Root node
  ├─ .timeout: 30           # Child field
  ├─ .retries: 3            # Child field
  └─ .pgvar                 # Reserved subtree (metadata)
      ├─ .state: Ready      # State metadata
      └─ .history           # State transition history
```

**Why?**
- **Cross-language coordination** happens via tree serialization (JSON, YAML, etc.)
- **Type metadata** attached to tree nodes
- **State tracking** via reserved subtree
- **Async operations** coordinate through the tree

---

### Pipelines Write Automated Jobs, Not Scripts

**You Don't Run Polyglot Pipelines Like Scripts:**

❌ **WRONG (traditional thinking):**
```bash
polyglot run MyPipeline.pg    # This doesn't make sense!
```

✅ **CORRECT (automation thinking):**
```bash
# 1. Compile pipeline to IR
polyglot compile MyPipeline.pg

# 2. Register with Polyglot service
polyglot register MyPipeline

# 3. Activate with trigger
polyglot activate MyPipeline --input repo_path=/home/user/project

# 4. Pipeline runs automatically when trigger fires:
#    - Every day at 2 AM (cron-like)
#    - When file changes (file watch)
#    - When webhook received (event-driven)
#    - When condition met (database query, API check)
```

**Key Insight:**
- Pipelines are **registered** with the service and run **automatically** when triggered
- You don't "call" pipelines - you define **when they should run** (triggers)
- Three backend services orchestrate execution:
  - **Trigger Monitor** - Watches for trigger conditions
  - **Queue Manager** - Prioritizes and dispatches jobs
  - **Runner** - Executes pipeline boxes

---

### Architecture: Three Services, Not a Compiler

**Traditional compiled language:**
```
Source Code → Compiler → Binary Executable → Run directly
```

---

**Polyglot automation architecture:**
```
Pipeline.pg → Compiler → 3 IRs → Database
                           ↓
         ┌─────────────────┴─────────────────┐
         │   Polyglot Service (3 backends)   │
         ├───────────────────────────────────┤
         │ Trigger Monitor  (watches events) │
         │ Queue Manager    (prioritizes)    │
         │ Runner           (executes boxes) │
         └───────────────────────────────────┘
```

**Why this architecture?**
- **Event-driven:** Trigger Monitor continuously watches for conditions
- **Priority queues:** Queue Manager handles resource contention
- **Async execution:** Runner manages parallel boxes and state transitions
- **Persistent state:** PostgreSQL stores pipeline IR and variable states

---

### Summary: The Mental Shift

To understand Polyglot, you need to shift your mental model:

| Unlearn This (Traditional) | Learn This (Polyglot) |
|---------------------------|----------------------|
| "I'm writing a script/program" | **"I'm writing an automated job"** |
| "Functions that I call" | **"Boxes triggered by events"** |
| "Variables have values now" | **"Variables transition through states (Pending → Ready)"** |
| "IF/ELSE with optional else" | **"Exhaustive conditional triggers"** |
| "Synchronous with explicit async" | **"Async-centric with automatic waiting"** |
| "Run my code directly" | **"Register job, service orchestrates execution"** |
| "Keywords (if, for, while)" | **"Zero keywords, operator prefixes only"** |
| "Data in memory" | **"Data as hierarchical serialized tree"** |

---

## Core Architectural Philosophy: Async-Centric Coordination

### The Paradigm Shift

Polyglot represents a fundamental departure from traditional programming language design. Unlike languages where async is a feature bolted onto a synchronous foundation, **Polyglot is async-centric by design**—async operations are the default, not the exception.

This architectural decision has profound implications for how we think about variable semantics, execution flow, and cross-language coordination.

### Beyond "Mutability"

Traditional programming languages frame variable semantics around **mutability**:
- Immutable: Value never changes after assignment
- Mutable: Value can be modified after assignment

**This framing doesn't apply to Polyglot.** Here's why:

In synchronous languages, variables have values *immediately*:
```javascript
const name = "Alice";  // Available now
let count = 0;         // Can change now
```

In Polyglot, variables don't have values immediately—they **transition through states** as async operations complete:
```polyglot
[r] |FetchUser
[>] .name: pg\string >> .user_name  # Pending → Ready (async)
```

The variable `.user_name` isn't "immutable" or "mutable"—it's **state-aware**. It transitions from `Pending` (waiting for pipeline) to `Ready` (value available) or `Faulted` (operation failed).

### State-Aware Coordination, Not Immutability

**Key Insight:** Variables in Polyglot are **immutable once Ready**, but immutability is a *consequence* of async coordination, not a design goal.

The real model is:
- **Declared** → No value yet, schema only
- **Pending** → Async operation in progress
- **Ready** → Value available, won't change (because async op completed)
- **Faulted** → Operation failed, no value

Once a variable reaches `Ready`, there's no reason for it to change—the async operation fulfilled its purpose. This "immutability" emerges naturally from the async lifecycle.

Traditional debates about "const vs let" don't apply here. Instead, Polyglot has:
- **State transitions** (Declared → Pending → Ready/Faulted)
- **Assignment operators** that control state lifecycle
- **Automatic waiting** where pipelines block until variables are Ready

### The Four Foundational Principles

Polyglot's architecture is built on these principles:

#### 1. Async-Centric by Design
- Not sync with async features added
- Async operations are the default execution model
- No explicit `await` keyword—pipelines handle waiting automatically

#### 2. State-Aware Coordination
- Variables transition through explicit states
- State transitions coordinate async operations
- States are queryable via `.state` reserved field

#### 3. Serialization Foundation
- ALL Polyglot data is serialized for cross-language coordination
- Dot notation (`.field`, `.nested.field`) is the primary syntax
- Variables are serialized strings with type metadata
- Enables seamless FFI without manual marshaling

#### 4. Automatic Waiting
- Pipelines automatically wait for `Pending` variables
- Developers never write `await`—runtime handles blocking/unblocking
- `[i]` input blocks enforce `Ready` state before execution

### What This Means for Architecture

This async-centric design impacts every layer:

**Compiler:**
- Must track variable states through pipeline graph
- Validates state transitions at compile time
- Enforces `Ready` state at pipeline boundaries

**Runtime:**
- Implements automatic waiting (non-busy blocking)
- Manages state transitions atomically
- Propagates errors through pipeline chains

**Database:**
- Stores variable state metadata alongside values
- Tracks state transitions for debugging
- Supports reserved fields (`.state`, `.errors`)

**IR Representation:**
- Variables include state, override count, errors
- State transitions encoded in execution flow
- Reserved fields in serialization tree

### Why This Matters

Understanding Polyglot as "async-centric with state-aware coordination" vs "immutable variables" completely changes how implementers approach:
- Parser design (operators control state lifecycle)
- IR generation (state metadata required)
- Runtime execution (automatic waiting semantics)
- Error handling (Faulted state propagation)

The rest of this architecture document describes how these principles manifest in concrete implementation details.

---

## Fundamental Concepts: Operator-Centric Syntax

### No Keywords in Polyglot

**Critical Design Principle:** Polyglot has **NO keywords**. Every identifier MUST have an operator prefix.

This is a fundamental departure from traditional programming languages where keywords like `if`, `while`, `function`, `class` are reserved words without prefixes.

**Rule:** Any identifier without an operator prefix is **INVALID SYNTAX** and will result in a compile error.

---

### Operator Prefix Table

ALL identifiers in Polyglot are distinguished by their operator prefix:

| Prefix | Type | Purpose | Example |
|--------|------|---------|---------|
| `.` | **Variable** | Defines variables and navigates hierarchy | `.myvar`, `.config.timeout`, `.user.address.city` |
| `#` | **Enumeration/Schema** | Defines data structures and enumerations | `#Config`, `#UserProfile`, `#Colors.Red` |
| `\|` | **Pipeline** | Defines executable pipelines | `\|ProcessData`, `\|U.Log.Info`, `\|FetchUser` |
| `!` | **Error Type** | Defines error types for error handling | `!Network.Timeout`, `!Validation.Failed` |

**Examples:**

```polyglot
# ✅ VALID - All identifiers have operator prefixes:
[r] |FetchUser              # Pipeline (| prefix)
[<] .user: #UserProfile     # Variable (. prefix) with Schema (# prefix)
[!] !Network.Timeout        # Error type (! prefix)

# ❌ INVALID - No operator prefixes:
[r] FetchUser               # ERROR: Missing | prefix
[<] user: UserProfile       # ERROR: Missing . and # prefixes
[!] Network.Timeout         # ERROR: Missing ! prefix
```

---

### The Dot Operator (`.`) - THE Most Important Operator

The dot operator serves **two critical roles** in Polyglot:

#### 1. **Variable Declaration** - `.` at the start means "this is a variable"

```polyglot
.myvar                 # This is a variable
.config                # This is a variable
.user_data             # This is a variable
```

Without the leading `.`, an identifier is **NOT a variable** and is invalid syntax (since Polyglot has no keywords).

#### 2. **Hierarchy Navigation** - `.` navigates the serialization tree

```polyglot
.config.timeout        # Navigate: .config → .timeout field
.user.address.city     # Navigate: .user → .address → .city field
```

**Key Insight:** The dot operator is foundational because **ALL Polyglot data is hierarchical serialized tree structure**. Variables are nodes in this tree, and `.` is how you traverse it.

---

### Why This Design?

**1. Clarity:** No ambiguity about what type an identifier represents
```polyglot
.user          # Definitely a variable
#User          # Definitely a schema/enumeration
|ProcessUser   # Definitely a pipeline
```

**2. Consistency:** Same pattern everywhere in the language

**3. Serialization-Centric:** The `.` operator maps directly to the hierarchical serialization tree that underlies ALL Polyglot data

**4. Cross-Language Integration:** Operator prefixes make it unambiguous how to serialize/deserialize data when integrating with other languages

---

### Hierarchical Serialization Tree - The Foundation

**Core Concept:** ALL Polyglot data exists as a **hierarchical serialized tree structure**.

- Variables are **nodes** in the serialization tree
- Fields are **child nodes** in the tree
- The `.` operator **navigates** the tree structure
- Reserved namespaces (like `.*.pgvar`) are **subtrees** in the tree

**Example Tree Structure:**
```
.config                      (root variable node)
  ├─ .timeout: 30           (child field)
  ├─ .retries: 3            (child field)
  └─ .pgvar                 (reserved namespace subtree)
      ├─ .state: Ready      (metadata)
      └─ .history           (metadata)
```

**Why This Matters:**
- Cross-language coordination happens via tree serialization (JSON, YAML, TOML, etc.)
- Type information is metadata attached to tree nodes
- State information (`.pgvar`) is a reserved subtree
- All async operations coordinate through the serialization tree

---

### Async-Centric Control Flow: IF-ELSE → SWITCH

**Paradigm Shift:** Traditional synchronous programming uses IF-ELSE for control flow. Polyglot's async-centric design uses **SWITCH blocks with exhaustive conditions**.

#### Traditional Sync: IF-ELSE (Optional Else)

```javascript
if (condition1) {
  // Handle case 1
} else if (condition2) {
  // Handle case 2
}
// Else is optional - might not handle all cases
```

**Problem:** In async environments, unhandled conditions can lead to indefinite waiting or undefined behavior.

---

#### Polyglot Async: SWITCH (Exhaustive Conditions REQUIRED)

```polyglot
[?] condition1
[~]// Handle condition1

[?] condition2
[~]// Handle condition2

[?] *?  # ← REQUIRED: Exhaustive "catch-all" (compile error if missing)
[~]// Handle all other cases
```

**Rule:** ALL possible conditions MUST be exhausted, or the compiler will throw an error.

**Why:** In async-centric programming, pipelines must know what to do in EVERY possible state. Leaving conditions unhandled means the pipeline could wait indefinitely.

---

#### Switch Block Markers

Polyglot provides **block markers** for combining conditions:

**`[?]` - Condition Block**
Tests a single condition.

```polyglot
[?] .var.pgvar.state =? #PgVar.States.Ready
[~][<] .value << .var.*  # Execute if true
```

**`[+]` - OR Block**
Combines conditions with OR logic (any condition true).

```polyglot
[?] .timeout.pgvar.state =? #PgVar.States.Ready
[+] .result.pgvar.state =? #PgVar.States.Ready  # OR this
[~][r] |ProcessResults  # Execute if either true
```

**`[&]` - AND Block**
Combines conditions with AND logic (all conditions true).

```polyglot
[?] .user.pgvar.state =? #PgVar.States.Ready
[&] .permissions =? #True                        # AND this
[~][r] |AuthorizedAction  # Execute only if both true
```

**`[?] *?` - Exhaustive Catch-All**
Matches any condition not handled above (REQUIRED).

```polyglot
[?] .var.pgvar.state =? #PgVar.States.Ready
[~]// Handle Ready

[?] .var.pgvar.state =? #PgVar.States.Faulted
[~]// Handle Faulted

[?] *?  # ← Catches Pending, Declared, etc.
[~]// Handle all other states
```

---

#### Exhaustive Condition Example

```polyglot
[r] |ProcessData

# Enum-based exhaustive switch (compiler verifies all cases):
[?] .status =? #Status.Success
[~][r] |HandleSuccess

[?] .status =? #Status.Failure
[~][r] |HandleFailure

[?] .status =? #Status.Pending
[~][r] |HandlePending

# Compiler ERROR if Status has more cases not handled!
# OR must have exhaustive catch-all:
[?] *?
[~][r] |HandleUnknown
```

**Key Benefit:** Compiler-enforced exhaustiveness prevents bugs where async operations are left in undefined states.

---

### Block Markers Reference

**Block Marker Signature:** `[X]` - Single character between square brackets

Polyglot uses block markers to define structure and control flow. All block markers follow the `[character]` pattern.

| Marker | Name | Purpose | Example |
|--------|------|---------|---------|
| `[\|]` | Pipeline Definition | Define a named pipeline | `[\|] MyPipeline` |
| `[#]` | Enumeration Definition | Define data structure/enumeration | `[#] Config` |
| `[X]` | End Block | Close enumeration or pipeline definition | `[X]` |
| `[i]` | Input Block | Define input variables (must be Ready) | `[i] .var: Type << .source` |
| `[o]` | Output Block | Define output variables | `[o] .result >> .output` |
| `[<]` | Input Assignment | Assign input value (push left) | `[<] .var << value` |
| `[>]` | Output Assignment | Assign output value (push right) | `[>] .var >> .target` |
| `[r]` | Run Pipeline | Execute a pipeline | `[r] \|ProcessData` |
| `[?]` | Switch Condition | Test a condition (switch block) | `[?] .var =? value` |
| `[~]` | Switch Body | Execute if condition true | `[~][r] \|HandleCase` |
| `[+]` | OR Block | Combine conditions with OR logic | `[+] .var2 =? value2` |
| `[&]` | AND Block | Combine conditions with AND logic | `[&] .var2 =? value2` |
| `[!]` | Error Handler | Catch specific error type | `[!] !Network.Timeout` |
| `[Q]` | Queue Control | Control queue operations | `[Q] retry=3` |
| `[\]` | Setup Block | Runs before pipeline execution | `[\] // Setup code` |
| `[/]` | Cleanup Block | Runs after pipeline execution | `[/] // Cleanup code` |
| `[b]` | Background Execution | Fire-and-forget parallel execution | `[b][r] \|BackgroundTask` |
| `[s]` | Serial Load | Load serialized data (JSON/YAML/etc.) | `[s] .data << file.json` |
| `[p]` | Parallel Execution | Execute multiple pipelines in parallel | `[p][r] \|Task1` |
| `[Y]` | Yield Block | Yield control/results | `[Y] .result >> .output` |

**Pattern Recognition:**
- Definition blocks: `[|]`, `[#]`, `[X]`
- I/O blocks: `[i]`, `[o]`, `[<]`, `[>]`
- Execution blocks: `[r]`, `[p]`, `[b]`
- Control flow: `[?]`, `[~]`, `[+]`, `[&]`
- Lifecycle: `[\]`, `[/]`, `[Y]`
- Special: `[!]`, `[Q]`, `[s]`

**Note:** This is the complete set of 1-character block markers in Polyglot. No keywords, only operator prefixes and block markers.

---

### Unpack Operators: The Async Alternative to Loops

**Paradigm Shift:** Traditional programming uses loops (`for`, `while`, `foreach`). Polyglot's async-centric design uses **unpack operators** (`~ForEach`, `~Enumerate`, `~Zip`).

#### Traditional Loops vs Polyglot Unpack

**Traditional Sync (Python):**
```python
results = []
for item in array:
    result = process(item)
    results.append(result)
```

**Polyglot Async:**
```polyglot
[p] ~ForEach
[<] .array
[>] .item
[~][r] |ProcessItem
[~][<] .input << .item
[~][>] .result >> .output
[~]
[~][Y] ~Y.IntoArray
[~][<] .output
[~][>] .results
```

**Key Differences:**
- **Parallel by default** - `[p]` naturally parallelizes each iteration
- **Async-aware** - Each iteration can await without blocking others
- **Explicit join** - `[Y] ~Y.*` makes result collection explicit
- **No mutation** - No loop counters, immutable variables
- **Composable** - Unpack operators compose with pipelines

---

#### Unpack Operator Pattern Structure

```polyglot
[p/r/b] ~UnpackOperator       # Execution mode + unpack operator
[<] .input_collection         # Input: collection (PULLS, triggers auto-await)
[>] .iteration_variable       # Output: individual item (type inferred)
[~]                           # Start of unpack body
[~] ... operations            # Process each item
[~][Y] ~Y.PackOperator       # Join block + pack operator (optional)
[~][<] .result_per_iteration # Input to pack
[~][>] .output_collection    # Output: joined result (type inferred)
```

**Execution Modes:**
- `[p]` - **Parallel** execution + join results (requires `[Y]`)
- `[r]` - **Sequential** execution + join results (requires `[Y]`)
- `[b]` - **Background** fire-and-forget (no `[Y]`, no join)

---

#### Pull Semantics and Type Inference

**Critical:** Input collections for unpack operators:
1. Must be **already declared**
2. **Pull operation occurs** when unpack starts
3. **Auto-await** triggers if collection is `Pending`
4. **Type inference** - Output types derived from input collection type

**Example:**
```polyglot
[i] .items: pg\array{pg\string}  # May be Pending

[p] ~ForEach                      # ← PULLS from .items (auto-await)
[<] .items                        # Must be Ready before iteration starts
[>] .item                         # Type inferred: pg\string
                                  # (from pg\array{pg\string} element type)
```

**Type Inference Rules:**
- `pg\array{T}` → `.item: T`
- `pg\array{#Enum}` → `.item: #Enum`
- `pg\set{T}` → `.item: T`
- Multiple inputs (`~Zip`) → multiple inferred types

**Collection Type Constraint:**
- Collections **CANNOT** contain other collections directly
- ❌ `pg\array{pg\array{T}}` - Invalid
- ❌ `pg\set{pg\array{T}}` - Invalid
- ✅ `pg\array{#Enum}` - Valid (wrap collections in enumerations)
- ✅ `pg\array{pg\serial}` - Valid (if needed)

---

#### Unpack Operators Reference

**1. `~ForEach` - Iterate Over Collection**

**Purpose:** Process each element of a collection

**Syntax:**
```polyglot
[p] ~ForEach
[<] .collection: pg\array{T}
[>] .item                        # Type: T (inferred)
[~] // Process .item
[~][Y] ~Y.IntoArray
[~][<] .processed_item
[~][>] .results                  # Type: pg\array{T}
```

**Example:**
```polyglot
[i] .numbers: pg\array{pg\int}

[p] ~ForEach
[<] .numbers
[>] .num                         # Type: pg\int
[~][r] |DoubleNumber
[~][<] .input: pg\int << .num
[~][>] .doubled: pg\int >> .result
[~]
[~][Y] ~Y.IntoArray
[~][<] .result
[~][>] .doubled_numbers          # Type: pg\array{pg\int}
```

---

**2. `~Enumerate` - Iterate With Index**

**Purpose:** Process each element with its index (like Python's `enumerate()`)

**Syntax:**
```polyglot
[p] ~Enumerate
[<] .collection: pg\array{T}
[>] .index                       # Type: pg\uint (always)
[>] .item                        # Type: T (inferred from array)
[~] // Use both .index and .item
```

**Example:**
```polyglot
[i] .names: pg\array{pg\string}

[p] ~Enumerate
[<] .names
[>] .index                       # Type: pg\uint
[>] .name                        # Type: pg\string
[~][r] |U.Log.Info
[~][<] .msg << "Index {.index}: {.name}"
[~]
[~][Y] ~Y.IntoArray
[~][<] .name
[~][>] .indexed_names
```

---

**3. `~Zip` - Combine Multiple Collections**

**Purpose:** Iterate over multiple collections simultaneously (like Python's `zip()`)

**Syntax:**
```polyglot
[p] ~Zip
[<] .collection1: pg\array{T1}
[<] .collection2: pg\array{T2}
[>] .item1                       # Type: T1 (inferred)
[>] .item2                       # Type: T2 (inferred)
[~] // Use .item1 and .item2 together
```

**Example:**
```polyglot
[i] .names: pg\array{pg\string}
[i] .ages: pg\array{pg\int}

[p] ~Zip
[<] .names
[<] .ages
[>] .name                        # Type: pg\string
[>] .age                         # Type: pg\int
[~][r] |CreateUserProfile
[~][<] .name_input: pg\string << .name
[~][<] .age_input: pg\int << .age
[~][>] .profile: #UserProfile >> .user_profile
[~]
[~][Y] ~Y.IntoArray
[~][<] .user_profile
[~][>] .profiles                 # Type: pg\array{#UserProfile}
```

---

#### Join Operators: `[Y]` and `~Y.*`

**Y = Join** (not Yield) - The Y symbol visually represents **parallel processes converging/joining**.

**Purpose:** Collect results from parallel/sequential iterations into a single output collection.

**Join Block Syntax:**
```polyglot
[~][Y] ~Y.PackOperator
[~][<] .result_per_iteration
[~][>] .output_collection
```

---

**Join Operators:**

| Operator | Purpose | Output Type | Example |
|----------|---------|-------------|---------|
| `~Y.IntoArray` | Pack into array | `pg\array{T}` | `[Y] ~Y.IntoArray [<] .item [>] .items` |
| `~Y.IntoSerial` | Pack into serial/struct | `pg\serial` | `[Y] ~Y.IntoSerial [<] .field [>] .struct` |
| `~Y.IntoSet` | Pack into set (unique values) | `pg\set{T}` | `[Y] ~Y.IntoSet [<] .value [>] .unique_values` |

**Example - IntoArray:**
```polyglot
[p] ~ForEach
[<] .items: pg\array{pg\string}
[>] .item
[~] // Process .item → .processed
[~][Y] ~Y.IntoArray
[~][<] .processed
[~][>] .results: pg\array{pg\string}  # Joined array
```

**Example - IntoSet (Unique Values):**
```polyglot
[p] ~ForEach
[<] .items: pg\array{pg\string}
[>] .item
[~][<] .trimmed << .item.trim()
[~]
[~][Y] ~Y.IntoSet                      # Only unique values
[~][<] .trimmed
[~][>] .unique_items: pg\set{pg\string}
```

---

#### Execution Modes: Parallel, Sequential, Background

**1. `[p]` - Parallel Execution + Join**

**Use when:** Results needed, iterations can run concurrently

```polyglot
[p] ~ForEach                      # Parallel execution
[<] .items
[>] .item
[~] // Each iteration runs in parallel
[~][Y] ~Y.IntoArray              # Join results (REQUIRED with [p])
[~][<] .result
[~][>] .results
```

**Characteristics:**
- All iterations execute concurrently
- Auto-await on each iteration independently
- Results collected via `[Y]` join
- **Must have `[Y]` block** (compile error otherwise)

---

**2. `[r]` - Sequential Execution + Join**

**Use when:** Order matters, or sequential processing required

```polyglot
[r] ~ForEach                      # Sequential execution
[<] .items
[>] .item
[~] // Each iteration runs one at a time
[~][Y] ~Y.IntoArray              # Join results (REQUIRED with [r])
[~][<] .result
[~][>] .results
```

**Characteristics:**
- Iterations execute one at a time (order preserved)
- Auto-await blocks sequential progression
- Results collected via `[Y]` join
- **Must have `[Y]` block** (compile error otherwise)

---

**3. `[b]` - Background Fire-and-Forget**

**Use when:** Side effects only, no results needed

```polyglot
[b] ~ForEach                      # Background execution
[<] .items
[>] .item
[~][r] |LogItem                  # Side effect (no results collected)
[~][<] .msg << "Processing: {.item}"
                                  # NO [Y] block - fire and forget
```

**Characteristics:**
- Iterations run in background (non-blocking)
- No join, no results collected
- **Cannot have `[Y]` block** (compile error if present)
- Used for logging, notifications, background tasks

---

#### Nested Unpacks

**Unpack operators can nest** - same rules apply at each level.

**Important:** Since collections cannot contain collections directly, nested structures require **enumeration wrapping**.

**Example: Nested ~ForEach with Enumeration Wrapping**

**First, define enumeration to wrap inner collection:**
```polyglot
[#] Row
[<] .cells: pg\array{pg\int}              # Inner array wrapped in enumeration
[X]
```

**Then use nested unpacks:**
```polyglot
[i] .matrix: pg\array{#Row}               # Array of Row enumerations

[p] ~ForEach                              # Outer loop (parallel)
[<] .matrix
[>] .row                                  # Type: #Row
[~][p] ~ForEach                          # Inner loop (parallel)
[~][<] .row.cells                        # Access wrapped array
[~][>] .cell                             # Type: pg\int
[~][r] |ProcessCell
[~][<] .value: pg\int << .cell
[~][>] .processed: pg\int >> .cell_result
[~]
[~][Y] ~Y.IntoArray                      # Inner join
[~][<] .cell_result
[~][>] .processed_cells                  # Type: pg\array{pg\int}
[~]
[~]
[~][Y] ~Y.IntoArray                      # Outer join
[~][<] .processed_cells
[~][>] .processed_matrix                 # Type: pg\array{pg\array{pg\int}}
```

**Wait - the output still has nested arrays!**

**Corrected: Wrap output too:**
```polyglot
[#] Row
[<] .cells: pg\array{pg\int}
[X]

[#] ProcessedRow
[<] .processed_cells: pg\array{pg\int}
[X]

[i] .matrix: pg\array{#Row}

[p] ~ForEach
[<] .matrix
[>] .row                                  # Type: #Row
[~][p] ~ForEach
[~][<] .row.cells
[~][>] .cell                             # Type: pg\int
[~][r] |ProcessCell
[~][<] .value: pg\int << .cell
[~][>] .processed: pg\int >> .cell_result
[~]
[~][Y] ~Y.IntoArray
[~][<] .cell_result
[~][>] .processed_cells_array            # Type: pg\array{pg\int}
[~]
[~]
[~][<] .processed_row_instance: #ProcessedRow << #ProcessedRow{.processed_cells: .processed_cells_array}
[~]
[~][Y] ~Y.IntoArray
[~][<] .processed_row_instance
[~][>] .processed_matrix                 # Type: pg\array{#ProcessedRow}
```

**Key Points:**
- Each level has its own scope
- Inner collections must be **wrapped in enumerations**
- Output collections also wrapped if nested structure needed
- Can mix `[p]`, `[r]`, `[b]` at different levels
- Each level can have its own `[Y]` join

---

#### Wrapping Pattern for Nested Collections

**Problem:** Need nested collections (e.g., matrix, tree structure)

**Solution:** Wrap inner collections in enumerations

**Pattern:**
```polyglot
# Step 1: Define enumeration wrapper
[#] InnerStructure
[<] .inner_collection: pg\array{T}       # or pg\set{T}, etc.
[X]

# Step 2: Use outer collection with enumeration
[i] .outer: pg\array{#InnerStructure}

# Step 3: Access via enumeration fields
[p] ~ForEach
[<] .outer
[>] .item                                 # Type: #InnerStructure
[~][p] ~ForEach
[~][<] .item.inner_collection            # Access wrapped collection
[~][>] .inner_item                       # Type: T
```

**Real-World Examples:**

**Example 1: Matrix (2D Array)**
```polyglot
[#] Row
[<] .values: pg\array{pg\int}
[X]

[i] .matrix: pg\array{#Row}
```

**Example 2: Tree Structure**
```polyglot
[#] TreeNode
[<] .value: pg\int
[<] .children: pg\array{#TreeNode}       # Recursive structure
[X]
```

**Example 3: Grouped Data**
```polyglot
[#] UserGroup
[<] .group_name: pg\string
[<] .members: pg\array{pg\string}
[X]

[i] .groups: pg\array{#UserGroup}
```

**Example 4: Table with Rows**
```polyglot
[#] TableRow
[<] .columns: pg\array{pg\string}
[X]

[i] .table: pg\array{#TableRow}
```

---

#### Complete Example: File Processing Pipeline

```polyglot
[i] .pyfile: pg\path << \\FileDir\\process.py
[i] .input_files: pg\array{pg\path}

[p] ~ForEach
[<] .input_files
[>] .file                                # Type: pg\path
[~][r] |U.Python.Run
[~][<] .py: pg\path << .pyfile
[~][<] .args: pg\serial << {.input:python\str << .file}
[~][>] .output: pg\string >> .file_result
[~]
[~][Y] ~Y.IntoArray
[~][<] .file_result
[~][>] .all_results                      # Type: pg\array{pg\string}

[r] |ProcessResults
[<] .results: pg\array{pg\string} << .all_results
[>] .summary: pg\string >> .final_summary
```

**What happens:**
1. Pull `.input_files` (auto-await if Pending)
2. Infer `.file` type: `pg\path` (from array element)
3. Process each file in parallel (`[p]`)
4. Run Python script for each file
5. Join results into `.all_results` array
6. Process final results sequentially

---

#### Mini-Scope Rules

**Variables inside unpack body:**
- `.item`, `.index`, iteration variables → scoped to unpack body
- `.result` (pre-join) → scoped to unpack body
- `.packed_result` (post-join via `[Y]`) → **available outside** unpack scope

**Example:**
```polyglot
[p] ~ForEach
[<] .items
[>] .item                        # Scoped to unpack body
[~][<] .processed << .item       # Scoped to unpack body
[~][Y] ~Y.IntoArray
[~][<] .processed
[~][>] .results                  # Available OUTSIDE unpack

[r] |UseResults
[<] .input << .results           # ✅ .results available here
[<] .bad << .item                # ❌ .item NOT available (scoped)
```

---

#### Error Handling in Unpacks

**Question for clarification:** If one iteration fails, what happens?

**Option A: Fail-fast**
- First error aborts entire unpack
- No results collected

**Option B: Continue on error**
- Other iterations continue
- Failed iterations produce `Faulted` state
- Join collects successful results only

**Option C: Error collection**
- All iterations complete
- Errors collected separately (`.errors` array)
- Results include both successes and failures

---

## Project Initialization

**First Implementation Story: Initialize Cargo Workspace**

```bash
cargo init polyglot
```

Then configure `Cargo.toml` as a workspace with the following structure:

```toml
[workspace]
members = [
    "crates/polyglot-cli",
    "crates/polyglot-lexer",
    "crates/polyglot-parser",
    "crates/polyglot-ir",
    "crates/polyglot-trigger-monitor",
    "crates/polyglot-queue-manager",
    "crates/polyglot-runner",
    "crates/polyglot-runtime-wrappers",
    "crates/polyglot-db",
]
resolver = "2"
```

**Rationale:** Manual workspace setup provides precise control over service boundaries, shared library organization, and dependency management for Polyglot's specialized language implementation architecture.

## Decision Summary

| Category | Decision | Version | Affects FR Categories | Rationale |
| -------- | -------- | ------- | --------------------- | --------- |
| Project Setup | Manual Cargo Workspace | Rust 2021 Edition | All | Specialized architecture requires custom workspace structure |
| Database Client | SQLx | 0.8.6 | FR10-18, FR27-40, FR95-102 | Async-first design, compile-time query verification, lightweight |
| Database Migrations | sqlx-cli | (bundled) | FR10-18 | Version-controlled SQL migrations |
| Queue Client | redis | 0.32.7 | FR27-40 | Standard Rust Redis client with tokio-comp async support |
| IR Serialization | JSON (serde_json) | 1.0.140 | FR1-9 | Human-readable for MVP debugging, PostgreSQL JSONB native support |
| IR Storage | PostgreSQL JSONB | - | FR3-5, FR10-18 | Hybrid: document storage for IR + relational power for metadata |
| CLI Framework | clap (derive API) | 4.5 | FR54-74 | Standard Rust CLI framework, auto-generated help, type-safe parsing |
| Error Handling (Libs) | thiserror | 2.0.17 | All library crates | Structured error types with custom variants |
| Error Handling (Bins) | anyhow | 1.0.99 | All binary crates | Simple propagation with context chaining, async-safe (Send+Sync) |
| Logging Framework | tracing + tracing-subscriber | 0.1.41 + 0.3.19 | FR95-102 | Async-native structured logging, OpenTelemetry ready |
| Configuration Format | TOML | 0.9.8 | FR75-83 | Rust ecosystem standard, human-readable |
| Configuration Library | config | 0.15.15 | FR75-83 | Layered config (defaults → file → env vars), 12-factor app support |
| Testing Organization | Rust standard | - | All | Unit tests inline with #[cfg(test)], integration tests in tests/, E2E separate |
| IR Type Definitions | Rust structs + serde | - | FR1-9 | .pg types map to Rust enums/structs, serde for JSON serialization |
| Time-Series Database | InfluxDB | 2.x | FR19-26, FR95-102 | Stores time-based triggers, trigger results, resource metrics (CPU/RAM/GPU) |
| IR Structure | 3 Separate IRs | - | FR1-9 | Polyglot code → {Trigger IR, Queue IR, Runner IR} as separate JSONB columns |
| Lexer Generator | logos | 0.14 | FR1-2, FR6-7 | Declarative token definitions, compile-time DFA generation, 45+ token types, <100ms performance |

## Project Structure

```
polyglot/
├── Cargo.toml                          # Workspace root
├── polyglot.toml.example               # Example configuration
├── README.md
├── LICENSE
│
├── crates/
│   ├── polyglot-cli/                   # CLI binary (FR54-FR74)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── main.rs                 # CLI entry point
│   │   │   ├── commands/               # Subcommands
│   │   │   │   ├── mod.rs
│   │   │   │   ├── compile.rs
│   │   │   │   ├── register.rs
│   │   │   │   ├── activate.rs
│   │   │   │   ├── trigger.rs
│   │   │   │   ├── status.rs
│   │   │   │   └── services.rs
│   │   │   └── config.rs               # Config loading
│   │   └── tests/
│   │
│   ├── polyglot-lexer/                 # Lexer library (FR1-FR9)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── token.rs                # Token types
│   │   │   ├── lexer.rs                # Lexer implementation
│   │   │   └── error.rs                # LexerError (thiserror)
│   │   └── tests/
│   │       └── lexer_tests.rs
│   │
│   ├── polyglot-parser/                # Parser library (FR1-FR9)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── parser.rs               # Parser implementation
│   │   │   ├── ast.rs                  # AST types
│   │   │   └── error.rs                # ParserError (thiserror)
│   │   └── tests/
│   │       └── parser_tests.rs
│   │
│   ├── polyglot-ir/                    # Intermediate Representation (FR3-FR5)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── types.rs                # IR type definitions
│   │   │   ├── pipeline.rs             # Pipeline IR
│   │   │   ├── trigger.rs              # Trigger IR
│   │   │   ├── validation.rs           # IR validation
│   │   │   └── error.rs                # IrError (thiserror)
│   │   └── tests/
│   │
│   ├── polyglot-db/                    # Database layer (FR10-FR18)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── models.rs               # DB models
│   │   │   ├── pipelines.rs            # Pipeline queries
│   │   │   ├── instances.rs            # Instance queries
│   │   │   ├── triggers.rs             # Trigger queries
│   │   │   └── error.rs                # DbError (thiserror)
│   │   ├── migrations/                 # sqlx migrations
│   │   │   ├── 20250116_001_create_pipelines.sql
│   │   │   ├── 20250116_002_create_instances.sql
│   │   │   └── 20250116_003_create_triggers.sql
│   │   └── tests/
│   │
│   ├── polyglot-trigger-monitor/       # Service: Trigger Monitor (FR19-FR26)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── main.rs                 # Service entry point
│   │   │   ├── monitor.rs              # TriggerMonitor struct
│   │   │   ├── handlers/               # Trigger handlers
│   │   │   │   ├── mod.rs
│   │   │   │   ├── trait.rs            # TriggerHandler trait
│   │   │   │   ├── time.rs             # TimeTrigger
│   │   │   │   ├── webhook.rs          # WebhookTrigger
│   │   │   │   ├── file_watch.rs       # FileWatchTrigger
│   │   │   │   └── manual.rs           # ManualTrigger
│   │   │   ├── registry.rs             # Dynamic trigger registry
│   │   │   └── config.rs
│   │   └── tests/
│   │
│   ├── polyglot-queue-manager/         # Service: Queue Manager (FR27-FR40)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── manager.rs              # QueueManager struct
│   │   │   ├── queue.rs                # Queue operations
│   │   │   └── config.rs
│   │   └── tests/
│   │
│   ├── polyglot-runner/                # Service: Runner (FR30-FR53)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── runner.rs               # Runner struct
│   │   │   ├── executor.rs             # Pipeline execution
│   │   │   └── config.rs
│   │   └── tests/
│   │
│   └── polyglot-runtime-wrappers/      # Runtime integration (FR41-FR53)
│       ├── Cargo.toml
│       ├── src/
│       │   ├── lib.rs
│       │   ├── trait.rs                # RuntimeWrapper trait
│       │   ├── python.rs               # Python runtime wrapper
│       │   └── error.rs                # WrapperError (thiserror)
│       └── tests/
│
├── docs/                               # Documentation (FR84-FR94)
│   ├── v0.0.1/                         # Archived v0.0.1 docs
│   ├── architecture.md                 # This document
│   ├── prd.md
│   └── product-brief-Polyglot-2025-11-15.md
│
├── examples/                           # Example .pg files (FR84-FR94)
│   ├── hello_world.pg
│   ├── python_integration.pg
│   └── automation_workflow.pg
│
└── migrations/                         # Global migrations (symlink to polyglot-db/migrations)
```

## FR Category to Architecture Mapping

| FR Category | Architecture Components |
| ----------- | ----------------------- |
| Pipeline Development & Compilation (FR1-FR9) | polyglot-lexer, polyglot-parser, polyglot-ir, polyglot-cli |
| Pipeline Registry & Lifecycle (FR10-FR18) | polyglot-db, polyglot-cli |
| Trigger System (FR19-FR26) | polyglot-trigger-monitor, polyglot-db |
| Queue Management & Execution (FR27-FR40) | polyglot-queue-manager, polyglot-runner, polyglot-db |
| Runtime Integration & FFI (FR41-FR53) | polyglot-runner, polyglot-runtime-wrappers |
| CLI & Developer Tools (FR54-FR74) | polyglot-cli |
| Installation & Configuration (FR75-FR83) | All crates |
| Documentation & Examples (FR84-FR94) | docs/, examples/ |
| Observability & Monitoring (FR95-FR102) | All services (logging) |
| IDE & Tooling Integration (FR103-FR106) | Future: LSP server |
| Package Ecosystem (FR107-FR111) | Future: registry service |
| Advanced Features (FR112-FR120) | Future enhancements |

## Technology Stack Details

### Core Technologies

**Async Runtime:**
- Tokio 1.x (latest stable)
- All services and I/O operations use Tokio async runtime

**Database Stack:**
- PostgreSQL (3 JSONB columns for Trigger/Queue/Runner IRs, relational for metadata)
- SQLx 0.8.6 with `tokio-comp`, `postgres`, `macros` features
- sqlx-cli for migrations
- Connection pooling via SQLx built-in pool

**Time-Series Database:**
- InfluxDB 2.x (MVP requirement)
- Stores: Time-based trigger schedules, trigger execution results, resource metrics
- Used by: Trigger Monitor, Resource Monitor subservice

**Queue System:**
- Redis 0.32.7 with `tokio-comp` feature
- Queue operations: Default Queue, Pause Queue (MVP), User-defined queues (post-MVP)
- Fallback: PostgreSQL polling when Redis is down (`pipeline_instances WHERE status='queued'`)

**Serialization:**
- serde 1.x for Rust struct serialization
- serde_json 1.0.140 for IR JSON format
- PostgreSQL JSONB native storage

**CLI:**
- clap 4.5 with derive API
- Auto-generated help and version info

**Error Handling:**
- thiserror 2.0.17 for library error types (Send + Sync for async)
- anyhow 1.0.99 for binary error propagation with context

**Logging:**
- tracing 0.1.41 (async-native structured logging)
- tracing-subscriber 0.3.19 with `env-filter` and `json` features
- Format: Structured JSON logs with spans

**Configuration:**
- config 0.15.15 (layered config system)
- TOML 0.9.8 (file format)
- Environment variable overrides

**Date/Time:**
- chrono (latest) for timestamp handling
- UTC timestamps, ISO 8601 format in logs
- PostgreSQL TIMESTAMPTZ columns

### Integration Points

**Service Communication:**
- Services communicate via PostgreSQL (state) and Redis (queues)
- Trigger Monitor → Queue Manager: Via Redis
- No direct HTTP/RPC between services (database-driven architecture)

**Database Interactions:**
- PostgreSQL: Pipeline registry, instance state, activation status, triggers table
- InfluxDB: Time-based trigger schedules, trigger program results, resource metrics
- Redis: Dispatch queues (default, pause), trigger events

**Trigger Monitor Architecture:**
- **Reads PostgreSQL**: Which trigger programs to activate (including Resource Monitor)
- **Reads InfluxDB**: Results from activated trigger programs
- **Spawns Trigger Programs**: Time-based, resource-based, webhooks (multiple instances if settings differ)
- **Resource Monitor Subservice**: Monitors CPU, RAM, GPU, Network at fixed intervals (selective monitoring)

**PostgreSQL LISTEN/NOTIFY:**
- Trigger IR updates: `NOTIFY trigger_updated, '{"pipeline_id": "uuid"}'`
- Trigger Monitor listens and reloads trigger programs

**Queue Manager Logic:**
- Receives trigger events via Redis
- Applies Queue IR `[t]` logic (timing/dispatch rules)
- Applies Queue IR `[Q]` logic (queue selection: default, pause, user-defined)
- Fallback: Polls `pipeline_instances WHERE status='queued'` if Redis down

**Runner Execution Modes (from Runner IR):**
- Sequential: Steps execute one after another
- Parallel: Steps execute concurrently
- Background: Fire-and-forget execution
- Join: Combine parallel execution results

## Novel Architectural Patterns

### Dynamic Trigger Loading System

**Pattern Name:** Dynamic Trigger Registry with Hybrid Monitoring

**Purpose:** Enable runtime-configurable pipeline triggers without service restarts

**Components:**

1. **Trigger IR Storage** (PostgreSQL JSONB)
   - Trigger configuration stored alongside pipeline IR
   - Supports multiple trigger types per pipeline

2. **PostgreSQL LISTEN/NOTIFY**
   - Database publishes `trigger_updated` notifications
   - Trigger Monitor subscribes and reacts to changes

3. **Dynamic Handler Registry**
   ```rust
   struct TriggerMonitor {
       handlers: HashMap<TriggerId, Box<dyn TriggerHandler>>,
       db_pool: PgPool,
   }

   trait TriggerHandler: Send + Sync {
       async fn start(&mut self) -> Result<()>;
       async fn stop(&mut self) -> Result<()>;
   }
   ```

4. **Hybrid Trigger Types:**
   - **Async Listening**: Webhook (HTTP server), File Watch (fs events), DB Events
   - **Sync Loop**: Time-based (cron schedules), Manual (CLI triggered)

**Data Flow:**
1. User activates pipeline → CLI updates database → PostgreSQL NOTIFY
2. Trigger Monitor receives notification → Parses trigger IR → Spawns appropriate handler
3. Handler monitors for trigger condition → Detects event → Creates pipeline instance in DB
4. Queue Manager picks up instance → Queues to Redis → Runner executes

**Implementation Guide for AI Agents:**
- All trigger handlers implement `TriggerHandler` trait
- Handlers are spawned as separate tokio tasks
- On IR update, old handler is stopped, new handler started
- Each handler type lives in `crates/polyglot-trigger-monitor/src/handlers/<type>.rs`

**Affects FR Categories:**
- FR19-FR26 (Trigger System)
- FR27-FR40 (Queue Management - trigger creates instances)

---

## Implementation Patterns

These patterns ensure AI agents write compatible code across all workspace crates.

### Naming Conventions

**Module/File Naming:**
- Snake_case for file names: `trigger_monitor.rs`, `queue_manager.rs`
- Match crate names: `polyglot-trigger-monitor` → `trigger_monitor/`

**Type Naming:**
- PascalCase for structs/enums: `TriggerMonitor`, `PipelineInstance`, `LexerError`
- Trait names describe capability: `TriggerHandler`, `RuntimeWrapper`

**Function Naming:**
- Snake_case: `load_triggers()`, `execute_pipeline()`, `connect_db()`
- Async functions: No special prefix (`.await` makes it clear)

**Database Naming:**
- Tables: Plural snake_case: `pipelines`, `pipeline_instances`, `triggers`
- Columns: Snake_case: `pipeline_id`, `created_at`, `activated`
- Foreign keys: `<table>_id` format: `pipeline_id`, `instance_id`
- JSONB columns: `ir` (for intermediate representation)

**Error Variants:**
- PascalCase with descriptive names: `LexerError::UnexpectedChar`, `DbError::ConnectionFailed`
- Include context in variant: `InvalidTrigger { trigger_id: Uuid, reason: String }`

### Structure Patterns

**Crate Organization:**
```
crate-name/
├── Cargo.toml
├── src/
│   ├── lib.rs or main.rs       # Entry point
│   ├── <domain>.rs             # Core logic (e.g., lexer.rs, monitor.rs)
│   ├── error.rs                # Error types (thiserror)
│   ├── config.rs               # Configuration structs
│   └── <sub>/                  # Sub-modules if complex
│       ├── mod.rs
│       └── <feature>.rs
└── tests/                      # Integration tests
    └── <crate>_tests.rs
```

**Test Organization:**
- Unit tests: Inline with `#[cfg(test)] mod tests { ... }`
- Integration tests: `tests/` directory
- Test file naming: `<feature>_tests.rs`

**Error Module Pattern (All Library Crates):**
```rust
// error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CrateNameError {
    #[error("Specific error: {0}")]
    SpecificError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

// Ensure Send + Sync for async
impl std::error::Error for CrateNameError {}
```

**Binary Main Pattern (All Services + CLI):**
```rust
// main.rs
use anyhow::{Context, Result};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Initialize tracing
    tracing_subscriber::fmt::init();

    // 2. Load config
    let config = Config::load()
        .context("Failed to load configuration")?;

    // 3. Connect to database
    let pool = connect_db(&config.database_url).await
        .context("Failed to connect to database")?;

    // 4. Run service/CLI
    run(pool, config).await
}
```

### Format Patterns

**Database Model Serialization:**
```rust
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Pipeline {
    pub id: Uuid,
    pub name: String,
    pub ir: sqlx::types::Json<PipelineIr>,  // JSONB column
    pub created_at: DateTime<Utc>,
    pub activated: bool,
}
```

**API Response Format (Future HTTP endpoints):**
```rust
#[derive(Serialize)]
struct ApiResponse<T> {
    data: Option<T>,
    error: Option<ApiError>,
}
```

**Date/Time Format:**
- Internal: `chrono::DateTime<Utc>`
- Database: PostgreSQL `TIMESTAMPTZ`
- Logs: ISO 8601 via tracing JSON formatter
- User-facing: ISO 8601 strings

**Logging Format:**
```rust
use tracing::{info, error, warn, debug, instrument};

#[instrument(skip(pool))]  // Auto-trace function entry/exit
async fn load_pipelines(pool: &PgPool) -> Result<Vec<Pipeline>> {
    info!("Loading active pipelines");

    match query_pipelines(pool).await {
        Ok(pipelines) => {
            info!(count = pipelines.len(), "Loaded pipelines");
            Ok(pipelines)
        }
        Err(e) => {
            error!(error = %e, "Failed to load pipelines");
            Err(e.into())
        }
    }
}
```

### Communication Patterns

**Service-to-Service (Via Database):**
- No direct HTTP/RPC calls between services
- State changes via PostgreSQL transactions
- Queue operations via Redis
- Event notifications via PostgreSQL LISTEN/NOTIFY

**Database Transaction Pattern:**
```rust
async fn create_and_queue_instance(
    pool: &PgPool,
    pipeline_id: Uuid,
) -> Result<Uuid> {
    let mut tx = pool.begin().await?;

    // 1. Create instance in database
    let instance_id = sqlx::query_scalar!(
        "INSERT INTO pipeline_instances (pipeline_id, status)
         VALUES ($1, 'created') RETURNING id",
        pipeline_id
    )
    .fetch_one(&mut *tx)
    .await?;

    // 2. Update status to queued
    sqlx::query!(
        "UPDATE pipeline_instances SET status = 'queued' WHERE id = $1",
        instance_id
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(instance_id)
}
```

**Redis Queue Pattern:**
```rust
use redis::AsyncCommands;

async fn queue_instance(
    redis: &mut RedisConnection,
    instance_id: Uuid,
) -> Result<()> {
    redis.rpush("dispatch_queue", instance_id.to_string()).await?;
    Ok(())
}

async fn dequeue_instance(
    redis: &mut RedisConnection,
) -> Result<Option<Uuid>> {
    let result: Option<String> = redis.blpop("dispatch_queue", 0.0).await?;
    Ok(result.map(|s| Uuid::parse_str(&s).unwrap()))
}
```

### Lifecycle Patterns

**Service Startup Pattern:**
1. Initialize tracing
2. Load configuration
3. Connect to database (with retry logic)
4. Connect to Redis (if needed)
5. Run main service loop
6. Handle graceful shutdown

**Graceful Shutdown Pattern:**
```rust
use tokio::signal;

async fn run_with_shutdown(service: Service) -> Result<()> {
    tokio::select! {
        result = service.run() => result,
        _ = signal::ctrl_c() => {
            info!("Received shutdown signal");
            service.shutdown().await
        }
    }
}
```

**Pipeline Instance State Transitions:**
- Created → Queued → Running → Exited (success or failure)
- All transitions logged with tracing
- State changes via database updates with timestamps

### Location Patterns

**Configuration File:**
- Location: `polyglot.toml` in project root or `~/.config/polyglot/polyglot.toml`
- Environment override: `POLYGLOT_CONFIG` env var
- Per-service sections: `[database]`, `[redis]`, `[services]`

**Database Migrations:**
- Location: `crates/polyglot-db/migrations/`
- Naming: `YYYYMMDD_NNN_description.sql` (e.g., `20250116_001_create_pipelines.sql`)
- Run via: `sqlx migrate run`

**Logs:**
- Stdout (JSON structured) for containerized deployment
- File logging (optional) via `tracing-appender`

**Example `.pg` Files:**
- Location: `examples/` directory
- Naming: `<use_case>.pg` (e.g., `hello_world.pg`, `python_integration.pg`)

### Consistency Patterns (Cross-Cutting)

**UUID Generation:**
```rust
use uuid::Uuid;

let id = Uuid::new_v4();  // Always v4 UUIDs
```

**Timestamp Creation:**
```rust
use chrono::Utc;

let now = Utc::now();  // Always UTC
```

**Configuration Loading:**
```rust
use config::Config;

let settings = Config::builder()
    .add_source(config::File::with_name("polyglot"))
    .add_source(config::Environment::with_prefix("POLYGLOT"))
    .build()?;
```

**Database Connections:**
```rust
use sqlx::postgres::PgPoolOptions;

let pool = PgPoolOptions::new()
    .max_connections(config.pool_size)
    .connect(&config.database_url)
    .await?;
```

**All Agents MUST Follow These Patterns**

These implementation patterns are the consistency contract. Any deviation will cause integration failures or agent conflicts.

## Data Architecture

### Database Schema

**pipelines table:**
```sql
CREATE TABLE pipelines (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT UNIQUE NOT NULL,
    version INTEGER NOT NULL DEFAULT 1,
    trigger_ir JSONB NOT NULL,          -- Trigger IR (JSON)
    queue_ir JSONB NOT NULL,            -- Queue IR (JSON) - contains [t] and [Q] logic
    runner_ir JSONB NOT NULL,           -- Runner IR (JSON) - execution modes
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    activated BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE INDEX idx_pipelines_activated ON pipelines(activated) WHERE activated = TRUE;
CREATE INDEX idx_pipelines_trigger_ir_gin ON pipelines USING GIN(trigger_ir);
CREATE INDEX idx_pipelines_queue_ir_gin ON pipelines USING GIN(queue_ir);
CREATE INDEX idx_pipelines_runner_ir_gin ON pipelines USING GIN(runner_ir);
```

**triggers table:**
```sql
CREATE TABLE triggers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pipeline_id UUID NOT NULL REFERENCES pipelines(id) ON DELETE CASCADE,
    trigger_type TEXT NOT NULL CHECK (trigger_type IN ('time', 'resource', 'webhook', 'manual', 'file_watch')),
    config JSONB NOT NULL,              -- Trigger-specific configuration
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_pipeline FOREIGN KEY (pipeline_id) REFERENCES pipelines(id)
);

CREATE INDEX idx_triggers_pipeline ON triggers(pipeline_id);
CREATE INDEX idx_triggers_type ON triggers(trigger_type);
CREATE INDEX idx_triggers_config_gin ON triggers USING GIN(config);
```

**pipeline_instances table:**
```sql
CREATE TABLE pipeline_instances (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pipeline_id UUID NOT NULL REFERENCES pipelines(id) ON DELETE CASCADE,
    status TEXT NOT NULL CHECK (status IN ('created', 'queued', 'running', 'exited')),
    exit_code INTEGER,                  -- NULL if not exited
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    queued_at TIMESTAMPTZ,
    started_at TIMESTAMPTZ,
    exited_at TIMESTAMPTZ,
    CONSTRAINT fk_pipeline FOREIGN KEY (pipeline_id) REFERENCES pipelines(id)
);

CREATE INDEX idx_instances_pipeline ON pipeline_instances(pipeline_id);
CREATE INDEX idx_instances_status ON pipeline_instances(status);
CREATE INDEX idx_instances_created ON pipeline_instances(created_at DESC);
```

**execution_logs table:**
```sql
CREATE TABLE execution_logs (
    id BIGSERIAL PRIMARY KEY,
    instance_id UUID NOT NULL REFERENCES pipeline_instances(id) ON DELETE CASCADE,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    level TEXT NOT NULL CHECK (level IN ('debug', 'info', 'warn', 'error')),
    message TEXT NOT NULL,
    context JSONB,                      -- Structured log context
    CONSTRAINT fk_instance FOREIGN KEY (instance_id) REFERENCES pipeline_instances(id)
);

CREATE INDEX idx_logs_instance ON execution_logs(instance_id, timestamp DESC);
CREATE INDEX idx_logs_timestamp ON execution_logs(timestamp DESC);
```

**variable_state_transitions table:**
```sql
CREATE TABLE variable_state_transitions (
    id BIGSERIAL PRIMARY KEY,
    instance_id UUID NOT NULL REFERENCES pipeline_instances(id) ON DELETE CASCADE,
    variable_name TEXT NOT NULL,              -- Fully qualified variable path (e.g., ".user_data.address.city")
    from_state TEXT NOT NULL CHECK (from_state IN ('Declared', 'DefaultReady', 'Pending', 'Ready', 'Faulted', 'Retrying', 'Paused', 'Cached', 'Dirty')),
    to_state TEXT NOT NULL CHECK (to_state IN ('Declared', 'DefaultReady', 'Pending', 'Ready', 'Faulted', 'Retrying', 'Paused', 'Cached', 'Dirty')),
    transition_timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    context JSONB,                            -- Additional context: trigger type, error details, etc.
    CONSTRAINT fk_instance FOREIGN KEY (instance_id) REFERENCES pipeline_instances(id)
);

CREATE INDEX idx_state_transitions_instance ON variable_state_transitions(instance_id, transition_timestamp DESC);
CREATE INDEX idx_state_transitions_variable ON variable_state_transitions(variable_name, transition_timestamp DESC);
CREATE INDEX idx_state_transitions_timestamp ON variable_state_transitions(transition_timestamp DESC);
```

**Purpose:** Audit trail for variable state transitions, useful for debugging, performance analysis, and understanding variable lifecycle in production.

**PostgreSQL NOTIFY Trigger:**
```sql
-- Notify when pipeline activation changes
CREATE OR REPLACE FUNCTION notify_trigger_updated()
RETURNS trigger AS $$
BEGIN
    IF (TG_OP = 'UPDATE' AND OLD.activated != NEW.activated) OR (TG_OP = 'INSERT') THEN
        PERFORM pg_notify('trigger_updated', json_build_object(
            'pipeline_id', NEW.id,
            'action', CASE WHEN NEW.activated THEN 'added' ELSE 'removed' END
        )::text);
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_updated_notify
AFTER INSERT OR UPDATE ON pipelines
FOR EACH ROW EXECUTE FUNCTION notify_trigger_updated();
```

### Data Relationships

```
pipelines (1) ──< (N) triggers
pipelines (1) ──< (N) pipeline_instances (1) ──< (N) execution_logs
                                          (1) ──< (N) variable_state_transitions
```

- One pipeline has many triggers
- One pipeline can have many instances
- One instance has many log entries
- One instance has many state transition records
- Cascading delete: Deleting pipeline removes all triggers, instances, logs, and state transitions

### IR Structure

**Compilation Flow:**
```
Polyglot code (.pg file)
    ↓ polyglot compile
{Trigger IR, Queue IR, Runner IR}
    ↓ Stored in PostgreSQL
pipelines table (3 JSONB columns)
```

**Trigger IR (JSONB):**
```json
{
  "triggers": [
    {
      "type": "time",
      "schedule": "0 0 * * *",
      "enabled": true
    },
    {
      "type": "resource",
      "conditions": {
        "cpu_threshold": 50,
        "ram_available_gb": 4
      }
    },
    {
      "type": "manual"
    }
  ]
}
```

**Queue IR (JSONB) - Contains [t] and [Q] logic:**
```json
{
  "timing_logic": {
    "type": "immediate",
    "delay_seconds": 0
  },
  "queue_selector": "default",
  "priority": 5,
  "rate_limit": {
    "max_per_minute": 10
  },
  "resource_limits": {
    "max_cpu_percent": 80,
    "max_ram_gb": 8
  }
}
```

**Runner IR (JSONB) - Execution modes:**
```json
{
  "execution_mode": "sequential",
  "steps": [
    {
      "id": "step1",
      "type": "wrapper",
      "runtime": "Python3.11",
      "code": "print('Hello from Python')",
      "mode": "sequential"
    },
    {
      "id": "step2",
      "type": "wrapper",
      "runtime": "Python3.11",
      "code": "print('Step 2')",
      "mode": "parallel",
      "depends_on": ["step1"]
    }
  ]
}
```

---

### Variable State Model

**Status:** Core language feature (v1.0.0, 2025-11-24)
**Specification:** `docs/technical/variable-states-specification.md`

#### Overview

Polyglot variables are **state-aware** entities that transition through explicit states as async operations complete. This is not an implementation detail—it's the foundational semantic model for all Polyglot data.

Variables don't simply "have values" or "don't have values." They progress through a lifecycle:
- `Declared` → Schema defined, no value yet
- `Pending` → Async operation in progress
- `Ready` → Value available, immutable
- `Faulted` → Operation failed

This state model enables Polyglot's **automatic waiting** behavior: pipelines block on `Pending` variables without explicit `await` keywords.

#### The Nine Variable States

Polyglot defines 9 distinct variable states organized into two categories:

**Core States (5) - MVP Implementation - Required for basic variable lifecycle:**

| State | Description | Can Read Value? | Triggers Pipeline? | Transition Triggers |
|-------|-------------|----------------|-------------------|-------------------|
| `Declared` | Schema defined, no value, no default | ❌ No | ❌ No (must be Ready) | Pipeline assignment, explicit population |
| `DefaultReady` | Has default value, allows ONE override | ✅ Yes (default) | ✅ Yes | First use, override → `Ready` |
| `Pending` | Async operation in progress | ❌ No | ⚠️ Waits | Pipeline completion → `Ready`/`Faulted` |
| `Ready` | Value available, immutable | ✅ Yes | ✅ Yes | None (terminal state) |
| `Faulted` | Operation failed, has error info | ❌ No (check `.errors`) | ❌ No (error path) | Retry → `Retrying` |

**Queue Management States (4) - Post-MVP - Advanced resilience and performance:**

| State | Description | Purpose | Transition |
|-------|-------------|---------|------------|
| `Retrying` | Automatic retry attempt in progress | Transient failure recovery | → `Ready`/`Faulted` |
| `Paused` | Waiting for external trigger | Human approval, scheduled events | → `Pending` (when triggered) |
| `Cached` | Cached result, may be stale | Performance optimization | → `Dirty` (on invalidation) |
| `Dirty` | Cache invalid, needs refresh | Cache invalidation | → `Pending` (on refresh) |

**Implementation Priority:** The 5 core states are required for MVP (v0.1.0). The 4 queue management states will be implemented post-MVP as advanced features for enterprise use cases requiring retry logic, caching, and pause/resume capabilities.

#### Assignment Operators and State Control

**Critical Concept:** Polyglot's `<<` and `>>` operators are **NOT** like `=` in other languages. They are **directional push/pull operators** that define data flow and represent **final assignment** (immutability once Ready).

Polyglot has **three ways** to declare fields, each with different **push count** allowances:

**1. Schema-Only Declaration (No Operator) → 1 Push Allowed**
```polyglot
[#] UserProfile
[<] .id: pg\string          # Declared state - must populate later
[<] .name: pg\string        # No default, no value
[X]

[i] .user: #UserProfile << #UserProfile
[<] .user.id << "user_123"       # ← 1st push: FINAL (now Ready)
[<] .user.id << "user_456"       # ← ERROR: Already had final push
```
- **Initial State:** `Declared`
- **Push Count:** 1 final push with `<<` or `>>`
- **Semantics:** Field has no value until explicitly pushed to
- **After first push:** Transitions to `Ready` (immutable)
- **Use case:** Fields populated by pipelines, required parameters

**2. Default Assignment (`<~` / `~>`) → 2 Pushes Allowed**
```polyglot
[#] Config
[<] .timeout: pg\int <~ 30      # DefaultReady - can override once
[<] .retries: pg\int <~ 3        # Default value provided
[X]

[i] .config: #Config << #Config{.timeout: 60}  # ← 1st push: Override default
[<] .config.timeout << 90                       # ← 2nd push: FINAL (now Ready)
[<] .config.timeout << 120                      # ← ERROR: Already had final push
```
- **Initial State:** `DefaultReady`
- **Push Count:** 2 pushes with `<<` or `>>`
  - **1st push:** Override the default (stays DefaultReady temporarily)
  - **2nd push:** Final assignment (transitions to Ready, immutable)
- **Alternative:** If pulled from without override, uses default and becomes Ready
- **Use case:** Configuration with sensible defaults, optional parameters
- **Discouraged anti-pattern:** Using `<~`/`~>` to pull from constants (`<<`/`>>`)

**3. Constant/Async Assignment (`<<` / `>>`) → 0 or 1 Push**
```polyglot
[#] AppInfo
[<] .version: pg\string << "1.0.0"    # Ready immediately (constant)
[X]

[<] .version << "2.0.0"               # ← ERROR: Already Ready (0 additional pushes)

[r] |FetchData
[>] .result: pg\string >> .data       # Pending → Ready/Faulted (async)
```
- **Initial State:** `Ready` (constant) or `Pending` (async)
- **Push Count:**
  - **Constant:** 0 additional pushes (already final at declaration)
  - **Async:** 1 push when pipeline completes (transitions to Ready/Faulted)
- **Semantics:**
  - `<<` (constant): Immutable, always `Ready`, no further pushes allowed
  - `>>` (async): Starts `Pending`, receives final push from pipeline
- **Use case:** Version numbers, API keys (constant), pipeline outputs (async)

**Operator Summary (Push/Pull Semantics):**

| Declaration | Direction | Purpose | Initial State | Pushes Allowed | Final State |
|-------------|-----------|---------|---------------|----------------|-------------|
| (none) | N/A | Schema-only | `Declared` | **1 push** | `Ready` |
| `<~` / `~>` | `<~` push left, `~>` push right | Default value | `DefaultReady` | **2 pushes** | `Ready` |
| `<<` / `>>` | `<<` push left, `>>` push right | Constant/Async | `Ready` or `Pending` | **0 (const) or 1 (async)** | `Ready` or `Faulted` |

**Key Insight:** `<<` and `>>` are **directional push/pull operators**, not simple assignment (`=`). They define data flow direction and represent final assignment.

---

#### Enumeration vs Serial Fields: The Type Distinction

**Critical Concept:** Within `[#]` enumeration definitions, fields can be one of two types:

1. **Enum Fields** - Declaration only (no type, no value)
2. **Serial Fields** - Type-specified with value

This distinction is fundamental to understanding Polyglot's type system.

---

**1. Enum Fields (Declaration Only)**

**Syntax:** Field declared WITHOUT type, WITHOUT value assignment

```polyglot
[#] Colors
[<] .Red              # Enum field (declaration only)
[<] .Blue             # Enum field (declaration only)
[<] .Green            # Enum field (declaration only)
[X]
```

**Characteristics:**
- No type specified (no `pg\type`)
- No value assigned (no `<<`, `<~`, etc.)
- Pure declaration: defines that this field exists
- Like traditional enums in other languages

---

**2. Serial Fields (Type + Value)**

**Syntax:** Field declared WITH type, WITH value assignment

```polyglot
[#] Config
[<] .timeout: pg\int <~ 30        # Serial field (has type pg\int)
[<] .retries: pg\int <~ 3         # Serial field (has type pg\int)
[<] .version: pg\string << "1.0"  # Serial field (has type pg\string)
[X]
```

**Characteristics:**
- Type specified (`pg\int`, `pg\string`, etc.)
- Value assigned (via `<<`, `<~`, `~>`, or `>>`)
- Hierarchical key-value structure
- Serialized as nested data

---

**3. Polyglot's Unique Feature: Mixing Both in One Definition**

**Rule:** Enum fields and Serial fields **CANNOT be siblings** (same hierarchy level), but **CAN be uncles** (different hierarchy levels).

**❌ INVALID - Siblings (same level):**
```polyglot
[#] Bad
[<] .enumField              # Enum field
[<] .serialField: pg\string # Serial field - ERROR: Can't be siblings!
[X]
```

**✅ VALID - Uncles (different levels):**
```polyglot
[#] Good
[<] .category.option1              # Enum field (nested under .category)
[<] .category.option2              # Enum field (nested under .category)
[<] .metadata: pg\string <~ "info" # Serial field (sibling to .category)
[X]
```

**Another valid example:**
```polyglot
[#] MixedExample
[<] .enumField.constant1           # Enum nested
[<] .enumField.constant2           # Enum nested
[<] .serialField: pg\string        # Serial (uncle to .constant1/.constant2)
[X]
```

**Visual Hierarchy:**
```
MixedExample (root)
├─ enumField (parent node)
│   ├─ constant1 (enum field - child)
│   └─ constant2 (enum field - child)
└─ serialField (serial field - sibling to enumField parent)
```

**Why This Rule Exists:** Enables **exhaustive condition checking** in `[?]` switch blocks. When enum and serial are at different levels, the compiler can verify all enum cases are handled.

---

**4. How to Distinguish Enum vs Serial in Code**

| Feature | Enum Field | Serial Field |
|---------|-----------|--------------|
| Type specified? | ❌ No | ✅ Yes (`pg\type`) |
| Value assigned? | ❌ No | ✅ Yes (`<<`, `<~`, etc.) |
| Purpose | Define constants/options | Store hierarchical data |
| Like traditional... | `enum` in other languages | `struct` or `dictionary` |
| Example | `.Red`, `.Blue` | `.timeout: pg\int <~ 30` |

---

#### Collection Types and Constraints

**Collection Types in Polyglot:**
- `pg\array{T}` - Ordered collection of items
- `pg\set{T}` - Unordered collection of unique items
- `pg\map{K,V}` - Key-value pairs

---

**Critical Constraint: Collections Cannot Contain Collections Directly**

**Rule:** Collection types cannot have another collection as their item type.

**Invalid (Compile Error):**
```polyglot
pg\array{pg\array{pg\int}}           # ❌ Array of arrays - NOT allowed
pg\set{pg\array{pg\string}}          # ❌ Set of arrays - NOT allowed
pg\array{pg\set{pg\int}}             # ❌ Array of sets - NOT allowed
pg\map{pg\string, pg\array{pg\int}} # ❌ Map with array values - NOT allowed
```

**Valid (Allowed):**
```polyglot
pg\array{#UserProfile}               # ✅ Array of enumerations
pg\array{pg\serial}                  # ✅ Array of serial (if needed)
pg\set{#Status}                      # ✅ Set of enumerations
pg\map{pg\string, #Config}           # ✅ Map with enumeration values
```

---

**Why This Constraint Exists:**

1. **Serialization Consistency:** Ensures predictable serialization structure
2. **Type Safety:** Prevents deeply nested type complexity
3. **Performance:** Simplifies runtime type checking
4. **Clarity:** Forces explicit structure via enumerations

---

**Solution: Wrap Collections in Enumerations**

**To create nested collection structures, wrap the inner collection in an enumeration:**

```polyglot
# Define enumeration wrapper for inner collection
[#] Row
[<] .cells: pg\array{pg\int}              # Inner collection wrapped
[X]

# Use outer collection with enumeration
[i] .matrix: pg\array{#Row}               # ✅ Valid: Array of Row enumerations
```

**Benefits of Wrapping:**
- Named structures (`.cells` is more descriptive than `[0]`)
- Additional metadata possible (`.row_id`, `.created_at`, etc.)
- Type-safe access (`.row.cells` vs `[i][j]`)
- Enables recursive structures (trees)

---

**Collection Type Reference:**

**1. `pg\array{T}` - Ordered Array**

**Valid element types:**
- ✅ Primitive types: `pg\int`, `pg\string`, `pg\bool`, `pg\dt`
- ✅ Enumerations: `#UserProfile`, `#Config`
- ✅ Serial: `pg\serial` (if needed)
- ❌ Collections: `pg\array{T}`, `pg\set{T}`, `pg\map{K,V}`

**Example:**
```polyglot
[i] .numbers: pg\array{pg\int}            # ✅ Array of integers
[i] .users: pg\array{#User}               # ✅ Array of User enumerations
[i] .invalid: pg\array{pg\array{pg\int}} # ❌ Compile error
```

---

**2. `pg\set{T}` - Unordered Set (Unique Values)**

**Valid element types:** Same as `pg\array{T}`

**Example:**
```polyglot
[i] .unique_ids: pg\set{pg\string}        # ✅ Set of unique strings
[i] .statuses: pg\set{#Status}            # ✅ Set of Status enumerations
```

**Behavior:**
- Automatically deduplicates values
- No guaranteed order
- Use with `~Y.IntoSet` to collect unique values

---

**3. `pg\map{K,V}` - Key-Value Map**

**Valid key types:**
- ✅ Primitive types that are hashable: `pg\string`, `pg\int`, `pg\uint`
- ❌ Collections: Cannot use collections as keys

**Valid value types:**
- ✅ Primitive types: `pg\int`, `pg\string`, `pg\bool`, `pg\dt`
- ✅ Enumerations: `#Config`, `#User`
- ❌ Collections: Cannot use collections as values directly

**Example:**
```polyglot
[i] .user_ages: pg\map{pg\string, pg\int}     # ✅ Name → Age
[i] .configs: pg\map{pg\string, #Config}      # ✅ Key → Config enum
[i] .invalid: pg\map{pg\string, pg\array{pg\int}}  # ❌ Compile error
```

**Workaround for map with collection values:**
```polyglot
[#] UserData
[<] .scores: pg\array{pg\int}
[X]

[i] .user_data: pg\map{pg\string, #UserData}  # ✅ Map → UserData wrapper
```

---

**Recursive Structures (Trees, Graphs)**

**Enumerations can reference themselves**, enabling recursive structures:

**Example: Binary Tree**
```polyglot
[#] TreeNode
[<] .value: pg\int
[<] .left: #TreeNode                      # ✅ Recursive reference
[<] .right: #TreeNode                     # ✅ Recursive reference
[X]
```

**Example: Graph Node**
```polyglot
[#] GraphNode
[<] .id: pg\string
[<] .neighbors: pg\array{#GraphNode}      # ✅ Array of same type
[X]
```

**Example: Nested Menu**
```polyglot
[#] MenuItem
[<] .label: pg\string
[<] .children: pg\array{#MenuItem}        # ✅ Recursive nested menu
[X]
```

---

#### State Lifecycle Diagram

```
ENUMERATION FIELD DECLARATION
         |
         ↓
    ┌────┴────┬──────────┬──────────┐
    |         |          |          |
 Schema    Default   Constant    Async
  Only      <~ ~>     << >>     Pipeline
    |         |          |          |
    ↓         ↓          ↓          ↓
Declared  DefaultReady Ready    Pending
(no value) (has default)(const)  (waiting)
    |         |          |          |
    |         ↓          |          ↓
    |    [i] block       |      Ready/Faulted
    |   (expected        |          |
    |    Ready)          |          |
    |         |          |          |
    |    ┌────┴────┐     |          |
    |    |         |     |          |
    |    ↓         ↓     |          |
    | Override   Use     |          |
    |    or    Default   |          |
    | Populate           |          |
    |    |      |        |          |
    └────┴──────┴────────┴──────────┘
              |
              ↓
       Ready (immutable)
```

**Extended Lifecycle with Queue States:**
```
Declared ────────┐
                 │
DefaultReady ────┤
                 │
                 ↓
             Pending ──────→ Ready (success)
                 ↓              ↑
                 ↓              │
            Faulted ─→ Retrying ┘
                 │         ↑
                 │         │
            [Retry Logic]  │
                 │         │
                 └─────────┘

Ready ──────→ Cached ──────→ Dirty ──────→ Pending
(result)   (performance) (invalidated)  (refresh)
   │
   ↓
Paused ──────→ [External Trigger] ──────→ Pending
(waiting)      (human approval, etc.)     (resume)
```

#### Reserved Schema: `.*.pgvar.*` (Always Ready)

**Critical Concept:** Every variable in Polyglot has a reserved namespace `.*.pgvar` that provides metadata access.

**Key Property:** ALL fields under `.*.pgvar` are **ALWAYS Ready** (no wait time) because they are database-tracked metadata, not async operations.

---

**Reserved Schema Structure:**

```polyglot
.*                                    # The variable's value itself
.*.pgvar.state: #PgVar.States.*      # Current state (enum)
.*.pgvar.history.{state}.at: pg\dt   # State transition timestamps
```

---

**1. `.*` - Variable Value Access**

Access the variable's actual value:

```polyglot
[<] .result << .user_data.*  # Pull the value of .user_data
```

**Note:** `.*` triggers auto-await if variable is `Pending`.

---

**2. `.*.pgvar.state` - Current State**

**Type:** `#PgVar.States.*` (Reserved Enumeration)

**Purpose:** Query current variable state

**Always Ready:** ✅ No wait time (database-tracked)

**Example:**
```polyglot
[?] .user_data.pgvar.state =? #PgVar.States.Ready
[~][r] |ProcessUser
[~][<] .user << .user_data.*

[?] .user_data.pgvar.state =? #PgVar.States.Faulted
[~][r] |HandleError

[?] *?
[~][r] |U.Log.Warn
[~][<] .msg << "Variable not ready"
```

**State Enumeration:** `#PgVar.States.*` includes:
- `Declared`, `DefaultReady`, `Pending`, `Ready`, `Faulted`
- (Post-MVP: `Retrying`, `Paused`, `Cached`, `Dirty`)

---

**3. `.*.pgvar.history.{state}.at` - State Transition History**

**Type:** `pg\dt` (DateTime timestamp)

**Purpose:** Track when variable entered each state

**Always Ready:** ✅ No wait time (database-tracked)

**Example:**
```polyglot
# When did .user_data become Pending?
[<] .pending_time: pg\dt << .user_data.pgvar.history.Pending.at

# How long in Pending state?
[<] .duration: pg\dt << DT.ToNow"{.user_data.pgvar.history.Pending.at}"

# When did it become Ready?
[<] .ready_time: pg\dt << .user_data.pgvar.history.Ready.at
```

**Structure:**
```
.var.pgvar.history
  ├─ .Declared.at: pg\dt
  ├─ .Pending.at: pg\dt
  ├─ .Ready.at: pg\dt
  └─ .Faulted.at: pg\dt  (if applicable)
```

---

**Why Always Ready?**

The `.*.pgvar` namespace is **metadata tracked in the database**, not async operations. When you access `.var.pgvar.state` or `.var.pgvar.history`, you're querying the variable tracking system, which is **always available** without async wait.

**Contrast:**
- `.var.*` (value) → May be `Pending` (triggers auto-await)
- `.var.pgvar.state` (metadata) → Always `Ready` (no wait)

---

**Error Handling:**

Errors are stored in a separate field:

**`.errors` Field:**
- **Type:** `pg\array{!}` (Array of error objects)
- **Purpose:** Store error details when `Faulted`
- **Availability:** ALL variables
- **Always Ready:** ✅ (metadata)

**Error Object Structure:**
```polyglot
[#] ErrorObject
[<] .type: pg\string              # Error type (e.g., "!pg.Network.Timeout")
[<] .message: pg\string           # Human-readable message
[<] .code: pg\int                 # Error code (optional)
[<] .timestamp: pg\dt             # Timestamp
[<] .context: pg\map{pg\string,pg\string}  # Additional context
[X]
```

**Example:**
```polyglot
[?] .var.pgvar.state =? #PgVar.States.Faulted
[~][r] |U.Log.Error
[~][<] .error_details << .var.errors
```

---

#### DateTime System: `pg\dt` and String Literal Pipelines

**Core Type:** Polyglot uses **ONLY `pg\dt`** for ALL datetime operations (timestamps, durations, differences).

---

**1. `DT.Now` - Current Timestamp**

**Syntax:** `DT.Now` (no arguments)

**Returns:** `pg\dt` (current timestamp)

**Example:**
```polyglot
[r] .start_time: pg\dt << DT.Now  # Capture current time
```

**Note:** Use `[r]` to pull from `DT.Now` pipeline (not `[<]`).

---

**2. `DT.ToNow"{.var}"` - Time Difference**

**Syntax:** `DT.ToNow"{.datetime_var}"` (string literal with variable reference)

**Returns:** `pg\dt` (duration between `.datetime_var` and now)

**String Literal = Inline Pipeline:**
```polyglot
DT.ToNow"{.started}"

# Equivalent to:
[<] .args: pg\serial << {.started: pg\dt}  # ← PULLS from .started (auto-await)
[r] |DT.ToNow
```

**Key:** String literals `"{.var}"` create inline pipelines that **pull from variables**, triggering auto-await if `Pending`.

**Example:**
```polyglot
[r] .started: pg\dt << DT.Now

# Later...
[<] .elapsed: pg\dt << DT.ToNow"{.started}"  # Time since .started
```

---

**3. Duration Literals**

**Syntax:** `DT.Minutes"3"`, `DT.Seconds"30"`, `DT.Hours"2"`

**Returns:** `pg\dt` (duration value)

**Examples:**
```polyglot
[<] .timeout: pg\dt << DT.Minutes"5"   # 5 minute duration
[<] .delay: pg\dt << DT.Seconds"30"    # 30 second duration
[<] .window: pg\dt << DT.Hours"2"      # 2 hour duration
```

**Note:** String literal syntax (`"3"` not `(3)`), not function calls.

---

**4. DateTime Comparisons**

**Operators:** `=?` (equal), `>?` (greater), `<?` (less), `>=?` (greater or equal), `=<?` (less or equal)

**Example:**
```polyglot
[?] DT.ToNow"{.started}" >? DT.Minutes"5"
[~][r] |U.Log.Warn
[~][<] .msg << "Operation took longer than 5 minutes"
```

---

**5. Timeout Pattern Using Switch Blocks**

**Idiomatic way to handle timeouts in Polyglot:**

```polyglot
[r] .timeout_start: pg\dt << DT.Now  # Start timeout timer

# Switch block waits until ONE branch becomes true:
[?] DT.ToNow"{.timeout_start}" >? DT.Minutes"3"  # Timeout branch
[~][r] |U.Log.Error
[~][<] .msg << "Timeout after 3 minutes"
[~][r] |HandleTimeout

[?] DT.ToNow"{.timeout_start}" =<? DT.Minutes"3"  # Within timeout
[&] .var.pgvar.state =? #PgVar.States.Ready       # AND variable Ready
[~][<] .result << .var.*                          # Process result
[~][r] |ProcessResult
```

**How it works:**
1. `[?]` switch block continuously re-evaluates branches
2. First branch: Check if 3 minutes elapsed
3. Second branch with `[&]` AND: Check if within timeout AND variable Ready
4. Whichever branch becomes true first executes
5. `[&]` combines multiple conditions (all must be true)

**Key:** Switch blocks provide built-in timeout mechanism without explicit `await` or sleep.

---

#### String Literal Processing Architecture

**CRITICAL CONCEPT:** String literals in Polyglot are NOT primitive values - they are **inline pipeline calls**.

---

##### Overview

Every string literal in Polyglot syntax is actually syntactic sugar for a pipeline call. This architectural decision enables:
1. **Type flexibility** - String literal syntax can return ANY type (not just strings)
2. **Unified formatting** - All value-to-string conversions go through pipelines
3. **Extensibility** - Developers can define custom format pipelines
4. **Consistency** - No "magic" string formatting - everything is explicit pipelines

---

##### Syntax Forms

**1. Plain String Literal**
```polyglot
"hello world"
```
**Desugars to:**
```polyglot
U.String"hello world"
```
**Pipeline called:** `|U.String`
**Returns:** `pg\string`

---

**2. Explicit Pipeline String Literal**
```polyglot
DT.Now""
```
**Pipeline called:** `|DT.Now`
**Parameter:** Empty string `""`
**Returns:** `pg\dt` (NOT a string!)

---

**3. Parameterized String Literal**
```polyglot
DT.Minutes"5"
```
**Pipeline called:** `|DT.Minutes`
**Parameter:** `"5"` (passed as `.formatted_argument_string`)
**Returns:** `pg\dt` (duration)

---

**4. Interpolated String Literal**
```polyglot
"Count: {.count:Hex}"
```
**Processing (5 steps):**
1. Extract `{.count:Hex}` placeholder
2. Infer type of `.count` → `pg\int`
3. Call format pipeline: `|U.String.Polyglot.Int.Hex`
4. Get result: `"FF"` (if `.count` is 255)
5. Substitute: `"Count: FF"`
6. Call `|U.String` with final string

---

##### Pipeline Signature Requirements

**MANDATORY for all string literal pipelines:**

```polyglot
[|] PipelineName
[i] .formatted_argument_string: pg\string    # MANDATORY input name
[t] |T.String.Call                           # MANDATORY trigger type
[W] RT.SomeWrapper"..." or |W.Polyglot.Scope # MANDATORY wrapper
[o] .result: AnyType                         # Can be ANY type!
[X]
```

**Key constraints:**
- Input MUST be named `.formatted_argument_string`
- Trigger MUST be `|T.String.Call`
- Single output (can be any type, not limited to strings)
- Standard pipeline structure with `[i]`, `[t]`, `[W]`, `[o]`

---

##### Example: DateTime Pipeline Definition

```polyglot
[|] DT.Now
[i] .formatted_argument_string: pg\string  # Required (even though unused)
[t] |T.String.Call                         # Required trigger
[W] RT.Rust"chrono::Utc::now"             # Rust wrapper for datetime
[o] .timestamp: pg\dt                      # Returns pg\dt, not string!
[X]
```

**Usage:**
```polyglot
[r] .now: pg\dt << DT.Now""   # Empty string required
```

---

##### Format Pipeline Resolution Algorithm

**Given:** `"{.variable:FormatIdentifier}"`

**Step 1: Type Inference**
```
Infer type of .variable → T (e.g., pg\int)
```

**Step 2: Language Context**
```
Determine language context → L (default: Polyglot)
```

**Step 3: Pipeline Name Construction**
```
Construct pipeline name: |U.String.{L}.{T}.{FormatIdentifier}
Example: |U.String.Polyglot.Int.Hex
```

**Step 4: Pipeline Lookup**
```
Check if pipeline exists in registry
- Found: Call pipeline with variable value
- Not found: Compile error with suggestion
```

**Step 5: Result Substitution**
```
Replace {.variable:FormatIdentifier} with pipeline output
```

---

##### Processing Workflow (Interpolated Strings)

**Input:** `"Total: {.price:Currency}, Items: {.count}"`

**Step 1: Extract Placeholders**
```rust
placeholders = [
    {variable: ".price", format: Some("Currency")},
    {variable: ".count", format: None}
]
```

**Step 2: Infer Types**
```rust
.price → pg\float
.count → pg\int
```

**Step 3: Construct Format Pipeline Names**
```rust
.price → |U.String.Polyglot.Float.Currency
.count → |U.String.Polyglot.Int.Default  # No format specified
```

**Step 4: Pack into Serial Array**
```rust
pg\array{pg\serial} = [
    {variable: .price, format_pipeline: |U.String.Polyglot.Float.Currency},
    {variable: .count, format_pipeline: |U.String.Polyglot.Int.Default}
]
```

**Step 5: Call Format Pipelines (Parallel)**
```rust
results = [
    call |U.String.Polyglot.Float.Currency with .price → "$1,234.56"
    call |U.String.Polyglot.Int.Default with .count → "42"
]
```

**Step 6: Substitute Back**
```rust
"Total: {.price:Currency}, Items: {.count}"
→ "Total: $1,234.56, Items: 42"
```

**Step 7: Pass to Target Pipeline**
```rust
Call |U.String with .formatted_argument_string = "Total: $1,234.56, Items: 42"
Returns: pg\string
```

---

##### Auto-Await Triggers

**CRITICAL:** When interpolation accesses a variable (`{.var}`), it triggers **auto-await** on that variable.

```polyglot
[r] |FetchPrice          # Async operation
[>] .price >> .result    # .result is Pending

[r] .msg: pg\string << "Price: {.result:Currency}"
                       # ↑ Auto-await triggered here!
                       # Runtime blocks until .result is Ready/Faulted
```

**Why this matters:**
- Interpolation PULLS from variables
- Pulling triggers auto-await (if variable is Pending)
- No explicit `await` needed - happens automatically
- Consistent with Polyglot's async-centric model

---

##### Type Mismatch Error Handling

**Scenario:** Developer uses incorrect format for type

```polyglot
[r] .name: pg\string << "Alice"
[r] .msg: pg\string << "{.name:Hex}"  # ERROR!
```

**Compiler behavior:**

**Step 1: Infer type**
```
.name → pg\string
```

**Step 2: Construct pipeline name**
```
|U.String.Polyglot.String.Hex
```

**Step 3: Lookup fails**
```
Error: Pipeline not found: |U.String.Polyglot.String.Hex
Note: Format identifier 'Hex' expects type 'pg\int', but variable '.name' is 'pg\string'
Suggestion: Available formats for 'pg\string': [ToUpper, ToLower, Trim, ...]
```

**Why this works elegantly:**
- No special type checking needed
- Format pipelines are namespaced by type
- Missing pipeline = compile error
- Natural error messages

---

##### Bootstrap Formatters (Base Case)

**Problem:** Format pipelines need to format values, but formatting requires pipelines - circular dependency?

**Solution:** Base case formatters implemented directly in Rust (not Polyglot)

**Base formatters (Rust implementations):**
```
|U.String.Polyglot.Int.Default       → Rust: value.to_string()
|U.String.Polyglot.Float.Default     → Rust: format!("{}", value)
|U.String.Polyglot.Bool.Default      → Rust: if value {"true"} else {"false"}
|U.String.Polyglot.DateTime.Default  → Rust: value.to_rfc3339()
|U.String.Polyglot.Path.Default      → Rust: path.to_string_lossy()
```

**Higher-level formatters (can be Polyglot):**
```polyglot
[|] U.String.Polyglot.Int.Hex
[i] .formatted_argument_string: pg\string
[t] |T.String.Call
[W] |W.Polyglot.Scope
[r] .value: pg\int << .formatted_argument_string
[r] .result: pg\string << // Rust code: format!("{:X}", value)
[o] .result: pg\string
[X]
```

**Hierarchy:**
1. Base formatters: Rust implementations (no dependencies)
2. Standard formatters: Polyglot pipelines calling base formatters
3. Custom formatters: User-defined Polyglot pipelines

**No circular dependency:** Base formatters don't use string literals internally (use Rust directly).

---

##### Language-Specific Formatters

**Pattern:** `|U.String.{language}.{type}.{format}`

**Languages supported:**
- `Polyglot` - Language-agnostic (default)
- `Python` - Python-specific formatting (future)
- `Rust` - Rust-specific formatting (future)
- `JavaScript` - JS-specific formatting (future)

**Example use case:**
```polyglot
[r] .timestamp: pg\dt << DT.Now""

# Polyglot format (ISO8601)
[r] .msg1: pg\string << "{.timestamp:ISO8601}"  # "2024-01-15T14:30:00Z"

# Python format (future - uses Python's strftime)
[r] .msg2: pg\string << "{.timestamp:Python.Strftime}"  # Locale-aware
```

---

##### Custom Format Pipeline Example

**Define custom format:**
```polyglot
[|] U.String.Polyglot.Int.PhoneUS
[i] .formatted_argument_string: pg\string
[t] |T.String.Call
[W] |W.Polyglot.Scope

[r] .number: pg\int << .formatted_argument_string

# Extract parts (555-123-4567)
[r] .area: pg\int << (.number / 10000000)
[r] .exchange: pg\int << ((.number / 10000) % 1000)
[r] .line: pg\int << (.number % 10000)

[r] .result: pg\string << "({.area}) {.exchange}-{.line}"

[o] .result: pg\string
[X]
```

**Usage:**
```polyglot
[r] .phone: pg\int << 5551234567
[r] .msg: pg\string << "Call: {.phone:PhoneUS}"
// Result: "Call: (555) 123-4567"
```

---

##### Compiler Implementation Requirements

**Lexer:**
1. Recognize `Pipeline.Name"..."` as inline pipeline call
2. Tokenize string content (extract interpolations)
3. Parse `{.variable:format}` patterns
4. Handle escape sequences

**Parser:**
1. Build AST for string literal nodes
2. Extract placeholder expressions
3. Validate format identifier syntax
4. Generate pipeline call nodes

**Type Checker:**
1. Infer types of interpolated variables
2. Construct format pipeline names
3. Verify pipelines exist in registry
4. Check output types match expected types

**Code Generator:**
1. Generate format pipeline calls
2. Generate string substitution logic
3. Generate final pipeline call with `.formatted_argument_string`
4. Insert auto-await for interpolated variables

---

##### Runtime Implementation Requirements

**String Literal Handler:**
1. Receive string with placeholders
2. Extract placeholder metadata
3. Call format pipelines (can parallelize)
4. Wait for all results (auto-await)
5. Substitute results into string
6. Return formatted string

**Format Pipeline Registry:**
1. Register all available format pipelines
2. Support dynamic lookup by name
3. Cache pipeline handles for performance
4. Provide error messages for missing formatters

---

##### Performance Considerations

**Optimization strategies (future):**
1. **Compile-time formatting** - If all variables are constants, format at compile time
2. **Format pipeline caching** - Cache pipeline handles to avoid repeated lookups
3. **Lazy formatting** - Only format when string is actually used
4. **String pooling** - Reuse identical formatted strings
5. **Parallel formatting** - Call multiple format pipelines concurrently

**Initial implementation:**
- **Correctness first** - Make it work, then optimize
- **Naive execution** - Sequential formatting, no caching
- **Profiling-driven** - Measure before optimizing

**Quote from user:**
> "will start using pipelines then will think of way to optimize it. Make it work first optimize later."

---

##### Why This Architecture?

**1. Consistency**
- No "magic" formatting - everything is explicit pipelines
- Same mechanism for all types
- Predictable behavior

**2. Extensibility**
- Developers can define custom formatters
- No language changes needed for new formats
- Community can share formatter libraries

**3. Type Safety**
- Format pipelines namespaced by type
- Type mismatches = compile errors
- Clear error messages

**4. Flexibility**
- String literal syntax can return ANY type
- Not limited to string outputs
- Unified mechanism for literals

**5. Separation of Concerns**
- Lexer: Tokenize syntax
- Parser: Build AST
- Compiler: Resolve types and pipelines
- Runtime: Execute formatting

---

##### Edge Cases

**1. Nested Interpolation**
```polyglot
"{.name} at {DT.Now\"\"}"
```
**Status:** To be determined (likely disallowed or requires explicit nesting)

**2. Escape Sequences**
```polyglot
"Literal \{.var\} not interpolated"
```
**Status:** Standard escape rules apply (`\{` = literal brace)

**3. Empty Format Identifier**
```polyglot
"{.var:}"  # Format identifier empty
```
**Status:** Compile error (format identifier required if `:` present)

**4. Multiple Formats**
```polyglot
"{.var:Hex,Padded}"  # Two formats?
```
**Status:** Not supported (single format per placeholder)

---

##### Related Documentation

- **Canonical Reference:** [String Literals Internals](string-literals-internals.md)
- **Format Catalog:** [Standard Library - Format Identifiers](../user/standard-library/03-utilities-catalog.md#format-identifier-pipelines)
- **Type System:** [Type System](../user/language/02-type-system.md)
- **Pipeline Definitions:** [Pipeline Syntax](../user/language/01-syntax-complete.md)

---

#### Valid State Transitions

| From State | To State(s) | Trigger | Notes |
|------------|-------------|---------|-------|
| `Declared` | `Pending` | Pipeline assignment | Field populated via `>>` |
| `Declared` | `Ready` | Direct assignment | Explicit value provided |
| `DefaultReady` | `Pending` | Override with async | Override with pipeline result |
| `DefaultReady` | `Ready` | First use or override | Default used or overridden |
| `Pending` | `Ready` | Pipeline success | Value fulfilled |
| `Pending` | `Faulted` | Pipeline failure | Error occurred |
| `Pending` | `Retrying` | Transient failure | Auto-retry triggered |
| `Faulted` | `Retrying` | Retry attempt | Manual or auto retry |
| `Retrying` | `Ready` | Retry success | Operation succeeded |
| `Retrying` | `Faulted` | Retry exhausted | All retries failed |
| `Ready` | `Cached` | Cache enabled | Result cached |
| `Cached` | `Dirty` | Invalidation event | Cache invalidated |
| `Dirty` | `Pending` | Refresh triggered | Re-fetch data |
| `Pending` | `Paused` | External trigger needed | Wait for approval |
| `Paused` | `Pending` | Trigger received | Resume execution |

**Forbidden Transitions (compiler/runtime must prevent):**
- `Ready` → `Declared` (cannot "un-ready" a variable)
- `Ready` → `Pending` (cannot make Ready async again)
- `Faulted` → `Ready` (must go through `Retrying`)
- `DefaultReady` → `Declared` (cannot remove default)
- Any state → `DefaultReady` (DefaultReady is initial state only)

#### Runtime Semantics: Automatic Waiting

**Core Behavior:** Automatic waiting occurs when **pulling from** a variable (via `<<`, `>>`, `<~`, `~>`, comparison operators `=?`, `>?`, etc., or `[o]` output blocks). No explicit `await` keyword.

**Algorithm (When Pulling From Variable V):**
```
When operation pulls from variable V:
  IF V.state == Pending:
    Block until V.state ∈ {Ready, Faulted}
    Use non-busy wait (yield CPU, condition variable)
    Support timeout and cancellation
  ELSE IF V.state == Declared:
    Throw CompileError: "Variable must be Ready when pulled from"
  ELSE IF V.state ∈ {Ready, DefaultReady, Cached}:
    Proceed (return value)
  ELSE IF V.state == Faulted:
    Propagate error to error handler or abort pipeline
```

**Triggers for Auto-Await (Pulling From Variable):**
1. **Assignment operators:** `<< .var`, `>> .var`, `<~ .var`, `~> .var`
2. **Comparison operators:** `[?] .var =? value`, `[?] .var >? threshold`
3. **Output blocks:** `[o] .result >> .var`
4. **Any value access:** Reading variable's value requires it to be Ready

**`[i]` Block Entry Semantics:**

All `[i]` (input) variables MUST be `Ready` (or `DefaultReady`) before pipeline triggers.

```
On pipeline trigger:
  FOR EACH variable V in [i] block:
    IF V.state == DefaultReady:
      Apply default value
      Transition V.state to Ready
    ELSE IF V.state == Declared:
      Throw RuntimeError: "[i] variable not ready"
    ELSE IF V.state == Pending:
      Wait until V.state ∈ {Ready, Faulted}
    ELSE IF V.state == Faulted:
      Invoke error handler

  IF ALL [i] variables Ready:
    Execute pipeline body
  ELSE:
    Abort pipeline
```

**DefaultReady Push Count Semantics:**

`DefaultReady` fields allow **2 pushes** with `<<` or `>>` operators before becoming immutable `Ready`.

```
Enumeration field F with default D (declared with <~ or ~>):

Scenario A: No override (default used)
  F.value = D
  F.state = DefaultReady
  push_count = 0

  On first pull (value accessed):
    F.state = Ready
    F.value = D (immutable)
    push_count = 0 (no pushes, just default applied)

Scenario B: Override with pushes
  1st push: << new_value
    F.value = new_value
    F.push_count = 1
    F.state = DefaultReady (still allows one more push)

  2nd push: << final_value
    F.value = final_value
    F.push_count = 2
    F.state = Ready (now immutable)

  3rd push attempt:
    Throw RuntimeError: "Cannot push to field (already had 2 pushes)"
```

**Concurrency Safety:**
- Push count: atomic increment
- State transition: atomic with value update
- Race conditions prevented via locking

#### IR Representation of Variable States

Variables in IR include state metadata:

**Conceptual Variable Structure (informative):**
```
Variable {
  state: enum Variables.States     // Current state (4 bytes)
  push_count: u8                    // Track push count (1 byte)
  value: SerializedData             // Actual value (variable size)
  errors: Array<ErrorObject>        // Error details if Faulted (variable size)
  metadata: TypeInfo                // Type information (variable size)
}
```

**Note:** `push_count` tracks how many times `<<` or `>>` operators have pushed to this field. Max: 1 for schema-only, 2 for defaults, 0 for constants.

**IR JSON Representation (Runner IR example):**
```json
{
  "runner_ir": {
    "execution_mode": "sequential",
    "steps": [...],
    "variables": {
      ".user_data": {
        "type": "#UserProfile",
        "state": "Pending",
        "push_count": 0,
        "errors": [],
        "assignment_operator": ">+",
        "source": "pipeline_output",
        "max_pushes": 1
      },
      ".config": {
        "type": "#Config",
        "state": "DefaultReady",
        "push_count": 0,
        "errors": [],
        "default_values": {
          ".timeout": 30,
          ".retries": 3
        },
        "assignment_operator": "<~",
        "max_pushes": 2
      }
    }
  }
}
```

**Note:** `max_pushes` indicates how many pushes allowed based on declaration type (1 for schema-only, 2 for defaults, 0 for constants).

**State Metadata in JSONB:**
- Stored in `trigger_ir`, `queue_ir`, `runner_ir` columns as JSONB
- Each IR tracks state information relevant to its phase
- State transitions logged to `execution_logs` table

#### Compiler Requirements

**Type Checking:**
- Enforce type safety for all operators
- Validate default value matches field type
- Check state comparisons use correct enumerations (`#Variables.States.*`)

**State Analysis:**
- Track state flow through pipeline graph
- Detect unreachable code after `Faulted` checks
- Warn on unused `.errors` fields

**Optimization:**
- Eliminate redundant state checks
- Inline constant assignments (`<<`)
- Optimize away `DefaultReady` → `Ready` transitions when no override

**Error Detection:**
- Detect second override attempts on `DefaultReady` fields (compile-time if possible)
- Warn when `Declared` fields reach `[i]` blocks without population
- Flag invalid state transitions

#### Runtime Requirements

**State Management:**
- Atomic state transitions (use mutexes or atomic operations)
- Thread-safe state reads
- Optional: State history for debugging

**Waiting Mechanism:**
- Non-busy wait for `Pending` variables (condition variables)
- Timeout handling (configurable per pipeline)
- Cancellation support (abort waiting pipeline)
- Deadlock detection (optional, post-MVP)

**Memory Management:**
- Free `Pending` variable resources on transition to `Ready`/`Faulted`
- Clean up error objects after handling
- Cache eviction for `Cached` state (LRU policy)

**Observability:**
- State transition logging in debug mode
- Metrics: state distribution, wait times, cache hit rates
- Tracing: variable lifecycle visualization (future)

#### Performance Considerations

**Goal:** Variable state operations must be fast enough to not impact pipeline execution (NFR-P1, NFR-P2).

**State Check Performance:**
- **Target:** State lookup < 1μs (nanosecond-level for hot path)
- **Strategy:**
  - Use fast-path for common case (Ready state)
  - Inline state comparisons where possible
  - Branch prediction hints for error paths (Faulted is rare)
- **Benchmark:** Measure state checks per second in tight loop

**Waiting Mechanism Performance:**
- **Target:** Context switch overhead < 100μs
- **Strategy:**
  - Use futex/condition variables (not busy-waiting)
  - Batch wake-ups for multiple waiters on same variable
  - Lock-free state reads (only writes need mutex)
- **Benchmark:** Measure wait/wake latency with concurrent pipelines

**State Transition Performance:**
- **Target:** Transition time < 10μs (excluding async operation time)
- **Strategy:**
  - Atomic state updates (compare-and-swap)
  - Minimize lock hold time
  - Defer audit logging to background thread
- **Benchmark:** Measure transitions per second under load

**Memory Footprint:**
- **Target:** Variable metadata < 64 bytes overhead per variable
- **Layout:**
  ```
  Variable {
    state: u8           (1 byte)
    override_count: u8  (1 byte)
    padding: u16        (2 bytes)
    value_ptr: *mut     (8 bytes)
    errors_ptr: *mut    (8 bytes)
    metadata_ptr: *mut  (8 bytes)
    -- Total: ~32 bytes + value size
  }
  ```
- **Strategy:**
  - Pack state and override_count into single cache line
  - Use heap allocation for errors (rare case)
  - Share type metadata across instances

**Cache Performance (Post-MVP):**
- **Target:** Cache hit rate > 80% for repeated queries
- **Strategy:**
  - LRU eviction policy
  - Configurable TTL (default: 5 minutes)
  - Memory limits (default: 256MB per pipeline)
- **Benchmark:** Monitor cache hit rate, eviction frequency

**NFR Alignment:**
- **NFR-P1:** Compilation < 1s for 1000-line files (state model doesn't impact compilation speed)
- **NFR-P2:** Trigger to execution < 2s (state checks must be sub-millisecond)
- **NFR-P3:** Type conversion < 10ms for <1MB data (state transitions add negligible overhead)
- **NFR-P4:** Queue throughput 100+ instances/second (state management must not bottleneck)
- **NFR-P5:** Database queries < 100ms (state transition audit logs must be async)

**Optimization Checklist:**
- [ ] Profile state check hot paths
- [ ] Minimize allocations in state transitions
- [ ] Use atomic operations where possible
- [ ] Defer expensive operations (logging) to background threads
- [ ] Monitor lock contention on state mutations
- [ ] Benchmark wait/wake latency under load

#### Database Persistence Strategy

**Decision:** Store variable states in IR JSONB columns (Option A)

**Rationale:**
- Simpler schema (no new tables)
- Variable state is part of IR definition
- Can query using PostgreSQL JSONB operators if needed

**Alternative (Future):** If querying variable states across pipelines becomes critical, migrate to separate `variable_states` table:
```sql
CREATE TABLE variable_states (
    id UUID PRIMARY KEY,
    instance_id UUID NOT NULL REFERENCES pipeline_instances(id),
    variable_name TEXT NOT NULL,
    state TEXT NOT NULL CHECK (state IN ('Declared', 'DefaultReady', 'Pending', 'Ready', 'Faulted', 'Retrying', 'Paused', 'Cached', 'Dirty')),
    value JSONB,
    errors JSONB,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

#### Implementation Patterns for AI Agents

**State Transition Pattern (Rust):**
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VariableState {
    Declared,
    DefaultReady,
    Pending,
    Ready,
    Faulted,
    Retrying,
    Paused,
    Cached,
    Dirty,
}

pub struct Variable {
    pub state: VariableState,
    pub override_count: u8,
    pub value: Option<serde_json::Value>,
    pub errors: Vec<ErrorObject>,
}

impl Variable {
    pub async fn transition_to_ready(&mut self, value: serde_json::Value) -> Result<()> {
        match self.state {
            VariableState::Pending | VariableState::Retrying => {
                self.state = VariableState::Ready;
                self.value = Some(value);
                Ok(())
            }
            VariableState::Declared => {
                Err(RuntimeError::InvalidTransition {
                    from: "Declared",
                    to: "Ready",
                    reason: "Must go through Pending".to_string(),
                })
            }
            VariableState::Ready => {
                Err(RuntimeError::AlreadyReady)
            }
            _ => Err(RuntimeError::InvalidTransition {
                from: format!("{:?}", self.state),
                to: "Ready".to_string(),
                reason: "Invalid transition".to_string(),
            }),
        }
    }
}
```

**Automatic Waiting Pattern (Rust):**
```rust
use tokio::sync::Notify;
use std::sync::Arc;

pub struct VariableWaiter {
    notify: Arc<Notify>,
}

impl VariableWaiter {
    pub async fn wait_for_ready(&self, var: &Variable, timeout_ms: u64) -> Result<()> {
        if matches!(var.state, VariableState::Ready | VariableState::Cached) {
            return Ok(());
        }

        if matches!(var.state, VariableState::Declared) {
            return Err(CompileError::VariableNotReady {
                var_name: var.name.clone(),
            });
        }

        // Wait with timeout
        tokio::select! {
            _ = self.notify.notified() => {
                if matches!(var.state, VariableState::Ready) {
                    Ok(())
                } else if matches!(var.state, VariableState::Faulted) {
                    Err(RuntimeError::VariableFaulted {
                        var_name: var.name.clone(),
                        errors: var.errors.clone(),
                    })
                } else {
                    Err(RuntimeError::UnexpectedState {
                        expected: "Ready or Faulted",
                        actual: format!("{:?}", var.state),
                    })
                }
            }
            _ = tokio::time::sleep(Duration::from_millis(timeout_ms)) => {
                Err(RuntimeError::WaitTimeout {
                    var_name: var.name.clone(),
                    timeout_ms,
                })
            }
        }
    }
}
```

#### Edge Cases

**1. Nested Field States:**
- Nested fields (`.user.address.city`) have independent `.state` introspection
- Accessing `.user.address.state` is valid
- All serialized fields have state metadata

**2. Faulted Variable Passed to Pipeline:**
- Pipeline waits for `Ready`/`Faulted`, then propagates error
- If unhandled, downstream pipeline aborts
- Error blocks `[!]` can catch specific error types

**3. Concurrent State Access:**
- State reads: atomic, lock-free
- State writes: mutex-protected
- State transitions: atomic compare-and-swap or mutex
- Waiting: condition variable or similar

**4. Second Override Attempt:**
- Each instance tracks its own override count
- Second override on SAME INSTANCE is runtime error
- Different instances are independent

#### Future Innovations

**Status:** Exploratory concepts for post-MVP consideration

**1. Partial/Streaming State**

**Concept:** Variable that receives data incrementally as it becomes available.

**Motivation:**
- Large dataset processing (logs, file downloads)
- Streaming data pipelines
- Progressive rendering/display

**Design Challenges:**
- How does streaming fit with "all variables are serialized strings"?
- Is partial data queryable mid-stream?
- What guarantees exist about data completeness?
- How to handle partial failures?

**Possible Approaches:**

**Option A: Streaming as Iterator Pattern**
- Variable doesn't hold full data, holds iterator/cursor
- Each iteration yields Ready chunk
- Terminal state when iterator exhausted
```polyglot
[r] |StreamLogs
[>] .logs: pg\stream{pg\string} >> .log_stream  # Hypothetical stream type

# Consumer iterates:
[r] |ProcessLogChunk
[<] .chunk: pg\string << .log_stream.next()
```

**Option B: Partial State with Progress Metadata**
- New state: `Partial` (between Pending and Ready)
- Variable holds accumulated data + progress indicator
- Transition: Declared → Pending → Partial (N%) → Ready (100%)
```polyglot
[?] .download.state =? #Variables.States.Partial
[~][r] |U.Log.Info
[~][<] .msg << "Download {.download.progress}% complete"
```

**Option C: No Special State (Defer to Application)**
- Use Ready state with array that grows over time
- Application-level chunking via pipeline chaining
- No language-level streaming support

**Recommendation:** Further research needed. Streaming conflicts with serialization model. Consider Option C (application-level) for MVP, Option A (iterator) for future if demand exists.

**2. Reactive State System**
- Automatic re-execution when dependencies change
- Dirty state triggers downstream pipeline refresh
- DAG-based dependency tracking

**3. Distributed State Coordination**
- Variables that span multiple machines
- Pipeline states synchronized across cluster
- Distributed caching and retry logic

#### References

- **Technical Specification:** `docs/technical/variable-states-specification.md` (v1.0.0)
- **User Guide:** `docs/user/language/variables-user-guide.md`
- **Brainstorming Session:** `docs/brainstorming-session-results-2025-11-23.md`
- **Reserved Enumeration:** `#Variables.States.*` (defined in next section)

---

### Reserved Enumerations

**Status:** Core language feature (v0.0.2)

#### Overview

Reserved Enumerations are **pre-compiled, immutable constants** defined by the Polyglot language itself. Unlike user-defined enumerations, Reserved Enumerations:
- Cannot be altered or redefined
- Are available globally without imports
- Are part of the serialization tree
- Exist at compile time (not runtime-created)

They serve as the foundation for Polyglot's introspection capabilities and standard library.

#### Purpose

Reserved Enumerations provide:
1. **Language Introspection:** Query variable/pipeline states at runtime
2. **Type-Safe Constants:** Enum-based comparisons prevent typo bugs
3. **Standard Library Foundation:** Pre-defined types for common operations
4. **Cross-Language Consistency:** Same enumerations across all runtime wrappers

#### Core Reserved Enumerations

**1. `#Variables.States.*` - Variable State Model**

Defines all 9 variable states as immutable constants:

```polyglot
[#] Variables.States
[<] .Declared: pg\string << "Declared"
[<] .DefaultReady: pg\string << "DefaultReady"
[<] .Pending: pg\string << "Pending"
[<] .Ready: pg\string << "Ready"
[<] .Faulted: pg\string << "Faulted"
[<] .Retrying: pg\string << "Retrying"
[<] .Paused: pg\string << "Paused"
[<] .Cached: pg\string << "Cached"
[<] .Dirty: pg\string << "Dirty"
[X]
```

**Usage:**
```polyglot
[?] .var.state =? #Variables.States.Ready
[~][r] |ProcessData

[?] .var.state =? #Variables.States.Faulted
[~][r] |HandleError
```

**Implementation:** Compiler ensures these are available without declaration.

---

**2. `#Pipelines.States.*` - Pipeline Execution States**

**Status:** Epic 2 Future Work - Will be finalized during IR Generation implementation

Defines 10 pipeline execution states (draft):

```polyglot
[#] Pipelines.States
[<] .Registered: pg\string << "Registered"       # Pipeline defined, not triggered
[<] .Awaiting: pg\string << "Awaiting"           # Waiting for trigger condition
[<] .Triggered: pg\string << "Triggered"         # Trigger fired, ready for dispatch
[<] .DispatchQueue: pg\string << "DispatchQueue" # Queued for execution
[<] .Executing: pg\string << "Executing"         # Currently running
[<] .Paused: pg\string << "Paused"               # Execution paused (queue control)
[<] .Cached: pg\string << "Cached"               # Result cached for reuse
[<] .Retry: pg\string << "Retry"                 # Retry attempt in progress
[<] .Failed: pg\string << "Failed"               # Execution failed
[<] .Completed: pg\string << "Completed"         # Successfully finished
[X]
```

**Usage (future):**
```polyglot
[?] |SomePipeline.state =? #Pipelines.States.Executing
[~][r] |U.Log.Info
[~][<] .msg: pg\string << "Pipeline is running"
```

**Note:** Pipeline state introspection syntax is not yet finalized. This enumeration will be fully specified and validated during Epic 2 (IR Generation). The 10-state model shown here is a draft from brainstorming session 2025-11-23 and may be refined based on implementation requirements.

---

**3. `#Boolean` - True/False Constants**

**Status:** Core language feature

Standard boolean enumeration with aliases:

```polyglot
[#] Boolean
[<] .True: pg\string << "True"    # Alias: #True
[<] .False: pg\string << "False"  # Alias: #False
[X]
```

**Usage:**
```polyglot
[#] Config
[<] .debug_mode: #Boolean <~ #False
[<] .verbose: #Boolean <~ #True
[X]

[?] .config.debug_mode =? #True
[~][r] |EnableDebugLogging
```

**Aliases:** `#True` and `#False` are global aliases for `#Boolean.True` and `#Boolean.False`.

---

**4. Template Pattern Reserved Enumerations**

Some Reserved Enumerations follow a **template pattern** for systematic organization.

**Example:** `DT.Business.Week.*` (Datetime business week constants)

```polyglot
[#] DT.Business.Week
[<] .Monday: pg\int << 1
[<] .Tuesday: pg\int << 2
[<] .Wednesday: pg\int << 3
[<] .Thursday: pg\int << 4
[<] .Friday: pg\int << 5
[X]
```

**Pattern Rationale:**
- Namespaced organization (`DT` for datetime)
- Hierarchical structure (`Business.Week` for business week days)
- Consistent naming across related constants

**Other Template Patterns (future):**
- `DT.Month.*` - Month constants (1-12)
- `DT.Quarter.*` - Quarter constants (Q1-Q4)
- `HTTP.Status.*` - HTTP status codes (200, 404, 500, etc.)
- `Encoding.*` - Character encodings (UTF8, ASCII, etc.)

---

#### Reserved vs User-Defined Enumerations

| Aspect | Reserved Enumerations | User-Defined Enumerations |
|--------|----------------------|---------------------------|
| **Mutability** | Immutable (compiler-enforced) | Immutable once instantiated |
| **Availability** | Global, no imports needed | Must be defined in code |
| **Definition Time** | Pre-compile time (language-provided) | Compile time (user-provided) |
| **Namespace** | Special syntax (`#Variables.States.*`, `#Boolean`, etc.) | User-defined (`#MyEnum.*`) |
| **Purpose** | Language introspection, standard library | Application-specific data structures |
| **Override** | Cannot be redefined | Cannot conflict with reserved names |

**Critical Rule:** User-defined enumerations **cannot use reserved names**. Compiler will reject:
```polyglot
[#] Variables.States  # ERROR: Cannot redefine reserved enumeration
[<] .Custom: pg\string << "Custom"
[X]
```

---

#### Compiler Implementation

**Reserved Enumeration Handling:**

1. **Pre-Compilation Phase:**
   - Load all reserved enumerations into symbol table
   - Mark as immutable and globally available
   - Register reserved names (prevent user redefinition)

2. **Type Checking Phase:**
   - Validate comparisons use correct enum types
   - Ensure reserved fields (`.state`, `.errors`) reference correct enumerations
   - Reject attempts to redefine reserved names

3. **Code Generation Phase:**
   - Reserved enum access is constant folding (no runtime lookup)
   - Direct value substitution in IR

**Example Compiler Pattern (Rust):**
```rust
pub struct ReservedEnumerations {
    enums: HashMap<String, Enumeration>,
}

impl ReservedEnumerations {
    pub fn new() -> Self {
        let mut enums = HashMap::new();

        // Variables.States
        enums.insert(
            "Variables.States".to_string(),
            Enumeration {
                name: "Variables.States".to_string(),
                fields: vec![
                    ("Declared".to_string(), json!("Declared")),
                    ("DefaultReady".to_string(), json!("DefaultReady")),
                    ("Pending".to_string(), json!("Pending")),
                    ("Ready".to_string(), json!("Ready")),
                    ("Faulted".to_string(), json!("Faulted")),
                    ("Retrying".to_string(), json!("Retrying")),
                    ("Paused".to_string(), json!("Paused")),
                    ("Cached".to_string(), json!("Cached")),
                    ("Dirty".to_string(), json!("Dirty")),
                ],
                reserved: true,
            },
        );

        // Boolean
        enums.insert(
            "Boolean".to_string(),
            Enumeration {
                name: "Boolean".to_string(),
                fields: vec![
                    ("True".to_string(), json!("True")),
                    ("False".to_string(), json!("False")),
                ],
                reserved: true,
            },
        );

        // Add aliases
        // #True -> #Boolean.True
        // #False -> #Boolean.False

        Self { enums }
    }

    pub fn is_reserved(&self, name: &str) -> bool {
        self.enums.contains_key(name)
    }

    pub fn get(&self, name: &str) -> Option<&Enumeration> {
        self.enums.get(name)
    }
}
```

---

#### Runtime Behavior

**Reserved Enumerations at Runtime:**

1. **No Runtime Allocation:**
   - Values are compile-time constants
   - Direct substitution in generated code
   - Zero runtime overhead

2. **Serialization:**
   - Reserved enum values are serialized as strings
   - Cross-language consistency maintained
   - Example: `#Variables.States.Ready` → `"Ready"` (JSON string)

3. **Comparison:**
   - String equality checks in serialized form
   - Runtime wrappers match string values

**Example Runtime Check (Python wrapper):**
```python
# Polyglot code:
# [?] .var.state =? #Variables.States.Ready

# Runtime wrapper (Python):
if var_state == "Ready":
    # Process ready variable
    pass
```

---

#### IR Representation

Reserved enumerations appear in IR as string constants:

**Before Compilation (Polyglot):**
```polyglot
[?] .user_data.state =? #Variables.States.Ready
[~][r] |ProcessUser
```

**After Compilation (IR JSON):**
```json
{
  "condition": {
    "type": "comparison",
    "left": {
      "type": "field_access",
      "object": ".user_data",
      "field": "state"
    },
    "operator": "=?",
    "right": {
      "type": "constant",
      "value": "Ready",
      "enum_type": "Variables.States"
    }
  },
  "then_block": {
    "type": "pipeline_call",
    "pipeline": "|ProcessUser"
  }
}
```

**Note:** `enum_type` metadata preserves type information for validation.

---

#### Future Reserved Enumerations

**Planned (Post-MVP):**

1. **`#HTTP.Status.*`** - HTTP status codes
   ```polyglot
   [#] HTTP.Status
   [<] .OK: pg\int << 200
   [<] .NotFound: pg\int << 404
   [<] .InternalServerError: pg\int << 500
   [X]
   ```

2. **`#Encoding.*`** - Character encodings
   ```polyglot
   [#] Encoding
   [<] .UTF8: pg\string << "UTF-8"
   [<] .ASCII: pg\string << "ASCII"
   [X]
   ```

3. **`#DT.Format.*`** - Datetime format patterns
   ```polyglot
   [#] DT.Format
   [<] .ISO8601: pg\string << "YYYY-MM-DDTHH:mm:ss.sssZ"
   [<] .USDate: pg\string << "MM/DD/YYYY"
   [X]
   ```

4. **`#Queue.Priority.*`** - Queue priority levels
   ```polyglot
   [#] Queue.Priority
   [<] .Critical: pg\int << 1
   [<] .High: pg\int << 2
   [<] .Normal: pg\int << 3
   [<] .Low: pg\int << 4
   [X]
   ```

---

#### Documentation Requirements

**For Each Reserved Enumeration:**

1. **Language Specification:** Formal definition with all fields
2. **Standard Library Docs:** Usage examples and patterns
3. **Compiler Implementation:** How it's loaded and validated
4. **Runtime Behavior:** Serialization format and cross-language handling

**Current Documentation Status:**

| Enumeration | Spec Defined? | Documented? | Implemented? |
|-------------|---------------|-------------|--------------|
| `#Variables.States.*` | ✅ Yes (v1.0.0) | ✅ Yes | ⚠️ Pending (Epic 2) |
| `#Pipelines.States.*` | ⚠️ Draft (brainstorming) | ⚠️ Partial | ❌ No |
| `#Boolean` | ✅ Yes (v0.0.2) | ✅ Yes | ⚠️ Pending (Epic 1) |
| `DT.Business.Week.*` | ✅ Yes (v0.0.2) | ✅ Yes | ⚠️ Pending (Epic 4) |

---

#### References

- **Brainstorming Session:** `docs/brainstorming-session-results-2025-11-23.md` (Lines 300-310, 432-448)
- **Variable States Spec:** `docs/technical/variable-states-specification.md` (Lines 377-402)
- **Syntax Spec:** `docs/v0.0.2/language/03-enumerations.md` (Reserved enumeration semantics)

---

## Security Architecture

### Authentication & Authorization (Future)

**MVP:** No authentication (local development only)

**Post-MVP:**
- CLI: API key or token-based auth
- Services: mTLS for inter-service communication (if distributed)
- Database: PostgreSQL role-based access control

### Data Protection

**At Rest:**
- PostgreSQL: Enable encryption at rest (OS-level or managed database)
- Redis: Persistence encryption if enabled

**In Transit:**
- PostgreSQL: TLS connections (`sslmode=require`)
- Redis: TLS enabled (`tls-port` configuration)

### Input Validation

**CLI:**
- clap validates argument types
- Path traversal prevention for `.pg` file paths
- Sanitize user input before database queries (SQLx parameterized queries prevent SQL injection)

**Lexer/Parser:**
- Reject malformed `.pg` files
- Limit file size (prevent DoS via massive files)
- Timeout for compilation (prevent infinite loops)

### Process Isolation

**Services:**
- Each service runs as separate process
- Failures isolated (one service crash doesn't affect others)
- Database connection pooling prevents connection exhaustion

**Runtime Wrappers:**
- Future: Sandbox Python/Node/Rust execution (namespaces, cgroups, or containers)
- MVP: Subprocess isolation only

### Secrets Management

**Configuration:**
- Never hardcode credentials in code
- Database URL via environment variable `DATABASE_URL`
- Redis URL via environment variable `REDIS_URL`
- Future: Integration with HashiCorp Vault or AWS Secrets Manager

---

## Performance Considerations

### Compilation Speed (NFR-P1)

**Target:** <1s compilation for 1000-line `.pg` files

**Strategies:**
- Efficient lexer (single-pass)
- Parser uses zero-copy where possible
- IR generation avoids unnecessary clones
- Benchmark compilation in CI

### Pipeline Execution Latency (NFR-P2)

**Target:** <2s from trigger to execution start

**Strategies:**
- PostgreSQL connection pooling (reuse connections)
- Redis pipelining for queue operations
- Minimize database roundtrips
- Index optimization on `pipelines(activated)` and `pipeline_instances(status)`

### Type Conversion Overhead (NFR-P3)

**Target:** <10ms for typical data sizes (<1MB)

**Strategies:**
- JSON serialization (serde_json is highly optimized)
- Streaming deserialization for large payloads
- Benchmark runtime wrapper performance
- Future: Upgrade to bincode for production if needed

### Queue Throughput (NFR-P4)

**Target:** 100+ instances/second

**Strategies:**
- Redis `RPUSH`/`BLPOP` are O(1) operations
- Batch queue operations where possible
- Monitor queue depth and lag

### Database Query Performance (NFR-P5)

**Target:** <100ms for registry queries, <500ms for logs

**Strategies:**
- Indexes on frequently queried columns
- Use `EXPLAIN ANALYZE` to optimize slow queries
- Connection pooling (default: 10 connections)
- Prepared statements via SQLx (cache query plans)

---

## Deployment Architecture

### MVP Deployment (Local/Single-Node)

**All services on one machine:**
```
┌─────────────────────────────────────┐
│         Local Machine               │
│                                     │
│  ┌──────────────┐  ┌──────────────┐│
│  │ PostgreSQL   │  │    Redis     ││
│  └──────────────┘  └──────────────┘│
│                                     │
│  ┌──────────────────────────────┐  │
│  │  Polyglot Services           │  │
│  │  - Trigger Monitor           │  │
│  │  - Queue Manager             │  │
│  │  - Runner                    │  │
│  └──────────────────────────────┘  │
│                                     │
│  ┌──────────────────────────────┐  │
│  │  polyglot CLI                │  │
│  └──────────────────────────────┘  │
└─────────────────────────────────────┘
```

**Setup:**
```bash
# Install dependencies
brew install postgresql redis  # macOS
sudo apt install postgresql redis  # Linux

# Start services
brew services start postgresql redis  # macOS
systemctl start postgresql redis  # Linux

# Build Polyglot
cargo build --release

# Run migrations
sqlx migrate run

# Start services (3 terminals)
./target/release/polyglot-trigger-monitor
./target/release/polyglot-queue-manager
./target/release/polyglot-runner
```

### Production Deployment (Future: Docker + Orchestration)

**Docker Compose:**
```yaml
services:
  postgres:
    image: postgres:16
    environment:
      POSTGRES_DB: polyglot
      POSTGRES_USER: polyglot
      POSTGRES_PASSWORD: ${DB_PASSWORD}
    volumes:
      - postgres_data:/var/lib/postgresql/data

  redis:
    image: redis:7-alpine
    volumes:
      - redis_data:/data

  trigger-monitor:
    build: .
    command: /usr/local/bin/polyglot-trigger-monitor
    depends_on:
      - postgres
      - redis
    environment:
      DATABASE_URL: postgresql://polyglot:${DB_PASSWORD}@postgres/polyglot
      REDIS_URL: redis://redis:6379

  queue-manager:
    build: .
    command: /usr/local/bin/polyglot-queue-manager
    depends_on:
      - postgres
      - redis

  runner:
    build: .
    command: /usr/local/bin/polyglot-runner
    depends_on:
      - postgres
      - redis
```

### Scalability (Post-MVP)

**Horizontal Scaling:**
- Multiple Runner instances (stateless, can run in parallel)
- Queue Manager can be sharded by queue
- Trigger Monitor: Single instance (LISTEN/NOTIFY limitation), future: leader election

**Managed Services:**
- AWS RDS PostgreSQL
- AWS ElastiCache Redis
- Deploy services to ECS/EKS or Railway

---

## Development Environment

### Prerequisites

**Required:**
- Rust 1.84+ (2024 edition)
- PostgreSQL 14+
- InfluxDB 2.x
- Redis 7+
- SQLx CLI: `cargo install sqlx-cli`
- Python 3.11+ with uv: `pip install uv`

**Optional:**
- Docker & Docker Compose (for containerized development)
- rust-analyzer (LSP for IDE)

### Setup

```bash
# 1. Clone repository
git clone https://github.com/yourusername/polyglot.git
cd polyglot

# 2. Install Rust dependencies
cargo build

# 3. Set up PostgreSQL
createdb polyglot  # or use Docker
export DATABASE_URL="postgresql://localhost/polyglot"

# 4. Set up InfluxDB
influxd  # or Docker
# Create org, bucket, and token via UI (http://localhost:8086)
export INFLUX_URL="http://localhost:8086"
export INFLUX_TOKEN="your-token"

# 5. Set up Redis
redis-server  # or Docker

# 6. Run migrations
sqlx migrate run

# 7. Install Python with uv
pip install uv

# 8. Copy example config
cp polyglot.toml.example polyglot.toml
# Edit polyglot.toml with your database/InfluxDB/Redis URLs

# 9. Run tests
cargo test --workspace

# 10. Run services (in separate terminals)
cargo run --bin polyglot-trigger-monitor
cargo run --bin polyglot-queue-manager
cargo run --bin polyglot-runner

# 11. Use CLI
cargo run --bin polyglot-cli -- compile examples/hello_world.pg  # Validates + converts + registers
cargo run --bin polyglot-cli -- activate hello_world
cargo run --bin polyglot-cli -- trigger hello_world  # Test/debug: bypass trigger logic
cargo run --bin polyglot-cli -- status <instance_id>
cargo run --bin polyglot-cli -- logs <instance_id>
```

### Environment Variables

```bash
# Required
export DATABASE_URL="postgresql://localhost/polyglot"
export INFLUX_URL="http://localhost:8086"
export INFLUX_TOKEN="your-influxdb-token"
export INFLUX_ORG="polyglot"
export INFLUX_BUCKET="metrics"
export REDIS_URL="redis://localhost:6379"

# Optional
export POLYGLOT_CONFIG="./polyglot.toml"
export RUST_LOG="info,polyglot=debug"  # Logging level
export RUST_BACKTRACE=1  # Stack traces on panic
```

### IDE Configuration

**VSCode (.vscode/settings.json):**
```json
{
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.check.command": "clippy",
  "rust-analyzer.rustfmt.extraArgs": ["--edition", "2024"]
}
```

**IntelliJ IDEA / RustRover:**
- Enable Rust plugin
- Set Rust toolchain to 1.84+
- Enable Clippy lints

---

## Architecture Decision Records (ADRs)

### ADR-001: Manual Cargo Workspace Over Starter Template

**Status:** Accepted

**Context:** Polyglot's architecture (3 backend services + CLI + shared libraries) is specialized for a language implementation.

**Decision:** Use manual Cargo workspace setup instead of generic Rust starter templates.

**Rationale:**
- Starter templates don't support multi-service architecture
- Custom workspace gives precise control over crate boundaries
- Polyglot's dependencies (PostgreSQL, Redis, Tokio, serde) are specific

**Consequences:**
- More initial setup work
- Full control over project structure
- Clear separation of concerns

---

### ADR-002: SQLx Over Diesel

**Status:** Accepted

**Context:** Need async-first database client for PostgreSQL.

**Decision:** Use SQLx 0.8.6 with `tokio-comp` feature.

**Rationale:**
- Async-first design (Diesel added async later)
- Compile-time query verification against actual schema
- Lightweight, no heavy ORM abstraction
- Direct SQL queries clearer for complex IR storage

**Consequences:**
- More SQL knowledge required (less abstraction than ORM)
- Better performance for async workloads
- sqlx-cli provides migration tooling

---

### ADR-003: PostgreSQL JSONB for IR Storage

**Status:** Accepted

**Context:** IR is serialized data, but metadata is relational.

**Decision:** Use PostgreSQL with JSONB column for IR, relational columns for metadata.

**Rationale:**
- Best of both worlds: document storage + relational queries
- JSONB supports indexing and querying
- One database instead of PostgreSQL + MongoDB
- ACID guarantees for state management

**Consequences:**
- IR stored as JSON (human-readable for debugging)
- Can query inside IR if needed: `WHERE ir->>'trigger_type' = 'manual'`
- PostgreSQL remains single source of truth

---

### ADR-004: thiserror + anyhow Error Handling

**Status:** Accepted

**Context:** Need error handling strategy for libraries + binaries in async context.

**Decision:**
- thiserror 2.0.17 for library error types
- anyhow 1.0.99 for binary error propagation
- All errors must be `Send + Sync` for async

**Rationale:**
- Rust community standard pattern
- Structured errors in libraries enable caller matching
- Anyhow simplifies context chaining in binaries
- Both support async error propagation

**Consequences:**
- Library errors define custom types with variants
- Binaries use `anyhow::Result<T>` and `.context()`
- Errors cross async boundaries safely

---

### ADR-005: Dynamic Trigger Loading with PostgreSQL LISTEN/NOTIFY

**Status:** Accepted

**Context:** Triggers must reload when pipeline IR changes without service restarts.

**Decision:** Use PostgreSQL LISTEN/NOTIFY for trigger updates, dynamic handler registry in Trigger Monitor.

**Rationale:**
- PostgreSQL NOTIFY is real-time (no polling lag)
- Dynamic loading enables runtime configuration
- Hybrid trigger types (async listening + sync loop) support diverse use cases

**Consequences:**
- Novel architectural pattern (documented in this architecture)
- Trigger Monitor maintains in-memory registry
- Handlers are spawned/stopped on IR changes
- Scalability limited to single Trigger Monitor instance (future: leader election)

---

### ADR-006: Database-Driven Service Communication

**Status:** Accepted

**Context:** 3 backend services need to communicate and share state.

**Decision:** Services communicate via PostgreSQL (state) and Redis (queues), no direct HTTP/RPC.

**Rationale:**
- Simplifies deployment (no service discovery needed)
- Database provides strong consistency
- Redis provides fast queue operations
- Failure isolation (services can restart independently)

**Consequences:**
- All services depend on PostgreSQL/Redis availability
- No need for HTTP servers in services (except future webhooks)
- State transitions visible in database
- Future: May need distributed tracing for debugging

---

### ADR-007: InfluxDB for Time-Series Data

**Status:** Accepted

**Context:** Time-based triggers, trigger execution results, and resource metrics are time-series data.

**Decision:** Add InfluxDB 2.x as MVP requirement for time-series storage.

**Rationale:**
- Time-series database optimized for temporal data
- Efficient storage for resource metrics (CPU, RAM, GPU, Network)
- Better query performance for time-based trigger schedules
- Separate concern from relational data (PostgreSQL)

**Consequences:**
- Additional dependency for MVP (InfluxDB required)
- Trigger Monitor reads from both PostgreSQL and InfluxDB
- Resource Monitor writes metrics to InfluxDB at fixed intervals
- More complex deployment (3 databases: PostgreSQL, InfluxDB, Redis)

---

### ADR-008: 3-IR Structure (Trigger, Queue, Runner)

**Status:** Accepted

**Context:** Pipeline IR has distinct concerns: triggering, queuing, and execution.

**Decision:** Split compiled IR into 3 separate IRs stored as separate JSONB columns.

**Rationale:**
- Separation of concerns (trigger logic ≠ queue logic ≠ execution logic)
- Each service reads only its relevant IR (Trigger Monitor → trigger_ir, etc.)
- Easier to query and index (GIN indexes on each IR column)
- Polyglot syntax `[t]` and `[Q]` map cleanly to Queue IR

**Consequences:**
- Database schema has 3 JSONB columns instead of 1
- Compilation produces 3 IRs from single .pg file
- Each IR has its own schema and validation rules
- Clear boundaries for AI agents implementing services

---

### ADR-009: Resource Monitor as Trigger Monitor Subservice

**Status:** Accepted

**Context:** Resource monitoring (CPU, RAM, GPU, Network) is needed for resource-based triggers.

**Decision:** Implement Resource Monitor as a subservice of Trigger Monitor, not a 4th separate service.

**Rationale:**
- Resource Monitor is triggered by Trigger Monitor
- Tight coupling: Trigger Monitor activates resource monitoring based on pipeline needs
- Reduces complexity (3 services instead of 4)
- Selective monitoring: Only monitor resources specified in pipeline IRs

**Consequences:**
- Trigger Monitor crate has resource monitoring submodule
- Resource metrics written to InfluxDB
- Continuous polling at fixed interval (configurable)
- Single service failure affects both trigger checking and resource monitoring

---

### ADR-010: Compile = Validate + Convert + Register

**Status:** Accepted

**Context:** User workflow for getting .pg files into the system.

**Decision:** `polyglot compile <file>.pg` performs validation, IR conversion, AND database registration in one command.

**Rationale:**
- Simplifies user workflow (one command instead of two)
- Compilation without storage is rarely useful (can compile but not use)
- After compile, pipelines are available for activation
- Matches user mental model: "compile" means "make it ready"

**Consequences:**
- No separate `polyglot register` command needed
- Users must have database connection to compile
- "Compile without register" means validation only (future flag: `--dry-run`)
- After compile, users run `polyglot activate <name>` to enable pipelines

---

### ADR-011: Pause Types (Process Pause + Checkpoints)

**Status:** Accepted

**Context:** Pipelines may need to pause execution for various reasons.

**Decision:** Support 2 pause types:
1. **Process Pause** (OS-level): Freeze memory or cache RAM in Redis
2. **Checkpoints** (Programmatic): User-defined pause points in Polyglot code

**Rationale:**
- Process pause handles resource constraints (reduce CPU load, free RAM)
- Checkpoints enable user-defined pause/resume logic based on conditions
- Pause queue separates paused instances from active dispatch queue
- Flexibility for different pause scenarios (system resources vs. business logic)

**Consequences:**
- Pause queue added to Redis (MVP)
- Runner must support pause/resume operations
- Process pause: OS-level signals (SIGSTOP/SIGCONT) or Redis caching
- Checkpoint pause: Save execution state, resume from checkpoint

---

### ADR-012: PostgreSQL Fallback for Redis Queues

**Status:** Accepted

**Context:** Redis failure would halt all pipeline queuing and execution.

**Decision:** Use PostgreSQL as fallback when Redis is unavailable. Queue Manager polls `pipeline_instances WHERE status='queued'`.

**Rationale:**
- High availability: System continues functioning if Redis fails
- PostgreSQL already stores instance state
- No data loss (instances already in database)
- Graceful degradation (slower but functional)

**Consequences:**
- Queue Manager checks Redis health
- On Redis failure, switch to PostgreSQL polling
- Performance degradation during fallback (polling vs. push/pop)
- When Redis recovers, switch back to Redis-based queuing

---

### ADR-013: Logos Lexer Generator

**Status:** Accepted

**Context:** Story 1.2 (Lexer Token Definitions) requires tokenizing 45+ distinct token types from Polyglot v0.0.2 syntax specification, including operators, block markers, comparison operators, negation operators, range operators, and pattern operators. The lexer must achieve <100ms performance for 1000-line files (NFR-P1) while maintaining high code quality and maintainability.

**Decision:** Use the `logos` crate (version 0.14) for declarative token definition and automatic lexer generation, rather than hand-writing a manual lexer.

**Rationale:**
- **"Don't Reinvent the Wheel"** - Core Polyglot philosophy applies to Polyglot's own implementation
- **Battle-tested** - Logos is used by tree-sitter, rustpython, and other production parsers (2.5k+ GitHub stars, actively maintained)
- **Performance** - Proc macro generates optimized DFA at compile time, proven to meet <100ms requirement
- **Maintainability** - 45+ token types as regex annotations vs. 500-800 lines of manual matching logic
- **Conciseness** - Token definitions are self-documenting and easier to update
- **Fast iteration** - Adding new token types requires only enum variant + regex annotation

**Alternatives Considered:**
- **Manual lexer** - Rejected due to high bug surface area with 45+ token types, slower development time, and no performance advantage over logos
- **Nom parser combinators** - Rejected as better suited for parsing than lexing, unnecessary complexity
- **LALRPOP** - Rejected as it's a parser generator, not a lexer generator

**Consequences:**
- ✅ Adds `logos = "0.14"` dependency to workspace (via workspace inheritance per ADR-001)
- ✅ Compile times increase by ~2-5 seconds for initial build due to proc macro generation
- ✅ Lexer implementation (Story 1.3) reduces from ~500-800 LOC to ~200 LOC
- ✅ Token definitions in `token.rs` use `#[derive(Logos)]` with regex annotations
- ⚠️ Logos errors wrapped in custom `LexerError` type (using `thiserror` per ADR-004) for consistent error handling
- ⚠️ Source location tracking (line, column) implemented manually on top of logos byte positions
- ⚠️ If logos proves insufficient for special cases (unlikely), fallback to manual implementation for specific token types only

**Implementation Notes:**
- Story 1.2: Define `TokenType` enum with ~60 variants + 2 compound token types
- Story 1.3: Add `#[derive(Logos)]` and regex patterns for each token type
- Custom `LexerError` wrapper maintains `Send + Sync` requirement for async compatibility
- Unit tests still verify token recognition correctness (>80% coverage per AC#4)

**Compound Token Types (Design Decision 2025-11-17):**

1. **StringLiteral** - Pipeline syntax sugar for string processing
   ```rust
   pub struct StringLiteral {
       /// Optional pipeline for string processing
       /// If None, defaults to |String.Formatted
       /// Format: String.{runtime_lang}.{type}.Format.{format_id}
       /// Example: "String.Python.int.Format.Hex"
       pipeline: Option<String>,

       /// UTF-8 string content (Unicode supported)
       content: String,

       location: Location,
   }
   ```
   - **Rationale:** Literals in Polyglot are syntax sugar for pipeline invocations
   - Bare strings `"text"` default to `|String.Formatted` with `{.var:fmt}` substitution support
   - Pipeline-prefixed strings invoke specific formatting pipelines: `String.Python.int.Format.Hex"42"`
   - Compiler validates (Epic 2): pipeline exists and yields `pg\string` type

2. **RangeCheck** - Bracket notation for range comparisons
   ```rust
   pub enum RangeValue {
       Variable(String),      // .var_name placeholder
       Value(LiteralValue),   // Evaluatable literal
   }

   pub struct RangeCheck {
       left: RangeValue,           // Variable being checked
       start: RangeValue,          // Range start
       end: RangeValue,            // Range end
       start_inclusive: bool,      // [ = true, ( = false
       end_inclusive: bool,        // ] = true, ) = false
       location: Location,
   }
   ```
   - **Rationale:** `.var ?(start, end]` is a single logical construct, not separate tokens
   - Supports inclusive/exclusive boundaries: `[` vs `(` for start, `]` vs `)` for end
   - Can contain variables or literal values for dynamic range checking

**Token Count Update:**
- 27 block markers (single-char in brackets)
- ~24 operators (including comparison, negation, range, pattern)
- Type notation tokens (`:` colon, `\` backslash in type context)
- Comments, identifiers, literals, structural tokens
- **Total: ~60 base token types + 2 compound types**

**Cross-bridges When We Come To Them:**
If logos cannot handle a specific Polyglot syntax pattern (e.g., nested block markers with complex state), implement manual tokenization for ONLY that pattern while keeping logos for the rest. No such special cases identified as of 2025-11-17.

---

### ADR-014: String Concatenation Operator Change (`+"` vs `>"`)

**Status:** Accepted

**Context:** During Story 1.4 (Parser AST Definitions) review, the string concatenation operator `>"` was identified as non-intuitive and inconsistent with industry standards. The operator is used with line continuation marker `[*]` for explicit string literal concatenation in Polyglot v0.0.2.

**Decision:** Change the string concatenation operator from `>"` to `+"` throughout the language specification, documentation, lexer implementation, and all examples.

**Rationale:**
- **Industry Standards** - `+` is universally recognized for concatenation (JavaScript, Python, Java, C#, etc.)
- **Intuitive** - `+"` visually suggests "add/append string", while `>"` looks like "greater than quote"
- **Consistency** - Aligns with Polyglot's principle of "explicit over implicit" while using familiar operators
- **Visual Clarity** - `+"` is clearer in code: `"Hello" +" " +" "World"` vs `"Hello" >" " >" "World"`
- **Low Risk** - Language not yet in production, lexer just implemented, no user code to migrate

**Alternatives Considered:**
- **Keep `>"` operator** - Rejected due to poor intuitiveness and deviation from industry standards
- **Use `++` operator** - Rejected to avoid confusion with increment operators in other languages
- **Use `&` operator** - Rejected as it's used for boolean AND block marker `[&]` in Polyglot

**Consequences:**
- ✅ Improved developer experience for new Polyglot users (familiar syntax)
- ✅ Reduced cognitive load when reading multi-line string concatenations
- ✅ Better alignment with "Don't Reinvent the Wheel" philosophy
- ✅ Documentation updated (57 occurrences in `docs/user/language/08-line-continuation.md`)
- ✅ Lexer updated with new `TokenKind::OpStringConcat` token type
- ✅ All 26 lexer tests still passing after change
- ⚠️ Parser must validate `+"` only used between string literals (not variables - use interpolation instead)
- ℹ️ Total implementation time: ~2 hours (Medium impact, High value)

**Implementation Summary (2025-11-27):**
1. Updated 24 documentation files (`sed` global replacement)
2. Added `TokenKind::OpStringConcat` to `polyglot-lexer/src/token.rs`
3. Implemented `+"` recognition in `polyglot-lexer/src/lexer.rs`
4. Validated with `cargo test --workspace` (all tests passing)
5. Story 1.4 drafted with correct syntax (no updates needed)

**Parser Validation Requirements:**
- `+"` operator MUST only appear between string literals
- Variables use interpolation: `"{.var1} {.var2}"` NOT `"text" +" .var +" "text"`
- Line continuation `[*]` joins token streams; `+"` is required between literals

**Cross-Reference:**
- User Documentation: `docs/user/language/08-line-continuation.md`
- Token Definition: `polyglot-lexer/src/token.rs:69` (`OpStringConcat`)
- Lexer Implementation: `polyglot-lexer/src/lexer.rs`

---

_Generated by BMAD Decision Architecture Workflow v1.0_
_Date: 2025-11-16_
_For: hhj_