# Architecture Decision Record: Base Datetime Format for Polyglot

**Date:** 2025-12-02
**Status:** ✅ Accepted
**Decision Makers:** Architecture, Product, Development, Scrum teams
**Related Session:** [Brainstorming Session 2025-12-02](../project/brainstorming-session-results-2025-12-02.md)

---

## Decision

**Use Unix timestamp (milliseconds since 1970-01-01 UTC) as the base intermediate datetime format for Polyglot automation language.**

The originally proposed "Millennium Date (MD)" format will not be implemented as a distinct type. Instead, Unix timestamp with millisecond precision serves as the universal intermediary for all calendar conversions.

---

## Context

### Problem Statement

Polyglot automation language requires robust datetime handling with support for multiple calendar systems (Gregorian, Julian Date, and future calendars like Hijri, Hebrew, etc.). The system needs:

1. **Universal intermediary format** - All calendar conversions route through a common base format
2. **Millisecond precision** - Required for automation and scheduling accuracy
3. **Mathematical purity** - Simple arithmetic operations without calendar-specific complexity
4. **Database storage** - Efficient storage and querying of temporal records
5. **Temporal profiles** - Data-driven architecture where adding new calendars requires only data, not code changes

### Initial Design: Millennium Date (MD)

The brainstorming session (2025-12-02) initially designed a custom "Millennium Date" format:

```rust
struct MillenniumDate {
    days: i128,           // Days since 2000-01-01
    milliseconds: u32,    // 0-86,399,999 within day
    gmt_offset: i16,      // UTC offset in minutes
}
```

**Design rationale:**
- Epoch at year 2000 (clean millennium boundary, modern focus)
- Separate days + milliseconds counters (human-readable structure)
- Built-in timezone offset field

### Investigation of Database Support

During implementation planning, the team investigated database support for temporal records:

1. **InfluxDB Julian Date support?** → **No** - InfluxDB uses Unix epoch nanoseconds or RFC3339
2. **InfluxDB millisecond precision?** → **Yes** - Full support for Unix timestamp with millisecond precision via `precision=ms` parameter

This discovery prompted reevaluation: Why create a custom datetime format when Unix timestamp exists?

---

## Alternatives Considered

### Alternative 1: Millennium Date (MD) - Custom Format ❌

**Pros:**
- Clean epoch at 2000-01-01 (conceptually modern)
- Separate day/millisecond counters (human-readable)
- Built-in timezone offset

**Cons:**
- Custom format with zero ecosystem support
- No native database support (must store as custom fields)
- Requires custom arithmetic, formatting, parsing libraries
- Every external system integration requires MD ↔ Unix conversion
- Increased implementation complexity and maintenance burden
- Developers must learn a non-standard format

**Rejected because:** The aesthetic benefits (2000 epoch, structured fields) do not justify creating a custom time standard that the entire ecosystem must work around.

### Alternative 2: Julian Date (JD) ❌

**Pros:**
- Established astronomical standard
- Very large historical range (-4712 BCE epoch)
- Continuous day count

**Cons:**
- No native database support
- Primarily used in astronomy, not general computing
- Fractional days less intuitive than milliseconds for programmers
- Still requires custom implementation and conversions

**Rejected because:** While well-established in astronomy, JD is not a computing standard and shares the same ecosystem integration issues as MD.

### Alternative 3: Unix Timestamp (milliseconds) ✅ **SELECTED**

**Pros:**
- **Universal standard** - Every programming language, database, and system supports it
- **Native database support** - InfluxDB, PostgreSQL, MySQL all have first-class Unix timestamp support
- **Millisecond precision** - Meets requirements exactly (`i64` milliseconds since 1970-01-01)
- **Huge ecosystem** - Battle-tested libraries (chrono, time), tooling, documentation
- **Simple arithmetic** - Add/subtract milliseconds directly
- **Massive range** - `i64` supports years -290,307 to +294,246 (far exceeds requirements)
- **Interoperability** - Every external system can consume Unix timestamps natively
- **Zero custom implementation** - Leverage existing, proven libraries

**Cons:**
- Epoch at 1970 instead of 2000 (aesthetic preference, no functional impact)
- No structured days/milliseconds split (users never see internal format anyway)
- No built-in timezone field (timezone belongs in temporal profiles, not base timestamp)

**Selected because:** Provides all required functionality with maximum simplicity, leveraging decades of proven implementation and universal ecosystem support.

### Alternative 4: PostgreSQL Instead of InfluxDB 🤔

**Pros:**
- Could store MD as numeric fields directly
- ACID guarantees, strong relational model
- JSONB for flexible rule logic

**Cons:**
- Time-series queries less optimized than InfluxDB
- More complex setup than InfluxDB for time-based data
- Doesn't solve the "should we use MD or Unix" question

**Status:** Deferred. Can revisit if InfluxDB proves inadequate, but using Unix timestamp makes either database viable.

---

## Decision Rationale

### Core Principles from Brainstorming Session

The brainstorming session established critical architectural principles:

1. **Separation of Concerns** - Mathematical time representation must be separate from calendar complexity
2. **Composable Temporal Profiles** - Calendar logic lives in data-driven profiles, not code
3. **Universal Intermediary** - All conversions route through a common base format

**Key insight:** These principles don't require a custom time format. Unix timestamp already provides the "pure mathematical abstraction" we need.

### The Innovation Is Temporal Profiles, Not the Time Format

**What makes Polyglot's datetime system unique:**
- ✅ Composable temporal profiles with conditional inclusion logic
- ✅ Data-driven architecture (add calendars without code changes)
- ✅ Temporal records with validity ranges
- ✅ Support for probabilistic futures (observational calendars)
- ✅ Multiple composition strategies (additive, hierarchical, multiplicative)

**What's NOT unique about needing a base time format:**
- Every datetime system needs a universal intermediary
- Unix timestamp has solved this problem for 50+ years
- Reinventing this wheel adds no value

### User-Facing vs Internal Architecture

**User perspective:**
```polyglot
[r] .var:pg\dt << DT"2024-03-15"      // Gregorian syntax
[r] .var:pg\jd << DT"2459750.5"       // Julian Date syntax
[r] .var:pg\hijri << DT"1445-09-01"   // Future: Hijri syntax
```

Users write dates in their preferred calendar format. They never see the internal representation (Unix timestamp). The base format is an implementation detail, not a user-facing concern.

**Internal architecture:**
- All calendar dates convert to Unix timestamp for storage/computation
- All calendar-to-calendar conversions route through Unix timestamp
- Temporal records use Unix timestamp (native InfluxDB time field)

### Bootstrapping Problem Solved

**The circular dependency with custom formats:**
- To convert Gregorian → MD, we need temporal records
- Temporal records need timestamps
- What format? If MD, we need Gregorian → MD conversion to query them!

**Unix timestamp breaks the cycle:**
- Gregorian → Unix timestamp is well-established (hardcoded, no temporal records needed)
- Unix timestamp → Julian Date is pure math (hardcoded constant)
- Temporal records use Unix timestamp (native database format)
- No circular dependencies

### Performance and Reliability

**Unix timestamp advantages:**
- ✅ InfluxDB's native time-series indexing optimizes queries automatically
- ✅ Battle-tested libraries (chrono, time) handle edge cases correctly
- ✅ No custom arithmetic code to debug and maintain
- ✅ Ecosystem tools (monitoring, debugging) work out-of-box

**Risk mitigation:**
- Custom formats = custom bugs
- Standard formats = bugs already found and fixed by millions of users

---

## Implementation Implications

### Architecture Changes

**Before (with MD):**
```
User Syntax → MD Format → Database (custom fields)
              ↑
    (custom conversion code)
```

**After (with Unix timestamp):**
```
User Syntax → Unix Timestamp → Database (native time field)
              ↑
    (standard library conversion)
```

### Code Structure

```rust
// Base type - simple alias to standard type
type UnixMillis = i64;  // Milliseconds since 1970-01-01 UTC

// Temporal profile interface (unchanged)
trait TemporalProfile {
    fn to_unix_ms(&self, calendar_date: &str) -> Result<UnixMillis>;
    fn from_unix_ms(&self, unix_ms: UnixMillis) -> Result<String>;
}

// Leverage ecosystem
use chrono::{DateTime, Utc, NaiveDateTime};
// All the datetime utilities we need, already implemented
```

### Database Schema

**InfluxDB schema (using native time field):**
```
Bucket: temporal_records
Measurement: calendar_rules

Timestamp: Unix ms (InfluxDB native - automatic indexing!)
Tags:
  - profile_id (gregorian, julian, hijri, etc.)
  - record_type (rule, offset, event)
Fields:
  - valid_to (Unix ms - end of validity range)
  - offset_value (numeric - offsets to apply)
  - rule_logic (JSON string - conditional logic)
  - metadata (JSON string - additional context)
```

**Query example:**
```flux
from(bucket: "temporal_records")
  |> range(start: unix_ms_value, stop: unix_ms_value)
  |> filter(fn: (r) => r["profile_id"] == "gregorian")
```

### Conversion Pipeline

All calendar conversions follow this pattern:

```
Calendar A → Unix ms → Calendar B

Examples:
Gregorian "2024-03-15" → 1710460800000 → Julian Date 2460388.5
Hijri "1445-09-01" → 1710460800000 → Gregorian "2024-03-15"
```

**No circular dependencies:**
- Gregorian ↔ Unix ms: Standard library (chrono)
- Julian Date ↔ Unix ms: Hardcoded formula (no temporal records needed)
- Future calendars use temporal profiles (query by Unix ms)

---

## Consequences

### Positive Consequences

1. **✅ Reduced implementation complexity** - ~4 weeks vs ~5+ weeks with custom format
2. **✅ Native database optimization** - InfluxDB time-series indexing "just works"
3. **✅ Ecosystem leverage** - chrono, time crates provide battle-tested utilities
4. **✅ Universal interoperability** - Every external system understands Unix timestamps
5. **✅ Lower maintenance burden** - No custom datetime arithmetic to debug
6. **✅ Faster onboarding** - Developers already understand Unix timestamps
7. **✅ Better tooling** - Monitoring, debugging, logging tools work natively

### Negative Consequences

1. **❌ Epoch at 1970, not 2000** - Aesthetic preference for "modern" epoch lost
   - **Mitigation:** Users never see epoch; this is internal only

2. **❌ No structured days/milliseconds split** - Single counter instead of day + time components
   - **Mitigation:** Users see human-readable calendar formats; internal structure doesn't matter

3. **❌ No built-in timezone field** - Unix timestamps are UTC
   - **Mitigation:** Timezone logic belongs in temporal profiles anyway (separation of concerns)

### Neutral Consequences

1. **⚪ Range limited to i64** - Years -290,307 to +294,246
   - **Context:** Far exceeds any realistic automation requirement; i128 would be overkill

2. **⚪ "Millennium Date" brand unused** - Original naming concept abandoned
   - **Context:** Internal format names don't affect user experience

---

## Validation

### How We'll Know This Was the Right Decision

**Success criteria:**

1. **Timeline met** - MVP ships in ~4 weeks with Gregorian + Julian Date support
2. **Extensibility proven** - Adding 3rd calendar (e.g., Hijri) takes <3 days (data only, no code changes)
3. **Performance acceptable** - Calendar conversions complete in <1ms for typical dates
4. **No precision loss** - Millisecond accuracy maintained through conversion chains
5. **Developer productivity** - New contributors understand architecture within 1 day

**Failure signals (would trigger reassessment):**

- Unix timestamp range proves insufficient (extremely unlikely)
- Precision degradation in conversion chains (should not occur with ms precision)
- InfluxDB time-series queries perform poorly (unlikely with native indexing)
- Interop with external systems harder than expected (opposite should be true)

---

## Timeline Impact

**Original estimate (with MD):** ~5 weeks
**Revised estimate (with Unix timestamp):** ~4 weeks

**Story effort reductions:**
- Story 1: Core datetime operations - 3 days → 2 days (leverage chrono crate)
- Story 3: Temporal record storage - 1 week → 5 days (native InfluxDB time field)

---

## References

### Related Documents

- [Brainstorming Session 2025-12-02](../project/brainstorming-session-results-2025-12-02.md) - Original datetime system design
- [Story 0: InfluxDB Infrastructure](../project/stories/) - Database setup (pending)
- [Story 1: Core Datetime Operations](../project/stories/) - Implementation (pending)

### External References

- [InfluxDB Time Precision Documentation](https://docs.influxdata.com/influxdb/v2/query-data/influxql/explore-data/time-and-timezone/)
- [Unix Timestamp Standard](https://en.wikipedia.org/wiki/Unix_time)
- [Rust chrono crate](https://docs.rs/chrono/latest/chrono/)

---

## Revision History

| Date | Version | Changes | Author |
|------|---------|---------|--------|
| 2025-12-02 | 1.0 | Initial ADR - Unix timestamp decision | Architecture Team |

---

**Decision Status:** ✅ **APPROVED AND LOCKED**

**Next Steps:**
1. Begin Story 0: InfluxDB Docker setup with `precision=ms`
2. Implement Story 1: Core Unix timestamp operations using chrono crate
3. Update all documentation references from "MD" to "Unix timestamp"
