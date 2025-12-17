<!-- ARCHIVED: 2025-12-16 | Reason: Historical context, not current specification | Superseded by: Current specifications in /language/ and /features/ -->

# Additional Polyglot Syntax Improvement Ideas

**Date:** 2025-12-10
**Status:** Brainstorming - Not yet prototyped
**Context:** Building on v0.0.4 indentation syntax success

---

## Quick Wins (Easy, High Impact)

### 1. Simplify Nested I/O Syntax

**Current (v0.0.4):**
```polyglot
[p] ~ForEach
   <array << .numbers
   >item >> .number

   [r] |DoubleValue
      <value << .number
      >result >> .doubled
```

**Proposed Alternative A - Drop the direction marker:**
```polyglot
[p] ~ForEach
   array << .numbers      // < is implied by <<
   item >> .number        // > is implied by >>

   [r] |DoubleValue
      value << .number
      result >> .doubled
```

**Rationale:**
- `<<` already shows input direction
- `>>` already shows output direction
- The `<` and `>` prefixes are redundant
- Cleaner, less visual noise

---

### 2. Infer Types More Aggressively

**Current:**
```polyglot
[<] .name:pg.string
[<] .count:pg.int
[<] .items:pg.array{pg.string}
```

**Proposed - Optional types:**
```polyglot
[<] .name            // Infer from caller or require explicit pass
[<] .count <~ 0      // Infer pg.int from default value
[<] .items           // Infer from first assignment or usage
```

**When to require types:**
- Public API boundaries (exported pipelines)
- When type can't be inferred
- When developer wants to be explicit

**Benefits:**
- Less boilerplate for internal pipelines
- Faster prototyping
- Still type-safe (inference, not dynamic)

---

### 3. Simplify Pipeline Calls

**Current:**
```polyglot
[r] |DoubleValue
   value << .number
   result >> .doubled
```

**Proposed - Function-style syntax:**
```polyglot
[r] .doubled << |DoubleValue(.number)
```

**Or keep current for multi-output:**
```polyglot
[r] |DoubleValue
   value << .number
   result >> .doubled
   errors >> .errs      // Multiple outputs need explicit syntax
```

**Rule:**
- Single input + single output: Function style
- Multiple inputs/outputs: Block style
- Developer chooses what's clearer

---

### 4. Optional Curly Braces for Registry Close

**Current:**
```polyglot
{@} @Local::MyApp:1.0.0.0
[A] @App
{x}

{|} |Pipeline.Example
[<] .input
[>] .output
{x}
```

**Proposed - Infer close from next section:**
```polyglot
{@} @Local::MyApp:1.0.0.0
[A] @App

{|} |Pipeline.Example
[<] .input
[>] .output

{|} |Pipeline.Next     // Previous pipeline closes implicitly
...
```

**Last pipeline still needs explicit:**
```polyglot
{|} |Pipeline.Last
[<] .input
[>] .output
{x}                    // Required: EOF close
```

**Benefit:** Less noise, clearer section boundaries

---

## Medium Impact (Moderate Effort)

### 5. Named Parameters in Pipeline Calls

**Current:**
```polyglot
[r] |ProcessData
   input << .rawData
   config << .settings
   mode << "strict"
```

**Proposed - Named params:**
```polyglot
[r] |ProcessData(
   input: .rawData
   config: .settings
   mode: "strict"
)
   result >> .output
```

**Or single-line for simple calls:**
```polyglot
[r] .output << |ProcessData(input: .rawData, mode: "strict")
```

**Benefits:**
- Clearer parameter intent
- Familiar from other languages
- Better for refactoring

---

### 6. Simplified Boolean Logic in Conditionals

**Current:**
```polyglot
[y] .age >? 65
\&\ .is_member =? #Boolean.True
   [r] .discount << 20
```

**Proposed - Inline operators:**
```polyglot
[y] .age >? 65 && .is_member
   [r] .discount << 20
```

**Mapping:**
- `\&\` → `&&`
- `\|\` → `||`
- `\^\` → `^^` (XOR)
- Boolean values: `#Boolean.True` → `true`, `#Boolean.False` → `false`

**Rationale:**
- More familiar to developers
- Already using `?` suffix for comparisons
- Shorter and cleaner

---

### 7. Multi-line Strings and Block Comments

**Current - Only single-line:**
```polyglot
// Comment line 1
// Comment line 2
[r] .sql << "SELECT * FROM users WHERE active = true"
```

**Proposed:**
```polyglot
/*
 * Block comment
 * Multiple lines
 */
[r] .sql << """
   SELECT * FROM users
   WHERE active = true
   AND created > '2024-01-01'
"""
```

**Triple-quote rules:**
- Preserves indentation relative to closing `"""`
- First line break after opening `"""` is stripped
- Common indentation stripped (like Python textwrap.dedent)

---

### 8. Shorthand for Common Patterns

**Current - Default value:**
```polyglot
[<] .timeout:pg.int <~ 30
```

**Proposed - Type inference from default:**
```polyglot
[<] .timeout <~ 30     // Infer pg.int
```

**Current - Pipeline with no special setup:**
```polyglot
{|} |Pipeline.Simple
[<] .input
[t] |T.Call
[W] |W.Polyglot.Scope
[r] .output << .input
[>] .output
{x}
```

**Proposed - Minimal syntax:**
```polyglot
{|} |Pipeline.Simple(.input) -> .output
[t] |T.Call

[r] .output << .input
{x}
```

**When this works:**
- Single input, single output
- No wrapper needed (Polyglot.Scope is default)
- Trigger still explicit

---

## Bigger Ideas (More Discussion Needed)

### 9. Destructuring in ForEach

**Current:**
```polyglot
[p] ~ForEach
   array << .users
   item >> .user

   [r] .name << .user.name
   [r] .email << .user.email
```

**Proposed:**
```polyglot
[p] ~ForEach
   array << .users
   item >> {.name, .email}    // Destructure into variables

   [r] .greeting << "Hello {.name}"
```

**Benefits:**
- Less boilerplate
- Clearer intent
- Common pattern in modern languages

---

### 10. Pipeline Composition Operator

**Current:**
```polyglot
[r] |FetchData
   url << .endpoint
   data >> .raw

[r] |ParseData
   input << .raw
   parsed >> .result

[r] |ValidateData
   input << .result
   validated >> .final
```

**Proposed - Pipe operator:**
```polyglot
[r] .final << .endpoint
   |> |FetchData
   |> |ParseData
   |> |ValidateData
```

**Or with parameters:**
```polyglot
[r] .final << .endpoint
   |> |FetchData(timeout: 30)
   |> |ParseData(format: "json")
   |> |ValidateData
```

**Rationale:**
- Elixir/F# style composition
- Natural data flow
- Reduces intermediate variables

---

### 11. Pattern Matching Instead of Exhaustive Conditionals

**Current:**
```polyglot
[y] .status =? "pending"
   [r] .color << "yellow"

[y] .status =? "complete"
   [r] .color << "green"

[y] .status =? "failed"
   [r] .color << "red"

[y] *?
   [r] .color << "gray"
```

**Proposed - Match expression:**
```polyglot
[r] .color << match .status
   "pending" => "yellow"
   "complete" => "green"
   "failed" => "red"
   _ => "gray"
```

**Benefits:**
- More concise
- Familiar from Rust/ML languages
- Expression-oriented (returns value)
- Less repetition

---

### 12. Range Operator Simplification

**Current:**
```polyglot
[y] .temperature ?[20, 25]      // Closed interval
   [r] .description << "comfortable"

[y] .temperature ?(25, 35)      // Open interval
   [r] .description << "warm"
```

**Proposed - Use familiar syntax:**
```polyglot
[y] .temperature >= 20 && .temperature <= 25
   [r] .description << "comfortable"

// Or introduce 'in' keyword:
[y] .temperature in 20..25      // Inclusive
   [r] .description << "comfortable"

[y] .temperature in 25..<35     // Exclusive end
   [r] .description << "warm"
```

**Rationale:**
- Rust/Swift style ranges
- More intuitive than `?[` syntax
- Could support any comparable type

---

## Controversial / Discussion Required

### 13. Keywords for Common Operations

**Current - No keywords:**
```polyglot
[<] .input
[>] .output
[t] |T.Call
[W] |W.Polyglot.Scope
```

**Proposed - Optional keyword syntax:**
```polyglot
input .data:pg.string
output .result:pg.string
trigger Call
wrapper Polyglot.Scope
```

**Tradeoffs:**
- Pro: More readable for non-programmers
- Pro: Closer to natural language
- Con: Loss of consistency (operators vs keywords)
- Con: May be harder to parse
- Con: Reduces language's unique character

**Verdict:** Probably not worth it - operators are a feature

---

### 14. Eliminate Reserved Symbol Prefixes

**Current:**
```polyglot
.variable        // Variables
#Enumeration     // Types/Enums
|Pipeline        // Pipelines
!Error          // Errors/Exceptions
```

**Proposed - Use naming conventions:**
```polyglot
myVariable       // Variables (camelCase)
MyEnum           // Types (PascalCase)
MyPipeline       // Pipelines (PascalCase)
MyError          // Errors (PascalCase)
```

**Analysis:**
- Pro: More familiar to mainstream developers
- Pro: Less syntactic noise
- Con: Loss of visual distinctiveness (can't grep for `|Pipeline`)
- Con: Namespace collisions (is `Data` a type or variable?)
- Con: Parser complexity (more context-sensitive)

**Verdict:** Keep prefixes - they're a valuable feature for scanning and tooling

---

### 15. JSON/YAML-like Syntax for Structs

**Current:**
```polyglot
{#} #PersonRecord
[.] .name:pg.string
[.] .age:pg.int
{x}

[r] .person:#PersonRecord << #PersonRecord
[r] .person.name << "Alice"
[r] .person.age << 30
```

**Proposed - Literal syntax:**
```polyglot
{#} #PersonRecord
[.] .name:pg.string
[.] .age:pg.int
{x}

[r] .person << #PersonRecord {
   name: "Alice"
   age: 30
}
```

**Benefits:**
- Familiar from JSON/YAML
- More concise
- Clearer structure

---

## Summary & Recommendations

### Immediate (Bundle with v0.0.4)
1. ✅ **Simplify nested I/O** - Drop redundant `<` and `>` prefixes
2. ✅ **Inline boolean operators** - `&&`, `||` instead of `\&\`, `\|\`
3. ✅ **Infer types from defaults** - `.timeout <~ 30` infers `pg.int`

### Next Version (v0.0.5)
4. **Named parameters** in pipeline calls
5. **Match expressions** for cleaner pattern matching
6. **Multi-line strings** with triple quotes
7. **Struct literals** for easier struct construction

### Future Exploration
8. **Pipeline composition** operator `|>`
9. **Destructuring** in loops and assignments
10. **Range operators** like Rust (`20..25`)

### Not Recommended
- ❌ Keywords (lose consistency)
- ❌ Remove symbol prefixes (lose greppability)

---

## Next Steps

1. **Gather feedback** on quick wins
2. **Prototype** 1-2 improvements alongside indentation
3. **Measure impact** on readability and learning curve
4. **Iterate** based on real usage

**Question for you:** Which improvements resonate most? Should we prototype any of these?
