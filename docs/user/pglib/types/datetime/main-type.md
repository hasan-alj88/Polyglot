---
audience: automation-builder
type: reference
updated: 2026-04-09
---

# Main Type

<!-- @source:main-type -->

### #DateTime

`#DateTime` is a multi-level type (not `##Scalar`) because it contains both fixed structure (`.Instant`, `.Civil`, `.Relative`) and flexible extension points (`.Calendar`, `.Week`, `.TimeUnit`, `.Cultural`). Leaf content is typed data (`###Value`).

```polyglot
{#} #DateTime
   [%] .description << "Comprehensive date/time with multi-calendar support"
   [%] .version << "1.0.0"
   [#] ###Value
   [#] %##Alias << "dt"

   [ ] Absolute point in time
   [.] .Instant
      [.] .epoch#int
      [.] .nano#int
      [.] .precision#Precision

   [ ] Human-readable date/time
   [.] .Civil
      [.] .date#Date
      [.] .time#Time
      [.] .zone#Zone

   [ ] Calendar system representations (extensible)
   [.] .Calendar
      [:] :system#CalendarSystem
      [:] :gregorian#GregorianDate
      [:] :hijri#HijriDate
      [:] :hebrew#HebrewDate
      [:] :chinese#ChineseDate
      [:] :persian#PersianDate
      [:] :buddhist#BuddhistDate
      [:] :hindu#HinduDate
      [:] :japanese#JapaneseDate
      [:] :ethiopian#EthiopianDate
      [:] :coptic#CopticDate
      [ ] users can add :balinese, :mayan, etc.

   [ ] Durations, periods, intervals
   [.] .Relative
      [.] .duration#Duration
      [.] .period#Period
      [.] .interval#Interval
      [.] .recurrence#Recurrence

   [ ] Week system (extensible)
   [.] .Week
      [:] :day#Weekday
      [:] :number#int
      [:] :system#WeekSystem
      [:] :business#BusinessWeek
      [ ] users can add :pawukon etc.

   [ ] Non-standard time divisions (extensible)
   [.] .TimeUnit
      [:] :chinese#ChineseTime
      [:] :hindu#HinduTime
      [:] :decimal#DecimalTime
      [ ] users can add custom time divisions

   [ ] Cultural/religious extensions (extensible)
   [.] .Cultural
      [:] :dayBoundary#DayBoundary
      [:] :holidays#array.Holiday
      [:] :observances#array.Observance
      [:] :seasons#array.Season
      [ ] users can add custom cultural data
```

`.Calendar`, `.Week`, `.TimeUnit`, and `.Cultural` use entirely flexible (`:`) fields at their child level. Known entries are pre-declared with their types; users can add more keys. This avoids `.`/`:` mixing (PGE05001).

---

See also: [[core-components]], [[supporting-enums]], [[calendar-infrastructure]], [[calendar-date-types]], [[non-standard-time]], [[cultural-types]], [[related]]
