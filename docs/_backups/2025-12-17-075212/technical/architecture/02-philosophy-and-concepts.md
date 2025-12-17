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

