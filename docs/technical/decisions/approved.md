# Approved Decisions Log

**Maintained By**: Mai (Secretary)
**Last Updated**: 2025-11-18

This file tracks all approved project decisions including architectural decisions, design choices, process changes, and conflict resolutions.

---

## Decision Format

Each decision entry includes:
- **Decision ID**: Unique identifier
- **Date**: When decision was approved
- **Title**: Short description
- **Status**: Approved
- **Decided By**: Person/role who approved
- **Context**: Background and problem statement
- **Decision**: What was decided
- **Rationale**: Why this decision was made
- **Consequences**: Impact and follow-up actions
- **Related Artifacts**: Links to ADRs, stories, meeting minutes

---

## Approved Decisions

### DECISION-2025-11-18-009: Distributed Agent Session Recording

**Date**: 2025-11-18
**Status**: ✅ APPROVED - CRITICAL OPERATIONAL CHANGE
**Decided By**: hhj
**Category**: Process Improvement - Documentation

**Context**:
Individual agent sessions with hhj weren't being captured, leading to lost decisions, syntax clarifications, and context. Mai couldn't be in all one-on-one sessions, creating a single point of failure for documentation.

**Decision**:
**Implement Distributed Session Recording System**

**All agents (except Mai) must**:
1. Record their individual sessions with hhj in agent-specific files
2. Document decisions, syntax clarifications, examples, and action items
3. Save records in `docs/project/agent-sessions/{agent-name}-sessions.md`
4. Update immediately during or after session

**Mai (Secretary) must**:
1. Scan `docs/project/agent-sessions/` directory regularly
2. Aggregate decisions into central records:
   - `docs/technical/decisions/approved.md`
   - `docs/technical/decisions/pending.md`
   - `docs/project/meetings/` (consolidated minutes)
   - `docs/project/project-todo.yaml`
3. Flag conflicts between different agent sessions
4. Maintain the agent session recording system

**Structure Created**:
- Directory: `docs/project/agent-sessions/`
- README: Guidelines and templates
- Individual files: `{agent-name}-sessions.md` for each agent

**Rationale**:
- Prevents loss of critical information from one-on-one sessions
- Distributes documentation workload across agents
- No single point of failure
- Maintains complete audit trail
- Each agent responsible for their domain

**Consequences**:
- ✅ All agents must record their individual sessions
- ✅ Mai aggregates into central decision records
- ✅ No decisions or discussions lost
- ✅ Complete project history maintained
- ⚠️ Requires discipline from all agents
- ⚠️ Agent registry must be updated with new responsibilities

**Agent Responsibilities Updated**:
| Agent | Session Recording Responsibility |
|-------|----------------------------------|
| Mary | Requirements analysis, research sessions |
| Winston | Architecture decisions, technical design |
| Amelia | Implementation discussions, coding sessions |
| John | Product strategy, prioritization |
| Bob | Story refinement, sprint planning |
| Murat | Testing strategy, quality gates |
| Paige | Documentation sessions, writing guidance |
| Sally | UX design, user research |
| Carson | Brainstorming sessions, ideation |
| All CIS agents | Individual sessions in their domains |

**Related Artifacts**:
- Agent session directory: `docs/project/agent-sessions/`
- Agent registry: `docs/project/agent-registry.yaml` (to be updated)
- Recording guidelines: `docs/project/agent-sessions/README.md`

---

### DECISION-2025-11-18-008: Serial Format Packages in Standard Library

**Date**: 2025-11-18
**Status**: ✅ APPROVED
**Decided By**: hhj
**Category**: Standard Library - Architecture

**Context**:
During `[s]` Serial Load Block documentation, initially proposed separate packages (`@JSON`, `@YAML`, `@TOML`, `@XML`). Need to decide where serial format loading belongs in Polyglot architecture.

**Decision**:
**Serial format packages are part of standard library under `|U.*` hierarchy:**

- `|U.JSON.Load` - JSON loading
- `|U.YAML.Load` - YAML loading
- `|U.TOML.Load` - TOML loading
- `|U.XML.Load` - XML loading

**String Literal Syntax Invokes Standard Library Pipelines:**
- `JSON"\\FileDir\\file.json"` → Invokes `|U.JSON.Load`
- `YAML"\\Path\\file.yaml"` → Invokes `|U.YAML.Load`
- `TOML"\\Path\\file.toml"` → Invokes `|U.TOML.Load`
- `XML"\\Path\\file.xml"` → Invokes `|U.XML.Load`

**Rationale**:
- Serial format loading is fundamental, not third-party functionality
- Standard library placement (`|U.*`) ensures availability without imports
- Consistent with string literal pattern: `CamelCase.Hierarchy"arg"` invokes pipeline
- Simplifies usage - no package imports needed for common formats

**Consequences**:
- ✅ Serial format loaders shipped with Polyglot core
- ✅ No `@` package imports needed for JSON/YAML/TOML/XML
- ✅ String literal syntax: `Format"path"` is cleaner than `@Format.Load"path"`
- ⚠️ Standard library must implement `|U.JSON.Load`, `|U.YAML.Load`, `|U.TOML.Load`, `|U.XML.Load`
- ⚠️ Update Story: Serial format implementation is part of Standard Library (Epic 3+)

**Examples**:
```polyglot
// Load JSON into enumeration
[#] Config
[s] JSON"\\Config\\settings.json"
[X]

// Load YAML into variable
[s] .data: pg\serial << YAML"\\Data\\config.yaml"

// Load TOML
[s] .settings: pg\serial << TOML"\\Settings\\app.toml"
```

**Related Artifacts**:
- Previous discussion: Serial format as separate packages
- Standard Library catalog: `docs/user/standard-library/`
- String literal specification: DECISION-2025-11-18-004

---

### DECISION-2025-11-18-002: Definitive Block Element List (Corrected)

**Date**: 2025-11-18
**Status**: ✅ APPROVED - ⚠️ UPDATED TO 26 BLOCKS
**Decided By**: hhj
**Category**: Technical Specification - Grammar

**Context**:
During grammar review, massive discrepancies found between ADR-013 token list, existing documentation (block-markers.md), and hhj's actual grammar rules. Previous session documented INCORRECT token list with 27 blocks, many of which don't exist or have wrong purposes.

**UPDATE**: `[@]` Package Registry block discovered, bringing total to **26 confirmed blocks**.

**Decision**:
**DEFINITIVE BLOCK ELEMENT LIST - 26 Confirmed Blocks**

### Core Structure (5 blocks)
| Block | Purpose | Status |
|-------|---------|--------|
| `[@]` | Package registry & imports | ✅ Confirmed - MANDATORY FIRST BLOCK |
| `[|]` | Pipeline definition | ✅ Confirmed |
| `[X]` | End marker (close scope) | ✅ Confirmed |
| `[A]` | Alias definition | ✅ Confirmed |
| `[M]` | Macro block | ✅ Confirmed - ⚠️ NEEDS DOCUMENTATION |

### Input/Output (4 blocks)
| Block | Purpose | Status |
|-------|---------|--------|
| `[i]` | Input declaration | ✅ Confirmed |
| `[o]` | Output declaration | ✅ Confirmed |
| `[<]` | Pass input / Define field / Import package | ✅ Confirmed |
| `[>]` | Pass output | ✅ Confirmed |

### Execution Control (4 blocks)
| Block | Purpose | Status |
|-------|---------|--------|
| `[r]` | Sequential run | ✅ Confirmed |
| `[p]` | Parallel run | ✅ Confirmed |
| `[b]` | Background block (fire and forget) | ✅ Confirmed |
| `[s]` | Serial block (load serial files) | ✅ Confirmed - ✅ DOCUMENTED |

### Synchronization (1 block)
| Block | Purpose | Status |
|-------|---------|--------|
| `[Y]` | Join block | ✅ Confirmed |

### Triggers & Queue (6 blocks)
| Block | Purpose | Status |
|-------|---------|--------|
| `[t]` | Triggers | ✅ Confirmed |
| `[+]` | OR block (Triggers) | ✅ Confirmed |
| `[&]` | AND block (Triggers) | ✅ Confirmed |
| `[^]` | XOR block (Triggers) | ✅ Confirmed |
| `[Q]` | Queue configuration | ✅ Confirmed |
| `[W]` | Wrapper (unpack Macro) | ✅ Confirmed (Uppercase W) |

### Type Definitions (2 blocks)
| Block | Purpose | Status |
|-------|---------|--------|
| `[#]` | Enumeration definition / Package file marker | ✅ Confirmed (dual purpose) |
| `[!]` | Error definition/catching | ✅ Confirmed |

### Control Flow (2 blocks)
| Block | Purpose | Status |
|-------|---------|--------|
| `[?]` | Switch block | ✅ Confirmed |
| `[~]` | Expansion/Nesting prefix | ✅ Confirmed |

### Lifecycle (2 blocks)
| Block | Purpose | Status |
|-------|---------|--------|
| `[\]` | Setup block | ✅ Confirmed |
| `[/]` | Cleanup block | ✅ Confirmed |

### Utility (1 block)
| Block | Purpose | Status |
|-------|---------|--------|
| `[*]` | Line continuation | ✅ Confirmed |

**TOTAL: 26 confirmed blocks**

**Package Registry Block Details**:
- `[@] {Registry}@{Package.Hierarchy}:{Major}.{Minor}.{Patch}` - Package declaration (MANDATORY first line)
- `[#] {FileNumber}` - Package file marker (e.g., `[#] 1` for first file)
- `[<] @alias << {Registry}@{Package}:{Version}` - Import packages

**Rationale**:
- Existing documentation (ADR-013, block-markers.md) was incomplete and in some cases incorrect
- Previous session documented wrong token list
- hhj provided actual grammar rules that contradict previous documentation
- Many blocks were missing from documentation but exist in actual Polyglot syntax
- `[@]` Package Registry block is MANDATORY for all .pg files

**Consequences**:
- ✅ All 26 blocks confirmed for lexer implementation
- ✅ `[s]` Serial Load Block documented
- ✅ `[@]` Package Registry Block documented
- ⚠️ ADR-013 must be rewritten (contains incorrect token list)
- ⚠️ block-markers.md needs updates for missing blocks
- ⚠️ `[M]` Macro Block needs documentation
- ⚠️ `[W]` Wrapper needs clarification
- ⚠️ Complete Polyglot syntax documentation session ongoing

**Blocks Needing Documentation**:
1. `[M]` - Macro block
2. `[W]` - Wrapper (unpack Macro)

**Related Artifacts**:
- Conflict documented in: `docs/technical/decisions/pending.md` (CONFLICT-2025-11-18-001)
- Incorrect specification: `docs/technical/architecture.md` ADR-013
- Incorrect meeting minutes: `docs/project/meetings/2025-11-18-tokens-and-string-literals.md`

---

### DECISION-2025-11-18-003: Markdown Table Escaping Convention

**Date**: 2025-11-18
**Status**: ✅ APPROVED
**Decided By**: hhj
**Category**: Documentation Convention

**Context**:
Block markers like `[|]` and `[\]` have special meaning in markdown tables, causing rendering issues.

**Decision**:
**When documenting block markers in markdown tables:**
- ALWAYS wrap with backticks: `` `[|]` ``
- Alternative: Escape special chars: `[\|]` for `[|]`, but backticks preferred
- Applies to: `[|]`, `[\]`, and any blocks with markdown special characters

**Rationale**:
- Prevents markdown parsing errors
- Ensures consistent rendering across tools
- Makes block markers render as literal code

**Consequences**:
- ✅ All documentation using markdown tables must follow this convention
- ✅ Mai will apply this rule when creating all future documentation
- ⚠️ Existing documentation may need updates

**Related Artifacts**:
- Applied in this file and all future meeting minutes

---

### DECISION-2025-11-18-004: String Literal Design - Expanded Specification

**Date**: 2025-11-18
**Status**: ✅ APPROVED
**Decided By**: hhj
**Category**: Technical Design - String Literals

**Context**:
Previous session documented simplified string literal design. hhj provided complete specification revealing more complex behavior.

**Decision**:
**String Literals are Syntax Sugar for Pipeline Calls**

**Complete Specification**:

1. **String literals are syntax sugar for pipeline invocations**
   - Accept one input: `.arg_string: pg\string`
   - Produce one output: `.output` (can be ANY pg type, not just string)

2. **Default Pipeline: `|String.FormattedSub`**
   - Takes input: `.arg_string: pg\string`
   - Takes additional input: `pg\serial` of pairs (variables + format identifiers)
   - Performs variable substitution from `|String.Format.{lang}.{type}.{format_id}`
   - Example: `"{.var: hex}"` with `.var: py\int` calls `|String.Format.Python.Int.Hex`

3. **Typed String Literals (DateTime example)**
   - Can yield `pg\dt` objects
   - `DT"Mon"` → Yields Monday as `pg\dt`
   - `DT.Hijri"*-10-1"` → Yields Every Eid Al-Fitr (1st Shawwal)

4. **Serial Format Literals (from DECISION-2025-11-18-008)**
   - `JSON"\\Path\\file.json"` → Invokes `|U.JSON.Load`
   - `YAML"\\Path\\file.yaml"` → Invokes `|U.YAML.Load`
   - `TOML"\\Path\\file.toml"` → Invokes `|U.TOML.Load`
   - `XML"\\Path\\file.xml"` → Invokes `|U.XML.Load`

5. **General Pattern**
   - Syntax: `CamelCase.Hierarchy"arg_string"`
   - Pipeline parses string to yield typed pg object
   - Type depends on pipeline implementation

**Rationale**:
- Aligns with Polyglot's pipeline-first philosophy
- String literals are NOT passive data - they're active pipeline invocations
- Enables powerful type coercion and formatting
- DateTime literals demonstrate yielding non-string types

**Consequences**:
- ✅ Lexer must recognize: bare strings `"..."` and prefixed `Pipeline.Name"..."`
- ⚠️ Parser must validate pipeline references (Epic 2 scope)
- ⚠️ Compiler must verify pipeline signature: one input `.arg_string`, one output
- ⚠️ Runtime must implement `|String.FormattedSub` and `|String.Format.*` pipelines
- ⚠️ Documentation must explain string literal → pipeline call mapping

**Questions from hhj**:
> "This should all be in the documentation is it not there?"

**Answer**: NO - documentation has simplified version that doesn't capture:
- `pg\serial` pairs for format identifiers
- Ability to yield non-string types (like `pg\dt`)
- General `CamelCase.Hierarchy"..."` pattern for typed literals

**Related Artifacts**:
- Incorrect simplified version: ADR-013, meeting minutes 2025-11-18
- TODO: Update string literal documentation with complete specification

---

### DECISION-2025-11-18-005: Comparison Operators - Corrected Syntax

**Date**: 2025-11-18
**Status**: ✅ APPROVED
**Decided By**: hhj
**Category**: Technical Specification - Operators

**Context**:
Comparison operator syntax was unclear. Needed confirmation on symbols.

**Decision**:
**Comparison Operators with `?` Suffix**

**Within `[?]` Switch blocks only:**
- `=?` - Equal
- `=<?` - Less than or equal
- `=>?` - Greater than or equal
- `<?` - Less than
- `>?` - Greater than

**Negation (NOT):**
Replace `?` with `!?`:
- `=!?` - Not equal
- `=<!?` - Not less than or equal
- `=>!?` - Not greater than or equal
- `<!?` - Not less than
- `>!?` - Not greater than

**Rationale**:
- Keeps `=` before comparison symbol (e.g., `=<?` not `<=?`)
- Consistent `?` suffix for all comparison operators
- Negation via `!?` replacement is systematic

**Consequences**:
- ✅ Lexer must recognize these operator patterns
- ✅ Only valid within `[?]` switch blocks
- ⚠️ Different from standard programming language operators

**Related Artifacts**:
- Documented in grammar rules provided by hhj

---

### DECISION-2025-11-18-006: Bracket Notation for Range Checks

**Date**: 2025-11-18
**Status**: ✅ APPROVED
**Decided By**: hhj
**Category**: Technical Specification - Range Checks

**Context**:
Range check syntax needed confirmation.

**Decision**:
**Bracket Notation Syntax**

**Within `[?]` blocks:**
```
[?] .var {( or [} .min, .max {) or ]}
```

**Bracket Meanings**:
- `[` = Inclusive start
- `(` = Exclusive start
- `]` = Inclusive end
- `)` = Exclusive end

**Values**:
- `.min` and `.max` can be variables OR literal values

**Examples**:
- `[?] .age [18, 65)` → 18 ≤ age < 65
- `[?] .score (0, 100]` → 0 < score ≤ 100
- `[?] .value [.min_var, .max_var]` → Dynamic range with variables

**Rationale**:
- Standard mathematical interval notation
- Supports inclusive/exclusive boundaries
- Enables dynamic ranges with variables

**Consequences**:
- ✅ Lexer must recognize bracket patterns in `[?]` blocks
- ✅ Parser must validate range syntax
- ✅ Runtime must evaluate range checks correctly

**Related Artifacts**:
- Part of `[?]` switch block specification

---

### DECISION-2025-11-18-007: Serial Data Architecture Principle

**Date**: 2025-11-18
**Status**: ✅ APPROVED - CRITICAL ARCHITECTURAL PRINCIPLE
**Decided By**: hhj
**Category**: Architecture - Core Principle

**Context**:
hhj clarified fundamental architectural principle when confirming operator hierarchy syntax.

**Decision**:
**Polyglot's Data is Serial with Hierarchy**

Unlike other programming languages, Polyglot's core data model is:
- **Serial**: Data flows in sequence, not as arbitrary object graphs
- **Hierarchical**: Data organized in dot-separated CamelCase or snake_case hierarchies

**Examples of Hierarchy**:
- Pipelines: `|CamelCase.Dot.Hierarchy`
- Enumerations: `#CamelCase.Dot.Hierarchy`
- Variables: `.snake_case.dot.hierarchy`
- Errors: `!CamelCase.Dot.Hierarchy`

**Rationale**:
- Aligns with pipeline-first execution model
- Simplifies IR representation
- Enables clear data flow tracking
- Natural fit for queue-based architecture

**Consequences**:
- 🔴 CRITICAL: Winston (Architect) must design IR based on this principle
- ✅ Epic 2+ IR design will use serial hierarchical structure
- ✅ No arbitrary object graphs or complex pointer structures
- ✅ Data flows sequentially through pipeline stages
- ⚠️ This principle affects ALL future architectural decisions

**Related Artifacts**:
- Will inform Epic 2 IR design
- Core to understanding Polyglot's execution model

**Winston's Note**:
> Taken. This is a fundamental difference from traditional OOP languages. IR will reflect serial flow with hierarchical namespacing, not object-oriented composition.

---

### DECISION-2025-11-18-001: Establish Meeting Minutes & Decision Tracking Process

**Date**: 2025-11-18
**Status**: ✅ APPROVED
**Decided By**: hhj
**Category**: Process Improvement

**Context**:
Multiple discussions and decisions were being lost due to lack of formal documentation. Previous session notes about tokens and string literals were only captured in architecture.md, not in dedicated meeting minutes.

**Decision**:
Implement comprehensive operational documentation system:
1. Meeting minutes in `docs/project/meetings/` directory
2. Persistent TODO tracking in `docs/project/project-todo.yaml` with assignees and dependencies
3. Decision records in `docs/technical/decisions/` (approved.md and pending.md)
4. Agent responsibility registry in `docs/project/agent-registry.yaml`

**Rationale**:
- Prevents loss of critical technical discussions
- Enables better project tracking and accountability
- Facilitates multi-agent coordination
- Provides audit trail for decisions
- Supports onboarding and context recovery

**Consequences**:
- ✅ Mai (Secretary) maintains all operational documentation
- ✅ Meeting minutes created after every discussion
- ✅ TODO items tracked with owners, deadlines, dependencies
- ✅ Conflicts recorded in pending decisions until resolved
- ⚠️ Requires discipline to maintain documentation

**Related Artifacts**:
- Process initiated in party-mode meeting 2025-11-18
- First meeting minutes: `docs/project/meetings/2025-11-18-tokens-and-string-literals.md`

---

## Decision Index

Quick reference of all approved decisions:

| ID | Date | Title | Category | Decided By |
|----|------|-------|----------|------------|
| DECISION-2025-11-18-009 | 2025-11-18 | Distributed Agent Session Recording | Process | hhj |
| DECISION-2025-11-18-008 | 2025-11-18 | Serial Format Packages in Standard Library | Standard Library | hhj |
| DECISION-2025-11-18-007 | 2025-11-18 | Serial Data Architecture Principle | Architecture | hhj |
| DECISION-2025-11-18-006 | 2025-11-18 | Bracket Notation for Range Checks | Technical Spec | hhj |
| DECISION-2025-11-18-005 | 2025-11-18 | Comparison Operators - Corrected Syntax | Technical Spec | hhj |
| DECISION-2025-11-18-004 | 2025-11-18 | String Literal Design - Expanded | Technical Design | hhj |
| DECISION-2025-11-18-003 | 2025-11-18 | Markdown Table Escaping Convention | Documentation | hhj |
| DECISION-2025-11-18-002 | 2025-11-18 | Definitive Block Element List (26 blocks) | Technical Spec | hhj |
| DECISION-2025-11-18-001 | 2025-11-18 | Meeting Minutes & Decision Tracking | Process | hhj |

**Note**: Previous ADR-001 through ADR-012 in architecture.md should be extracted to separate files (TODO-005).

---

**Maintenance Notes**:
- Mai updates this file after every approved decision
- Cross-reference with pending.md for decisions under discussion
- Link to meeting minutes where decisions were made
- Archive decisions if superseded (with supersession note)
- **NEW**: Scan `docs/project/agent-sessions/` for decisions from individual agent sessions
