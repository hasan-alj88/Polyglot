# Code Violations Log

**Version:** 0.0.2
**Purpose:** Track syntax errors and violations found during documentation creation
**Created:** 2025-11-12

---

## Violation Categories

1. **Missing Required Elements**
2. **Incorrect Syntax**
3. **Formatting Violations**
4. **Undocumented Features Used**

---

## Session 1: Hello World Examples Creation (2025-11-12)

### Violation #1: Missing Wrapper Declaration

**Severity:** CRITICAL
**Category:** Missing Required Elements

**Description:**
ALL valid Polyglot code MUST start with one of: `[t]`, `[i]`, `[W]`, or `{[\], [/]}` after the pipeline definition `[|]`. Multiple examples were created without the required wrapper.

**Invalid Code Pattern:**
```polyglot
[|] PipelineName
[o] .output: pg\string << "value"
[X]
```

**Correct Code Pattern:**
```polyglot
[|] PipelineName
[W] |W.NoSetup.NoCleanup
[o] .output: pg\string << "value"
[X]
```

**Files Affected:**
- `examples/01-hello-world.md` - Examples 1, 2, 3 (helper), 5 (helpers), 6 (helpers), Common Patterns 1 & 2

**Root Cause:**
Documentation did not emphasize the strict requirement that pipelines MUST have `[t]`, `[i]`, `[W]`, or setup/cleanup blocks.

**Fix Required:**
- Add explicit rule to syntax documentation
- Update all example files
- Add to quick language reference

---

### Violation #2: Missing String Continuation Syntax

**Severity:** HIGH
**Category:** Undocumented Features Used

**Description:**
The `[^]` line continuation block element for multi-line strings was not documented but is required for proper string concatenation.

**Undocumented Syntax:**
```polyglot
[|] CombineGreetings
[i] .english: pg\array{pg\string}
[i] .spanish: pg\array{pg\string}
[i] .french: pg\array{pg\string}
[W] |W.NoSetup.NoCleanup

[o] .all: pg\string << ""
[^] +"English: {.english}, "
[^] +"Spanish: {.spanish}, "
[^] +"French: {.french}"
[X]
```

**Explanation:**
- `[^]` = Line continuation block element
- `+""` = String literal continuation (explicit, not automatic like Python)
- Must be explicit to avoid accidental concatenation

**Fix Required:**
- Document `[^]` in block markers reference
- Document `+""` string continuation syntax
- Add examples to syntax reference
- Add to operators documentation

---

### Violation #3: Missing Formatting Rules

**Severity:** MEDIUM
**Category:** Formatting Violations

**Description:**
Code formatting rules were not documented, leading to inconsistent spacing.

**Required Formatting Rules:**

1. **Before/After Pipeline Calls:**
   - 1 blank line BEFORE block element with pipeline call
   - 1 blank line AFTER block element with pipeline call

   ```polyglot
   [r] |SomeOperation

   [r] |AnotherOperation
   ```

2. **Before Pipeline/Enumeration Definitions:**
   - 4 blank lines BEFORE pipeline definition `[|]`
   - 4 blank lines BEFORE enumeration definition `[#]`

   ```polyglot
   [X]




   [|] NextPipeline
   ```

**Files Affected:**
- ALL example files created in this session

**Fix Required:**
- Create `audit/formatting-rules.md`
- Add formatting section to syntax reference
- Update all examples to follow formatting rules

---

### Violation #4: Missing Required Trigger Declaration

**Severity:** CRITICAL
**Category:** Missing Required Elements

**Description:**
ALL pipelines MUST have a trigger declaration using `[t] |T.*` format. Compiler will throw an error if trigger is missing. This was completely missing from all examples.

**Invalid Code Pattern:**
```polyglot
[|] PipelineName
[i] .input: pg\string
[W] |W.NoSetup.NoCleanup
[o] .output: pg\string << "value"
[X]
```

**Correct Code Patterns:**
```polyglot
// Manual call trigger (called by other pipelines)
[|] PipelineName
[i] .input: pg\string
[t] |T.Call
[W] |W.NoSetup.NoCleanup
[o] .output: pg\string << "value"
[X]

// CLI trigger (polyglot run Local@pkg|PipelineName)
[|] CliPipeline
[t] |T.Cli
[W] |W.NoSetup.NoCleanup
[o] .result: pg\string << "done"
[X]

// Scheduled trigger
[|] DailyTask
[t] |T.Daily
[<] .time: pg\dt << DT"09:00:"
[W] |W.NoSetup.NoCleanup
[r] |DoWork
[X]
```

**Critical Rules:**
1. **ALL pipelines MUST have `[t] |T.*` trigger** - compiler error otherwise
2. **`[t] |T.Call`** - For pipelines called by other pipelines using `|PipelineName`
3. **`[t] |T.Cli`** - For pipelines run via `polyglot run` command
4. **DO NOT use boolean variables as triggers** - triggers are continuously-run operations
5. **Invalid:** `[t] .condition: pg\bool << .should_greet` - This is WRONG syntax

**Files Affected:**
- `examples/01-hello-world.md` - ALL examples (Examples 1-6, all helpers, all patterns)
- `audit/quick-language-reference.md` - Missing from critical rules
- `audit/formatting-rules.md` - Missing from required structure

**Root Cause:**
Complete misunderstanding of `[t]` marker. It is NOT for conditional execution - it's for trigger declaration and is REQUIRED for all pipelines.

**Fix Required:**
- Add trigger to ALL examples
- Update quick language reference with mandatory trigger rule
- Update formatting rules with trigger in required structure
- Document trigger types: T.Call, T.Cli, T.Daily, T.File.Modified, etc.
- Remove invalid conditional trigger syntax from Example 3

---

### Violation #5: Output Declaration with Assignment

**Severity:** CRITICAL
**Category:** Incorrect Syntax
**Compiler Rule:** `[o]` marker is for declaration ONLY, NOT for assignment

**Description:**
The `[o]` output marker should ONLY declare which variables are outputs, NOT perform computation or assignment. All computation must use `[r]` (sequential) or occur in pipeline calls.

**Invalid Code Pattern:**
```polyglot
[|] GreetUser
[i] .name: pg\string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[o] .greeting: pg\string << "Hello, {.name}!"  // ✗ WRONG - assignment in [o]
[X]
```

**Correct Code Pattern:**
```polyglot
[|] GreetUser
[i] .name: pg\string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[r] .greeting: pg\string << "Hello, {.name}!"  // ✓ Compute with [r]
[o] .greeting: pg\string                        // ✓ Declare as output
[X]
```

**Rationale:**
1. **Separation of Concerns:** Computation (`[r]`) vs Declaration (`[o]`)
2. **Symmetry with Input:** `[i]` declares inputs, `[o]` declares outputs
3. **Clarity:** Complex pipelines benefit from clear distinction
4. **Consistency:** Matches the documented pattern in block-markers.md

**Exception (Discouraged):**
For trivial constant-only outputs, `[o] .field: type << constant` MAY be acceptable, but `[r]` followed by `[o]` is still preferred for consistency.

**Files Affected:**
- `examples/01-hello-world.md` - Examples 1, 2, 4 (line 243), 5, 6 helpers
- All examples using combined `[o]` + `<<` syntax

**Fix Required:**
- Split all `[o] .field: type << value` into `[r]` + `[o]` pairs
- Update all documentation to use this pattern consistently
- Add this rule to quick-language-reference.md

---

### Violation #6: Inconsistent Output Sets Across Branches

**Severity:** CRITICAL
**Category:** Type Safety Violation
**Compiler Rule:** All branches (switch/parallel) MUST produce the same set of outputs

**Description:**
When a pipeline has multiple execution paths (switch branches `[?]` or parallel blocks `[p]`), EVERY path must declare the EXACT SAME set of outputs with the SAME TYPES. The compiler enforces this for type safety.

**Invalid Code Pattern:**
```polyglot
[|] ConditionalProcess
[i] .condition: pg\bool
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[?] .condition ?> True
[~][r] .result: pg\string << "Success"
[~][o] .result: pg\string              // ✓ Has .result output

[?] .condition ?> False
[~][r] .status: pg\int << 404
[~][o] .status: pg\int                 // ✗ WRONG - different output name

[X]
```

**Correct Code Pattern (Same Output Set):**
```polyglot
[|] ConditionalProcess
[i] .condition: pg\bool
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[?] .condition ?> True
[~][r] .result: pg\string << "Success"
[~][r] .status: pg\int << 200
[~][o] .result: pg\string              // ✓ Both outputs
[~][o] .status: pg\int

[?] .condition ?> False
[~][r] .result: pg\string << "Failed"
[~][r] .status: pg\int << 404
[~][o] .result: pg\string              // ✓ Same outputs
[~][o] .status: pg\int

[X]
```

**Applies to:**
1. **Switch Branches (`[?]`)**: All branches of a switch must output the same fields
2. **Parallel Blocks (`[p]`)**: All parallel blocks at the same level must output the same fields
3. **Error Handling (`[!]`)**: Error branches must maintain the same output contract

**Compiler Behavior:**
```
Error: Inconsistent output sets across branches
Pipeline: ConditionalProcess
Branch 1 outputs: .result (pg\string)
Branch 2 outputs: .status (pg\int)
Expected: All branches must declare identical output sets
```

**Example with Error Handling:**
```polyglot
[|] SafeOperation
[i] .input: pg\string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[r] |MightFail
[<] .data: pg\string << .input
[>] .result: pg\string >> success_result

[!] !SomeError
[~][r] .result: pg\string << "error fallback"
[~][o] .result: pg\string              // ✓ Same output as success path

[r] .result: pg\string << success_result
[o] .result: pg\string                  // ✓ Consistent across all paths
[X]
```

**Rationale:**
1. **Type Safety:** Caller knows exactly what outputs to expect
2. **Predictability:** No undefined behavior based on execution path
3. **Interface Contract:** Clear API contract for pipeline consumers
4. **Static Analysis:** Compiler can verify correctness at compile-time

**Files Affected:**
- `examples/01-hello-world.md` - Example 4 (ValidateAndGreet - line 237-243)

**Current Issue in Example 4:**
```polyglot
[?] .input_name == ""
[~][o] .error: !ValidationError        // ✓ Outputs .error
// ✗ MISSING: Should also declare .greeting (or use consistent error pattern)

[?] .input_name != ""
[~][o] .greeting: pg\string            // ✓ Outputs .greeting
// ✗ MISSING: Should also declare .error (or use consistent success pattern)
```

**Fix Options:**

**Option A - Both branches output both fields:**
```polyglot
[?] .input_name == ""
[~][r] .error: !ValidationError
[~][<] .message: pg\string << "Name cannot be empty"
[~][<] .code: pg\int << 1001
[~][<] .trace: pg\string << ""
[~][r] .greeting: pg\string << ""      // Empty when error
[~][o] .error: !ValidationError
[~][o] .greeting: pg\string

[?] .input_name != ""
[~][r] .greeting: pg\string << "Hello, {.input_name}!"
[~][r] .error: !ValidationError << \\NoError\\  // No error value
[~][o] .greeting: pg\string
[~][o] .error: !ValidationError
```

**Option B - Use single consistent output (recommended):**
```polyglot
// Either output ONLY .greeting with error as variant,
// OR use Result type pattern
[?] .input_name == ""
[~][r] .result: !ValidationError       // Output is error type
[~][<] .message: pg\string << "Name cannot be empty"
[~][<] .code: pg\int << 1001
[~][<] .trace: pg\string << ""
[~][o] .result: !ValidationError

[?] .input_name != ""
[~][r] .result: pg\string << "Hello, {.input_name}!"
[~][o] .result: pg\string              // ✗ DIFFERENT TYPE - needs union type
```

**Note:** This reveals a potential need for union types or Result<T, E> pattern in Polyglot.

**Fix Required:**
- Document this compiler rule in language/01-syntax-complete.md
- Add to quick-language-reference.md as CRITICAL RULE
- Fix Example 4 to demonstrate proper error handling pattern
- Consider documenting Result/Union type patterns for error handling

---

### Violation #7: Undocumented Switch/Conditional System & Invalid Comparison Operators

**Severity:** CRITICAL
**Category:** Undocumented Features Used + Incorrect Syntax
**Compiler Rule:** Switch blocks enforce exhaustive pattern matching with total probability = 1

**Description:**
The entire switch/conditional system using `[?]` block marker is used throughout examples but is **completely undocumented** in the language specification. Additionally, examples incorrectly use comparison operators (`==`, `!=`) which are **NOT part of Polyglot**. Polyglot prefers explicit named operations.

**What's Missing (Needs Documentation):**
1. `[?]` switch block marker
2. `?>` match operator
3. Range operator `..` (Rust-style)
4. Exhaustive matching requirement (total probability = 1)
5. Default/else pattern for remaining cases

**What's WRONG (Invalid Syntax Used in Examples):**
- **Comparison operators: `==`, `!=`, `<`, `>`, `<=`, `>=` are NOT valid Polyglot syntax**
- These appear in examples but should be replaced with explicit named operations

**Invalid Examples Using Wrong Syntax:**
```polyglot
// ✗ WRONG - Comparison operators don't exist in Polyglot
[?] .input_name == ""
[~][r] |HandleEmpty

[?] .input_name != ""
[~][r] |HandleNotEmpty

// ✗ WRONG - Multiple errors:
// 1. Errors are handled by [!] block, NOT [?]
// 2. \\NoError\\ doesn't exist (backslash literals are for pg\path and pg\url only)
// 3. No-error state doesn't need handling (no error to catch)
[?] greeting_error != \\NoError\\
[~][r] |HandleError

[?] .age >= 18
[~][r] |HandleAdult
```

**Correct Polyglot Approach - Explicit Named Operations:**

Polyglot philosophy: **Explicit over implicit**. Instead of comparison operators, use:

1. **Explicit Named Pipelines** (Preferred):
```polyglot
// ✓ CORRECT - Use explicit named operations
[Q] |Q.DispatchIf.RAM.MoreThan
[<] .ram_gb: pg\uint << 3

[Q] |Q.DispatchIf.String.IsEmpty
[<] .value: pg\string << .input_name

// Note: No need to check "IsNoError" - that's the default behavior.
// If no error occurs, execution continues normally.
```

2. **Range Matching** (When necessary):
```polyglot
// ✓ CORRECT - Range operator with ?>
[?] .ram_gb ?> 2..5
[~][r] |HandleModerateRAM

[?] .age ?> 18..65
[~][r] |HandleAdult

[?] .age ?> 0..18
[~][r] |HandleMinor
```

3. **Boolean Pattern Matching**:
```polyglot
// ✓ CORRECT - Pattern match with ?>
[?] .should_greet ?> True
[~][r] |GreetUser

[?] .should_greet ?> False
[~][r] |SkipGreeting

// OR with Default
[?] .should_greet ?> True
[~][r] |GreetUser

[?] Default
[~][r] |SkipGreeting
```

4. **Enumeration Matching**:
```polyglot
// ✓ CORRECT - Match enumeration values
[?] .status ?> \\Status.Success\\
[~][r] |HandleSuccess

[?] .status ?> \\Status.Failed\\
[~][r] |HandleFailure

[?] Default
[~][r] |HandleUnknown
```

5. **Error Handling** (Use `[!]` NOT `[?]`):
```polyglot
// ✓ CORRECT - Catch errors with [!] block
[r] |RiskyOperation
[<] .data: pg\string << .input
[~]
[~][!] !Error.Network.Timeout
[~][<] .timeout: pg\dt << DT"5m"
[~][r] |HandleTimeout
[~]
[~][!] !Error.FileSystem.NotFound
[~][r] |HandleFileNotFound

// Note: No need to handle "no error" state -
// if no error occurs, execution continues normally
```

**Critical Error Handling Rules:**
- Errors use `[!]` block, NOT `[?]` switch
- Backslash literals `\\..\\` are ONLY for `pg\path` and `pg\url` types
- There is NO `\\NoError\\` literal - absence of error doesn't need handling
- Error types are based on reserved enumeration: `!Error.Category.Type`
- Only catch errors that actually occurred; "success" path continues normally

**Why No Comparison Operators?**
1. **Readability:** `|Q.DispatchIf.RAM.MoreThan` is clearer than `>= 3`
2. **Discoverability:** Named operations are self-documenting
3. **Consistency:** Aligns with Polyglot's explicit pipeline-based design
4. **Type Safety:** Named operations have strict type contracts
5. **Extensibility:** Easy to add domain-specific comparisons

**Missing Documentation for:**

1. **`[?]` Block Marker**
   - Purpose: Pattern matching / conditional branching
   - Syntax: `[?] expression ?> pattern` or `[?] expression comparison value`
   - Nesting with `[~]` expansion marker
   - Not documented in block-markers.md

2. **`?>` Match Operator**
   - Purpose: Pattern matching against value
   - Syntax: `.variable ?> pattern`
   - Used for boolean True/False matching
   - Used for enumeration matching
   - Not documented in operators.md

3. **Comparison Operators**
   - Operators: `==`, `!=`, `<`, `>`, `<=`, `>=`
   - Briefly mentioned in type-system.md (line 143)
   - **NOT documented in operators.md**
   - Used with `[?]` for conditional logic

4. **Range Operator `..`**
   - Rust-style range syntax
   - Example: `1..10`, `0..100`
   - Used for range matching in switch
   - Completely undocumented

5. **Exhaustive Matching**
   - **Compiler Rule:** All switch branches must cover total probability = 1
   - For booleans: Must have both True and False, OR Default
   - For comparisons: Must be exhaustive or have Default
   - For ranges: Must cover entire domain or have Default
   - **Not documented anywhere**

6. **Default/Else Pattern**
   - Catches all remaining cases
   - Syntax: TBD (needs documentation)
   - Required when branches aren't exhaustive
   - Not documented

**Correct Exhaustive Pattern (Boolean):**
```polyglot
// Option 1: Explicit both cases
[?] .condition ?> True
[~][r] .result: pg\string << "yes"
[~][o] .result: pg\string

[?] .condition ?> False
[~][r] .result: pg\string << "no"
[~][o] .result: pg\string

// Option 2: With Default
[?] .condition ?> True
[~][r] .result: pg\string << "yes"
[~][o] .result: pg\string

[?] Default
[~][r] .result: pg\string << "no"
[~][o] .result: pg\string
```

**Correct Exhaustive Pattern (Range):**
```polyglot
// Using range operator
[?] .age >= 0 && .age < 18
[~][r] .category: pg\string << "minor"
[~][o] .category: pg\string

[?] .age >= 18 && .age < 65
[~][r] .category: pg\string << "adult"
[~][o] .category: pg\string

[?] .age >= 65
[~][r] .category: pg\string << "senior"
[~][o] .category: pg\string

// OR with Rust-style ranges
[?] .age in 0..18
[~][r] .category: pg\string << "minor"
[~][o] .category: pg\string

[?] .age in 18..65
[~][r] .category: pg\string << "adult"
[~][o] .category: pg\string

[?] Default
[~][r] .category: pg\string << "senior"
[~][o] .category: pg\string
```

**Compiler Exhaustiveness Checking:**
```
Error: Non-exhaustive switch block
Pipeline: MyPipeline
Branches cover: True case only
Missing: False case or Default branch
Total probability: 0.5 (requires 1.0)
```

**Design Philosophy (Rust-inspired):**
1. **Exhaustive by default** - compiler enforces all cases covered
2. **Total probability = 1** - mathematical guarantee
3. **No implicit fallthrough** - each branch is explicit
4. **Type-safe matching** - patterns must match type
5. **Default as catchall** - explicit "else" behavior

**Files Affected:**
- `examples/01-hello-world.md` - Uses `[?]` in Examples 3, 4, and Pattern 3
- `language/06-block-markers.md` - Missing `[?]` documentation
- `language/05-operators.md` - Missing `?>`, `==`, `!=`, `..`, comparison operators
- `language/01-syntax-complete.md` - Missing switch/conditional section

**Fix Required:**
1. **Create new documentation file:** `language/XX-switch-conditionals.md`
   - Document `[?]` block marker
   - Document `?>` match operator
   - Document comparison operators: `==`, `!=`, `<`, `>`, `<=`, `>=`
   - Document range operator `..`
   - Document exhaustive matching rules
   - Document Default pattern
   - Show compiler exhaustiveness checking
   - Provide comprehensive examples

2. **Update `language/06-block-markers.md`:**
   - Add `[?]` - Switch/Conditional Block section
   - Cross-reference to switch-conditionals.md

3. **Update `language/05-operators.md`:**
   - Add "Comparison Operators" section with `==`, `!=`, `<`, `>`, `<=`, `>=`
   - Add "Match Operator `?>`" section
   - Add "Range Operator `..`" section
   - Show usage with `[?]` blocks

4. **Update `audit/quick-language-reference.md`:**
   - Add switch/conditional syntax to grammar
   - Add exhaustiveness rules to critical rules

5. **Fix all examples:**
   - Ensure all switch blocks are exhaustive
   - Add comments explaining exhaustiveness
   - Show both explicit and Default patterns

**Priority:** CRITICAL - This is a fundamental control flow mechanism that is completely missing from the specification.

---

### Violation #8: String Interpolation vs Multiline Continuation Confusion

**Severity:** HIGH
**Category:** Incorrect Syntax / Documentation Gap
**Compiler Rule:** `+"text"` is for multiline continuation, NOT string interpolation

**Description:**
The `+"` prefix is used for **multiline string continuation**, not for string interpolation. Regular strings use `{variable}` syntax for interpolation without any prefix.

**Invalid Code Pattern:**
```polyglot
// ✗ WRONG - Using +"" for interpolation
[r] .greeting: pg\string << +"Hello, {user_name}!"
```

**Correct Code Patterns:**
```polyglot
// ✓ CORRECT - Regular string interpolation
[r] .greeting: pg\string << "Hello, {user_name}!"

// ✓ CORRECT - Multiline continuation with +"
[r] .report: pg\string << +"This is a very long string "
[^] +"that continues on multiple lines "
[^] +"with interpolation: {user_name}"
```

**Rules:**
1. **String interpolation**: Use `{variable}` inside regular strings
2. **Multiline continuation**: Use `+"text"` followed by `[^] +"text"` for continuation
3. **Both together**: Multiline strings can contain `{variable}` interpolation

**Files Affected:**
- All documentation showing string usage
- Examples with string formatting

**Fix Required:**
- Document `+"` as multiline continuation marker, not interpolation
- Show correct string interpolation syntax with `{variable}`
- Add to quick-language-reference.md

---

### Violation #9: Variable Immutability - Attempting Reassignment

**Severity:** CRITICAL
**Category:** Type System Violation
**Compiler Rule:** Variables are immutable by default; reassignment is NOT allowed

**Description:**
All variables in Polyglot are **immutable by default**. Once a variable is assigned a value, it cannot be reassigned. Using `pg.mutable\type` for mutable variables is highly discouraged and may be banned entirely.

**Invalid Code Pattern:**
```polyglot
// ✗ INVALID - Cannot reassign immutable variable
[r] .age_category: pg\string << ""
[~][r] .age_category: pg\string << "adult"  // COMPILER ERROR
```

**Correct Code Pattern:**
```polyglot
// ✓ CORRECT - Use switch to assign value based on condition
[?] .user_age ?> 18..
[~][r] .age_category: pg\string << "adult"
[~][o] .age_category: pg\string

[?] .user_age ?> ..17
[~][r] .age_category: pg\string << "minor"
[~][o] .age_category: pg\string
```

**Rationale:**
1. **Immutability by default** - Functional programming principle
2. **Thread safety** - No shared mutable state
3. **Predictability** - Variables don't change unexpectedly
4. **Explicit over implicit** - Use switches for branching logic

**Note on Mutable Types:**
- `pg.mutable\type` exists but is **highly discouraged**
- May be banned entirely in future versions
- Use functional patterns instead

**Files Affected:**
- All examples showing variable assignment
- Documentation on type system

**Fix Required:**
- Document immutability as core language principle
- Show switch-based patterns for conditional assignment
- Add to quick-language-reference.md CRITICAL RULES

---

### Violation #10: Missing `[~]` Expansion for Error Blocks

**Severity:** CRITICAL
**Category:** Ambiguous Syntax
**Compiler Rule:** `[!]` error blocks MUST have `[~]` expansion to clarify scope

**Description:**
Error handling blocks using `[!]` MUST be wrapped in `[~]` expansion markers to explicitly show which pipeline operation's errors are being caught. Without this, it's ambiguous which operation the error handler applies to.

**Invalid Code Pattern:**
```polyglot
// ✗ INVALID - Ambiguous error scope
[r] |U.File.Read
[r] |U.File.Write
[!] !pg.FileSystem.NotFound  // Which operation's error?
```

**Correct Code Pattern:**
```polyglot
// ✓ CORRECT - Explicit error scope with [~]
[r] |U.File.Read
[<] .path: pg\path << .file_path
[>] .content: pg\string >> file_data
[~]
[~][!] !pg.FileSystem.NotFound
[~][>] .message: pg\string >> err_msg
[~][r] |U.Log.Warning
[~][<] .msg: pg\string << "File not found: {err_msg}"
[~][r] .file_data: pg\string << "default content"
```

**Formatting Rule:**
- Must have level-1 `[~]` blank line before `[!]` block
- All error handling code must be nested under `[~][!]`

**Rationale:**
1. **Clarity** - Explicitly shows which operation can fail
2. **Scoping** - Clear boundaries for error handling
3. **Maintainability** - Easy to identify error handling code

**Files Affected:**
- All examples with error handling
- Error handling documentation

**Fix Required:**
- Update all error handling examples to use `[~]` expansion
- Add formatting rule to formatting-rules.md
- Add to quick-language-reference.md

---

### Violation #11: Missing Blank Lines Before Block Elements

**Severity:** MEDIUM
**Category:** Formatting / Readability
**Formatting Rule:** Level-1 `[~]` blank line required before pipeline/error/switch blocks

**Description:**
For improved readability and explicit scoping, a blank line using `[~]` (expansion marker without content) must precede:
- Pipeline calls with `[r]` or `[p]`
- `[!]` error handlers
- `[?]` switch blocks

**Invalid Code Pattern:**
```polyglot
// ✗ INVALID - No blank line before error block
[r] |U.File.Read
[!] !pg.FileSystem.NotFound
```

**Correct Code Pattern:**
```polyglot
// ✓ CORRECT - Blank [~] line before error block
[r] |U.File.Read
[<] .path: pg\path << .file_path
[>] .content: pg\string >> file_data
[~]  // Blank line for readability
[~][!] !pg.FileSystem.NotFound
```

**Formatting Rules:**
1. Add `[~]` blank line before `[!]` error blocks
2. Add `[~]` blank line before `[?]` switch blocks
3. Add `[~]` blank line before major `[r]` pipeline sections

**Files Affected:**
- All examples with error handling or switches
- Formatting rules documentation

**Fix Required:**
- Document formatting rule in formatting-rules.md
- Apply consistently across all examples
- Add to style guide

---

### Violation #12: Queue Control `[Q]` Misuse for Conditional Logic

**Severity:** HIGH
**Category:** Incorrect Usage
**Compiler Rule:** `[Q]` is for queue management, NOT conditional logic

**Description:**
The `[Q]` queue control marker is for managing pipeline execution queues and dispatch, NOT for implementing conditional logic or comparisons. Use utility pipelines like `|U.String.IsEmpty` instead.

**Invalid Code Pattern:**
```polyglot
// ✗ INVALID - Using [Q] for conditionals
[Q] |Q.DispatchIf.String.IsEmpty
[<] .value: pg\string << .username
[~][r] .error: pg\string << "Username required"
```

**Correct Code Pattern:**
```polyglot
// ✓ CORRECT - Use utility pipelines for checks
[r] |U.String.IsEmpty
[<] .value: pg\string << .username
[>] .is_empty: pg\bool >> username_empty

[?] username_empty ?> True
[~][r] .error: pg\string << "Username required"
[~][o] .error: pg\string
```

**What is `[Q]` Actually For:**
- Queue assignment: `[Q] |Q.Queue.Assign`
- Priority control: `[Q] |Q.PriorityBump`
- Pause/Resume: `[Q] |Q.Pause`, `[Q] |Q.Resume`
- Pipeline journey control (post-trigger)

**Files Affected:**
- Quick language reference (incorrect examples)
- Documentation showing `[Q]` usage

**Fix Required:**
- Remove all `|Q.DispatchIf.*` examples
- Document correct `[Q]` usage for queue management only
- Show utility pipeline patterns for conditionals
- Update quick-language-reference.md

---

### Violation #13: Non-Exhaustive Switch Patterns

**Severity:** CRITICAL
**Category:** Type Safety Violation
**Compiler Rule:** All switch branches must be exhaustive (total probability = 1)

**Description:**
Switch statements using `[?]` must cover ALL possible values (exhaustive matching). The compiler enforces that branches cover the entire domain with total probability = 1.

**Invalid Code Pattern:**
```polyglot
// ✗ INVALID - Not exhaustive (age > 200 not covered)
[?] .user_age ?> 18..200
[~][r] .age_category: pg\string << "adult"

[?] .user_age ?> 0..17
[~][r] .age_category: pg\string << "minor"
```

**Correct Code Patterns:**
```polyglot
// ✓ CORRECT - Unbounded ranges for exhaustiveness
[?] .user_age ?> 18..
[~][r] .age_category: pg\string << "adult"
[~][o] .age_category: pg\string

[?] .user_age ?> ..17
[~][r] .age_category: pg\string << "minor"
[~][o] .age_category: pg\string

// ✓ CORRECT - Using Default catchall
[?] .user_age ?> 0..17
[~][r] .age_category: pg\string << "minor"
[~][o] .age_category: pg\string

[?] .user_age ?> 18..65
[~][r] .age_category: pg\string << "adult"
[~][o] .age_category: pg\string

[?] Default
[~][r] .age_category: pg\string << "senior"
[~][o] .age_category: pg\string
```

**Range Operator Syntax:**
- `..N` = Less than or equal to N (unbounded below)
- `N..` = Greater than or equal to N (unbounded above)
- `N..M` = Between N and M inclusive
- `Default` = Catchall for remaining cases

**Compiler Error:**
```
Error: Non-exhaustive switch block
Pipeline: ProcessUserData
Branches cover: 0..200 (integers > 200 not covered)
Missing: Default branch or unbounded range
Total probability: 0.95 (requires 1.0)
```

**Files Affected:**
- All examples with switch statements
- Switch/conditional documentation

**Fix Required:**
- Update all switches to be exhaustive
- Document unbounded range syntax `..N` and `N..`
- Document `Default` pattern
- Add compiler exhaustiveness checking to documentation
- Add to quick-language-reference.md CRITICAL RULES

---

### Violation #14: Enumeration Definition - Alias Position

**Severity:** LOW
**Category:** Syntax Order
**Compiler Rule:** `[A]` alias must come AFTER `[#]` namespace definition

**Description:**
When defining enumerations, the alias `[A]` marker must appear AFTER the full namespace path `[#]` definition, not before.

**Invalid Code Pattern:**
```polyglot
// ✗ INVALID - Alias before namespace (if this were the syntax)
[A] OrderStatus
[#] Shipping.ProductA.Order.Status
[<] .Pending: pg\int << 0
[X]
```

**Correct Code Pattern:**
```polyglot
// ✓ CORRECT - Namespace first, then alias
[#] Shipping.ProductA.Order.Status
[A] OrderStatus
[<] .Pending: pg\int << 0
[<] .Shipped: pg\int << 1
[<] .Delivered: pg\int << 2
[<] .Cancelled: pg\int << 3
[X]

// Usage: Can use #OrderStatus (alias) instead of full path
[?] .status ?> #OrderStatus.Shipped
```

**Files Affected:**
- Enumeration definition examples
- Type system documentation

**Fix Required:**
- Document correct enumeration structure
- Show alias usage in examples
- Add to quick-language-reference.md

---

## Violation Summary

| Violation | Severity | Count | Status |
|-----------|----------|-------|--------|
| #1 Missing Trigger | CRITICAL | ALL examples (~20+ instances) | Pending Fix |
| #2 Missing Wrapper | CRITICAL | ~15 instances | Partially Fixed |
| #3 Invalid Trigger Syntax | CRITICAL | Example 3 | Pending Fix |
| #4 Missing `[^]` Syntax | HIGH | Not documented | Pending |
| #5 `[o]` with Assignment | CRITICAL | ~15+ instances | Pending Fix |
| #6 Inconsistent Branch Outputs | CRITICAL | Example 4 | Pending Fix |
| #7 Undocumented Switch/Conditionals | CRITICAL | Entire system | Pending Doc |
| #8 String Interpolation Confusion | HIGH | Documentation | Pending Fix |
| #9 Variable Reassignment | CRITICAL | Multiple examples | Pending Fix |
| #10 Missing `[~]` for `[!]` | CRITICAL | All error examples | Pending Fix |
| #11 Missing Blank Lines | MEDIUM | All examples | Pending Fix |
| #12 `[Q]` Misuse | HIGH | Quick reference | Pending Fix |
| #13 Non-Exhaustive Switches | CRITICAL | Multiple examples | Pending Fix |
| #14 Enum Alias Position | LOW | Documentation | Pending Fix |

**Total Critical Violations**: 8
**Total High Violations**: 3
**Total Medium Violations**: 1
**Total Low Violations**: 1

---

## Action Items

### Immediate (Critical)
- [x] Create audit folder structure
- [x] Move decision files to audit/
- [x] Create code-violations-log.md
- [ ] Fix all missing wrapper declarations in examples
- [ ] Document `[^]` line continuation syntax
- [ ] Document `+""` string continuation syntax

### High Priority
- [ ] Create comprehensive quick-language-reference.md
- [ ] Document all formatting rules in formatting-rules.md
- [ ] Add strict validation rules to syntax documentation

### Medium Priority
- [ ] Review all Phase 1-3 documentation for violations
- [ ] Update documentation-plan.md with new audit requirements
- [ ] Add formatting validation to checklist

---

## Documentation Updates Required

### Files to Create/Update

1. **`language/01-syntax-complete.md`**
   - Add CRITICAL RULE: All pipelines MUST have `[t]`, `[i]`, `[W]`, or `{[\], [/]}`
   - Document `[^]` line continuation
   - Document `+""` string continuation

2. **`language/06-block-markers.md`**
   - Add `[^]` line continuation block element
   - Explain usage and purpose

3. **`language/05-operators.md`**
   - Add `+""` string continuation operator
   - Explain explicit concatenation vs automatic

4. **`audit/quick-language-reference.md`**
   - Comprehensive grammar for code generation
   - All block elements
   - All operators
   - Type syntax
   - Required elements checklist

5. **`audit/formatting-rules.md`**
   - Blank line rules
   - Indentation rules (if any)
   - Comment placement
   - Code organization

---

## Lessons Learned

1. **Always validate against complete grammar before generating code**
2. **Create quick reference FIRST before writing examples**
3. **Missing critical syntax requirements leads to invalid examples**
4. **Formatting rules should be explicit, not assumed**
5. **Need systematic validation process for all generated code**

---

## Next Steps

1. Complete quick-language-reference.md by scanning all v0.0.2 docs
2. Complete formatting-rules.md
3. Fix all violations in hello-world examples
4. Create validation checklist for future examples
5. Continue with data processing examples (with validation)