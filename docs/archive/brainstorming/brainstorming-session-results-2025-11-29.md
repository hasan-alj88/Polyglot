# Brainstorming Session Results

**Session Date:** 2025-11-29
**Facilitator:** Carson (Elite Brainstorming Specialist)
**Participant:** hhj

## Session Start

**Context:** Building on party-mode discussion about Primitive Polyglot Data Types and Type System Architecture

**Focus Area:** Answering the 7 critical questions about variable state machine design

**Seven Critical Questions to Address:**
1. Complete state list - What are ALL variable states beyond Declared/Default/Ready/Fetching/Computing/Error?
2. Generic vs Per-Type implementation - Which approach to use?
3. Async future storage - How to persist `Fetching` state when Futures aren't `Serialize`?
4. Language field meaning - What does `Declared { language: Language }` represent?
5. State transition triggers - What causes state changes in practice? Who orchestrates?
6. Backward state transitions - Can variables go backwards (Ready → Fetching for re-fetch)?
7. Compound states - Can variables have multiple states simultaneously?

**Approach:** AI-Recommended Progressive Flow (First Principles → Mind Mapping → Six Thinking Hats → Assumption Reversal)

## Executive Summary

**Topic:** Polyglot Variable State Machine and DateTime Type System Architecture

**Session Goals:**
- Answer 7 critical questions about variable state machine design from party-mode discussion
- Design complete DateTime type system with multi-calendar support
- Define primitive data types and their state management
- Establish MVP vs Post-MVP implementation scope

**Techniques Used:** First Principles Thinking

**Total Ideas Generated:** 25+ core design decisions across state machine, DateTime structure, calendar systems, and operations

### Key Themes Identified:

1. **Simplicity Through Rust Enums** - State machines don't need intermediate states; Rust's enum system provides compile-time safety
2. **Profile-Based Flexibility** - Calendar variations (Islamic moon sighting, Hebrew authorities) handled through community profiles with manual overrides
3. **Compile-Time Safety** - Invalid state transitions caught at compile time, not runtime
4. **Community Respect** - DateTime system designed for religious/cultural calendar requirements, not just technical features
5. **Leverage Existing Libraries** - ICU4X provides battle-tested calendar operations instead of reinventing the wheel
6. **Async Without Intermediate States** - State changes when async operations complete, no Fetching/Computing states needed
7. **Equality as Membership** - Duration equality is membership testing (is point inside interval?)

## Technique Sessions

### Technique #1: First Principles Thinking

**Focus:** Questions 2, 3, 7 (State Machine Design)

**Key Insight #1: No Future Storage Needed**
User insight: "Don't need to use Future. Use Rust's Enum system instead - when state changes, another variable is instantiated out of the old state."

**Confirmed:** Rust enums are THE standard pattern for state machines. State transitions create new enum variant instances.

**Key Insight #2: Complete State Machine Definition**

**States:**
1. **Declared** - Type defined, no value yet
2. **Default** (DefaultValued) - Has default value, ONE override push allowed
3. **Ready** - Final value, immutable
4. **Error** - Operation failed, contains error info + stack trace
5. **Close** - Out of scope, memory cleared

**Operations:**
- `Push` - Push value to variable (async)
- `Pull` - Pull value from variable (async)
- `Default Push` - Push default value
- `Default Pull` - Pull default value

**State Transition Table:**

| State | Push (`<<`) | Pull (`>>`) | Default Push (`<~`) | Default Pull |
|-------|------|------|--------------|--------------|
| **Declared** | → Ready | Compile Error | → Default | Compile Error |
| **Ready** | Compile Error | get value | Compile Error | Compile Error |
| **Default** | → Ready | → Ready (materialized) | Compile Error | get default value |
| **Error** | Compile Error | get Error info | Compile Error | Compile Error |

**Async Operation Model:**
- `Push` (`<<`) and `Pull` (`>>`) are async operations
- State changes when async operation **completes**
- Example: `[>] .output: pg\int >> .var` - when operation completes, pushes value and transitions state

**Variable Declaration Examples:**
```polyglot
[#] #SomeEnum
[<] .var1: pg\int          // Declared state
[<] .var2: pg\uint <~ 0    // Declared -DefaultPush-> Default state

[|] SomePipeline
[r] e: #SomeEnum <<{
[*] .var1 << 6,   // Declared-Push->Ready
[*] .var2 << 2 }  // Default-Push->Ready

[r] x: pg:int << e.var1 // Pull from Ready, returns value
```

**Key Design Principles:**
1. **Compile-time validation** - Invalid state transitions caught at compile time
2. **No intermediate states** - No Fetching/Computing states needed
3. **Error is readable** - Pull from Error state returns error info
4. **Default materialization** - Pull from Default transitions to Ready (locks in default)
5. **Immutability** - Ready state is final (no push allowed)
6. **Default Push restriction** - `<~` only valid on Declared state (not Default)
7. **ONE override rule** - Default state allows exactly ONE Push to Ready (enforced by FSM)

---

**Key Insight #3: DateTime Type System Design**

**DateTime Structure:**
- **Pattern**: Once or Recurrence (with count/unlimited)
- **Value**: One or more of (Time, Date, DayOfWeek) - enforces "at least one" requirement
- **Type**: Instant or Duration (with units)

**DateTimeValue Variants (7 total):**
1. `TimeOnly(Time)` - e.g., "3:00 PM every day"
2. `DateOnly(Date)` - e.g., "December 25, 2025"
3. `DayOfWeekOnly(DayOfWeek)` - e.g., "Every Friday"
4. `TimeAndDate { time, date }` - e.g., "3:00 PM on Dec 25, 2025"
5. `TimeAndDayOfWeek { time, day }` - e.g., "3:00 PM every Friday"
6. `DateAndDayOfWeek { date, day }` - e.g., "Dec 25, 2025 (must be Wednesday)"
7. `All { time, date, day }` - e.g., "3:00 PM on Dec 25, 2025 (must be Wednesday)"

**Validation Rules:**
- DateAndDayOfWeek variant validates date matches specified day of week (using ICU4X)
- If mismatch: compile error `DateDayOfWeekMismatch` with "specified Sunday but date is Saturday"
- Works across all calendar systems via ICU4X conversion

---

**Key Insight #4: Multi-Calendar Support with Profiles**

**Supported Calendars (MVP):**
- **Gregorian** (algorithmic)
- **Julian** (algorithmic)
- **Assyrian/Syriac** (Julian + 311 year offset)
- **Islamic Civil** (algorithmic)
- **Islamic Observational** (profile-based with moon sighting variations)

**Profile System Design:**

User insight: "for MVP will use the avaible but post MVP will need to implenent for all convertions"

Profile-based system handles calendar variations (e.g., Islamic moon sighting differences):
- **Saudi Arabia**: Umm al-Qura calculation
- **UK**: Local moon sighting
- **Global**: Multiple regional authorities

**3-Tier Priority System:**
1. **Manual Overrides** (Priority 1) - HR manually enters observed dates
2. **API Cache** (Priority 2) - Moon sighting service data
3. **Calculated** (Priority 3) - ICU4X algorithmic fallback

**Profile Structure:**
```yaml
active_profile_id: "saudi-arabia"

profiles:
  - id: "saudi-arabia"
    name: "Saudi Arabia (Umm al-Qura)"
    region: "Middle East"
    authority: "Supreme Court of Saudi Arabia"
    editable: false
    default_method: "UmmAlQura"
    manual_overrides:
      - year: 1447
        month: 9  # Ramadan
        day: 1
        gregorian_date: "2025-02-28"
        source: "Supreme Court Announcement"
```

**Built-in Profiles:**
- `saudi-arabia` (Umm al-Qura)
- `uk-moonsighting` (UK local sighting)
- `turkey` (Diyanet)
- `egypt` (Dar al-Ifta)
- User-defined custom profiles

**Post-MVP Calendars:**
- Hebrew (with rabbinical authority profiles)
- Chinese (regional variations)
- Buddhist (regional variations)
- Hindu (Panchang API integration for regional variations)
- Persian, Coptic, Ethiopian

---

**Key Insight #5: Relative Date Patterns**

User question: "ok another point how to support the notion of Last Friday in Ramadan or first monday on novermber or 2nd Sun of Jun"

**RelativeDatePattern Structure:**
```rust
pub struct RelativeDatePattern {
    occurrence: WeekdayOccurrence,  // First, Second, Third, Fourth, Fifth, Last
    weekday: DayOfWeek,
    month: MonthSpecifier,          // Specific(u8) or Any
    year: YearSpecifier,            // Specific(i32) or Any
    calendar: Calendar,
}
```

**Examples:**
- "Last Friday in Ramadan" → `Last, Friday, Specific(9), Any, IslamicObservational`
- "First Monday in November" → `First, Monday, Specific(11), Any, Gregorian`
- "2nd Sunday in June" → `Second, Sunday, Specific(6), Any, Gregorian`
- "4th Thursday in November" → `Fourth, Thursday, Specific(11), Any, Gregorian` (Thanksgiving)

**Resolution Algorithm:**
1. Convert to target calendar if needed (via ICU4X)
2. Find all instances of specified weekday in month
3. If "Last": select final occurrence
4. If numeric: select nth occurrence (error if doesn't exist)
5. Return resolved date

**Integration with Recurrence:**
- Pattern can be "Once" or "Recurrence { count, unlimited }"
- Example: "Last Friday in Ramadan every year" = Annual recurrence

---

**Key Insight #6: DateTime Operations**

User clarification: "lastly, equlity is memebership test if time inide the duction then yes. That was for stright forword comparsion. also include datetime operation like Overlap or is_overloap (suggest more)"

**Equality Semantics:**
- **Instant**: Exact equality check (`2025-11-29 3:00 PM == 2025-11-29 3:00 PM`)
- **Duration**: Membership test (`contains()` method)
  - Example: "Is 3:15 PM inside duration 3:00-5:00 PM?" → true

**Complete Operation Set:**

**1. Membership & Comparison:**
- `contains(point)` - Is instant inside this duration?
- `is_before(other)` - Is this instant before other?
- `is_after(other)` - Is this instant after other?
- `is_between(start, end)` - Is this instant between two others?

**2. Interval Operations:**
- `overlaps(other)` - Do two durations overlap?
- `intersection(other)` - Returns overlapping portion (or None)
- `union(other)` - Returns combined duration if adjacent/overlapping (or None)
- `gap(other)` - Returns gap between non-overlapping durations (or None)
- `is_adjacent_to(other)` - Do durations touch with no gap?
- `contains_interval(other)` - Does this duration fully contain another?
- `split(point)` - Split duration at instant into two durations

**3. Arithmetic:**
- `add_duration(duration)` - Add duration to instant → new instant
- `subtract_duration(duration)` - Subtract duration from instant → new instant
- `duration_until(other)` - Calculate duration between two instants

**4. Recurrence:**
- `occurs_on(date)` - Does this recurrence occur on specified date?
- `next_occurrence_after(datetime)` - Find next occurrence after specified instant
- `all_occurrences_between(start, end)` - List all occurrences in range

**Error Handling:**
- All operations return `Result<T, DateTimeError>`
- Calendar conversion failures handled gracefully
- Profile mismatches reported clearly

## Idea Categorization

### Immediate Opportunities

_Ideas ready to implement now_

1. **Variable State Machine with Rust Enums** - Implement 5-state FSM (Declared, Default, Ready, Error, Close) using Rust enums for compile-time safety
2. **ICU4X Integration** - Use `icu_calendar` crate for calendar conversions and day-of-week calculations
3. **Basic Calendar Support** - Implement Gregorian, Julian, Assyrian, and Islamic Civil (algorithmic only)
4. **DateTime Core Structure** - Implement `DateTime { pattern, value, type }` with 7 DateTimeValue variants
5. **State Transition Validation** - Compile-time checks for invalid operations (e.g., Push to Ready state)
6. **Serialized Value Storage** - Store all variable values as strings with metadata for persistence
7. **Date/DayOfWeek Validation** - Use ICU4X to validate "Dec 25, 2025 is Wednesday" type constraints
8. **Error State with Stack Trace** - Capture complete error context when operations fail
9. **Default Materialization** - Implement Pull from Default → Ready transition
10. **Basic Relative Dates** - Support "First/Last DayOfWeek in Month" patterns

### Future Innovations

_Ideas requiring development/research_

1. **Islamic Profile System** - Implement profile-based moon sighting with 3-tier priority (manual overrides, API, calculated)
2. **Profile Configuration Format** - YAML-based profile definitions with community contributions
3. **Hebrew Calendar Profiles** - Rabbinical authority variations for festival dates
4. **Chinese Calendar Profiles** - Regional variations for lunar new year and festivals
5. **Panchang API Integration** - Hindu calendar with regional Panchang systems (post-MVP)
6. **Manual Override UI** - HR portal for entering observed religious dates
7. **Moon Sighting API Integration** - Cache data from moon sighting services (priority 2 in profile system)
8. **Calendar Conversion Matrix** - Full any-to-any calendar conversion support
9. **Advanced Relative Patterns** - "Last Friday in Ramadan" with automatic Hijri calendar resolution
10. **Duration Arithmetic** - Complex operations like "3 months + 2 weeks" accounting for variable month lengths
11. **Recurrence Rules** - RRULE-like system for complex recurring patterns
12. **Profile Community Marketplace** - Share and download calendar profiles created by communities

### Moonshots

_Ambitious, transformative concepts_

1. **AI-Powered Calendar Prediction** - ML model to predict moon sighting based on historical patterns and meteorological data
2. **Universal Calendar Converter** - Support for ALL world calendars including obscure regional variations
3. **Real-Time Moon Sighting Network** - Global network of observers reporting moon sightings for instant profile updates
4. **Smart Profile Selection** - Automatically detect user's region/community and suggest appropriate calendar profile
5. **Calendar Collaboration Platform** - Community-driven platform for maintaining and voting on calendar date accuracy
6. **Time Zone Intelligence** - Automatic handling of daylight saving time changes across all calendar systems
7. **Historical Calendar Support** - Accurate date conversions for historical dates (e.g., Julian to Gregorian cutover dates by region)
8. **Natural Language DateTime Parsing** - "Next Ramadan" or "Three Fridays from now" parsed to DateTime objects

### Insights and Learnings

_Key realizations from the session_

1. **Rust Enums Are Perfect for State Machines** - No need for complex async state tracking; enum variants represent states naturally
2. **Don't Store Futures** - State changes when async operations complete, not during execution
3. **Existing Libraries Solve Hard Problems** - ICU4X provides battle-tested calendar math instead of reinventing algorithms
4. **Religious Calendars Are Community-Driven** - Technical solutions must respect community authority structures (profiles)
5. **Manual Overrides Are Essential** - Algorithmic calculations can't replace official announcements for religious dates
6. **Equality Depends on Context** - Instant equality is exact match; Duration equality is membership testing
7. **Compile-Time Safety Prevents Bugs** - Invalid state transitions caught before code runs
8. **Default Push Complexity** - Allowing Default → Default transition would break "one override" invariant
9. **Duration is a DateTime Type** - Not a separate primitive; it's `pg\dt` with `Type::Duration`
10. **Profiles Need Priority Hierarchy** - Manual overrides (P1) → API cache (P2) → Calculated (P3)
11. **Day-of-Week Validation Is Non-Trivial** - Leap years, calendar variations make ICU4X essential
12. **Relative Dates Need Calendar Context** - "Last Friday in Ramadan" requires Hijri calendar resolution

## Action Planning

### Top 3 Priority Ideas

#### #1 Priority: Variable State Machine with Rust Enums

- **Rationale:** Foundation for entire type system. All variables (int, string, dt, etc.) use this state machine. Must be implemented first before any other IR or runtime work can proceed. Answers Questions 2, 3, 5, 6, 7 from critical questions list.

- **Next steps:**
  1. Update Story 2.1 acceptance criteria with state machine design
  2. Define `VariableState` enum in `polyglot-ir` crate
  3. Implement state transition validation methods
  4. Add serialization support (serde) for persistence
  5. Write comprehensive unit tests for all state transitions
  6. Document compile-time error messages for invalid transitions

- **Resources needed:**
  - `polyglot-ir` crate (existing workspace)
  - Serde crate for serialization
  - Documentation of Rust enum patterns for state machines
  - Story 2.1: IR Type Definitions & IR Structure (already in sprint backlog)

#### #2 Priority: DateTime Core Structure with ICU4X

- **Rationale:** DateTime is most complex primitive type. Needs design validation early in development. ICU4X integration is critical dependency for calendar math. Basic implementation needed for MVP; profile system can be post-MVP.

- **Next steps:**
  1. Add `icu_calendar` dependency to `polyglot-ir` crate
  2. Implement `DateTime`, `Date`, `Time`, `TimeZone` structures
  3. Implement `DateTimeValue` enum with 7 variants
  4. Implement basic calendars (Gregorian, Julian, Assyrian, Islamic Civil)
  5. Add ICU4X-based day-of-week validation
  6. Write unit tests for each DateTimeValue variant
  7. Document DateTime type in language specification

- **Resources needed:**
  - `icu_calendar` crate from ICU4X project
  - ICU4X documentation and examples
  - Calendar conversion algorithm references
  - Story 2.1 IR Type Definitions (includes DateTime design)

#### #3 Priority: Islamic Calendar Profile System

- **Rationale:** Demonstrates real-world calendar variation handling. Validates profile architecture before implementing other calendar profiles (Hebrew, Chinese, etc.). Critical for users in regions with Islamic calendar requirements. Shows respect for community-driven date determination.

- **Next steps:**
  1. Design YAML profile configuration format
  2. Implement `CalendarProfile` structure with 3-tier priority system
  3. Create built-in profiles (saudi-arabia, uk-moonsighting, turkey, egypt)
  4. Implement manual override mechanism
  5. Add profile loading and caching system
  6. Design API integration interface (implementation post-MVP)
  7. Write tests with profile-based date conversions
  8. Document profile system for community contributions

- **Resources needed:**
  - YAML parsing crate (serde_yaml)
  - Islamic calendar authority documentation (for profile data)
  - Moon sighting API documentation (for future integration)
  - Community consultation for profile accuracy
  - Post-MVP story for full profile implementation

## Reflection and Follow-up

### What Worked Well

1. **First Principles Approach** - Starting from fundamental truths (Rust enums, async completion) led to elegant solutions
2. **Building on Party-Mode Discussion** - Having 7 specific questions to answer provided clear focus
3. **User Expertise** - Direct insights about Polyglot's async nature and calendar requirements shaped practical solutions
4. **Progressive Refinement** - State machine design evolved from complex (7 states) to simple (5 states) through discussion
5. **Real-World Examples** - Concrete examples (Ramadan dates, Thanksgiving) validated design decisions
6. **Library Research** - Discovering ICU4X eliminated need to reinvent calendar algorithms
7. **Scoping MVP vs Post-MVP** - Clear boundaries between immediate implementation and future enhancements

### Areas for Further Exploration

1. **Relative Date Pattern Syntax** - How do users write "Last Friday in Ramadan" in Polyglot code? Need lexer/parser design
2. **Profile Distribution** - How are community profiles shared? Git repository? Package manager integration?
3. **Profile Conflict Resolution** - What happens when manual override conflicts with API data?
4. **Time Zone Handling** - Detailed design for time zone storage, DST transitions, and conversions
5. **Duration Units** - Precise semantics for "3 months" (calendar months vs 90 days?)
6. **Error Recovery** - How does Error state get cleared? Can variables transition Error → Declared for retry?
7. **Recurrence Limits** - Should unlimited recurrence be allowed, or require explicit count cap?
8. **Profile Validation** - How to detect corrupt or malicious profile data?
9. **Cross-Calendar Operations** - Detailed rules for "Islamic date + Gregorian duration"
10. **Historical Date Handling** - How to handle dates before calendar system inception (e.g., Hijri year -100)?

### Recommended Follow-up Techniques

1. **Mind Mapping** - Visualize relationships between DateTime components, calendar systems, and operations
2. **Six Thinking Hats** - Evaluate profile system from different perspectives:
   - White Hat: What data do we have about moon sighting APIs?
   - Red Hat: How do communities feel about algorithmic vs observed dates?
   - Black Hat: What could go wrong with manual overrides?
   - Yellow Hat: Benefits of community-driven profile marketplace
   - Green Hat: Creative solutions for profile conflict resolution
   - Blue Hat: Process for managing profile contributions
3. **Assumption Reversal** - Challenge assumptions:
   - "What if profiles were code instead of YAML?"
   - "What if each community ran their own calendar server?"
   - "What if equality for instants was also membership testing?"
4. **SCAMPER** - Enhance DateTime operations:
   - Substitute: Different equality semantics?
   - Combine: Merge interval operations?
   - Adapt: Borrow ideas from other datetime libraries?
   - Modify: Change recurrence rule format?
   - Put to other use: Use DateTime for scheduling beyond triggers?
   - Eliminate: Remove less-used operations?
   - Reverse: Different operation direction?

### Questions That Emerged

1. **Q: How are relative date patterns represented in Polyglot syntax?**
   - Status: Needs lexer/parser design session
   - Impact: Affects Story 1.3 (Lexer) or future syntax extension

2. **Q: Can Error state transition back to Declared for retry?**
   - Status: Not addressed in current design
   - Impact: Affects error recovery semantics

3. **Q: What happens when Date and DayOfWeek conflict in DateAndDayOfWeek variant?**
   - Status: Answered - Compile error with clear message
   - Impact: Requires ICU4X validation in compiler

4. **Q: How granular should Duration units be? (seconds, days, months, years?)**
   - Status: Mentioned but not fully designed
   - Impact: Affects DateTime arithmetic implementation

5. **Q: Should profiles be versioned? What if profile format changes?**
   - Status: Not addressed
   - Impact: Affects profile backward compatibility

6. **Q: How to handle time zones in multi-calendar scenarios?**
   - Status: Basic TimeZone structure defined, but conversion rules unclear
   - Impact: Complex interaction with calendar profiles

7. **Q: What is the Declared { language: Language } field for? (Original Question 4)**
   - Status: Not fully answered during session
   - Impact: May need follow-up discussion

### Next Session Planning

- **Suggested topics:**
  1. Relative Date Pattern Syntax Design (lexer/parser implications)
  2. Profile System Architecture Deep Dive (distribution, validation, versioning)
  3. Duration Arithmetic Semantics (calendar-aware vs fixed duration)
  4. Error Recovery and State Transition Edge Cases
  5. Time Zone and DST Handling Strategy

- **Preparation needed:**
  - Review ICU4X documentation for advanced features (recurrence, time zones)
  - Research existing calendar profile formats (iCalendar, Islamic calendar APIs)
  - Analyze competitor datetime libraries (Rust chrono, Java JSR-310, Python dateutil)
  - Gather sample Polyglot code with DateTime usage patterns
  - Consult with Islamic calendar authorities about profile data accuracy requirements

---

_Session facilitated using the BMAD CIS brainstorming framework_
