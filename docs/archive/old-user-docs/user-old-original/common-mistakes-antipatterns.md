# Common Mistakes & Anti-Patterns

**Version:** 0.0.2
**Last Updated:** 2025-11-22
**Status:** Critical Reference - Read Before Implementation

---

## ⚠️ WHY THIS DOCUMENT EXISTS

**If you've programmed in Python, JavaScript, C, Java, Rust, or Go, your muscle memory will lead you astray.**

Polyglot's syntax is fundamentally different from mainstream languages. During documentation validation, even with access to complete specifications, AI-assisted code generation produced **completely invalid syntax** by defaulting to familiar patterns.

**This document prevents those mistakes from reaching implementation.**

---

## 🚨 CRITICAL PRINCIPLES

### Polyglot is NOT:

❌ **NOT like Python** - Indentation is cosmetic, not structural
❌ **NOT like JavaScript/C/Rust** - No curly braces `{}` for scope
❌ **NOT like any mainstream language** - Scope = block markers ONLY

### Polyglot IS:

✅ **Block marker driven** - Every line starts with `[marker]`
✅ **Hierarchy based** - Parent-child relationships define scope
✅ **Explicit expansion** - Use `[~]` when nesting contexts

---

## 🎯 DESIGN PRINCIPLES

### Core Philosophy: "Don't Reinvent the Wheel"

**Polyglot is an orchestration language, NOT a general-purpose programming language.**

The moment you start implementing data structures, algorithms, or complex logic in Polyglot, you've missed the point. Polyglot exists to **coordinate battle-tested code** from Python, Rust, C++, and other languages - not to replace them.

---

### PRINCIPLE #1: Orchestrate, Don't Implement

**If you find yourself doing ANY of these in Polyglot, STOP:**

❌ Building arrays element-by-element
❌ String manipulation in loops
❌ Complex calculations or algorithms
❌ Data validation logic
❌ Parsing or serialization
❌ Regex matching
❌ Mathematical operations
❌ Business logic implementation

**Instead, delegate to specialized tools:**

| Task You're Tempted To Do | What To Do Instead |
|----------------------------|-------------------|
| Array transformation | Call Python list comprehension |
| String operations | Call Python `str` methods |
| JSON parsing | Call Python `json.loads()` |
| Data validation | Call Python Pydantic |
| Math calculations | Call Rust optimized functions |
| Regex matching | Call Python `re.match()` |
| CSV parsing | Call Python `csv.DictReader` |

**Example of WRONG approach:**
```polyglot
// ❌ WRONG - Implementing logic in Polyglot!
[r] .uppercase_items: pg\array{pg\string} << pg\array.new()
[r] ~ForEach
[<] .items
[>] .current_item
[~][r] .upper: pg\string << String.Upper"{.current_item}"
[~][r] .uppercase_items << Array.Append"{.uppercase_items}, {.upper}"
[~][o] !NoError
```

**Example of CORRECT approach:**
```polyglot
// ✅ CORRECT - Orchestrating Python!
[W] |W.Python3.11              // All macro unwraps use [W]
[r] |transform_to_uppercase    // Python does the work
[<] .items: pg\array{pg\string} << .items
[>] .result: pg\array{pg\string} >> .uppercase_items
```

**Polyglot's job:**
- ✅ Trigger the workflow (time, event, webhook)
- ✅ Route data between systems
- ✅ Handle errors and retries
- ✅ Manage priority and queuing
- ✅ Coordinate cross-language calls

**Polyglot's job is NOT:**
- ❌ Implement algorithms
- ❌ Transform data structures
- ❌ Perform calculations
- ❌ Validate business rules

---

### PRINCIPLE #2: Immutable by Default

**Decision tree for choosing between `#Enumeration` and `pg\serial`:**

```
Does the data schema change at runtime?
│
├─ NO  → Use #Enumeration (immutable schema)
│         - Compiler knows all fields
│         - Type-safe access
│         - Self-documenting
│
└─ YES → Use pg\serial (mutable keys)
          - Dynamic schema
          - Runtime flexibility
          - Use sparingly!
```

**Examples of when to use `#Enumeration`:**
- ✅ User profile (known fields: id, email, status, permissions)
- ✅ Application config (known fields: host, port, timeout)
- ✅ API request (known fields: method, url, headers, body)
- ✅ Database record with fixed schema

**Examples of when to use `pg\serial`:**
- ✅ JSON from external API (unknown structure)
- ✅ Dynamic configuration (keys vary by environment)
- ✅ Generic key-value store
- ✅ Truly schemaless data

**Wrong approach:**
```polyglot
// ❌ WRONG - Using pg\serial for known schema!
[r] @Auth|FetchUserProfile
[>] .profile: pg\serial >> .user_profile

[r] .email: pg\string << .user_profile.email
[r] .status: pg\string << .user_profile.status
```

**Correct approach:**
```polyglot
// ✅ CORRECT - Define enumeration for known schema!

// At file scope:
[#] UserProfile
[<] .user_id: pg\string << ""
[<] .email: pg\string << ""
[<] .status: #AccountStatus << #AccountStatus.Active
[<] .permissions: pg\array{pg\string} << pg\array.new()
[X]

// In pipeline:
[r] @Auth|FetchUserProfile
[>] .profile: #UserProfile >> .user_profile

[r] .email: pg\string << .user_profile.email  // Type-safe!
```

**Why immutable enumerations are better:**
- **Type safety** - Compiler catches field name typos
- **Self-documenting** - Schema visible in code
- **Performance** - No runtime key lookups
- **Refactoring** - Easy to find all usages

---

### PRINCIPLE #3: Minimal Data Transformation

**Polyglot should be a thin orchestration layer.**

If your Polyglot pipeline has more than ~20 lines of actual logic (excluding inputs, outputs, error handling), you're probably doing too much.

**Signs you're doing too much in Polyglot:**
- Multiple nested `[?]` conditionals with business logic
- Complex calculations spanning multiple lines
- String building and formatting operations
- Array/list manipulation loops
- Data validation rules

**What to do instead:**
1. Move logic to Python/Rust function
2. Call that function from Polyglot
3. Let Polyglot handle orchestration only

**Example of too much logic:**
```polyglot
// ❌ WRONG - Too much logic in Polyglot!
[r] .discount: pg\float << 0.0
[?] .customer_tier =? "gold"
[~][?] .order_total >? 1000.0
[~][~][r] .discount << 0.20
[~][?] .order_total >? 500.0
[~][~][r] .discount << 0.15
[~][?] .order_total >? 100.0
[~][~][r] .discount << 0.10

[?] .customer_tier =? "silver"
[~][?] .order_total >? 500.0
[~][~][r] .discount << 0.10
[~][?] .order_total >? 100.0
[~][~][r] .discount << 0.05

[r] .final_price: pg\float << .order_total * (1.0 - .discount)
```

**Example of proper orchestration:**
```polyglot
// ✅ CORRECT - Thin orchestration layer!
[W] |W.Python3.11
[r] |calculate_discount
[<] .customer_tier: pg\string << .customer_tier
[<] .order_total: pg\float << .order_total
[>] .final_price: pg\float >> .final_price
```

---

### PRINCIPLE #4: Leverage Existing Libraries

**Before writing ANY Polyglot code, ask:**

1. **Does Python have a library for this?** → Use it
2. **Does Rust have a crate for this?** → Use it
3. **Does C++ have a library for this?** → Use it
4. **Is this available as a web service?** → Call it

**Common scenarios:**

| Need | Don't Build | Use Instead |
|------|-------------|-------------|
| Email sending | Custom SMTP | Python `smtplib` |
| HTTP requests | Custom HTTP | Python `requests` |
| JSON handling | Custom parser | Python `json` |
| CSV processing | Custom parser | Python `csv` |
| Database queries | Custom SQL | Python `psycopg2` |
| Image processing | Custom code | Python `Pillow` |
| ML inference | Custom code | Python `scikit-learn` |
| Encryption | Custom crypto | Rust crypto crates |
| File compression | Custom zip | Python `zipfile` |

**The golden rule:**
> If it exists in a battle-tested library, DON'T reimplement it in Polyglot.

---

### PRINCIPLE #5: Variable Naming Convention

**ALL variables MUST start with dot (`.`) in ALL contexts:**

✅ **Declarations:** `.name: pg\string << "value"`
✅ **References:** `.result << .name`
✅ **Assignments:** `>> .output_var`
✅ **String interpolation:** `"Hello {.name}"`
✅ **Field access:** `.user_profile.email`

❌ **NEVER omit the dot:**
- ❌ `name: pg\string`
- ❌ `>> output_var`
- ❌ `"Hello {name}"`
- ❌ `user_profile.email`

**This is a syntax requirement, not a suggestion.**

---

### Summary: What Polyglot Does Well

**Polyglot excels at:**
- ⭐ Scheduling workflows (cron triggers)
- ⭐ Event-driven automation (file watches, webhooks)
- ⭐ Cross-language coordination (Python + Rust + C++)
- ⭐ Error handling and retry logic
- ⭐ Priority-based queue management
- ⭐ Parallel execution coordination
- ⭐ System integration glue code

**Polyglot is terrible at:**
- 💀 Implementing algorithms
- 💀 Data structure manipulation
- 💀 Complex business logic
- 💀 Mathematical calculations
- 💀 String/array transformations
- 💀 Parsing and serialization

**Use the right tool for the job. Polyglot is the orchestrator, not the implementer.**

---

## 🔴 ANTI-PATTERN #1: Using Curly Braces for Scope

### ❌ WRONG - Curly Braces (DON'T DO THIS!)

```polyglot
// THIS IS COMPLETELY INVALID!
[r] .process_data(
    .input: pg\string
) -> .result: pg\dict {
    [*] "Process the input"
    [|] .temp: pg\string << "value"
    [r] |Transform
    [<] .data << .input
}
```

**Why this is wrong:**
- Curly braces `{}` do NOT exist for scope in Polyglot
- Function signature syntax `(.param) -> .result` is invalid
- Polyglot has ZERO function-like definitions

### ✅ CORRECT - Block Marker Hierarchy

```polyglot
// This is the correct Polyglot syntax
[|] ProcessData
[i] .input: pg\string
[o] .result: pg\dict
[r] .temp: pg\string << "value"
[r] |Transform
[<] .data: pg\string << .input
[>] .result: pg\dict >> output
[X]
```

**Why this is correct:**
- Pipeline defined with `[|]...[X]`
- Input declared with `[i]`
- Output declared with `[o]`
- Operations use `[r]` with implicit child `[<]` and `[>]`
- NO curly braces anywhere

---

## 🔴 ANTI-PATTERN #2: Indentation-Based Scope

### ❌ WRONG - Treating Indentation as Meaningful

```polyglot
// THIS IS INVALID! (Looks like Python)
[?] .age >? 18
    [r] |ProcessAdult
    [r] .status: pg\string << "adult"
    [?] .verified =? #True
        [r] |GrantAccess
```

**Why this is wrong:**
- Indentation does NOT define scope in Polyglot
- Nested blocks require explicit `[~]` prefix
- This looks like Python but isn't

### ✅ CORRECT - Explicit Nesting with `[~]`

```polyglot
// This is the correct Polyglot syntax
[?] .age >? 18
[~][r] |ProcessAdult
[~][r] .status: pg\string << "adult"
[~][?] .verified =? #True
[~][~][r] |GrantAccess
```

**Why this is correct:**
- `[~]` explicitly marks nesting within `[?]` block
- `[~][~]` marks double nesting
- Indentation is cosmetic - can be any amount

**Alternative (if not nested):**

```polyglot
// If these are sequential, not nested:
[?] .age >? 18
[r] |ProcessAdult

[r] .status: pg\string << "adult"

[?] .verified =? #True
[r] |GrantAccess
```

---

## 🔴 ANTI-PATTERN #3: Function-Like Pipeline Definitions

### ❌ WRONG - Function Signature Syntax

```polyglot
// THIS IS INVALID!
[r] .calculate_total(.items: pg\array, .tax_rate: pg\float) -> pg\float {
    [r] .subtotal << sum(.items)
    [r] .total << .subtotal * (1 + .tax_rate)
    return .total
}
```

**Why this is wrong:**
- No function signature syntax in Polyglot
- No `return` keyword
- No inline parameter lists with `()`
- Curly braces don't exist for scope

### ✅ CORRECT - Pipeline Definition

```polyglot
// This is the correct Polyglot syntax
[|] CalculateTotal
[i] .items: pg\array{pg\float}
[i] .tax_rate: pg\float
[o] .total: pg\float
[r] .subtotal: pg\float << pg\sum(.items)
[r] .total: pg\float << .subtotal * (1 + .tax_rate)
[X]
```

**Why this is correct:**
- Pipeline defined with `[|] Name` ... `[X]`
- Inputs declared with `[i]`
- Output declared with `[o]`
- Operations use `[r]` for sequential execution
- Result automatically returned via `[o]` declaration

---

## 🔴 ANTI-PATTERN #4: Misunderstanding Implicit Expansion

### ❌ WRONG - Using `[~]` Unnecessarily

```polyglot
// THIS IS UNNECESSARILY VERBOSE!
[r] |ProcessData
[~][<] .input: pg\string << "value"
[~][>] .output >> result
```

**Why this is wrong:**
- `[<]` and `[>]` are **implicit children** of `[r]`
- `[~]` is not needed for direct parent-child relationships

### ✅ CORRECT - Implicit Child Relationships

```polyglot
// This is the correct Polyglot syntax
[r] |ProcessData
[<] .input: pg\string << "value"    // Implicit child of [r]
[>] .output >> result               // Implicit child of [r]
```

**Why this is correct:**
- `[<]` and `[>]` following `[r]` are automatically children
- NO `[~]` prefix needed
- Cleaner, more readable

### ✅ WHEN to use `[~]` - Explicit Nesting

```polyglot
// Use [~] when nesting inside expanded contexts
[p] |ParallelBlock
[<] .data: pg\string << input
[~][r] |NestedOperation              // [~] required - WITHIN parallel context
[~][<] .input: pg\string << .data    // Implicit child of [~][r]
```

**When `[~]` IS needed:**
- Operations inside `[p]` parallel blocks
- Operations inside `[?]` conditional blocks
- Operations inside unpack iterations
- Any time you're adding operations WITHIN an expanded context

---

## 🔴 ANTI-PATTERN #5: Confusing Legitimate Braces with Scope Braces

### ✅ CORRECT - Legitimate Uses of Braces

Polyglot DOES use curly braces in specific contexts - but **NEVER for scope**:

**1. Type Syntax (Collection Types):**
```polyglot
[i] .items: pg\array{pg\string}     // ✅ Array of strings
[i] .numbers: pg\set{pg\int}        // ✅ Set of integers
```

**2. String Interpolation:**
```polyglot
[r] .message: pg\string << "Hello {.name}!"              // ✅ Variable interpolation
[r] .length: pg\int << {.data|length}                    // ✅ With filter
```

**3. Path Construction:**
```polyglot
[r] .config_file: pg\path << {.base_path / ".config"}    // ✅ Path operations
```

**4. Serial Literals (TBD - check spec):**
```polyglot
[r] .config: pg\serial << serial{
    "host": "localhost",
    "port": 8080
}
```

### ❌ WRONG - Braces for Scope

```polyglot
// THIS IS INVALID!
[r] |ProcessData {
    [<] .input << value
    [>] .output >> result
}
```

**Rule of Thumb:**
- Braces in **type position** = ✅ OK
- Braces in **string position** = ✅ OK
- Braces after **block markers** = ❌ NEVER

---

## 🔴 ANTI-PATTERN #6: Missing Block Markers

### ❌ WRONG - Bare Statements

```polyglot
// THIS IS INVALID!
[|] ProcessFile
.file_path: pg\path              // ❌ Missing [i]
|ReadFile                        // ❌ Missing [r]
.content: pg\string              // ❌ Missing [o]
[X]
```

**Why this is wrong:**
- EVERY line must start with a block marker
- No exceptions

### ✅ CORRECT - Block Markers on Every Line

```polyglot
// This is the correct Polyglot syntax
[|] ProcessFile
[i] .file_path: pg\path          // ✅ [i] for input
[r] |ReadFile                    // ✅ [r] for run
[<] .path: pg\path << .file_path
[>] .content: pg\string >> data
[o] .content: pg\string          // ✅ [o] for output
[X]
```

---

## 🔴 ANTI-PATTERN #7: Keyword-Based Logic

### ❌ WRONG - Using Keywords

```polyglot
// THIS IS INVALID!
if .age > 18 {
    [r] |ProcessAdult
} else {
    [r] |ProcessMinor
}
```

**Why this is wrong:**
- Polyglot has ZERO keywords for control flow
- No `if`, `else`, `while`, `for`, `return`, etc.

### ✅ CORRECT - Block Marker Based Logic

```polyglot
// This is the correct Polyglot syntax
[?] .age >? 18
[~][r] |ProcessAdult

[?] .age <? 18
[~][r] |ProcessMinor
```

**Polyglot alternatives:**
- `[?]` for conditionals (not `if`)
- `[Y]` for iterations (not `for`)
- `[o]` for return (not `return`)
- `[!]` for error handling (not `try/catch`)

---

## 🔴 ANTI-PATTERN #8: Incorrect Pipeline Calls

### ❌ WRONG - Function Call Syntax

```polyglot
// THIS IS INVALID!
result = ProcessData("input_value", timeout=30)
```

**Why this is wrong:**
- No function call syntax `()`
- No keyword arguments
- No assignment operator `=`

### ✅ CORRECT - Pipeline Call with Block Markers

```polyglot
// This is the correct Polyglot syntax
[r] |ProcessData
[<] .input: pg\string << "input_value"
[<] .timeout: pg\int << 30
[>] .result >> output
```

**Pattern:**
- `[r]` or `[p]` for execution
- `[<]` for each input parameter
- `[>]` for output capture

---

## ✅ VERIFICATION CHECKLIST

Before submitting ANY Polyglot code, verify:

### Syntax Compliance:
- [ ] Every line starts with a block marker `[marker]`
- [ ] NO curly braces `{}` used for scope
- [ ] NO function signatures `func(.param) -> .result`
- [ ] NO keywords (`if`, `for`, `while`, `return`, `try`, etc.)
- [ ] Indentation is cosmetic only

### Block Hierarchy:
- [ ] Pipeline definitions use `[|]...[X]`
- [ ] Implicit children (`[<]`, `[>]` after `[r]`) have NO `[~]` prefix
- [ ] Explicit nesting (operations inside `[p]`, `[?]`) uses `[~]` prefix
- [ ] Nesting depth matches number of `[~]` prefixes

### Type Syntax:
- [ ] Braces in types are legitimate: `pg\array{pg\string}`
- [ ] Braces in strings are legitimate: `"Hello {.name}"`
- [ ] NO braces after block markers for scope

### Control Flow:
- [ ] Conditionals use `[?]` block marker
- [ ] Error handling uses `[!]` block marker
- [ ] Iterations use `[Y]` unpack operator
- [ ] NO keywords anywhere

---

## 🔴 ANTI-PATTERN #9: Incorrect Error Handling Nesting

### ❌ WRONG - Single-Level Error Nesting

```polyglot
// THIS IS INCOMPLETE!
[r] |MightFail
[<] .input: pg\string << value
[~][!] !FileNotFound
[~][>] .message: pg\string >> err_msg
[~][r] |HandleError              // ❌ Wrong nesting level!
[~][<] .msg: pg\string << err_msg
```

**Why this is wrong:**
- Error handler must be nested WITHIN the error catch block
- Needs `[~][~]` double nesting for operations inside `[~][!]`

### ✅ CORRECT - Double-Nested Error Handling

```polyglot
// This is the correct Polyglot syntax
[r] |MightFail
[<] .input: pg\string << value
[~]
[~][!] !FileNotFound
[~][>] .message: pg\string >> err_msg
[~][~][r] |HandleError           // ✅ [~][~] = WITHIN error catch
[~][~][<] .msg: pg\string << err_msg
[~][~]
[~][~][o] !FileNotFound          // ✅ Propagate error upward
```

**Why this is correct:**
- `[~]` marks start of error handling context
- `[~][!]` catches the error
- `[~][~]` nests operations WITHIN the catch block
- `[~][~][o]` propagates error to outer scope

---

## 🔴 ANTI-PATTERN #10: Function Call Syntax for String Operations

### ❌ WRONG - Method Call Syntax

```polyglot
// THIS IS INVALID!
[r] .result: pg\string << pg\string.uppercase(.input)
[r] .length: pg\int << pg\string.length(.input)
[r] .trimmed: pg\string << pg\string.trim(.input)
```

**Why this is wrong:**
- No method call syntax `type.method()` in Polyglot
- Functions don't use parentheses

### ✅ CORRECT - String Literal Pipeline Syntax

```polyglot
// This is the correct Polyglot syntax
[r] .result: pg\string << String.Upper"{.input}"
[r] .length: pg\int << String.Length"{.input}"
[r] .trimmed: pg\string << String.Trim"{.input}"
```

**Why this is correct:**
- String operations use literal pipeline syntax
- Pattern: `Operation"{value}"` or `Operation"{.variable}"`
- Works for: `String.Upper`, `String.Lower`, `String.Trim`, etc.

**Note:** Standard library implementation comes after lexer/compiler

---

## 🔴 ANTI-PATTERN #11: Incorrect ForEach Iteration

### ❌ WRONG - Unpacking Collections Directly

```polyglot
// THIS IS INVALID!
[r] ~my_array
[~][r] .item: pg\string << .current_item
```

**Why this is wrong:**
- `~collection` is not the correct iteration syntax
- No automatic `.current_item` variable

### ✅ CORRECT - ForEach with Explicit Input/Output

```polyglot
// This is the correct Polyglot syntax
[r] ~ForEach
[<] .my_array
[>] .current_item
[~][r] .processed: pg\string << String.Upper"{.current_item}"
[~][r] |DoSomething
[~][<] .input: pg\string << .processed
[~][o] !NoError
```

**Why this is correct:**
- `~ForEach` is an actual operation
- `[<]` provides the collection as input
- `[>]` declares the output variable for current item
- Operations inside loop use `[~]` prefix
- `[~][o]` returns from each iteration

---

## 🔴 ANTI-PATTERN #12: Trigger with Unnecessary Parameters

### ❌ WRONG - Verbose Trigger Syntax

```polyglot
// THIS IS UNNECESSARILY VERBOSE!
[t] |T.Cron
[<] .schedule: pg\string << "0 2 * * *"
```

**Why this is wrong:**
- When trigger has single string argument, use literal syntax
- Extra verbosity makes code harder to read

### ✅ CORRECT - Trigger String Literal

```polyglot
// This is the correct Polyglot syntax
[t] T.Cron"0 2 * * *"
```

**Why this is correct:**
- Single-argument string triggers use literal syntax
- Pattern: `[t] T.Type"string_value"`
- NO pipe `|` before trigger name
- Much more concise

**Applies to:**
- Cron triggers: `T.Cron"schedule"`
- SQL queries: `sql"SELECT * FROM table"`
- Any single-string-argument operation

---

## 🔴 ANTI-PATTERN #13: Join Uses Wrong Direction

### ❌ WRONG - Using `[>]` for Join

```polyglot
// THIS IS BACKWARDS!
[Y] |Y.Join
[>] result_a
[>] result_b
```

**Why this is wrong:**
- Join doesn't extract data, it synchronizes it
- `[>]` means "pull FROM source" - wrong direction

### ✅ CORRECT - Join Uses `[<]`

```polyglot
// This is the correct Polyglot syntax
[Y] |Y.Join
[<] .result_a
[<] .result_b
```

**Why this is correct:**
- `[<]` provides variables TO the join operation
- Join synchronizes parallel results
- Variables were already assigned with `>>` earlier

**Pattern:**
```polyglot
[p] |ProcessA
[>] .output >> result_a    // Pull FROM ProcessA

[p] |ProcessB
[>] .output >> result_b    // Pull FROM ProcessB

[Y] |Y.Join
[<] .result_a              // Provide TO join
[<] .result_b              // Provide TO join
```

---

## 🔴 ANTI-PATTERN #14: Missing Package Alias in Cross-Package Calls

### ❌ WRONG - Calling Imported Pipeline Without Alias

```polyglot
// THIS IS INCOMPLETE!
[@] Local@MyApp:1.0.0
[#] 1
[<] @DB << Community@Database:2.0.0
[X]

[|] MyPipeline
[r] |ValidateData    // ❌ Which package is this from?
[X]
```

**Why this is wrong:**
- Doesn't specify which package the pipeline comes from
- Ambiguous if multiple packages have same pipeline name

### ✅ CORRECT - Use Package Alias Prefix

```polyglot
// This is the correct Polyglot syntax
[@] Local@MyApp:1.0.0
[#] 1
[<] @DB << Community@Database:2.0.0
[X]

[|] MyPipeline
[r] @DB|ValidateData    // ✅ Clear it's from @DB package
[<] .data: pg\serial << input_data
[X]
```

**Why this is correct:**
- `@PackageAlias|PipelineName` syntax
- Explicit package source
- Works with enumerations too: `@PackageAlias#Enumeration.Variant`

---

## 🔴 ANTI-PATTERN #15: Non-Exhaustive Switch Statements

### ❌ WRONG - Missing Catchall Case

```polyglot
// THIS IS INCOMPLETE - Compiler will reject!
[?] .status =? "active"
[~][r] |ProcessActive
[~][o] !NoError

[?] .status =? "inactive"
[~][r] |ProcessInactive
[~][o] !NoError

// ❌ What if status is neither? Not exhaustive!
// ❌ Code after switch - where does this belong?
[r] |LogResult
```

**Why this is wrong:**
- Switch statements **MUST be exhaustive** (compiler requirement)
- All possible values must be handled
- Code "after" conditionals is ambiguous - must be "within" a case

### ✅ CORRECT - Exhaustive with Catchall

```polyglot
// This is the correct Polyglot syntax
[?] .status =? "active"
[~][r] |ProcessActive
[~][o] !NoError

[?] .status =? "inactive"
[~][r] |ProcessInactive
[~][o] !NoError

[?] *?                    // ✅ Catchall - handles all other cases
[~][r] |LogResult
[~][r] |ProcessUnknown
[~][o] !NoError
```

**Why this is correct:**
- `[?] *?` is the catchall/default case (like `else` or `default`)
- Covers ALL remaining possibilities
- Compiler enforces total probability = 1
- All code belongs to a specific case

**Pattern for exhaustive matching:**
```polyglot
[?] condition1
[~]// ... handle case 1
[~][o] result1

[?] condition2
[~]// ... handle case 2
[~][o] result2

[?] *?                    // REQUIRED - catchall
[~]// ... handle all other cases
[~][o] default_result
```

---

## 🔴 ANTI-PATTERN #16: Using Undefined Enumerations

### ❌ WRONG - Reference Before Definition

```polyglot
// THIS IS INVALID - #UserStatus not defined!
[@] Local@MyApp:1.0.0
[#] 1
[X]

[#] EmailRecipient
[<] .user_id: pg\string << ""
[<] .status: #UserStatus << #UserStatus.Active    // ❌ What is UserStatus?
[X]

[|] MyPipeline
[X]
```

**Why this is wrong:**
- Can't use `#UserStatus` without defining it first
- Compiler doesn't know what variants exist
- Type checking impossible

### ✅ CORRECT - Define Before Use

```polyglot
// This is the correct Polyglot syntax
[@] Local@MyApp:1.0.0
[#] 1
[X]

// ✅ Define enumeration FIRST
[#] UserStatus
[<] .Active
[<] .Inactive
[<] .Suspended
[X]

// ✅ Now can use it
[#] EmailRecipient
[<] .user_id: pg\string << ""
[<] .email: pg\string << ""
[<] .status: #UserStatus << #UserStatus.Active    // ✅ UserStatus is defined
[X]

[|] MyPipeline
[X]
```

**Why this is correct:**
- Enumeration defined at file scope before use
- All variants explicitly listed
- Type-safe references: `#UserStatus.Active`
- Compiler knows valid variants

**Enumeration definition rules:**
- Variants are simple: `[<] .VariantName` (no type annotation needed)
- Define at file scope (before pipelines)
- Each variant on its own `[<]` line
- Close with `[X]`

**Example of variant usage:**
```polyglot
[#] AccountStatus
[<] .Active
[<] .Inactive
[<] .Suspended
[<] .Locked
[X]

// Later in code:
[?] .user_status =? #AccountStatus.Active
[~][r] |GrantAccess

[?] .user_status =? #AccountStatus.Suspended
[~][r] |SendReactivationEmail

[?] *?
[~][r] |DenyAccess
```

---

## 📚 CORRECT PATTERN REFERENCE

### Minimal Pipeline:
```polyglot
[|] MinimalPipeline
[i] #None
[t] |T.Call
[r] |DoSomething
[o] #None
[X]
```

### Pipeline with Input/Output:
```polyglot
[|] Transform
[i] .input: pg\string
[r] |ProcessData
[<] .data: pg\string << .input
[>] .result: pg\string >> output
[o] .result: pg\string
[X]
```

### Conditional Logic:
```polyglot
[?] .age >? 18
[~][r] |ProcessAdult

[?] .age =<? 18
[~][r] |ProcessMinor
```

### Parallel Execution with Join:
```polyglot
[p] |ProcessPartA
[<] .input: pg\string << data
[>] .output >> result_a

[p] |ProcessPartB
[<] .input: pg\string << data
[>] .output >> result_b

[Y] |Y.Join
[<] .result_a        // ✅ Use [<] for join, not [>]
[<] .result_b
```

### Error Handling (Double-Nested):
```polyglot
[r] |MightFail
[<] .input: pg\string << value
[~]
[~][!] !FileNotFound
[~][>] .message: pg\string >> err_msg
[~][~][r] |HandleError       // ✅ [~][~] = nested within error catch
[~][~][<] .msg: pg\string << err_msg
[~][~]
[~][~][o] !FileNotFound      // ✅ Propagate error
```

### ForEach Iteration:
```polyglot
[r] ~ForEach
[<] .my_array
[>] .current_item
[~][r] .processed: pg\string << String.Upper"{.current_item}"
[~][o] !NoError
```

### Trigger String Literal:
```polyglot
[t] T.Cron"0 2 * * *"        // ✅ Single-arg string literal
```

---

## 🎯 GOLDEN RULE

**When in doubt, compare your code to approved examples:**

1. `/docs/project/prd.md` (lines 450, 551)
2. `/docs/user/examples/07-approved-examples.md`
3. `/docs/user/language/01-syntax-complete.md` (lines 1190-1230)

**If your code looks different, it's probably wrong.**

---

## 📖 SEE ALSO

- [Complete Syntax Reference](language/01-syntax-complete.md) - Canonical syntax
- [Block Markers](language/06-block-markers.md) - All block markers
- [Block Hierarchy Reference](../technical/block-hierarchy-reference.md) - Parent-child relationships
- [Quick Start](quick-start.md) - First Polyglot pipeline

---

**Status:** ✅ COMPLETE - Required reading before Epic 1 implementation
