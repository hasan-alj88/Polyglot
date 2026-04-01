---
audience: user
type: specification
updated: 2026-04-01
status: complete
---

# #DateTime and Related Types

<!-- @types -->

`#DateTime` is the comprehensive date and time type. It represents absolute instants, civil date/time, multi-calendar projections, relative durations, week systems, non-standard time units, and cultural extensions. All calendars are projections of a single epoch value -- converting between calendars always goes through `.Instant.epoch`.

The alias `#dt` is available for type annotations.

---

## Main Type

### #DateTime

`#DateTime` is a multi-level type (not `##Scalar`) because it contains both fixed structure (`.Instant`, `.Civil`, `.Relative`) and flexible extension points (`.Calendar`, `.Week`, `.TimeUnit`, `.Cultural`). Leaf content is typed data (`###Value`).

```polyglot
{#} #DateTime
   [%] .description << "Comprehensive date/time with multi-calendar support"
   [%] .version << "1.0.0"
   [#] << ###Value
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

## Core Component Types

### #Date

```polyglot
{#} #Date
   [%] .description << "Calendar date with year, month, day"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "date"
   [.] .year#int
   [.] .month#int
   [.] .day#int
```

### #Time

```polyglot
{#} #Time
   [%] .description << "Time of day with nanosecond precision"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "time"
   [.] .hour#int
   [.] .minute#int
   [.] .second#int
   [.] .nano#int
```

### #Zone

```polyglot
{#} #Zone
   [%] .description << "Timezone with IANA identifier, offset, and DST flag"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "zone"
   [.] .iana#string
   [.] .offset#ZoneOffset
   [.] .dst#bool
```

### #ZoneOffset

```polyglot
{#} #ZoneOffset
   [%] .description << "UTC offset in hours and minutes"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "zoneoffset"
   [.] .hours#int
   [.] .minutes#int
```

### #Duration

```polyglot
{#} #Duration
   [%] .description << "Absolute time span in seconds and nanoseconds"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "duration"
   [.] .seconds#int
   [.] .nanos#int
```

### #Period

```polyglot
{#} #Period
   [%] .description << "Calendar-relative time span in years, months, weeks, days"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "period"
   [.] .years#int
   [.] .months#int
   [.] .weeks#int
   [.] .days#int
```

### #Interval

```polyglot
{#} #Interval
   [%] .description << "Start/end date-time range with inclusivity flags"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "interval"
   [.] .start#dt
   [.] .end#dt
   [.] .startInclusive#bool <~ #Boolean.True
   [.] .endInclusive#bool <~ #Boolean.False
```

### #Recurrence

```polyglot
{#} #Recurrence
   [%] .description << "Repeating date-time pattern with optional end"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "recurrence"
   [.] .pattern#RecurrencePattern
   [.] .count#int <~ 0
   [.] .until#dt
```

---

## Supporting Enums

### #Precision

```polyglot
{#} #Precision
   [%] .description << "Date-time resolution level"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "precision"
   [.] .Year
   [.] .Month
   [.] .Day
   [.] .Hour
   [.] .Minute
   [.] .Second
   [.] .Millisecond
   [.] .Microsecond
   [.] .Nanosecond
```

### #RecurrencePattern

`#RecurrencePattern.Custom` has a nested `.rule#string` value sub-field for user-defined recurrence rules (cron-like or RRULE).

```polyglot
{#} #RecurrencePattern
   [%] .description << "Recurrence frequency pattern"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "recurrencepattern"
   [.] .Daily
   [.] .Weekly
   [.] .Monthly
   [.] .Yearly
   [.] .Custom
      [.] .rule#string
```

### #CalendarSystem

```polyglot
{#} #CalendarSystem
   [%] .description << "Supported calendar system identifier"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "calendarsystem"
   [.] .Gregorian
   [.] .Hijri
   [.] .Hebrew
   [.] .Chinese
   [.] .Persian
   [.] .Buddhist
   [.] .Hindu
   [.] .Japanese
   [.] .Ethiopian
   [.] .Coptic
   [.] .Custom
```

### #Weekday

```polyglot
{#} #Weekday
   [%] .description << "Day of the week"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "weekday"
   [.] .Monday
   [.] .Tuesday
   [.] .Wednesday
   [.] .Thursday
   [.] .Friday
   [.] .Saturday
   [.] .Sunday
```

### #Month

Gregorian month names. Other calendars define their own month enums.

```polyglot
{#} #Month
   [%] .description << "Gregorian calendar month"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "month"
   [.] .January
   [.] .February
   [.] .March
   [.] .April
   [.] .May
   [.] .June
   [.] .July
   [.] .August
   [.] .September
   [.] .October
   [.] .November
   [.] .December
```

### #DayBoundary

```polyglot
{#} #DayBoundary
   [%] .description << "When a calendar day begins"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "dayboundary"
   [.] .Midnight
   [.] .Sunset
   [.] .Sunrise
   [.] .Noon
```

### #WeekSystem

Each variant carries a `.config#BusinessWeek` sub-field that defines the week configuration for that system.

```polyglot
{#} #WeekSystem
   [%] .description << "Week system standard"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "weeksystem"
   [.] .ISO
      [.] .config#BusinessWeek
   [.] .US
      [.] .config#BusinessWeek
   [.] .MiddleEastern
      [.] .config#BusinessWeek
   [.] .Custom
      [.] .config#BusinessWeek
```

### #BusinessWeek

All fields must be explicitly set by the user. No defaults are assumed for regional norms.

```polyglot
{#} #BusinessWeek
   [%] .description << "Configurable work/off day schedule"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "businessweek"
   [.] .firstDay#Weekday
   [.] .workDays#array.Weekday
   [.] .offDays#array.Weekday
   [.] .hoursPerDay#int
```

### #MonthStructure

```polyglot
{#} #MonthStructure
   [%] .description << "How a calendar organizes month lengths"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "monthstructure"
   [.] .Fixed
   [.] .Mixed
   [.] .Lunisolar
   [.] .Lunar
```

---

## Calendar Infrastructure

### #CalendarProjection

Defines how any calendar maps from epoch. Every calendar is a projection of `.Instant.epoch` (Unix epoch seconds). Converting between calendars always goes through epoch: CalendarA -> epoch -> CalendarB.

```polyglot
{#} #CalendarProjection
   [%] .description << "Epoch-to-calendar mapping definition"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "calendarprojection"
   [.] .epochOffset#int
   [.] .leapRule#LeapRule
   [.] .monthStructure#MonthStructure
```

### #LeapRule

`#LeapRule.FixedCycle` has nested value sub-fields for cycle configuration. `#LeapRule.Custom` uses flexible fields for user-defined leap logic.

```polyglot
{#} #LeapRule
   [%] .description << "Leap year calculation method"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "leaprule"
   [.] .None
   [.] .FixedCycle
      [.] .cycleYears#int
      [.] .leapYears#array.int
   [.] .Astronomical
   [.] .Tabular
   [.] .Custom
      [:] :rule
```

---

## Calendar-Specific Date Types

### #GregorianDate

```polyglot
{#} #GregorianDate
   [%] .description << "Gregorian calendar date"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "gregoriandate"
   [.] .year#int
   [.] .month#Month
   [.] .day#int
```

### #HijriDate

The Hijri calendar is regional -- month starts depend on moon sighting authority. Saudi uses Umm al-Qura (astronomical), Pakistan/India often differ by 1-2 days. The design supports regional authorities, multiple methods, and fully custom Hijri configurations.

```polyglot
{#} #HijriDate
   [%] .description << "Islamic Hijri calendar date"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "hijridate"
   [.] .year#int
   [.] .month#HijriMonth
   [.] .day#int
   [.] .authority#HijriAuthority
   [.] .method#HijriMethod
   [.] .leap#HijriLeap
```

### #HijriMonth

```polyglot
{#} #HijriMonth
   [%] .description << "Islamic Hijri calendar month"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "hijrimonth"
   [.] .Muharram
   [.] .Safar
   [.] .RabiAlAwwal
   [.] .RabiAlThani
   [.] .JumadaAlUla
   [.] .JumadaAlThani
   [.] .Rajab
   [.] .Shaban
   [.] .Ramadan
   [.] .Shawwal
   [.] .DhuAlQidah
   [.] .DhuAlHijjah
```

### #HijriAuthority

`#HijriAuthority.Custom` has a nested `.name#string` value sub-field and flexible `:rules` for user-defined sighting or calculation rules.

```polyglot
{#} #HijriAuthority
   [%] .description << "Authority that determines Hijri month start"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "hijriauthority"
   [.] .UmmAlQura
   [.] .Local
   [.] .Regional
   [.] .Custom
      [.] .name#string
      [:] :rules
```

### #HijriMethod

`#HijriMethod.Custom` has a nested `.name#string` value sub-field and flexible `:logic` for user-defined calculation or sighting logic.

```polyglot
{#} #HijriMethod
   [%] .description << "Method for determining Hijri month start"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "hijrimethod"
   [.] .Tabular
   [.] .Astronomical
   [.] .Observational
   [.] .Custom
      [.] .name#string
      [:] :logic
```

### #HijriLeap

`#HijriLeap.Custom` uses flexible `:rule` for user-defined leap calculation.

```polyglot
{#} #HijriLeap
   [%] .description << "Hijri leap year calculation method"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "hijrileap"
   [.] .Tabular30
   [.] .UmmAlQura
   [.] .Custom
      [:] :rule
```

### #HebrewDate

```polyglot
{#} #HebrewDate
   [%] .description << "Hebrew calendar date"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "hebrewdate"
   [.] .year#int
   [.] .month#HebrewMonth
   [.] .day#int
```

### #HebrewMonth

```polyglot
{#} #HebrewMonth
   [%] .description << "Hebrew calendar month"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "hebrewmonth"
   [.] .Tishrei
   [.] .Cheshvan
   [.] .Kislev
   [.] .Tevet
   [.] .Shevat
   [.] .AdarI
   [.] .AdarII
   [.] .Nisan
   [.] .Iyyar
   [.] .Sivan
   [.] .Tammuz
   [.] .Av
   [.] .Elul
```

### #ChineseDate

```polyglot
{#} #ChineseDate
   [%] .description << "Chinese calendar date with stem-branch cycle"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "chinesedate"
   [.] .year#int
   [.] .cycle#int
   [.] .month#int
   [.] .leapMonth#bool <~ #Boolean.False
   [.] .day#int
   [.] .stem#HeavenlyStem
   [.] .branch#EarthlyBranch
```

### #HeavenlyStem

```polyglot
{#} #HeavenlyStem
   [%] .description << "Chinese celestial stem in the sexagenary cycle"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "heavenlystem"
   [.] .Jia
   [.] .Yi
   [.] .Bing
   [.] .Ding
   [.] .Wu
   [.] .Ji
   [.] .Geng
   [.] .Xin
   [.] .Ren
   [.] .Gui
```

### #EarthlyBranch

```polyglot
{#} #EarthlyBranch
   [%] .description << "Chinese terrestrial branch in the sexagenary cycle"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "earthlybranch"
   [.] .Zi
   [.] .Chou
   [.] .Yin
   [.] .Mao
   [.] .Chen
   [.] .Si
   [.] .Wu
   [.] .Wei
   [.] .Shen
   [.] .You
   [.] .Xu
   [.] .Hai
```

### #PersianDate

```polyglot
{#} #PersianDate
   [%] .description << "Persian (Solar Hijri) calendar date"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "persiandate"
   [.] .year#int
   [.] .month#PersianMonth
   [.] .day#int
```

### #PersianMonth

```polyglot
{#} #PersianMonth
   [%] .description << "Persian calendar month"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "persianmonth"
   [.] .Farvardin
   [.] .Ordibehesht
   [.] .Khordad
   [.] .Tir
   [.] .Mordad
   [.] .Shahrivar
   [.] .Mehr
   [.] .Aban
   [.] .Azar
   [.] .Dey
   [.] .Bahman
   [.] .Esfand
```

### #BuddhistDate

Uses Gregorian `#Month` for months (Buddhist Era = Gregorian + 543).

```polyglot
{#} #BuddhistDate
   [%] .description << "Buddhist calendar date"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "buddhistdate"
   [.] .year#int
   [.] .month#Month
   [.] .day#int
```

### #HinduDate

```polyglot
{#} #HinduDate
   [%] .description << "Hindu calendar date with era and lunar fortnight"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "hindudate"
   [.] .year#int
   [.] .era#HinduEra
   [.] .month#HinduMonth
   [.] .day#int
   [.] .paksha#Paksha
```

### #HinduEra

```polyglot
{#} #HinduEra
   [%] .description << "Hindu calendar era system"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "hinduera"
   [.] .VikramSamvat
   [.] .Saka
```

### #HinduMonth

```polyglot
{#} #HinduMonth
   [%] .description << "Hindu calendar month"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "hindumonth"
   [.] .Chaitra
   [.] .Vaishakha
   [.] .Jyeshtha
   [.] .Ashadha
   [.] .Shravana
   [.] .Bhadrapada
   [.] .Ashvina
   [.] .Kartika
   [.] .Margashirsha
   [.] .Pausha
   [.] .Magha
   [.] .Phalguna
```

### #Paksha

```polyglot
{#} #Paksha
   [%] .description << "Hindu lunar fortnight"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "paksha"
   [.] .Shukla
   [.] .Krishna
```

### #JapaneseDate

Uses Gregorian `#Month` for months.

```polyglot
{#} #JapaneseDate
   [%] .description << "Japanese calendar date with imperial era"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "japanesedate"
   [.] .year#int
   [.] .era#JapaneseEra
   [.] .eraYear#int
   [.] .month#Month
   [.] .day#int
```

### #JapaneseEra

Known modern eras are fixed enum fields. Users can add older historical eras via the flexible `:historical` field.

```polyglot
{#} #JapaneseEra
   [%] .description << "Japanese imperial era"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "japaneseera"
   [.] .Reiwa
   [.] .Heisei
   [.] .Showa
   [.] .Taisho
   [.] .Meiji
   [:] :historical
```

### #EthiopianDate

```polyglot
{#} #EthiopianDate
   [%] .description << "Ethiopian calendar date"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "ethiopiandate"
   [.] .year#int
   [.] .month#EthiopianMonth
   [.] .day#int
```

### #EthiopianMonth

```polyglot
{#} #EthiopianMonth
   [%] .description << "Ethiopian calendar month"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "ethiopianmonth"
   [.] .Meskerem
   [.] .Tikimt
   [.] .Hidar
   [.] .Tahsas
   [.] .Tir
   [.] .Yekatit
   [.] .Megabit
   [.] .Miyazya
   [.] .Ginbot
   [.] .Sene
   [.] .Hamle
   [.] .Nehase
   [.] .Pagume
```

### #CopticDate

```polyglot
{#} #CopticDate
   [%] .description << "Coptic calendar date"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "copticdate"
   [.] .year#int
   [.] .month#CopticMonth
   [.] .day#int
```

### #CopticMonth

```polyglot
{#} #CopticMonth
   [%] .description << "Coptic calendar month"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "copticmonth"
   [.] .Thout
   [.] .Paopi
   [.] .Hathor
   [.] .Koiak
   [.] .Tobi
   [.] .Meshir
   [.] .Paremhat
   [.] .Parmouti
   [.] .Pashons
   [.] .Paoni
   [.] .Epip
   [.] .Mesori
   [.] .PiKogiEnavot
```

### #CustomCalendar

User-extensible calendar type. Fixed fields define the basic structure; flexible fields allow user-defined month names, leap rules, and epoch offset.

```polyglot
{#} #CustomCalendar
   [%] .description << "User-defined calendar system"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "customcalendar"
   [.] .name#string
   [.] .epochOffset#int
   [:] :months
   [:] :leapRule
```

---

## Non-Standard Time Units

Some calendars divide the day differently than hours/minutes/seconds. These all map back to epoch seconds -- they are alternative projections of `.Instant`.

### #ChineseTime

```polyglot
{#} #ChineseTime
   [%] .description << "Chinese traditional time divisions"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "chinesetime"
   [.] .shichen#ChineseShichen
   [.] .ke#int
   [.] .fen#int
   [.] .yeGeng#int
```

### #ChineseShichen

The twelve double-hours (shichen) that divide the Chinese day.

```polyglot
{#} #ChineseShichen
   [%] .description << "Chinese double-hour period"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "chineseshichen"
   [.] .Zi
   [.] .Chou
   [.] .Yin
   [.] .Mao
   [.] .Chen
   [.] .Si
   [.] .Wu
   [.] .Wei
   [.] .Shen
   [.] .You
   [.] .Xu
   [.] .Hai
```

### #HinduTime

```polyglot
{#} #HinduTime
   [%] .description << "Hindu traditional time divisions"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "hindutime"
   [.] .prahara#int
   [.] .muhurta#int
   [.] .ghati#int
   [.] .pala#int
   [.] .vipala#int
```

### #DecimalTime

French Republican decimal time system (10 hours/day, 100 minutes/hour, 100 seconds/minute).

```polyglot
{#} #DecimalTime
   [%] .description << "French Republican decimal time"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "decimaltime"
   [.] .hour#int
   [.] .minute#int
   [.] .second#int
```

### #CustomTimeUnit

User-defined time division. Fixed fields define the basic unit; flexible fields allow user-defined subdivisions and epoch-to-unit conversion mapping.

```polyglot
{#} #CustomTimeUnit
   [%] .description << "User-defined time division"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "customtimeunit"
   [.] .name#string
   [.] .unitsPerDay#int
   [:] :subdivisions
   [:] :mapping
```

---

## Cultural Types

### #Holiday

```polyglot
{#} #Holiday
   [%] .description << "Named holiday with optional recurrence"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "holiday"
   [.] .name#string
   [.] .date#Date
   [.] .recurring#bool <~ #Boolean.True
   [.] .calendar#CalendarSystem <~ #CalendarSystem.Gregorian
   [:] :extra
```

### #Observance

```polyglot
{#} #Observance
   [%] .description << "Named observance with type classification"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "observance"
   [.] .name#string
   [.] .date#Date
   [.] .type#ObservanceType
   [.] .recurring#bool <~ #Boolean.True
   [:] :extra
```

### #ObservanceType

```polyglot
{#} #ObservanceType
   [%] .description << "Classification of observance"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "observancetype"
   [.] .Religious
   [.] .National
   [.] .Cultural
   [.] .Personal
```

### #Season

```polyglot
{#} #Season
   [%] .description << "Named season with start and end dates"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Value
   [#] %##Alias << "season"
   [.] .name#string
   [.] .start#Date
   [.] .end#Date
   [.] .type#SeasonType
   [:] :extra
```

### #SeasonType

```polyglot
{#} #SeasonType
   [%] .description << "Classification of season"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###Enum
   [#] %##Alias << "seasontype"
   [.] .Astronomical
   [.] .Meteorological
   [.] .Cultural
   [.] .Agricultural
   [.] .Custom
```

---

## Related

- [[enums]] -- stdlib enum types
- [[structs]] -- stdlib struct types (#path, #Queue)
- [[boolean]] -- #Boolean enum type
- [[string]] -- #String foundation type
- [[syntax/types/schema-properties]] -- `##`/`###` schema property rules
- [[syntax/types/INDEX|types]] -- full type system specification
