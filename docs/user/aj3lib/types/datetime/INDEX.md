---
audience: automation-builder
type: reference
updated: 2026-04-03
---

# #DT (DateTime Types)

<!-- @c:types -->

`#DateTime` is the comprehensive date and time type. It represents absolute instants, civil date/time, multi-calendar projections, relative durations, week systems, non-standard time units, and cultural extensions. All calendars are projections of a single epoch value -- converting between calendars always goes through `.Instant.epoch`.

The alias `#dt` is available for type annotations.

---

## Sections

| Section | Description |
|---------|-------------|
| [[main-type]] | `#DateTime` multi-level type definition |
| [[core-components]] | `#Date`, `#Time`, `#Zone`, `#Duration`, `#Period`, `#Interval`, `#Recurrence` |
| [[supporting-enums]] | `#Precision`, `#RecurrencePattern`, `#CalendarSystem`, `#Weekday`, `#Month`, `#DayBoundary`, `#WeekSystem`, `#BusinessWeek`, `#MonthStructure` |
| [[calendar-infrastructure]] | `#CalendarProjection`, `#LeapRule` |
| [[calendar-date-types]] | Gregorian, Hijri, Hebrew, Chinese, Persian, Buddhist, Hindu, Japanese, Ethiopian, Coptic, Custom |
| [[non-standard-time]] | `#ChineseTime`, `#HinduTime`, `#DecimalTime`, `#CustomTimeUnit` |
| [[cultural-types]] | `#Holiday`, `#Observance`, `#Season` and related enums |
| [[related]] | Cross-references to other aj3lib type docs |
