# Pending Decisions Log

**Maintained By**: Mai (Secretary)
**Last Updated**: 2025-11-19

This file tracks decisions under discussion, conflicts requiring resolution, and open questions requiring hhj's approval.

---

## ACTIVE CONFLICTS

### CONFLICT-2025-11-18-001: Token List vs. Grammar Rules Misalignment

**Date Raised**: 2025-11-18
**Status**: 🔴 CRITICAL CONFLICT
**Raised By**: Mai (Secretary) during grammar review
**Category**: Technical Specification

**Context**:
During grammar review session, hhj provided actual Polyglot grammar rules that **contradict** the token list documented in ADR-013 from the previous session (2025-11-17).

**Conflict Details**:

**1. Block Element Count Mismatch**:
- **ADR-013 (Previous Session)**: 27 block markers
- **hhj Grammar Rules (Current Session)**: 15 block elements
- **Discrepancy**: 12 missing blocks, different purposes, case sensitivity conflicts

**2. Grammar Structure Not Documented**:
- **ADR-013**: No mention of line structure requirements
- **hhj Grammar Rules**: STRICT - ALL lines MUST be `<Block element> <content>`

**3. Operator Prefix System Not Documented**:
- **ADR-013**: Listed operators but no systematic prefix rules
- **hhj Grammar Rules**:
  - Pipeline: `|CamelCase.Hierarchy`
  - Unpack: `~CamelCase.Hierarchy`
  - Enumeration: `#CamelCase.Hierarchy`
  - Error: `!CamelCase.Hierarchy`
  - Variables: `.snake_case.hierarchy`

**4. Comparison Operator Syntax Different**:
- **ADR-013**: Standard comparison operators
- **hhj Grammar Rules**: Comparison within `[?]` blocks with `?` suffix: `>?`, `=>?`, `<?`, `=<?`, `=?`
- Negation replaces `?` with `!?`: `>!?`, `=>!?`, etc.

**Questions for hhj**:

1. **Block Elements - Which is correct?**
   - Are there 15 or 27 block elements?
   - Is case significant? (`[r]` vs `[R]`, `[p]` vs `[P]`)
   - What are the complete, definitive block elements?

2. **Missing Block Elements - Should they exist?**
   From ADR-013 but not in your list:
   - `[E]` - Error handling
   - `[C]` - Configuration
   - `[D]` - Data block
   - `[L]` - Loop block
   - `[I]` - If block
   - `[W]` - While block
   - `[F]` - For block
   - `[S]` - Step block
   - `[V]` - Variable block
   - `[N]` - Notification
   - `[H]` - Hook block
   - `[G]` - Guard block
   - `[K]` - Checkpoint
   - `[U]` - User-defined
   - `[Z]` - Reserved

   **Question**: Should any of these exist? Or were they incorrectly documented in previous session?

3. **New Block Elements - Confirm purposes?**
   From your list but not in ADR-013:
   - `[+]` - OR block (Triggers)
   - `[&]` - AND block (Triggers)
   - `[s]` - Serial block (load serial files)
   - `[Y]` - Join block (was `[J]` in ADR-013?)
   - `[i]` - Input block

   **Question**: Confirm these are correct? Note `[Y]` vs previous `[J]` for Join.

4. **Strict Line Structure - Confirm?**
   **Rule**: ALL Polyglot lines MUST start with `<Block element> <content>`
   **Question**: Is this absolute? No free-form code outside block elements?

5. **Operator Hierarchy Syntax - Confirm?**
   - Pipeline: `|CamelCase.Dot.Hierarchy` ✓
   - Unpack: `~CamelCase.Dot.Hierarchy` (predefined only) ✓
   - Enumeration: `#CamelCase.Dot.Hierarchy` ✓
   - Error: `!CamelCase.Dot.Hierarchy` ✓
   - Variables: `.snake_case.dot.hierarchy` ✓

   **Question**: Are these formats exactly correct?

6. **Comparison Operators - Confirm syntax?**
   Within `[?]` blocks only:
   - `>?` (greater than)
   - `=>?` (greater than or equal) ← Is this `>=?` instead?
   - `<?` (less than)
   - `=<?` (less than or equal) ← Is this `<=?` instead?
   - `=?` (equal)

   Negation (NOT):
   - `>!?` (not greater than)
   - `=>!?` (not greater than or equal)
   - `<!?` (not less than)
   - `=<!?` (not less than or equal)
   - `=!?` (not equal)

   **Question**: Confirm operator symbols. Should `=>?` be `>=?` and `=<?` be `<=?`?

7. **Bracket Notation in `[?]` blocks**:
   Syntax: `[?] .var {( or [} .min, .max {) or ]}`
   - `[` = inclusive start
   - `(` = exclusive start
   - `]` = inclusive end
   - `)` = exclusive end

   Examples:
   - `[?] .age [18, 65)` → 18 ≤ age < 65
   - `[?] .score (0, 100]` → 0 < score ≤ 100

   **Question**: Is this syntax exactly correct?

8. **String Literals - Still valid?**
   From previous session:
   - Bare strings: `"text"` defaults to `|String.Formatted`
   - Prefixed: `String.Python.int.Format.Hex"42"`

   **Question**: Do string literals still work this way in the actual grammar?

9. **Any Other Block Elements Missing?**
   You said "I may have forgotten some if any doubt ask me to confirm"

   **Question**: Are there any additional block elements beyond the 15 you listed?

---

**Impact**:
- 🔴 BLOCKS Story 1.2 implementation until resolved
- 🔴 ADR-013 may contain incorrect specification
- 🔴 Architecture document needs major corrections
- 🔴 Meeting minutes from 2025-11-17 may document wrong decisions

**Next Steps**:
1. hhj answers all questions above
2. Mai creates corrected grammar specification document
3. Winston updates architecture.md with correct token/grammar rules
4. Amelia updates Story 1.2 implementation based on correct spec
5. Previous meeting minutes annotated with correction note

**Related Artifacts**:
- Incorrect specification: `docs/Tech/implementation/technical/architecture.md` ADR-013 (lines 1400-1487)
- Incorrect meeting minutes: `docs/Agile/meetings/2025-11-18-tokens-and-string-literals.md`
- Blocked story: `docs/Agile/stories/1-2-lexer-token-definitions.md`

---

## PENDING ENHANCEMENT REQUESTS

### ENH-2025-11-19-001: ITIL Ticket Tracking Integration with Agent Responsibilities

**Date Requested**: 2025-11-19
**Status**: 🟡 PENDING APPROVAL
**Requested By**: hhj
**Recorded By**: Mai (Secretary)
**Category**: Agent Enhancement
**Priority**: MEDIUM

**Enhancement Description**:
Add ITIL ticket tracking capability to the Secretary agent menu system, integrating with the existing agent responsibility tracking to provide comprehensive workload and ticket management.

**Current State**:
- Secretary agent has 16 menu commands for various administrative functions
- ITIL configuration exists in `docs/Agile/itil-config.yaml` with comprehensive ticket types, priorities, and assignment groups
- Agent responsibility tracking is planned (TODO-008) but not yet implemented
- No current mechanism to track ITIL tickets by agent or link tickets to agent responsibilities

**Proposed Enhancement**:
Add new Secretary agent menu command:
- **Command**: Track ITIL tickets linked to agent responsibilities
- **Action**: View, create, and manage ITIL tickets assigned to specific agents and assignment groups

**Key Features Needed**:
1. **Ticket-Agent Linking**:
   - View tickets assigned to specific agents
   - Track ticket status by assignment group (from itil-config.yaml)
   - Link tickets to agent responsibility areas

2. **Reporting Capabilities**:
   - Agent workload reports (tickets per agent)
   - Status distribution by assignment group
   - Priority breakdown per agent
   - SLA compliance tracking per agent

3. **Ticket Management**:
   - Create new tickets and assign to agents
   - Update ticket status following workflow (from itil-config.yaml)
   - Record resolution details
   - Close tickets with proper documentation

4. **Integration Points**:
   - Link with agent registry (TODO-008)
   - Reference existing itil-config.yaml for assignment groups
   - Integrate with meeting minutes (agents assigned action items)
   - Connect with TODO tracking (TODO items → ITIL tickets)

**Benefits**:
- Centralized agent workload visibility
- Better task distribution and balancing
- Improved accountability and tracking
- Structured issue resolution process
- Integration with existing ITIL framework

**Implementation Considerations**:
- **Blocker**: Agent configuration files (e.g., `bmad/bmm/agents/secretary.md`) cannot be modified by agents themselves
- **Solution Options**:
  1. Manual addition to agent configuration by hhj
  2. Create separate workflow/task for ITIL management
  3. Extend agent capability framework to allow configuration enhancements
- **Dependencies**:
  - Agent registry creation (TODO-008)
  - ITIL ticket storage mechanism (YAML files, database, or GitHub Issues sync)

**Questions for hhj**:
1. Should this be a Secretary menu command, or a separate workflow?
2. Where should ITIL tickets be stored? (YAML files in docs/Agile/tickets/, GitHub Issues, or other?)
3. Should tickets auto-sync with GitHub Issues when enabled in itil-config.yaml?
4. Priority: Should this be implemented before or after agent registry (TODO-008)?

**Related Artifacts**:
- Existing configuration: `docs/Agile/itil-config.yaml`
- TODO item: TODO-014 in `docs/Agile/project-todo.yaml`
- Related: TODO-008 (Agent responsibility registry)

**Next Steps**:
1. hhj reviews and provides approval/feedback
2. Determine implementation approach (menu command vs. workflow)
3. Design ticket storage mechanism
4. Create implementation story if approved
5. Assign to appropriate agent(s) for development

---

## Decision Workflow

```
Issue Identified
    ↓
Added to pending.md (PENDING_DISCUSSION)
    ↓
Agent discussion & analysis
    ↓
Recommendation formed (PENDING_APPROVAL)
    ↓
Present to hhj for approval
    ↓
IF approved → Move to approved.md
IF rejected → Document rationale, explore alternatives
IF deferred → Update status to BLOCKED with reason
```

---

## Conflict Resolution Process

When agents have conflicting recommendations:

1. **Document Conflict**: Mai creates PENDING entry with CONFLICT status
2. **Gather Perspectives**: Each agent states their recommendation and rationale
3. **Facilitate Discussion**: Mai leads structured discussion to find common ground
4. **Escalate to hhj**: If no consensus, present options with pros/cons
5. **Record Decision**: Once approved, move to approved.md with full context

---

**Maintenance Notes**:
- Mai monitors all agent discussions for conflicts
- Proactively identifies decisions requiring approval
- Moves approved decisions to approved.md
- Archives rejected options with rationale
- Tracks blocked decisions until blockers are resolved
