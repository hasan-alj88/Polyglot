# Lexer & Parser Design Session Summary

**Date:** 2025-11-26
**Session Focus:** Lexer tokenization strategy, pattern trees, and documentation updates
**Status:** Complete ✅

---

## Session Overview

This session focused on clarifying the lexer's role in Polyglot, establishing clear tokenization patterns, and documenting comprehensive pattern trees for all block markers. Key decisions were made about string literal handling, error detection boundaries, and system extensibility.

---

## Key Decisions Made

### 1. String Literal Tokenization Strategy ✅

**Question:** Should string literals be tokenized as single structured tokens or multiple tokens?

**Decision:** **Token sequence with interpolation tokenization** (UPDATED)

**Pattern:**
```
Input: "Count: {.num:Hex}"

Lexer output:
  → STRING_START
  → STRING_CONTENT("Count: ")
  → INTERPOLATION_START
  → IDENTIFIER_VARIABLE(".num")
  → DELIMITER_COLON
  → FORMAT_IDENTIFIER("Hex")
  → INTERPOLATION_END
  → STRING_END
```

**Rationale:**
- **CRITICAL UPDATE:** Interpolations `{.variable:format}` MUST be tokenized by lexer
- Clean separation: Lexer handles syntax, Parser handles semantics
- Better error reporting: Lexer can detect unterminated interpolations
- Simpler parser: Receives structured tokens, not raw string content
- Consistent with block marker strategy: `[X]` is one token, `{...}` is token sequence

**User Quote (Initial):**
> "The pattern is <pipeline identifier> + \" + <string literal> + \" as far the lexer does this all it need to parse it is up to the complier to figure out the other stuff"

**User Clarification (Final Decision):**
> "string literals are of the pattern {pipeline}+\"+Formatted_argument_string+\"
>
> the formatted argment will contain any number of {value\or .variable: format identifers}
>
> so those need to be tokized also"

**New Token Types Added:**
- `STRING_START` - Opening quote `"`
- `STRING_CONTENT` - Static text between interpolations
- `STRING_END` - Closing quote `"`
- `INTERPOLATION_START` - Opening brace `{`
- `INTERPOLATION_END` - Closing brace `}`
- `FORMAT_IDENTIFIER` - Format specifier (e.g., `Hex`, `Currency`)

**Token Count Updated:** 95 → 100 token types

---

### 2. Empty Input Declaration Syntax ✅

**Old Pattern:**
```polyglot
[i] #Pipeline.NoInput
```

**New Pattern (UPDATED):**
```polyglot
[i]                    // Empty - no inputs
```

**Rationale:**
- Simpler for lexer (just `BLOCK_INPUT` + `NEWLINE`)
- Easier for users (no special identifier needed)
- More intuitive ("no inputs" = empty line)

---

### 3. Macro Include Pattern ✅

**New Pattern Added:**
```polyglot
[<] Macro.MustInclude"path/with\/escaped/slashes"
```

**Token sequence:**
```
[<] → IDENTIFIER(Macro)
    → DOT
    → IDENTIFIER(MustInclude)
    → STRING_LITERAL("path...")
```

**Context:** Within `[@]` package scope
**Purpose:** Include required macro files

---

### 4. Lexer vs Parser vs Compiler Boundaries ✅

**Clear separation established:**

| Component | Responsibility | Example |
|-----------|----------------|---------|
| **Lexer** | Tokenize patterns | Recognize `[r]` as `BLOCK_SEQUENTIAL` |
| **Parser** | Validate structure | Ensure `[i]` comes before `[t]` |
| **Compiler** | Validate semantics | Check pipeline exists, infer types |

**Error Detection:**

| Error Type | Detected By | Example |
|------------|-------------|---------|
| Unknown block marker `[z]` | **Lexer** | Invalid character in block marker |
| Missing block marker | **Parser** | Statement doesn't start with block |
| Missing pipeline definition | **Compiler** | Called pipeline doesn't exist |
| Type mismatch | **Compiler** | Wrong type passed to pipeline input |

---

### 5. Block Marker Tokenization ✅

**How block markers work:**

**State Machine:**
```
INITIAL state:
  See '[' → Enter IN_BLOCK_MARKER state

IN_BLOCK_MARKER state:
  Read character (e.g., 'r', '?', '<')
  See ']' → Emit single BLOCK_* token
  Return to INITIAL state
```

**Key insight:** Each `[X]` is ONE token, not three (`[` + `X` + `]`)

**Example:**
```
Input: [r] .variable

Tokens:
  → BLOCK_SEQUENTIAL    // [r] as one token
  → IDENTIFIER_VARIABLE // .variable
```

---

### 6. Comparison Operator Tokenization ✅

**Strategy:** Longest match wins

**Order matters:**
```rust
// Check 3-char operators first
if chars == "=!?" → OPERATOR_NOT_EQUAL
if chars == "=>?" → OPERATOR_GREATER_EQUAL
if chars == "=<?" → OPERATOR_LESS_EQUAL

// Then 2-char operators
if chars == "=?" → OPERATOR_EQUAL
if chars == ">?" → OPERATOR_GREATER
if chars == "<?" → OPERATOR_LESS
```

**Critical:** Must check longer operators before shorter ones to avoid tokenizing `=!?` as `=` + `!` + `?`

---

## Documentation Updates Completed

### 1. Format Identifier Catalog ✅
**File:** `/docs/user/standard-library/utilities-catalog.md`
**Added:** 200+ lines documenting format pipelines
**Contents:**
- Integer formats (Hex, Binary, Decimal, Ordinal, etc.)
- Float formats (Currency, Percent, Scientific, etc.)
- DateTime formats (ShortDate, LongDate, ISO8601, etc.)
- Boolean formats (YesNo, OnOff, TrueFalse, etc.)
- Path formats (Basename, Directory, Extension, etc.)
- Custom format pipeline examples
- Type mismatch handling

---

### 2. String Literal Processing Architecture ✅
**File:** `/docs/technical/architecture.md`
**Added:** 470+ lines documenting complete architecture
**Contents:**
- Overview and rationale
- Syntax forms (plain, explicit, parameterized, interpolated)
- Pipeline signature requirements
- Format resolution algorithm
- 5-step processing workflow
- Auto-await behavior
- Type mismatch error handling
- Bootstrap formatters (base case)
- Compiler implementation requirements
- Runtime implementation requirements
- Performance considerations
- Edge cases

---

### 3. Empty String Rationale ✅
**File:** `/docs/technical/string-literals-internals.md`
**Added:** Explanation of why empty string is mandatory

**Key addition:**
> "Because inline pipeline calls are invoked via string literal syntax. The string - even if empty - is the syntactic marker that distinguishes:
> - `DT.Now""` (inline pipeline call - VALID)
> - `DT.Now` (bare pipeline reference - INVALID)
>
> The empty string `""` serves as the mandatory invocation operator for inline pipeline calls."

---

### 4. Type Mismatch FAQ ✅
**File:** `/docs/technical/string-literals-internals.md`
**Updated:** Fixed existing FAQ entry (compile error, not runtime error)
**Added:** New FAQ about type mismatches

**Key FAQ:**
> **Q: What if I use the wrong format for a type (e.g., `{.string_var:Hex}`)?**
>
> **A: Compile error - missing pipeline definition.** Since format pipelines are namespaced by type, using `Hex` (which expects `pg\int`) on a `pg\string` variable will result in the compiler looking for `|U.String.Polyglot.String.Hex`, which doesn't exist.

---

### 5. Complete Pattern Trees ✅
**File:** `/docs/project/examples/LEXER-PATTERN-TREES.md` (NEW)
**Size:** 1000+ lines
**Contents:**
- Comprehensive pattern tree for all 25 block markers
- Context annotations for each pattern
- Token-by-token expectations
- VALUE_EXPRESSION expansion
- All operator definitions
- Context summary table
- Error detection patterns
- Extensibility guidance
- Version history

**Example pattern:**
```
[<] → Context 1: Package Import (within [@] scope)
      │
      └─→ @
          → IDENTIFIER (import alias)
          → <<
          → IDENTIFIER (registry)
          → @
          → IDENTIFIER (package name)
          → :
          → VERSION
          → NEWLINE
```

---

## Key Technical Insights

### 1. Operator Prefixes Enable Unambiguous Tokenization
- `.` prefix → Variables
- `#` prefix → Enumerations
- `|` prefix → Pipelines
- `!` prefix → Errors
- `~` prefix → Unpack/Join operators

**Benefit:** Lexer can immediately classify identifiers without context.

---

### 2. Context Determines Pattern Meaning
Some block markers have different meanings in different contexts:

| Block | Context 1 | Context 2 | Context 3 | Context 4 |
|-------|-----------|-----------|-----------|-----------|
| `[#]` | File number (in `[@]`) | Enum definition (top-level) | — | — |
| `[<]` | Import (in `[@]`) | Macro (in `[@]`) | Enum field (in `[#]`) | Binding (in parent) |

**Implication:** Parser must track context to interpret tokens correctly.

---

### 3. Block Markers Provide Predictable Structure
Every Polyglot construct starts with a block marker, giving the parser clear entry points for structure validation.

**Pipeline structure (STRICT ORDER):**
```
[|] → [i] → [t] → [Q]? → [W] → execution blocks → [o] → [X]
```

**Benefit:** Parser always knows what to expect next, enabling precise error messages.

---

### 4. String Literals Are NOT Primitives
**Critical architectural decision:** String literals are inline pipeline calls.

**Pattern:**
```
"text" → U.String"text"
DT.Now"" → calls |DT.Now pipeline
"{.var:Hex}" → interpolation triggers format pipeline
```

**Implication:** All formatting goes through pipelines, making the system uniform and extensible.

---

## Extensibility Analysis

### Easy Changes (Lexer Level)

✅ **Adding block markers:** Trivial (1 line in switch)
✅ **Adding operators:** Easy (pattern matching)
✅ **Removing block markers:** Easy (emit error or warning)
✅ **Adding token types:** Easy (enum variant + logic)

**Estimate:** Hours, not days

---

### Medium Changes (Lexer Level)

⚠️ **Changing operator precedence:** Reorder checks, test extensively
⚠️ **Adding context-sensitive tokens:** Track state, validate contexts
⚠️ **Extending string handling:** New state machine paths

**Estimate:** Days, not weeks

---

### Hard Changes (Lexer Level)

❌ **Fundamental syntax changes:** Breaks core design
❌ **Removing operator prefixes:** Breaks unambiguous tokenization
❌ **Making whitespace significant:** Fundamental strategy change

**Estimate:** Weeks or months (avoid if possible)

---

**User Quote on Evolution:**
> "My question is can add\remove patterns as we Polyglot evolue? is easy change or difficalt?"

**Answer:** **Easy for most changes** at the lexer level. Polyglot's design (operator prefixes, block markers, no keywords) makes additive changes straightforward.

---

## Implementation Readiness

### Lexer Implementation ✅
**Status:** Ready to implement
**Documents:**
- Complete token enumeration (95 tokens)
- Pattern trees for all block markers
- State machine hints
- Error detection patterns
- Extensibility guidance

**Next steps:**
1. Implement Rust lexer with token enum
2. Implement state machine (INITIAL, IN_STRING, IN_BLOCK_MARKER, etc.)
3. Add error detection and reporting
4. Write comprehensive tests

---

### Parser Implementation 🔜
**Status:** Patterns documented, ready to design
**Documents:**
- Block marker sequences known
- Context requirements clear
- Structure validation patterns documented

**Next steps:**
1. Design AST structure
2. Implement recursive descent parser
3. Add context tracking
4. Implement structure validation
5. Write comprehensive tests

---

### Compiler Implementation 🔜
**Status:** Architecture documented, awaiting lexer/parser
**Documents:**
- String literal processing workflow
- Format resolution algorithm
- Type inference requirements
- Error handling patterns

**Next steps:**
1. Design IR (Intermediate Representation)
2. Implement type checker
3. Implement string literal processing
4. Implement format pipeline resolution
5. Generate executable code

---

## Statistics

### Documentation Created/Updated (Session 1)
- **Files created:** 2
- **Files updated:** 6
- **Total lines added:** ~2,000+
- **Pattern trees documented:** 25 block markers
- **Contexts documented:** 30+ distinct contexts

### Documentation Updated (Session 2 - String Literal Tokenization)
- **Files created:** 1 (`STRING-LITERAL-TOKENIZATION-STRATEGY.md`)
- **Files updated:** 3 (Token spec, Pattern trees, Session summary)
- **Total lines added:** ~900+
- **New token types added:** 6 (string interpolation tokens)
- **Token count updated:** 95 → 100 token types

### Session Metrics
- **Duration:** ~3 hours (including follow-up session)
- **Decisions made:** 6 major decisions + 1 critical clarification
- **Questions resolved:** 10+ technical questions
- **Examples provided:** 60+ code examples

---

## Pending Decisions

### 1. Crate Organization ⏳
**Question:** Where should base case formatters live?
- **Option A:** Include in `polyglot-runtime-wrappers` crate
- **Option B:** Separate `polyglot-formatters` crate

**Recommendation:** Option B (separate crate) for clean separation
**Status:** User preference pending

---

### 2. Nested Interpolation 🔜
**Question:** Should nested interpolation be supported?
```polyglot
"{.name} at {DT.Now\"\"}"    // DT.Now inside interpolation
```
**Status:** To be determined (likely disallowed for simplicity)

---

### 3. Escape Sequences in Interpolation 🔜
**Question:** How to write literal braces?
```polyglot
"\{.var\}"                  // Literal {.var} in output?
```
**Status:** Standard escape rules likely apply (`\{` = literal brace)

---

## Next Steps

### Immediate (Week 1)
1. ✅ Document pattern trees (DONE)
2. ✅ Update all documentation (DONE)
3. 🔜 Review with team/users
4. 🔜 Begin Rust lexer implementation

### Short-term (Weeks 2-4)
1. 🔜 Implement complete lexer with all tokens
2. 🔜 Write lexer test suite
3. 🔜 Begin parser design
4. 🔜 Define AST structure

### Medium-term (Months 2-3)
1. 🔜 Implement parser
2. 🔜 Implement type checker
3. 🔜 Implement string literal processing
4. 🔜 Begin code generation

---

## Final Update: String Literal Tokenization Clarification

### Critical Change

After initial documentation, user provided critical clarification that changed the lexer tokenization strategy:

**User's Clarification:**
> "string literals are of the pattern {pipeline}+\"+Formatted_argument_string+\"
>
> the formatted argment will contain any number of {value\or .variable: format identifers}
>
> so those need to be tokized also"

### Impact

This clarification means:
1. **Lexer responsibility expanded** - Must tokenize interpolations, not leave as raw string
2. **New tokens added** - STRING_START, STRING_CONTENT, STRING_END, INTERPOLATION_START, INTERPOLATION_END, FORMAT_IDENTIFIER
3. **State machine updated** - Added IN_STRING and IN_INTERPOLATION states
4. **Token count increased** - From 95 to 100 token types
5. **Parser simplified** - Receives structured tokens, not raw string content

### Documentation Updates Applied

1. **Created** `/docs/project/examples/STRING-LITERAL-TOKENIZATION-STRATEGY.md` (~500 lines)
   - Complete tokenization strategy
   - 9 comprehensive examples
   - State machine details
   - Error detection patterns
   - Implementation checklist

2. **Updated** `/docs/project/examples/LEXER-TOKEN-SPECIFICATION.md`
   - Added 6 new token types
   - Updated Section 14.1 with tokenization details
   - Updated state machine (Section 19)
   - Updated token enumeration (Section 20)
   - Updated token count: 95 → 100

3. **Updated** `/docs/project/examples/LEXER-PATTERN-TREES.md`
   - Updated VALUE_EXPRESSION expansion
   - Added complete "String Literal Tokenization Pattern" section (~300 lines)
   - 8 tokenization examples
   - State machine details
   - Error detection patterns

4. **Updated** This session summary document
   - Updated Decision #1 with final strategy
   - Updated statistics
   - Added this clarification section

### Result

**Status:** Lexer tokenization strategy finalized and fully documented ✅

The lexer implementation can now proceed with complete specifications for:
- All 100 token types
- State machine with 5 states (INITIAL, IN_STRING, IN_INTERPOLATION, IN_COMMENT, IN_BLOCK_MARKER)
- String literal tokenization with interpolation support
- Complete pattern trees for all constructs
- Error detection patterns

---

## References

### Primary Documents
- [Lexer Pattern Trees](project/examples/LEXER-PATTERN-TREES.md) - Complete pattern reference
- [Lexer Token Specification](project/examples/LEXER-TOKEN-SPECIFICATION.md) - Token enumeration (100 tokens)
- [String Literal Tokenization Strategy](project/examples/STRING-LITERAL-TOKENIZATION-STRATEGY.md) - Interpolation tokenization (NEW)
- [String Literals Internals](technical/string-literals-internals.md) - Processing mechanics
- [Architecture](technical/architecture.md) - System design

### AI Context Package
- [Grammar](ai-context/grammar.ebnf) - Formal EBNF grammar
- [Constraints](ai-context/constraints.yaml) - Validation rules
- [Operators](ai-context/operators.json) - Complete operator reference
- [Examples](ai-context/examples-annotated.pg) - Canonical patterns

---

## Team Feedback

### From Party Mode Agents

**Paige (Technical Writer):**
> "The format catalog and FAQ additions make the documentation complete. Developers now have everything they need to understand string literals."

**Winston (Architect):**
> "The lexer/parser/compiler boundary is now crystal clear. The pattern trees provide a perfect implementation roadmap."

**Amelia (Developer):**
> "With these patterns documented, I can start implementing the Rust lexer immediately. The token-by-token expectations eliminate ambiguity."

**Bob (Scrum Master):**
> "All documentation tasks complete. Ready to move into implementation phase. Excellent progress!"

---

## Conclusion

This session successfully:
- ✅ Clarified lexer tokenization strategy
- ✅ Established clear component boundaries (lexer/parser/compiler)
- ✅ Documented comprehensive pattern trees for all constructs
- ✅ Updated all relevant documentation
- ✅ Assessed system extensibility
- ✅ Prepared for implementation phase

**Overall Status:** Documentation phase complete, ready for lexer implementation.

---

**Document Status:** Session Summary ✅
**Last Updated:** 2025-11-26
**Session Lead:** Claude Code with User Guidance
