# Brainstorming Backlog

**Maintained By**: Mai (Secretary)
**Purpose**: Track topics requiring brainstorming sessions
**Last Updated**: 2025-11-21

---

## Pending Brainstorming Sessions

### ✅ 7. Primitive Polyglot Data Types - Complete Type System with State Machine

**Status**: COMPLETED (Brainstorming Session 2025-11-29)
**Session Results**: `docs/Agile/brainstorming-session-results-2025-11-29.md`
**Priority**: CRITICAL - BLOCKING Epic 2 (Type System & Runtime Foundation)
**Assigned To**: Carson (Brainstorming), Winston (Architecture), Bob (Story Prep)
**Context**: Epic restructuring - discovered need for type system before IR generation

**Triggered By**:
- Epic 2 planning (IR Generation & Validation)
- Winston identified that IR types reference undefined Polyglot primitives
- User decided to implement full type system foundation before IR work
- Epic renumbering: New Epic 2 = Type System, Epic 3 = IR Generation (formerly Epic 2)

**Core Problem**:
Cannot define IR (Intermediate Representation) types without knowing:
- What Polyglot primitive types exist
- How they map to Rust types
- How the variable state system works
- How async type resolution happens

**Critical Architectural Insight (2025-11-29)**:
**Polyglot is an ASYNC language with VARIABLE STATE SYSTEM**
- Types are NOT simple value containers
- Types are STATE MACHINES that transition through states
- Must fully leverage Rust's enum for state modeling

---

#### Type System Requirements

**1. Variable States (Confirmed)**:
All Polyglot variables transition through states:
- `Declared` - Variable declared with language and type info
- `Default` - Default value assigned
- `Ready` - Actual value available
- `Fetching` - Async fetch in progress (from file, API, etc.)
- `Computing` - Async computation in progress
- `Error` - Error state with error details
- **UNKNOWN**: Waiting? Locked? Stale? (need clarification)

**2. Primitive Types List (Confirmed)**:
```
1. pg\int      - Integer numbers (i64 in Rust)
2. pg\float    - Floating-point numbers (f64 in Rust)
3. pg\string   - Text strings (UTF-8)
4. pg\bool     - Boolean values
5. pg\path     - File system paths (PathBuf in Rust)
6. pg\dt       - DateTime (complex type - see below)
```

**~~pg\duration~~ REMOVED** - Duration is a SUBTYPE of `pg\dt`, not separate primitive

**3. pg\dt Complex Type System**:
User clarified: *"pg\dt have a lot of subtypes that capture recurrence\cycles, duration, AM\PM\Hr, different calendars"*

**DateTime Subtypes Required**:
- `Instant` - Specific point in time (with calendar support)
- `Duration` - Time span with units: s/m/h/d/w/mo/y
- `TimeOfDay` - Hour/minute/second with 12hr/24hr format, AM/PM support
- `Date` - Calendar date only
- `Recurrence` - Repeating patterns (daily, weekly, monthly, yearly, cron)
- `Cycle` - Cyclical periods with phase
- `Calendar` - Support multiple calendar systems (Gregorian minimum, Islamic, Hebrew, Chinese, etc.)

**Duration Units**:
From user examples: `"30s"`, `"5m"`, `"2h"`, `"1d"`, `"1w"`, `"1mo"`, `"1y"`
- Seconds (s)
- Minutes (m)
- Hours (h)
- Days (d)
- Weeks (w)
- Months (mo)
- Years (y)

**TimeOfDay Formats**:
- 24-hour format
- 12-hour format with AM/PM
- Example from trigger: `T.DT.Daily"3:00AM"`

**Recurrence Patterns**:
- Daily at specific time
- Weekly on specific day
- Monthly on specific date
- Yearly on specific date
- Interval-based (every N minutes/hours)
- Cron expression support

---

#### Rust Implementation Architecture

**Two Design Options Proposed**:

**Option A: Generic State Wrapper (Recommended by Winston)**
```rust
pub enum VariableState<T> {
    Declared { language: Language, type_name: String },
    Default { value: T },
    Ready { value: T },
    Fetching { source: DataSource },
    Computing { operation: Operation },
    Error { error: PgError },
}

pub type PgInt = VariableState<i64>;
pub type PgString = VariableState<String>;
pub type PgDateTime = VariableState<DateTimeValue>;
```
**Pros**: Less code duplication, uniform state transitions
**Cons**: Less type-specific customization

**Option B: Per-Type State Enums**
```rust
pub enum PgInt {
    Declared { language: Language, type_name: String },
    Default { value: i64 },
    Ready { value: i64 },
    Fetching { source: DataSource },
    // ...
}

pub enum PgString {
    Declared { language: Language, type_name: String },
    Default { value: String },
    Ready { value: String },
    // ...
}
```
**Pros**: Type-specific behaviors, more flexibility
**Cons**: Code duplication, harder to maintain consistency

---

#### Open Questions (CRITICAL - Need User Input)

**From Winston (Architect)**:
1. **Complete state list** - What are ALL variable states beyond Declared/Default/Ready/Fetching/Computing/Error?
   - Waiting? (for dependencies)
   - Locked? (in use by another task)
   - Stale? (needs refresh)
   - Other?

2. **Generic vs Per-Type** - Which implementation approach?
   - Option A: Generic `VariableState<T>` (Winston recommends)
   - Option B: Per-type enums `PgInt`, `PgString`, etc.

**From Amelia (Developer)**:
3. **Async future storage** - How to persist `Fetching` state?
   - Futures aren't `Serialize` - can't store in state enum
   - Solution: Don't store futures, manage externally?
   - State just tracks "Fetching" status, actual async work managed by runtime?

4. **Language field meaning** - What does `Declared { language: Language }` represent?
   ```rust
   pub enum Language {
       Polyglot,    // Native Polyglot
       Python,      // From Python wrapper
       Rust,        // From Rust wrapper
       // ...
   }
   ```
   Or is it something else?

**From Mary (Business Analyst)**:
5. **State transition triggers** - What causes state changes in practice?
   - Example: When does `Declared` → `Fetching` happen?
   - Who orchestrates state transitions - Parser? IR? Runner?

**From Carson (Brainstorming)**:
6. **Backward state transitions** - Can variables go backwards?
   - `Ready` → `Fetching` (re-fetch fresh data)
   - `Ready` → `Computing` (re-compute)
   - Or are transitions uni-directional?

7. **Compound states** - Can variables have multiple states simultaneously?
   - Example: `Ready` (has value) AND `Stale` (needs refresh)
   - Or mutually exclusive states only?

---

#### Context from Party Mode Discussion

**Participants**: User (hhj), Winston, Bob, Mary, Amelia, Carson, Mai (Party Mode 2025-11-29)

**Key Decisions Made**:
1. ✅ New Epic 2: Type System & Runtime Foundation (blocks old Epic 2)
2. ✅ Full type system (not minimal) - rich domain types
3. ✅ Bare minimum base programs: Python wrapper, Rust wrapper, trigger programs
4. ✅ Blocking strategy - Epic 2 must complete before Epic 3 (IR Generation)
5. ✅ Six primitive types: int, float, string, bool, path, dt
6. ✅ Duration is subtype of dt (not separate primitive)
7. ✅ State machine pattern for all types

**Epic 2 Story Breakdown (Planned)**:
- Story 2.1: Primitive Type Definitions (Full System) - 7-10 days
- Story 2.2: Type Conversion & Serialization Layer
- Story 2.3: Base Program Trait & Registry Architecture
- Story 2.4: Python Runtime Wrapper (Base Implementation)
- Story 2.5: Rust Runtime Wrapper (Base Implementation)
- Story 2.6: Trigger Base Programs (Time, Resource)
- Story 2.7: Type System Integration Tests

**Story 2.1 Complexity**:
- Originally estimated: 5-7 days (simple types)
- Revised estimate: 7-10 days (state machine + datetime complexity)
- Recommendation: Consider splitting into 2.1a (state machine + basic types) and 2.1b (datetime)

**Examples from User**:
```polyglot
# Trigger with complex boolean logic
[t] |T.JSON.Fetch.Boolean
[<] .file: pg\path << \\FileDir\\settings.json
[<] .field: pg\string << "Triggers.manual_override"
[+][.]  // OR block, Boolean Group block
[~][t] T.DT.Daily"2:00AM"
[~][&] T.RAM.Avaliable.MoreThan.GB"3" // AND block

# Time trigger examples
T.DT.Daily"3:00AM"
T.RAM.Avaliable.MoreThan.GB"3"

# Queue timeout configuration
Timeout: pg\dt
#*.Timeout
-> Unlimited (enum field)
-> Limited (enum field)
   -> Timeout (pg\dt)
```

---

#### Brainstorming Session Scope

**What Needs to be Brainstormed**:

1. **Complete Variable State Machine**
   - Enumerate ALL states
   - Define valid state transitions
   - Identify transition triggers
   - Handle backward/compound states

2. **DateTime Type System Design**
   - Finalize all subtypes (Instant, Duration, TimeOfDay, Date, Recurrence, Cycle)
   - Calendar system support (which calendars for MVP?)
   - Parsing strategy for string literals (`"3:00AM"`, `"30s"`, etc.)
   - Recurrence pattern syntax (cron vs custom)

3. **Type Implementation Strategy**
   - Generic `VariableState<T>` vs per-type enums
   - Async state management (how to handle futures)
   - Serialization requirements (JSON for IR storage)
   - Error handling across state transitions

4. **Language/Runtime Integration**
   - What does `Declared { language: Language }` mean?
   - How do Python/Rust wrappers interact with type system?
   - Type conversion between languages

5. **Base Program Types**
   - What types do trigger programs need?
   - What types do wrapper programs need?
   - Minimum viable type set for Epic 2

---

#### Deliverables Required

**From Brainstorming Session**:
1. Complete variable state enumeration and state machine diagram
2. Definitive DateTime subtype specifications
3. Rust implementation architecture (generic vs per-type decision)
4. Type parsing specifications for all literal formats
5. State transition rules and triggers
6. Calendar system support matrix (MVP vs post-MVP)
7. Example code for all primitive types in all states

**For Story 2.1 Acceptance Criteria**:
1. All primitive types with state machine support
2. DateTime complex type with all subtypes
3. Type conversion and serialization layer
4. Unit tests for state transitions
5. Parsing logic for all literal formats
6. Error handling specifications

---

#### Related Artifacts

**Epic Documents**:
- `docs/Agile/epics.md` - Will be updated with new Epic 2
- `docs/Agile/stories/sprint-status.yaml` - Will be updated with Epic 2 stories

**Architecture Documents**:
- Type system architecture (TBD)
- State machine design patterns (TBD)
- DateTime specification (TBD)

**Referenced Syntax**:
- Block markers: `[t]`, `[+]`, `[&]`, `[.]`, `[~]` (already documented)
- Trigger programs: `T.DT.Daily`, `T.RAM.Available`, etc.
- Type syntax: `pg\int`, `pg\dt`, etc.

---

#### Next Actions

~~**Immediate**:~~
~~1. **Carson**: Schedule comprehensive brainstorming session for type system~~

**✅ COMPLETED (2025-11-29)**

---

#### Session Completion Summary

**Brainstorming Session Date**: 2025-11-29
**Technique Used**: First Principles Thinking
**Participants**: User (hhj), Carson (Elite Brainstorming Specialist)

**ALL 7 CRITICAL QUESTIONS ANSWERED**:

1. ✅ **Complete state list** → 5 states: Declared, Default, Ready, Error, Close
2. ✅ **Generic vs Per-Type** → Generic `VariableState` with Rust enums
3. ✅ **Async future storage** → Don't store futures; state changes when async completes
4. ⚠️ **Language field meaning** → Not fully addressed; needs follow-up
5. ✅ **State transition triggers** → Push (`<<`) and Pull (`>>`) operations; async completion
6. ✅ **Backward state transitions** → No; transitions are uni-directional (enforced by FSM)
7. ✅ **Compound states** → No; mutually exclusive states only

**KEY DESIGN DECISIONS**:

**Variable State Machine**:
- **5 States**: Declared → Default → Ready → Close (Error as failure path)
- **No intermediate states**: Removed Fetching/Computing (state changes on async completion)
- **Compile-time validation**: Invalid transitions are compile errors
- **Default Push restriction**: `<~` only valid on Declared state (not Default)
- **ONE override rule**: Default allows exactly one Push to Ready

**DateTime Type System**:
- **Structure**: `DateTime { pattern, value, type }`
- **DateTimeValue**: 7 variants enforcing "one or more of (Time, Date, DayOfWeek)"
- **Calendars (MVP)**: Gregorian, Julian, Assyrian, Islamic Civil, Islamic Observational
- **Profile System**: 3-tier priority (Manual Overrides → API Cache → ICU4X Calculated)
- **Relative Patterns**: "Last Friday in Ramadan", "2nd Sunday in June"
- **Equality Semantics**: Instant = exact match; Duration = membership test

**DateTime Operations**:
- Membership & Comparison: `contains`, `is_before`, `is_after`, `is_between`
- Interval Operations: `overlaps`, `intersection`, `union`, `gap`, `split`
- Arithmetic: `add_duration`, `subtract_duration`, `duration_until`
- Recurrence: `occurs_on`, `next_occurrence_after`, `all_occurrences_between`

**Technology Decisions**:
- **ICU4X Library**: Use `icu_calendar` crate for calendar conversions and day-of-week validation
- **Profile Format**: YAML-based calendar profiles with manual overrides
- **Serialization**: All values stored as strings with metadata

**MVP vs Post-MVP Scope**:
- **MVP**: Basic calendars (Gregorian, Julian, Assyrian, Islamic Civil), basic relative dates
- **Post-MVP**: Hebrew/Chinese/Buddhist profiles, Panchang API, advanced recurrence rules

**DELIVERABLES CREATED**:
1. ✅ Complete state machine with transition table
2. ✅ DateTime subtype specifications
3. ✅ Rust implementation architecture design
4. ✅ Calendar system support matrix
5. ✅ State transition rules documented
6. ✅ Example Polyglot code with all states
7. ✅ 25+ core design decisions across all areas

**READY FOR IMPLEMENTATION**:
- Priority #1: Variable State Machine (Story 2.1)
- Priority #2: DateTime Core with ICU4X (Story 2.1)
- Priority #3: Islamic Calendar Profiles (Post-Story 2.1)

**FOLLOW-UP SESSIONS RECOMMENDED**:
1. Relative Date Pattern Syntax Design (lexer/parser implications)
2. Profile System Architecture Deep Dive
3. Duration Arithmetic Semantics
4. Error Recovery and Edge Cases
5. Time Zone and DST Handling

**Next Actions**:
1. **Bob (SM)**: Update Story 2.1 acceptance criteria with state machine design
2. **Winston (Architect)**: Review brainstorming results and validate architecture
3. **Amelia (Dev)**: Begin Story 2.1 implementation with state machine
4. **Carson**: Schedule follow-up sessions for remaining open questions

**After Brainstorming**:
1. Mai: Document all decisions in decision log
2. Paige: Create formal type system specification document
3. Winston: Update architecture.md with type system design
4. Bob: Finalize Story 2.1 with complete acceptance criteria
5. Mai: Update sprint-status.yaml with Epic 2 breakdown

---

**BLOCKING**: Epic 3 (IR Generation) cannot proceed until this is resolved.

**DEPENDENCIES**:
- Epic 2 Stories 2.2-2.7 depend on Story 2.1
- Epic 3 entirely depends on Epic 2 completion

---

---

## Completed Brainstorming Sessions

### ✅ 2. Polyglot Formatting Guidelines (PFG) - Style Guide & Syntax Highlighting

**Status**: COMPLETED
**Completed Date**: 2025-11-21
**Session Document**: `docs/brainstorming-session-results-2025-11-21.md`
**ITIL Ticket**: SR-2025-002

**Outcome**:
- ✅ Complete PFG specification: `docs/Tech/implementation/technical/polyglot-formatting-guidelines-v1.0.md`
- ✅ PFG-001: Code Layout & Style (3-character rule, no indentation, spacing rules)
- ✅ PFG-002: Naming Conventions (.snake_case, |CamelCase, #CamelCase, !CamelCase)
- ✅ PFG-003: Syntax Highlighting Specification (VS Code Dark+ color scheme, 9 categories)
- ✅ PFG-004: Editor Integration Standards (LSP Tier 1/2/3 roadmap, code snippets)
- ✅ PFG-005: Tooling Standards (polyglot-tools, polyglot.toml, numbered error codes)
- ✅ PFG-006: Documentation Generation (from block markers + comments)
- ✅ PFG-007: Code Scaffolding (standard templates)
- ✅ Visual color preview: `docs/polyglot-syntax-darkmode-preview.html`

**Key Decisions**:
- NO indentation philosophy - block markers have dual duty (scope + relationships)
- 3 blank lines between file-scope definitions
- 1 blank line before branch points
- Context-aware line length (comments: 79, code: 99, strings: 120)
- VS Code Dark+ Python color scheme as standard
- Unified `polyglot-tools` binary (Rust-based)
- TOML-only configuration format

**Brainstorming Techniques Used**:
1. Mind Mapping (visual design, spacing, naming)
2. First Principles Thinking (why no indentation, why 3 blank lines)
3. SCAMPER Method (alternatives exploration)
4. Morphological Analysis (comprehensive coverage)
5. Six Thinking Hats (convergent synthesis)

**Next Steps**: Implement `polyglot-tools` prototype, TextMate grammar, LSP server

---

### ✅ 1. `[s]` Serial Load Block - Complete Specification

**Status**: COMPLETED
**Completed Date**: 2025-11-19
**Session Document**: `docs/Agile/brainstorming-session-results-2025-11-19.md`
**ITIL Tickets**: SR-2025-001 (Documentation), SR-2025-003 (Story Creation)

**Outcome**:
- ✅ Complete syntax: `[s] .variable: type << Format"path"`
- ✅ Parallel execution with automatic join (three-step process: collect paths → load parallel → assign)
- ✅ Error-carrying variables (value OR error with details)
- ✅ Two-level error handling: variable-level (`.var.error`) and scope-level (`[s][!]`)
- ✅ Shared error scope (one handler for all [s] at same scope/level)
- ✅ Partial success model (successful loads complete even when others fail)
- ✅ Variable state: Success = data + !No.Output, Failure = #None.ErrorState + specific error
- ✅ Wildcard/array loading with combination strategies (FilenameKey, Index, Merge, Concat, FlatMap)
- ✅ Chained literal pipelines (NEW FEATURE): `JSON.FilenameKey"path".ExcludeFileName"*test*"`
- ✅ Reserved enumeration validation (!Serial.ReservedEnumeration.* errors)
- ✅ Filter syntax (chained ExcludeFileName)
- ✅ Complete examples for all use cases
- ✅ MVP scope clearly defined

**Key Decisions**:
- Parallel-first architecture for file loading
- Error resilience with partial success
- Type safety at runtime for reserved enumerations
- Extensibility via chained literal pipelines
- Fail-fast philosophy (empty files = errors)

**MVP Scope**:
- Basic file loading (JSON, YAML, TOML, XML)
- Parallel execution with automatic join
- Two-level error handling
- Wildcard/array loading
- Combination strategies (5 types)
- Chained literal pipelines
- Reserved enumeration validation

**Post-MVP** (Future):
- Security (path traversal, permissions)
- Caching, streaming, remote loading
- Compression/encryption

**Related**: Resolves brainstorming backlog item #1 (HIGH priority)

---

### ✅ 5. Error Handling Philosophy - !No.Output Explicit Checks

**Status**: COMPLETED
**Completed Date**: 2025-11-18
**Session Document**: `docs/Agile/agent-sessions/carson-2025-11-18-error-handling-philosophy.md`

**Outcome**:
- ✓ **Philosophy Confirmed**: "Success is implicit and preferred; explicit error checking is allowed for special cases" (Hybrid Approach)
- ✓ `!No.Output` is fully accessible (reserved enumeration with error handling responsibilities)
- ✓ Can be compared, assigned, caught, and output (but typically not needed)
- ✓ Default behavior is implicit success (continuation after error handling)
- ✓ Best practice: use implicit continuation, explicit checks only for special cases
- ✓ Error state after catching: caller has `!No.Output` unless it raises new error
- ✓ Callee that raised error keeps its error state
- ✓ Error aggregation patterns documented (3 patterns provided)
- ✓ Complete examples for all patterns

**Key Decisions**:
- `[~][!] !Error.Type` is the syntax for catching errors (scoped to previous block)
- `[o] !Error.Type` raises an error
- `[o] !No.Output` is valid but redundant (implicit default)
- `[~][!] !No.Output` can catch success case (useful for critical operation logging)
- Prefer implicit success (90% of cases), allow explicit for edge cases (10%)

**Best Practices Established**:
1. Use implicit continuation for success path
2. Catch specific errors only
3. Use error aggregation for multiple validations (extract to array or use boolean flag)
4. Catch `!No.Output` only for meaningful cases (audit logging, critical operations)
5. Avoid checking `!No.Output` explicitly unless necessary

**Related**: Resolves audit concerns about `!No.Output` usage philosophy

---

### ✅ 3. URL Literals - Type System & Syntax

**Status**: COMPLETED
**Completed Date**: 2025-11-18
**Session Document**: `docs/Agile/agent-sessions/carson-2025-11-18-url-literals-spec.md`

**Outcome**:
- ✅ `pg\url` defined as distinct type (like `pg\path`)
- ✅ Four URL literal variants specified: `url`, `urlencoded`, `urlraw`, `urltemplate`
- ✅ Seven reserved fields: `.protocol`, `.domain`, `.port`, `.path`, `.query`, `.anchor`, `.full`
- ✅ Query parameter parsing: `.query` returns `pg\serial` with key-value pairs
- ✅ Runtime protocol validation (not compile-time)
- ✅ Automatic port defaults by protocol
- ✅ String interpolation support in `urltemplate`
- ✅ Complete examples for all variants
- ✅ Standard library integration specified

**Key Decisions**:
- URLs use actual URL syntax (forward slashes) inside literals
- Type safety through distinct `pg\url` type
- Explicit encoding behavior through different literal variants
- Structured access via reserved fields

---

### ✅ 4. Comparison Operators & Range Notation - Syntax Finalization

**Status**: COMPLETED
**Completed Date**: 2025-11-18
**Session Document**: `docs/Agile/agent-sessions/carson-2025-11-18-comparison-operators-design.md`

**Outcome**:
- ✅ Comparison operators DO EXIST in Polyglot
- ✅ Complete operator family defined: `>?`, `<?`, `=>?`, `=<?`, `=?`, `=!?`
- ✅ Range operators defined: `?[a, b]`, `?(a, b)` with mathematical interval notation
- ✅ Pattern matching: `*?` wildcard, `re?` regex
- ✅ Complete boolean logic: `[&]`, `[+]`, `[-]`, `[^]`, `[.]`
- ✅ Implicit AND in trigger blocks documented
- ✅ Type compatibility matrix established
- ✅ Exhaustive matching rules documented
- ✅ Deprecated: `?>` match operator, `Default` keyword, `..` range operator
- ✅ Story 1.2 UNBLOCKED

**Related Tickets**: PRB-2025-001 (Resolved), INC-2025-001

---

### ✅ 6. Undocumented Syntax Features - Complete Specification

**Status**: COMPLETED (Merged with Item #4)
**Completed Date**: 2025-11-18
**Session Documents**:
- `docs/Agile/agent-sessions/carson-2025-11-18-comparison-operators-design.md`
- `docs/Agile/agent-sessions/carson-2025-11-18-line-continuation-spec.md`

**Outcome**:
- ✅ `[*]` line continuation block fully specified
- ✅ `+"` string concatenation operator defined
- ✅ `[^]` confirmed as XOR operator (not line continuation)
- ✅ `[?]` switch system completely documented
- ✅ Exhaustive matching rules established
- ✅ Multiline string syntax: `[*]` + `+"`
- ✅ String interpolation: `{.variable}` for variables
- ✅ Deprecated: `?>` operator, `..` range operator, `Default` keyword

**Related Tickets**: PRB-2025-002 (Resolved)

---

### ✅ 2. `[M]` Macro Block - Complete Specification

**Status**: COMPLETED
**Completed Date**: 2025-11-18
**Session Document**: `docs/Agile/agent-sessions/carson-2025-11-18-macro-system-spec.md`

**Outcome**:
- ✅ Macros are compile-time inline code templates
- ✅ Blocks insert by TYPE, not position
- ✅ `[M]` Macro definition block specified
- ✅ `[W]` Macro unwrap (inline insertion)
- ✅ `[{]` Scope input (variables IN from caller)
- ✅ `[}]` Scope output (variables OUT to caller)
- ✅ `[=]` Constant input (replaces `Fixed` keyword)
- ✅ `Macro.include"<chars+"` declaration syntax
- ✅ Multiple macro ordering: FIFO setup, LIFO cleanup
- ✅ Type safety at unwrap site
- ✅ Variable renaming flexibility
- ✅ Complete examples (database, logging, cache, file handling)

**Key Decisions**:
- Scope flow with braces: `{` brings IN, `}` sends OUT
- No keywords: `[=]` replaces `Fixed`
- RAII-like resource management (last acquired, first released)

---

## Guidelines for Brainstorming Sessions

When conducting brainstorming session:

1. **Preparation**:
   - Review existing documentation
   - List specific questions to answer
   - Gather related examples from other languages/systems

2. **Session Structure**:
   - Define the problem/feature clearly
   - Explore multiple approaches
   - Evaluate trade-offs
   - Propose recommendation

3. **Deliverables**:
   - Comprehensive syntax specification
   - Code examples demonstrating usage
   - Edge cases and error handling
   - Integration with existing features
   - Documentation-ready output

4. **Recording**:
   - Carson records in `docs/Agile/agent-sessions/carson-sessions.md`
   - Mai aggregates into approved decisions
   - Update this backlog with status

---

**Process Owner**: Mai (Secretary)
**Facilitator**: Carson (Brainstorming Coach)

---

## New Pending Topics

### 8. Pipeline Formatted String Capability Definition

**Status**: PENDING
**Priority**: MEDIUM
**Added**: 2025-12-03
**Context**: String literal syntax change from `{Pipeline}"string"` to `|{Pipeline}"string"`

**Problem**:
Only specific pipelines should be allowed to accept formatted strings, but there's no mechanism to define/enforce this capability.

**Current Situation**:
```polyglot
// This works (should be allowed)
[r] .formatted: pg\string << |{U.String.Format}"Hello {.name}"

// Should this be allowed? How do we know?
[r] .result: pg\string << |{SomeRandomPipeline}"formatted {.string}"
```

**Questions to Explore**:
1. How do we declare that a pipeline accepts formatted strings?
2. Should this be part of the type signature?
3. Should this be an annotation/marker?
4. Should this be a naming convention?
5. Should this be an explicit compiler whitelist?

**Proposed Options**:

**Option 1: Type Signature with `pg\fstring`**
```polyglot
[|] |U.String.Format
[i] <template: pg\fstring     // Formatted string type
[i] <args: pg\dict
[o] >result: pg\string
```

**Option 2: Pipeline Annotation**
```polyglot
[|] |U.String.Format
[@] FormattedStringSupport    // Capability annotation
[i] <template: pg\string
[o] >result: pg\string
```

**Option 3: Namespace/Naming Convention**
- Pipelines ending in `.Format` accept formatted strings
- Certain namespaces (U.String.*, U.Log.*) automatically support it

**Option 4: Explicit Compiler Whitelist**
- Hardcoded list of stdlib pipelines that support formatted strings
- User pipelines must opt-in somehow

**Option 5: Pipeline Capability Flags**
```polyglot
[|] |MyCustomFormatter
[!] Capabilities.FormattedString    // Capability declaration
[i] <input: pg\string
```

**Dependencies**:
- String literal syntax update (completed)
- Type system design
- Pipeline signature design

**Related To**:
- Runtime environment specification (2025-12-03)
- String literal interpolation
- Type safety

**Assigned To**: TBD (needs brainstorming session)

**Success Criteria**:
- Clear mechanism for declaring formatted string support
- Compile-time enforcement
- Easy for stdlib and user pipelines
- Consistent with Polyglot design philosophy

---

### 9. Runtime Environment Implementation

**Status**: SPECIFICATION COMPLETE
**Priority**: HIGH
**Added**: 2025-12-03
**Specification**: `docs/Agile/runtime-environments-specification-2025-12-03.md`

**Summary**:
Comprehensive controlled runtime architecture for Shell (systemd-run), Python (uv), and Rust (toolchain + workspace) with resource management, queue integration, and guaranteed cleanup.

**Status**: Ready for implementation

---

### 10. Rust Runtime Environment - Execution Model & Code Handling

**Status**: PENDING
**Priority**: HIGH
**Added**: 2025-12-04
**Context**: Runtime environments specification (2025-12-03) - Rust section needs design decisions

**Background**:
Rust runtime specification defines the architecture (toolchain + workspace + systemd control) but several critical design questions remain unresolved regarding how Rust code is executed within Polyglot pipelines.

**Current Design**:
```polyglot
[W] |W.RT.Rust.Stable
[<] <manifest: pg\path << \\FileDir\\Cargo.toml

[r] .result: pg\string << |RT.Rust.Run"
fn main() {
    let value = {.input};
    println!(\"Result: {}\", value * 2);
}
"
```

**Session Structure**:
```polyglot
[#] #Sessions.Rust
[<] .id: pg\string
[<] .toolchain: pg\string       // stable, beta, nightly, 1.75.0
[<] .target_dir: pg\path
[<] .workspace_path: pg\path
[X]
```

**Build Modes**:
```polyglot
[#] #Rust.Mode
[<] .Debug
[<] .Release
[<] .ReleaseOptimized
[X]
```

---

#### Critical Questions Needing Resolution

**1. Inline Code Execution Model**

**Question**: How should inline Rust code snippets be handled?

**Options**:

**A) Automatic main() Wrapping**
```polyglot
// User writes:
[r] .result: pg\string << |RT.Rust.Run"
let value = {.input};
println!(\"Result: {}\", value * 2);
"

// System generates:
fn main() {
    let value = {.input};
    println!(\"Result: {}\", value * 2);
}
```
- ✅ Simpler for users (no boilerplate)
- ✅ Consistent with other runtimes (Python, Shell)
- ❌ Less flexible for advanced use cases

**B) Require Explicit main()**
```polyglot
// User must write full program:
[r] .result: pg\string << |RT.Rust.Run"
fn main() {
    let value = {.input};
    println!(\"Result: {}\", value * 2);
}
"
```
- ✅ Explicit, no magic
- ✅ Supports multiple functions
- ❌ More boilerplate for simple cases

**C) Smart Detection**
```polyglot
// Auto-detect: has main()? Use as-is. No main()? Wrap it.
```
- ✅ Best of both worlds
- ❌ Complex logic, potential confusion

**D) Expression Mode vs Program Mode**
```polyglot
// Expression mode (auto-wrapped)
[r] .result: pg\string << |RT.Rust.Expr"value * 2"

// Program mode (explicit main)
[r] .result: pg\string << |RT.Rust.Run"
fn main() {
    println!(\"Result: {}\", value * 2);
}
"
```
- ✅ Explicit intent
- ✅ Supports both use cases
- ❌ Two different pipelines

**Sub-questions**:
- Should we support helper functions outside main()?
- How do we handle `use` statements?
- What about module structure?

---

**2. Dependency Management**

**Question**: How are Rust dependencies handled in inline/runtime code?

**Current Design**:
```polyglot
[W] |W.RT.Rust.Stable
[<] <manifest: pg\path << \\FileDir\\Cargo.toml
```

**Options**:

**A) Manifest-Based Dependencies**
```polyglot
// Cargo.toml in workspace
[dependencies]
serde = "1.0"
tokio = "1.35"

// Code can use dependencies
[r] .result: pg\string << |RT.Rust.Run"
use serde_json::json;
let data = json!({\"value\": {.input}});
println!(\"{}\", data);
"
```
- How: `|Setup.RT.Rust` runs `cargo fetch` during setup
- ✅ Full Cargo ecosystem support
- ✅ Explicit dependency declaration
- ❌ Requires external Cargo.toml file

**B) Inline Dependencies**
```polyglot
[r] .result: pg\string << |RT.Rust.Run"
//! dependencies:
//! serde_json = \"1.0\"

use serde_json::json;
let data = json!({\"value\": {.input}});
"
```
- How: Parse special comments, generate Cargo.toml dynamically
- ✅ Self-contained code blocks
- ✅ No external files needed
- ❌ Non-standard Rust syntax
- ❌ Complex parsing

**C) Standard Library Only**
```polyglot
// Only std library available
[r] .result: pg\string << |RT.Rust.Run"
use std::collections::HashMap;
let mut map = HashMap::new();
map.insert(\"key\", {.value});
"
```
- ✅ Simple, predictable
- ✅ Fast compilation
- ❌ Very limited functionality

**D) Precompiled Workspace Model**
```polyglot
// User provides complete Cargo workspace with dependencies
// |RT.Rust.Run executes specific binary/function from workspace
[W] |W.RT.Rust.Stable
[<] <workspace: pg\path << \\FileDir\\rust-workspace
[<] <binary: pg\string << "data_processor"

[r] .result: pg\string << |RT.Rust.Call"process_data"
[<] <input: pg\string << .data
```
- ✅ Full Rust project capabilities
- ✅ Proper testing, modules, etc.
- ✅ Build caching benefits
- ❌ More complex setup
- ❌ Different execution model

**Sub-questions**:
- How are dependencies cached across sessions?
- Should we cache compiled artifacts?
- How do we handle dependency version conflicts?

---

**3. Build vs Run Separation**

**Question**: Should compilation and execution be separate pipelines?

**Options**:

**A) Combined Build+Run (Current)**
```polyglot
[r] .result: pg\string << |RT.Rust.Run"
fn main() {
    println!(\"Hello\");
}
"
// Compiles and runs in one operation
```
- ✅ Simple API
- ❌ Can't inspect build errors separately
- ❌ Can't cache compilation

**B) Separate Build and Run**
```polyglot
// First: Build
[r] .binary: #Rust.Binary << |RT.Rust.Build"
fn main() {
    println!(\"Hello\");
}
"

// Then: Run (potentially multiple times)
[r] .result1: pg\string << |RT.Rust.Execute""
[<] <binary: #Rust.Binary << .binary
[<] <args: pg\array << ["arg1", "arg2"]

[r] .result2: pg\string << |RT.Rust.Execute""
[<] <binary: #Rust.Binary << .binary
[<] <args: pg\array << ["other", "args"]
```
- ✅ Reuse compiled binary
- ✅ Separate compile vs runtime errors
- ✅ Better caching
- ❌ More complex API

**C) Hybrid with Cache**
```polyglot
// First run: compiles and runs
[r] .result: pg\string << |RT.Rust.Run"code"

// Subsequent runs: cached binary (if code unchanged)
[r] .result2: pg\string << |RT.Rust.Run"code"  // Uses cache
```
- ✅ Simple API
- ✅ Performance benefits
- ❌ Cache invalidation complexity

**Sub-questions**:
- How long do cached artifacts live?
- How is cache invalidation handled?
- Can users explicitly control caching?

---

**4. Library vs Binary Execution**

**Question**: Should Rust runtime support libraries or only binaries?

**Options**:

**A) Binary-Only (Current)**
```polyglot
// Must have main()
[r] .result: pg\string << |RT.Rust.Run"
fn main() {
    println!(\"Output\");
}
"
```
- ✅ Simple model (stdout/stderr capture)
- ❌ Limited to program execution

**B) Library with Function Calls**
```polyglot
// Call specific functions
[r] .result: pg\int << |RT.Rust.Call"calculate_sum"
[<] <input: pg\array << [1, 2, 3, 4, 5]

// Rust code defines library:
pub fn calculate_sum(input: Vec<i64>) -> i64 {
    input.iter().sum()
}
```
- ✅ Direct value return (not just stdout)
- ✅ Better type safety
- ✅ Reusable functions
- ❌ Complex FFI/serialization layer
- ❌ Need to define calling convention

**C) Mixed Model**
```polyglot
// Binary execution (stdout capture)
[r] .output: pg\string << |RT.Rust.Run"main() code"

// Library function call (direct return)
[r] .value: pg\int << |RT.Rust.Call"function_name"
```
- ✅ Flexibility
- ❌ Two different execution models to maintain

**Sub-questions**:
- How do we serialize Rust types back to Polyglot types?
- What's the FFI/calling convention?
- How do we handle Rust errors vs Polyglot errors?

---

**5. Code Organization & Modules**

**Question**: How should multi-module Rust code be organized?

**Options**:

**A) Single-File Inline Only**
```polyglot
// All code must be in one string literal
[r] .result: pg\string << |RT.Rust.Run"
mod utils {
    pub fn helper() -> i32 { 42 }
}

fn main() {
    println!(\"{}\", utils::helper());
}
"
```
- ✅ Simple, self-contained
- ❌ Limited for complex code

**B) Workspace Directory Structure**
```polyglot
// Point to workspace directory
[W] |W.RT.Rust.Stable
[<] <workspace: pg\path << \\FileDir\\rust-project

// rust-project/
//   Cargo.toml
//   src/
//     main.rs
//     lib.rs
//     utils.rs
```
- ✅ Full Rust project capabilities
- ✅ Proper module system
- ✅ Testing, benchmarks, examples
- ❌ More setup overhead

**C) Hybrid: Inline + External Modules**
```polyglot
// Main code inline, but can import from workspace
[W] |W.RT.Rust.Stable
[<] <modules: pg\path << \\FileDir\\rust-modules

[r] .result: pg\string << |RT.Rust.Run"
use my_modules::utils;

fn main() {
    utils::process();
}
"
```
- ✅ Reusable module library
- ✅ Simple main logic inline
- ❌ Complex module resolution

---

**6. Resource Control & Compilation**

**Question**: How are resources managed during compilation vs execution?

**Current Design**:
```bash
systemd-run \
  --property=CPUQuota=80% \
  --property=MemoryMax=2G \
  rustup run stable cargo run
```

**Issues**:

**A) Compilation Resource Limits**
- Rust compilation can be memory-intensive (especially with proc macros)
- Should compilation have different limits than execution?

**Options**:
```polyglot
[#] #Rust.Config
[<] .compilation_timeout: pg\int <~ 300      // 5 minutes
[<] .compilation_memory: pg\uint <~ 2048     // 2GB for compilation
[<] .execution_timeout: pg\int <~ 120        // 2 minutes
[<] .execution_memory: pg\uint <~ 512        // 512MB for execution
[X]
```

**B) Parallel Compilation**
- Should `cargo build` use `-j` flag?
- Should Queue Manager control parallelism?

**C) Incremental Compilation**
- Should we enable incremental compilation for session?
- Cache location and cleanup strategy?

---

**7. Error Handling & Output Capture**

**Question**: How do we capture and handle Rust errors?

**Scenarios**:

**A) Compilation Errors**
```polyglot
[r] .result: pg\string << |RT.Rust.Run"
fn main() {
    let x: i32 = "string";  // Type error
}
"
// Should this raise Polyglot error?
// Should we capture rustc error messages?
```

**B) Runtime Panics**
```polyglot
[r] .result: pg\string << |RT.Rust.Run"
fn main() {
    panic!(\"Something went wrong\");
}
"
// How to handle panic?
// Capture backtrace?
```

**C) Result/Option Handling**
```polyglot
// If using |RT.Rust.Call model
[r] .value: pg\int << |RT.Rust.Call"parse_number"
[<] <input: pg\string << "not a number"

// Rust function:
pub fn parse_number(s: String) -> Result<i64, ParseIntError> {
    s.parse()
}

// How does Result map to Polyglot error system?
```

**Options**:

**Option 1: All Errors are Polyglot Errors**
- Compilation error → `!Rust.CompilationError`
- Runtime panic → `!Rust.RuntimePanic`
- Result::Err → `!Rust.FunctionError`

**Option 2: Separate Error Channels**
- Compilation errors prevent execution (compile-time)
- Runtime errors captured in `.stderr`
- User must parse stderr to handle errors

**Option 3: Structured Error Capture**
```polyglot
[r] .result: #Rust.Result << |RT.Rust.Run"code"

// .result can be:
// - #Rust.Result.CompilationError { message, line, column }
// - #Rust.Result.RuntimePanic { message, backtrace }
// - #Rust.Result.Success { stdout, stderr, exit_code }
```

---

**8. Workspace Lifecycle & Cleanup**

**Question**: How long do Rust workspaces persist?

**Current Design**:
```polyglot
[\] .rust_session: #Sessions.Rust << |Setup.RT.Rust""
// ... use session ...
[/] |Cleanup.RT.Rust
[<] <session: #Sessions.Rust << .rust_session
```

**Options**:

**A) Per-Pipeline Session**
- Setup at pipeline start
- Cleanup at pipeline end
- ✅ Clean isolation
- ❌ Recompile on every run

**B) Persistent Workspace Pool**
- Workspaces live across pipeline executions
- Reuse if code/dependencies unchanged
- ✅ Fast execution
- ❌ Complex cache management

**C) User-Controlled Lifecycle**
```polyglot
// Keep artifacts for future use
[/] |Cleanup.RT.Rust
[<] <session: #Sessions.Rust << .rust_session
[<] <keep_artifacts: pg\bool << #Boolean.True
[<] <keep_duration: pg\dt << "24h"
```

**Sub-questions**:
- Where are artifacts stored?
- How much disk space can they consume?
- Cleanup policy (LRU, size-based, time-based)?

---

#### Design Recommendations Needed

For each question above, we need to decide:

1. **MVP Approach** - What's the simplest viable implementation?
2. **Post-MVP Features** - What can be added later?
3. **API Design** - What pipelines/signatures are needed?
4. **Error Handling** - How errors flow through the system?
5. **Resource Management** - Limits, quotas, cleanup policies?

---

#### Proposed Discussion Flow

**Phase 1: Execution Model** (Questions 1, 4, 5)
- Inline vs workspace
- Expressions vs programs vs functions
- Module organization

**Phase 2: Dependency & Build** (Questions 2, 3, 6)
- Dependency declaration
- Build caching strategy
- Compilation resource management

**Phase 3: Integration & Errors** (Questions 7, 8)
- Error handling model
- Output capture
- Workspace lifecycle

---

#### Success Criteria

After brainstorming session, we should have:

1. ✅ **Clear execution model** - Inline code handling strategy
2. ✅ **Dependency strategy** - How Cargo.toml and dependencies work
3. ✅ **Build/run separation** - Combined vs separate, caching approach
4. ✅ **Error handling** - Compilation errors, panics, Result mapping
5. ✅ **Workspace lifecycle** - Setup, persistence, cleanup policies
6. ✅ **API design** - Pipeline signatures for all operations
7. ✅ **Resource management** - Compilation vs execution limits
8. ✅ **MVP scope** - Clear line between MVP and post-MVP features

---

#### Related Documentation

**Current Specification**:
- `docs/Agile/runtime-environments-specification-2025-12-03.md` (lines 777-1070)

**Related Designs**:
- Python runtime (uv environments)
- Shell runtime (systemd-run sessions)
- Queue Manager integration
- Resource management architecture

**Blocking**:
- Rust runtime implementation
- Standard library Rust wrapper pipelines
- Type system (how Rust types map to Polyglot types)

---

**Assigned To**: TBD (needs brainstorming session)
**Estimated Session Time**: 2-3 hours (comprehensive, multi-phase)
**Recommended Technique**: First Principles Thinking + Morphological Analysis

---

### 11. Package Versioning Syntax Update - Registry Prefix Change

**Status**: PENDING
**Priority**: HIGH
**Added**: 2025-12-05
**Context**: Major syntax change to package declaration format

**Problem**:
Package versioning syntax needs to be updated across the entire codebase to use the new format with `@` prefix and `::` separator.

**OLD FORMAT**:
```polyglot
{Registry}@{FullName}:{Major}.{Minor}.{patch}.{draft}
Local@Examples.HelloWorld:1.0.0.0
```

**NEW FORMAT**:
```polyglot
@{Registry}::{FullName}:{Major}.{Minor}.{patch}.{draft}
@Local::Examples.HelloWorld:1.0.0.0
```

**Key Changes**:
1. Registry name now prefixed with `@` at the start
2. Separator between Registry and FullName changed from `@` to `::`
3. Version separator remains `:` (unchanged)

**Implementation Required**:
- **Lexer Updates**: Update token definitions to recognize new syntax
  - Recognize `@` prefix for registry
  - Update separator from `@` to `::`
  - Maintain version parsing (unchanged)

- **Parser Updates**: Update AST to handle new package declaration format
  - Parse `@Registry::FullName:Version.Version.Version.Version`
  - Validate structure and components

- **Documentation Updates**: Update all examples and documentation
  - User documentation
  - AI context files
  - Example code
  - Story files

**Affected Components**:
- Lexer token definitions (Story 1.2)
- Parser package declaration handling
- Package registry system
- All documentation with package examples
- Test suites

**Examples After Change**:
```polyglot
[@] @Local::Examples.HelloWorld:1.0.0.0
[X]

[@] @GitHub::username.project:2.3.1.0
[X]

[@] @Private::Internal.Tools:1.0.0.5
[X]
```

**Success Criteria**:
- ✅ Lexer recognizes new `@Registry::Name:Version` format
- ✅ Parser correctly parses new syntax
- ✅ All documentation updated
- ✅ All tests updated and passing
- ✅ Backward compatibility handled (reject old format with clear error)

**Dependencies**:
- Story 1.2 (Lexer Token Definitions) - needs update
- Story 1.4 (Parser AST Definitions) - needs update
- Documentation consistency audit

**Related To**:
- Package registry implementation
- Versioning system
- Namespace resolution

**Assigned To**: Scrum Master backlog
**Estimated Work**:
- Lexer changes: 2-4 hours
- Parser changes: 2-4 hours
- Documentation updates: 4-6 hours
- Testing: 2-3 hours
**Total: 1-2 days**

**Recommended Approach**:
1. Update lexer token patterns for package declarations
2. Update parser AST for new syntax structure
3. Add validation for new format
4. Update all documentation files
5. Add comprehensive test cases
6. Update error messages for old format

---

**Last Updated**: 2025-12-05
