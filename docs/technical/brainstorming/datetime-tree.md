---
audience: developer
type: brainstorming
updated: 2026-04-05
---

# Brainstorm: #DateTime Full Data Tree

> **PGE05001 FIX (2026-03-20):** `.Calendar`, `.Week`, `.TimeUnit`, and `.Cultural` now use entirely flexible (`:`) fields at their child level. Known entries (`:gregorian`, `:hijri`, etc.) are pre-declared with their types; users can add more keys. No `.`/`:` mixing.

## Design Principles

1. **Epoch is truth** — all calendars are projections of `.Instant.epoch` with leap rules
2. **Fixed (`.`) for universal structure** — year, month, day, hour, etc.
3. **Flexible (`:`) for extensible parts** — user calendars, holidays, custom eras
4. **Enum fields for finite sets** — calendar systems, weekday names, months
5. **Schema matching** — serialized data must match target schema; mismatch is compile error (PGE04002)
6. **No mixing `.`/`:` at same sibling level** (PGE05001)
7. **Calendars carry their own leap logic** — each calendar defines how it maps epoch to its units

## Top-Level Tree

```aljam3
#DateTime
   .Instant                        — absolute point in time
      .epoch#int                   — Unix epoch seconds
      .nano#int                    — nanosecond offset within second
      .precision#Precision         — what resolution matters

   .Civil                          — human-readable date/time
      .date#Date
      .time#Time
      .zone#Zone

   .Calendar                       — calendar system representations
      [:] :system#CalendarSystem   — which calendar is active
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
      [ ] users can add :balinese, :mayan, etc. — schema must match their declared type

   .Relative                       — durations, periods, intervals
      .duration#Duration           — absolute (seconds-based)
      .period#Period               — calendar-relative (months/years)
      .interval#Interval           — start/end range
      .recurrence#Recurrence       — repeating patterns

   .Week                           — week system
      [:] :day#Weekday
      [:] :number#int              — week-of-year
      [:] :system#WeekSystem       — ISO, US, Middle-Eastern, etc.
      [:] :business#BusinessWeek   — configurable work/off days
      [ ] users can add :pawukon etc. — schema must match their declared type

   .TimeUnit                       — non-standard time divisions
      [:] :chinese#ChineseTime     — shichen, ke, fen
      [:] :hindu#HinduTime         — prahara, muhurta, ghati
      [:] :decimal#DecimalTime     — French Republican decimal
      [ ] users can add custom time divisions — schema must match their declared type

   .Cultural                       — cultural/religious extensions
      [:] :dayBoundary#DayBoundary — midnight, sunset, sunrise
      [:] :holidays#array:Holiday  — typed holiday list
      [:] :observances#array:Observance — typed observance list
      [:] :seasons#array:Season    — typed season markers
      [ ] users can add custom cultural data — schema must match their declared type
```

## Supporting Enums

```aljam3
#CalendarSystem
   .Gregorian
   .Hijri
   .Hebrew
   .Chinese
   .Persian
   .Buddhist
   .Hindu
   .Japanese
   .Ethiopian
   .Coptic
   .Custom

#Weekday
   .Monday
   .Tuesday
   .Wednesday
   .Thursday
   .Friday
   .Saturday
   .Sunday

#WeekSystem
   .ISO#BusinessWeek                — Monday-start, week 1 = first Thursday
   .US#BusinessWeek                 — Sunday-start, week 1 = contains Jan 1
   .MiddleEastern#BusinessWeek      — Saturday-start
   .Custom#BusinessWeek

#BusinessWeek
   .firstDay#Weekday                 — user must set
   .workDays#array:Weekday           — user must set
   .offDays#array:Weekday            — user must set
   .hoursPerDay#int                  — user must set

#DayBoundary
   .Midnight           — civil standard
   .Sunset             — Jewish, Islamic
   .Sunrise            — some Hindu traditions
   .Noon               — astronomical Julian Day

#Precision
   .Year
   .Month
   .Day
   .Hour
   .Minute
   .Second
   .Millisecond
   .Microsecond
   .Nanosecond

#Month                  — Gregorian (other calendars have own month enums)
   .January
   .February
   .March
   .April
   .May
   .June
   .July
   .August
   .September
   .October
   .November
   .December
```

## Epoch Foundation

Every calendar is a **projection** of `.Instant.epoch` (Unix epoch seconds).
A calendar defines: epoch offset, month structure, leap rules, and time unit mapping.
Converting between calendars always goes through epoch: CalendarA → epoch → CalendarB.

```aljam3
#CalendarProjection     — how any calendar maps from epoch
   .epochOffset#int     — seconds offset from Unix epoch to calendar epoch
   .leapRule#LeapRule   — how this calendar handles leap cycles
   .monthStructure#MonthStructure — fixed-length, variable, or lunisolar

#LeapRule
   .None                — no leap adjustment
   .FixedCycle          — repeating pattern (e.g., Gregorian 4/100/400)
      .cycleYears#int
      .leapYears#array:int  — which years in cycle are leap
   .Astronomical        — computed from astronomical observation
   .Tabular             — pre-calculated table
   .Custom
      :rule              — user-defined leap logic

#MonthStructure
   .Fixed               — all months same length (e.g., Coptic 30-day)
   .Mixed               — months have fixed but different lengths (e.g., Gregorian)
   .Lunisolar           — months follow lunar phases (e.g., Hebrew, Chinese)
   .Lunar               — pure lunar cycle (e.g., Hijri)
```

## Core Component Types

```aljam3
#Date
   .year#int
   .month#int
   .day#int

#Time
   .hour#int            — 0-23
   .minute#int          — 0-59
   .second#int          — 0-60 (leap second)
   .nano#int            — 0-999999999

#Zone
   .iana#string         — "America/New_York"
   .offset#ZoneOffset
   .dst#bool

#ZoneOffset
   .hours#int           — -12 to +14
   .minutes#int         — 0, 15, 30, 45

#Duration               — absolute time span
   .seconds#int
   .nanos#int

#Period                 — calendar-relative span
   .years#int
   .months#int
   .weeks#int
   .days#int

#Interval
   .start#DateTime
   .end#DateTime
   .startInclusive#bool <~ #Boolean.True
   .endInclusive#bool <~ #Boolean.False

#Recurrence
   .pattern#RecurrencePattern
   .count#int <~ 0      — 0 = infinite
   .until#DateTime       — optional end date

#RecurrencePattern
   .Daily
   .Weekly
   .Monthly
   .Yearly
   .Custom
      .rule#string       — cron-like or RRULE
```

## Non-Standard Time Units

Some calendars divide the day differently than hours/minutes/seconds.
These all map back to epoch seconds — they're alternative projections of `.Instant`.

### Chinese Traditional Time
```aljam3
#ChineseTime
   .shichen#ChineseShichen   — 2-hour double-hours (12/day)
   .ke#int                   — quarter-unit (classically 100/day = 14.4 min each)
   .fen#int                  — subdivision of ke (60 fen per ke)
   .yeGeng#int               — night watch number (1-5, for nighttime)

#ChineseShichen
   .Zi                       — 23:00-01:00
   .Chou                     — 01:00-03:00
   .Yin                      — 03:00-05:00
   .Mao                      — 05:00-07:00
   .Chen                     — 07:00-09:00
   .Si                       — 09:00-11:00
   .Wu                       — 11:00-13:00
   .Wei                      — 13:00-15:00
   .Shen                     — 15:00-17:00
   .You                      — 17:00-19:00
   .Xu                       — 19:00-21:00
   .Hai                      — 21:00-23:00
```

### Hindu Traditional Time
```aljam3
#HinduTime
   .prahara#int              — 3-hour watch (8/day, varies by season)
   .muhurta#int              — 48-minute unit (30/day)
   .ghati#int                — 24-minute unit (60/day)
   .pala#int                 — subdivision of ghati (60 pala per ghati)
   .vipala#int               — subdivision of pala (60 vipala per pala)
```

### French Republican Decimal Time
```aljam3
#DecimalTime
   .hour#int                 — 0-9 (10 hours/day)
   .minute#int               — 0-99 (100 minutes/hour)
   .second#int               — 0-99 (100 seconds/minute)
```

### Custom Time Unit
```aljam3
#CustomTimeUnit
   .name#string
   .unitsPerDay#int          — how many of this unit fit in a day
   :subdivisions             — user-defined sub-units
   :mapping                  — user-defined epoch-to-unit conversion
```

## Calendar-Specific Date Types

### Gregorian
```aljam3
#GregorianDate
   .year#int
   .month#Month
   .day#int
```

### Islamic / Hijri

The Hijri calendar is regional — month starts depend on moon sighting authority.
Saudi uses Umm al-Qura (astronomical), Pakistan/India often differ by 1-2 days.
The design supports regional authorities, multiple methods, and fully custom Hijri.

```aljam3
#HijriDate
   .year#int
   .month#HijriMonth
   .day#int
   .authority#HijriAuthority        — who determines month start
   .method#HijriMethod              — how month start is determined
   .leap#HijriLeap                  — leap year logic

#HijriMonth
   .Muharram
   .Safar
   .RabiAlAwwal
   .RabiAlThani
   .JumadaAlUla
   .JumadaAlThani
   .Rajab
   .Shaban
   .Ramadan
   .Shawwal
   .DhuAlQidah
   .DhuAlHijjah

#HijriAuthority
   .UmmAlQura           — Saudi Arabia (astronomical calculation)
   .Local               — local moon-sighting committee
   .Regional            — follows a regional bloc (Gulf, South Asia, etc.)
   .Custom              — user-defined authority
      .name#string
      :rules             — user-defined sighting/calculation rules

#HijriMethod
   .Tabular             — 30-year cycle calculation (intercalary days)
   .Astronomical        — Umm al-Qura style (moon conjunction + sunset)
   .Observational       — physical moon-sighting by committee
   .Custom              — user-defined method
      .name#string
      :logic             — user-defined calculation/sighting logic

#HijriLeap
   .Tabular30           — standard 30-year cycle (11 leap years)
   .UmmAlQura           — Saudi astronomical leap logic
   .Custom              — user-defined leap rule
      :rule              — user-defined leap calculation
```

### Hebrew
```aljam3
#HebrewDate
   .year#int
   .month#HebrewMonth
   .day#int

#HebrewMonth
   .Tishrei
   .Cheshvan
   .Kislev
   .Tevet
   .Shevat
   .AdarI
   .AdarII              — leap year only
   .Nisan
   .Iyyar
   .Sivan
   .Tammuz
   .Av
   .Elul
```

### Chinese
```aljam3
#ChineseDate
   .year#int
   .cycle#int            — 60-year cycle number
   .month#int
   .leapMonth#bool <~ #Boolean.False
   .day#int
   .stem#HeavenlyStem
   .branch#EarthlyBranch

#HeavenlyStem
   .Jia
   .Yi
   .Bing
   .Ding
   .Wu
   .Ji
   .Geng
   .Xin
   .Ren
   .Gui

#EarthlyBranch
   .Zi
   .Chou
   .Yin
   .Mao
   .Chen
   .Si
   .Wu
   .Wei
   .Shen
   .You
   .Xu
   .Hai
```

### Persian
```aljam3
#PersianDate
   .year#int
   .month#PersianMonth
   .day#int

#PersianMonth
   .Farvardin
   .Ordibehesht
   .Khordad
   .Tir
   .Mordad
   .Shahrivar
   .Mehr
   .Aban
   .Azar
   .Dey
   .Bahman
   .Esfand
```

### Buddhist
```aljam3
#BuddhistDate
   .year#int             — Buddhist Era (Gregorian + 543)
   .month#Month          — uses Gregorian months
   .day#int
```

### Hindu
```aljam3
#HinduDate
   .year#int
   .era#HinduEra
   .month#HinduMonth
   .day#int
   .paksha#Paksha        — lunar fortnight

#HinduEra
   .VikramSamvat
   .Saka

#HinduMonth
   .Chaitra
   .Vaishakha
   .Jyeshtha
   .Ashadha
   .Shravana
   .Bhadrapada
   .Ashvina
   .Kartika
   .Margashirsha
   .Pausha
   .Magha
   .Phalguna

#Paksha
   .Shukla               — waxing/bright
   .Krishna              — waning/dark
```

### Japanese
```aljam3
#JapaneseDate
   .year#int
   .era#JapaneseEra
   .eraYear#int          — year within era
   .month#Month          — Gregorian months
   .day#int

#JapaneseEra
   .Reiwa                — 2019-present
   .Heisei               — 1989-2019
   .Showa                — 1926-1989
   .Taisho               — 1912-1926
   .Meiji                — 1868-1912
   :historical           — USER-DEFINED older eras
```

### Ethiopian
```aljam3
#EthiopianDate
   .year#int
   .month#EthiopianMonth
   .day#int              — 1-30 (or 1-5/6 for Pagume)

#EthiopianMonth
   .Meskerem
   .Tikimt
   .Hidar
   .Tahsas
   .Tir
   .Yekatit
   .Megabit
   .Miyazya
   .Ginbot
   .Sene
   .Hamle
   .Nehase
   .Pagume             — 5-6 epagomenal days
```

### Coptic
```aljam3
#CopticDate
   .year#int
   .month#CopticMonth
   .day#int

#CopticMonth
   .Thout
   .Paopi
   .Hathor
   .Koiak
   .Tobi
   .Meshir
   .Paremhat
   .Parmouti
   .Pashons
   .Paoni
   .Epip
   .Mesori
   .PiKogiEnavot       — epagomenal days
```

### Custom Calendar
```aljam3
#CustomCalendar        — user-extensible via :
   .name#string
   :months             — user-defined month names/counts
   :leapRule           — user-defined leap year logic
   :epochOffset#int    — offset from Unix epoch
```

## Cultural Types

```aljam3
#Holiday
   .name#string
   .date#Date
   .recurring#bool <~ #Boolean.True
   .calendar#CalendarSystem <~ #CalendarSystem.Gregorian
   :extra              — user-defined metadata

#Observance
   .name#string
   .date#Date
   .type#ObservanceType
   .recurring#bool <~ #Boolean.True
   :extra              — user-defined metadata

#ObservanceType
   .Religious
   .National
   .Cultural
   .Personal

#Season
   .name#string
   .start#Date
   .end#Date
   .type#SeasonType
   :extra              — user-defined metadata

#SeasonType
   .Astronomical       — solstice/equinox based
   .Meteorological     — temperature based
   .Cultural           — culture-defined
   .Agricultural       — farming seasons
   .Custom
```

## Flexible Fields (`:`) Extension Points

All four extensible levels (`.Calendar`, `.Week`, `.TimeUnit`, `.Cultural`) are entirely flexible — users add new `:key#Type` entries alongside the pre-declared ones.

- `#DateTime.Calendar:*` — add any calendar (`:balinese#BalineseDate`, `:mayan#MayanDate`, etc.)
- `#DateTime.Week:*` — add any week system (`:pawukon#PawukonWeek`, etc.)
- `#DateTime.TimeUnit:*` — add any time division
- `#DateTime.Cultural:*` — add any cultural data
- `#Holiday:extra`, `#Observance:extra`, `#Season:extra` — per-item metadata
- `#JapaneseEra:historical` — older era names
- `#HijriAuthority.Custom:rules` — custom moon-sighting/calculation authority
- `#HijriMethod.Custom:logic` — custom Hijri month-start determination
- `#HijriLeap.Custom:rule` — custom Hijri leap logic
- `#LeapRule.Custom:rule` — custom leap logic for any calendar

## Alias

`#DateTime` alias is `#dt` — use `#dt` in type annotations:
```aljam3
[-] $now#dt
[-] $deadline#dt
```

## aj3lib Pipelines — -DT.*

All `-DT.*` pipelines yield `#dt` objects. No `[@]` import needed (aj3lib).

### Construction
```aljam3
-DT.Now                — current instant → #dt
-DT.From.Epoch         — epoch seconds → #dt
   <epoch#int
-DT.From.ISO           — ISO-8601 string → #dt (same as DT"..." literal)
   <iso#string
-DT.From.Parts         — explicit components → #dt
   <year#int
   <month#int
   <day#int
   <hour#int <~ 0
   <minute#int <~ 0
   <second#int <~ 0
   <nano#int <~ 0
   <zone#string <~ "UTC"
```

### Calendar Conversion
```aljam3
-DT.To.Gregorian       — #dt → #GregorianDate projection
   <source#dt
-DT.To.Hijri           — #dt → #HijriDate projection
   <source#dt
   <authority#HijriAuthority <~ #HijriAuthority.UmmAlQura
-DT.To.Hebrew          — #dt → #HebrewDate projection
   <source#dt
-DT.To.Chinese         — #dt → #ChineseDate projection
   <source#dt
-DT.To.Persian         — #dt → #PersianDate projection
   <source#dt
-DT.To.Buddhist        — #dt → #BuddhistDate projection
   <source#dt
-DT.To.Hindu           — #dt → #HinduDate projection
   <source#dt
   <era#HinduEra <~ #HinduEra.VikramSamvat
-DT.To.Japanese        — #dt → #JapaneseDate projection
   <source#dt
-DT.To.Ethiopian       — #dt → #EthiopianDate projection
   <source#dt
-DT.To.Coptic          — #dt → #CopticDate projection
   <source#dt
-DT.To.Custom          — #dt → custom calendar projection
   <source#dt
   <calendar#CustomCalendar
```

### Time Unit Conversion
```aljam3
-DT.To.ChineseTime     — #dt → #ChineseTime (shichen/ke/fen)
   <source#dt
-DT.To.HinduTime       — #dt → #HinduTime (prahara/muhurta)
   <source#dt
-DT.To.DecimalTime     — #dt → #DecimalTime (French Republican)
   <source#dt
```

### Arithmetic
```aljam3
-DT.Add.Duration       — #dt + #Duration → #dt
   <source#dt
   <duration#Duration
-DT.Add.Period         — #dt + #Period → #dt (calendar-aware)
   <source#dt
   <period#Period
-DT.Sub                — #dt - #dt → #Duration
   <a#dt
   <b#dt
```

### Comparison
```aljam3
-DT.Compare            — #dt vs #dt → #int (-1, 0, 1)
   <a#dt
   <b#dt
-DT.IsBefore           — #dt < #dt → #bool
   <a#dt
   <b#dt
-DT.IsAfter            — #dt > #dt → #bool
   <a#dt
   <b#dt
-DT.InInterval         — #dt within #Interval → #bool
   <source#dt
   <interval#Interval
```

### Extraction
```aljam3
-DT.Get.Year           — #dt → year as #int
   <source#dt
-DT.Get.Month          — #dt → month as #int
   <source#dt
-DT.Get.Day            — #dt → day as #int
   <source#dt
-DT.Get.Weekday        — #dt → #Weekday
   <source#dt
-DT.Get.WeekNumber     — #dt → week-of-year as #int
   <source#dt
   <system#WeekSystem <~ #WeekSystem.ISO
-DT.Get.Epoch          — #dt → epoch seconds as #int
   <source#dt
-DT.Get.Zone           — #dt → #Zone
   <source#dt
```

### Zone
```aljam3
-DT.Zone.Set           — set timezone on #dt → #dt
   <source#dt
   <iana#string
-DT.Zone.Convert       — convert #dt to different timezone → #dt
   <source#dt
   <iana#string
```

### Formatting
```aljam3
-DT.Format             — #dt → formatted string
   <source#dt
   <pattern#string     — e.g. "YYYY-MM-DD HH:mm:ss"
-DT.Format.ISO         — #dt → ISO-8601 string
   <source#dt
-DT.Format.Calendar    — #dt → string in specific calendar format
   <source#dt
   <system#CalendarSystem
```

### Business
```aljam3
-DT.Business.IsWorkDay — #dt + #BusinessWeek → #bool
   <source#dt
   <week#BusinessWeek
-DT.Business.NextWorkDay — next work day → #dt
   <source#dt
   <week#BusinessWeek
-DT.Business.AddWorkDays — add N work days → #dt
   <source#dt
   <days#int
   <week#BusinessWeek
```

## Open Questions (Decided)

### Q1 — DateTime inline notation (Decided 2026-03-20)

`-DateTime"..."` is the full inline pipeline call. `-DT"..."` is its alias (matching `#dt` alias for `#DateTime`). Both are sugar for `-DT.From.ISO`:

```aljam3
[ ] These three are equivalent:
[-] $deadline#dt << -DateTime"2026-03-20T12:00:00Z"
[-] $deadline#dt << -DT"2026-03-20T12:00:00Z"
[-] $deadline#dt
   [-] -DT.From.ISO
      (-) <iso << "2026-03-20T12:00:00Z"
      (-) >dt >> $deadline
```

Follows the `-Path"..."` precedent exactly.

### Q2 — BusinessWeek defaults per WeekSystem (Decided 2026-03-20)

No defaults. `#BusinessWeek` fields (`.firstDay`, `.workDays`, `.offDays`, `.hoursPerDay`) must all be explicitly set by the user. No assumptions about regional norms.

### Q3 — Projection caching (Decided 2026-03-20)

No caching. `-DT.To.*` pipelines recompute each call. Users store results in a `$variable` if they need the value again — explicit over implicit.
