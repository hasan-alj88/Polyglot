---
audience: pg-coder
type: reference
updated: 2026-04-09
---

# Calendar-Specific Date Types

<!-- @source:calendar-date-types -->

### #GregorianDate

```polyglot
{#} #GregorianDate
   [%] .description << "Gregorian calendar date"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarValue
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
   [#] ##Scalar
   [#] ###ScalarValue
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
   [#] ##Scalar
   [#] ###ScalarEnum
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
   [#] ##Scalar
   [#] ###ScalarEnum
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
   [#] ##Scalar
   [#] ###ScalarEnum
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
   [#] ##Scalar
   [#] ###ScalarEnum
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
   [#] ##Scalar
   [#] ###ScalarValue
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
   [#] ##Scalar
   [#] ###ScalarEnum
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
   [#] ##Scalar
   [#] ###ScalarValue
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
   [#] ##Scalar
   [#] ###ScalarEnum
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
   [#] ##Scalar
   [#] ###ScalarEnum
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
   [#] ##Scalar
   [#] ###ScalarValue
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
   [#] ##Scalar
   [#] ###ScalarEnum
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
   [#] ##Scalar
   [#] ###ScalarValue
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
   [#] ##Scalar
   [#] ###ScalarValue
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
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "hinduera"
   [.] .VikramSamvat
   [.] .Saka
```

### #HinduMonth

```polyglot
{#} #HinduMonth
   [%] .description << "Hindu calendar month"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarEnum
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
   [#] ##Scalar
   [#] ###ScalarEnum
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
   [#] ##Scalar
   [#] ###ScalarValue
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
   [#] ##Scalar
   [#] ###ScalarEnum
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
   [#] ##Scalar
   [#] ###ScalarValue
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
   [#] ##Scalar
   [#] ###ScalarEnum
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
   [#] ##Scalar
   [#] ###ScalarValue
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
   [#] ##Scalar
   [#] ###ScalarEnum
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
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "customcalendar"
   [.] .name#string
   [.] .epochOffset#int
   [:] :months
   [:] :leapRule
```

---

See also: [[calendar-infrastructure]], [[supporting-enums]], [[non-standard-time]], [[main-type]]
