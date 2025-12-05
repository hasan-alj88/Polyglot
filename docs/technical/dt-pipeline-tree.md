# DT.* Pipeline Tree

**Complete namespace hierarchy for DateTime pipelines**

**Last Updated**: 2025-11-30
**Related**: DateTime String Literal Specification

---

## Complete Pipeline Tree

```
DT.*                                    // Root namespace for DateTime pipelines
│
├─ DT.Now                              // Current instant (returns pg\dt)
│   └─ DT.Now.Local                    // Current instant in local timezone
│
├─ DT.Time.*                           // Time-only operations
│   └─ DT.Time                         // Specific time (param: "15:00", "3:00PM")
│
├─ DT.Day.*                            // Day-of-week operations
│   └─ DT.Day                          // Specific day (param: "Monday", "Fri")
│
├─ DT.Ago.*                            // Relative past time
│   └─ DT.Ago                          // Duration in past (param: "2h", "3d")
│
├─ DT.From.*                           // Relative future time
│   └─ DT.From                         // Duration from now (param: "2h", "30m")
│
├─ DT.ToNow                            // Duration from timestamp to now
│
├─ DT.Between                          // Duration between two instants
│
├─ DT.Since                            // Duration since instant
│
├─ DT.Until                            // Duration until instant
│
├─ DT.Every.*                          // Recurrence patterns
│   └─ DT.Every                        // Recurring pattern (param: "Sun", "5m", "15:00")
│
├─ DT.Daily.*                          // Daily recurrence (alias for Every with daily)
│   └─ DT.Daily                        // Every day at time (param: "3:00AM")
│
├─ DT.Weekly.*                         // Weekly recurrence
│   └─ DT.Weekly                       // Every week on day (param: "Monday 9:00AM")
│
├─ DT.Yearly.*                         // Yearly recurrence (Gregorian)
│   └─ DT.Yearly                       // Every year on date (param: "12-25", "12-25 3:00PM")
│
├─ DT.First.*                          // First weekday in month
│   └─ DT.First                        // Nth occurrence (param: "Monday in November")
│
├─ DT.Second.*                         // Second weekday in month
│   └─ DT.Second                       // Nth occurrence (param: "Sunday in June")
│
├─ DT.Third.*                          // Third weekday in month
│   └─ DT.Third                        // Nth occurrence
│
├─ DT.Fourth.*                         // Fourth weekday in month
│   └─ DT.Fourth                       // Nth occurrence (param: "Thursday in November")
│
├─ DT.Fifth.*                          // Fifth weekday in month
│   └─ DT.Fifth                        // Nth occurrence
│
├─ DT.Last.*                           // Last weekday in month
│   └─ DT.Last                         // Last occurrence (param: "Friday in Ramadan")
│
├─ DT.Range.*                          // Date/time ranges
│   └─ DT.Range                        // Range duration (param: "09:00 to 17:00")
│
├─ DT.Seconds.*                        // Duration in seconds
│   └─ DT.Seconds                      // Duration (param: "30")
│
├─ DT.Minutes.*                        // Duration in minutes
│   └─ DT.Minutes                      // Duration (param: "5")
│
├─ DT.Hours.*                          // Duration in hours
│   └─ DT.Hours                        // Duration (param: "2")
│
├─ DT.Days.*                           // Duration in days
│   └─ DT.Days                         // Duration (param: "7")
│
├─ DT.Weeks.*                          // Duration in weeks
│   └─ DT.Weeks                        // Duration (param: "2")
│
├─ DT.Months.*                         // Duration in months (calendar-aware)
│   └─ DT.Months                       // Duration (param: "3")
│
├─ DT.Years.*                          // Duration in years (calendar-aware)
│   └─ DT.Years                        // Duration (param: "1")
│
├─ DT.ISO.*                            // ISO 8601 formats
│   └─ DT.ISO                          // ISO datetime (param: "2025-11-30T15:00:00Z")
│
├─ DT.Business.*                       // Business day operations
│   ├─ DT.Business.Days                // N business days (param: "5")
│   ├─ DT.Business.Day                 // Next/prev business day (param: "next", "prev")
│   └─ DT.Business.Week.*
│       └─ DT.Business.Week.Start      // Start of business week
│
├─ DT.Gregorian.*                      // Gregorian calendar (algorithmic)
│   ├─ DT.Gregorian                    // Gregorian date (param: "2025-11-30")
│   ├─ DT.Gregorian.Yearly.*
│   │   └─ DT.Gregorian.Yearly         // Yearly recurrence (param: "12-25")
│   └─ DT.Gregorian.{Operations}       // All operations in Gregorian context
│
├─ DT.Julian.*                         // Julian calendar (algorithmic)
│   ├─ DT.Julian                       // Julian date (param: "2025-11-17")
│   └─ DT.Julian.{Operations}          // All operations in Julian context
│
├─ DT.Assyrian.*                       // Assyrian/Syriac calendar (algorithmic)
│   ├─ DT.Assyrian                     // Assyrian date (param: "2336-11-30")
│   └─ DT.Assyrian.{Operations}        // All operations in Assyrian context
│
├─ DT.Hijri.*                          // Islamic calendar (PROFILE-AWARE extendable enumeration)
│   │                                  // Namespace: #DT.Hijri.*
│   ├─ DT.Hijri                        // Hijri date (algorithmic - no profile)
│   │   ├─ DT.Hijri                    // Date (param: "1447-09-01")
│   │   ├─ DT.Hijri.Yearly             // Yearly recurrence (param: "09-01")
│   │   └─ DT.Hijri.{Operations}       // All operations in algorithmic Hijri
│   │
│   ├─ DT.Hijri.SaudiArabia.*          // Built-in: Umm al-Qura (Saudi Arabia)
│   │   ├─ DT.Hijri.SaudiArabia        // Date with Saudi profile
│   │   ├─ DT.Hijri.SaudiArabia.Yearly // Yearly recurrence with Saudi profile
│   │   ├─ DT.Hijri.SaudiArabia.First  // First weekday in month
│   │   ├─ DT.Hijri.SaudiArabia.Last   // Last weekday in month
│   │   └─ DT.Hijri.SaudiArabia.{Operations} // All operations
│   │
│   ├─ DT.Hijri.UKMoonSighting.*       // Built-in: UK local moon sighting
│   │   ├─ DT.Hijri.UKMoonSighting
│   │   ├─ DT.Hijri.UKMoonSighting.Yearly
│   │   └─ DT.Hijri.UKMoonSighting.{Operations}
│   │
│   ├─ DT.Hijri.Turkey.*               // Built-in: Diyanet (Turkey)
│   │   └─ DT.Hijri.Turkey.{Operations}
│   │
│   ├─ DT.Hijri.Egypt.*                // Built-in: Dar al-Ifta (Egypt)
│   │   └─ DT.Hijri.Egypt.{Operations}
│   │
│   ├─ DT.Hijri.ISNA.*                 // Built-in: Islamic Society of North America
│   │   └─ DT.Hijri.ISNA.{Operations}
│   │
│   ├─ DT.Hijri.MWL.*                  // Built-in: Muslim World League
│   │   └─ DT.Hijri.MWL.{Operations}
│   │
│   └─ DT.Hijri.{UserDefined}.*        // User-defined profiles (EXTENDABLE)
│       ├─ DT.Hijri.MyCompanyHR
│       ├─ DT.Hijri.MyCompanyHR.Yearly
│       └─ DT.Hijri.MyCompanyHR.{Operations}
│
├─ DT.Hebrew.*                         // Hebrew calendar (PROFILE-AWARE extendable enumeration)
│   │                                  // Namespace: #DT.Hebrew.*
│   ├─ DT.Hebrew                       // Hebrew date (algorithmic - no profile)
│   │   ├─ DT.Hebrew                   // Date (param: "5784-10-03")
│   │   ├─ DT.Hebrew.Yearly            // Yearly recurrence
│   │   └─ DT.Hebrew.{Operations}      // All operations in algorithmic Hebrew
│   │
│   ├─ DT.Hebrew.Sephardic.*           // Built-in: Sephardic rabbinical authority
│   │   └─ DT.Hebrew.Sephardic.{Operations}
│   │
│   ├─ DT.Hebrew.Ashkenazi.*           // Built-in: Ashkenazi rabbinical authority
│   │   └─ DT.Hebrew.Ashkenazi.{Operations}
│   │
│   ├─ DT.Hebrew.Israel.*              // Built-in: Chief Rabbinate of Israel
│   │   └─ DT.Hebrew.Israel.{Operations}
│   │
│   └─ DT.Hebrew.{UserDefined}.*       // User-defined profiles (EXTENDABLE)
│       └─ DT.Hebrew.{UserDefined}.{Operations}
│
├─ DT.Chinese.*                        // Chinese calendar (PROFILE-AWARE extendable enumeration)
│   │                                  // Namespace: #DT.Chinese.*
│   ├─ DT.Chinese                      // Chinese date (algorithmic - no profile)
│   │   ├─ DT.Chinese                  // Date (param: "4722-01-15")
│   │   ├─ DT.Chinese.Yearly           // Yearly recurrence
│   │   └─ DT.Chinese.{Operations}     // All operations in algorithmic Chinese
│   │
│   ├─ DT.Chinese.Mainland.*           // Built-in: Mainland China variant
│   │   └─ DT.Chinese.Mainland.{Operations}
│   │
│   ├─ DT.Chinese.HongKong.*           // Built-in: Hong Kong variant
│   │   └─ DT.Chinese.HongKong.{Operations}
│   │
│   ├─ DT.Chinese.Taiwan.*             // Built-in: Taiwan variant
│   │   └─ DT.Chinese.Taiwan.{Operations}
│   │
│   └─ DT.Chinese.{UserDefined}.*      // User-defined profiles (EXTENDABLE)
│       └─ DT.Chinese.{UserDefined}.{Operations}
│
├─ DT.Buddhist.*                       // Buddhist calendar (algorithmic)
│   ├─ DT.Buddhist                     // Buddhist date (param: "2568-11-30")
│   └─ DT.Buddhist.{Operations}        // All operations in Buddhist context
│
├─ DT.Persian.*                        // Persian (Solar Hijri) calendar (algorithmic)
│   ├─ DT.Persian                      // Persian date (param: "1404-09-09")
│   └─ DT.Persian.{Operations}         // All operations in Persian context
│
├─ DT.Coptic.*                         // Coptic calendar (algorithmic)
│   ├─ DT.Coptic                       // Coptic date
│   └─ DT.Coptic.{Operations}          // All operations in Coptic context
│
└─ DT.Ethiopian.*                      // Ethiopian calendar (algorithmic)
    ├─ DT.Ethiopian                    // Ethiopian date
    └─ DT.Ethiopian.{Operations}       // All operations in Ethiopian context
```

---

## Trigger Pipeline Tree (T.DT.*)

**Pattern**: `T.DT.*` mirrors `DT.*` but for triggers

**Semantic**: `T.DT.{Pattern}"{params}"` = Trigger when `DT.{Pattern}"{params}"` equals `DT.Now""`

```
T.DT.*                                 // Trigger namespace (mirrors DT.*)
│
├─ T.DT.Daily                          // Trigger: Every day at time
├─ T.DT.Weekly                         // Trigger: Every week on day
├─ T.DT.Every                          // Trigger: Generic recurrence
├─ T.DT.Yearly                         // Trigger: Yearly (Gregorian)
│
├─ T.DT.Gregorian.*
│   └─ T.DT.Gregorian.Yearly           // Trigger: Yearly Gregorian
│
├─ T.DT.Hijri.*                        // Islamic calendar triggers
│   ├─ T.DT.Hijri.Yearly               // Trigger: Yearly Hijri (algorithmic)
│   │
│   ├─ T.DT.Hijri.SaudiArabia.*        // PROFILE-AWARE TRIGGERS (Saudi)
│   │   ├─ T.DT.Hijri.SaudiArabia.Yearly
│   │   └─ T.DT.Hijri.SaudiArabia.Last
│   │
│   ├─ T.DT.Hijri.UKMoonSighting.*     // PROFILE-AWARE TRIGGERS (UK)
│   │   └─ T.DT.Hijri.UKMoonSighting.{Operations}
│   │
│   └─ T.DT.Hijri.MyCompanyHR.*        // User-defined profile triggers
│       ├─ T.DT.Hijri.MyCompanyHR.Yearly
│       └─ T.DT.Hijri.MyCompanyHR.{Operations}
│
├─ T.DT.Hebrew.*                       // Hebrew calendar triggers
│   ├─ T.DT.Hebrew.Sephardic.*
│   └─ T.DT.Hebrew.{UserDefined}.*
│
├─ T.DT.Chinese.*                      // Chinese calendar triggers
│   └─ T.DT.Chinese.{UserDefined}.*
│
├─ T.DT.First                          // Trigger: First weekday in month
├─ T.DT.Second                         // Trigger: Second weekday in month
├─ T.DT.Third                          // Trigger: Third weekday in month
├─ T.DT.Fourth                         // Trigger: Fourth weekday in month
├─ T.DT.Fifth                          // Trigger: Fifth weekday in month
└─ T.DT.Last                           // Trigger: Last weekday in month
```

---

## Shorthand Forms (Direct DT + String)

**Pattern**: `DT"{param}"` without namespace = inferred context

```
DT.*                                   // Shorthand namespace
│
├─ DT"2025-11-30"                     // Date (Gregorian assumed)
├─ DT"15:00"                          // Time (24-hour)
├─ DT"3:00PM"                         // Time (12-hour)
├─ DT"Monday"                         // Day of week
├─ DT"Mon"                            // Day of week (abbreviated)
├─ DT"5m"                             // Duration (5 minutes)
├─ DT"30s"                            // Duration (30 seconds)
├─ DT"2h"                             // Duration (2 hours)
├─ DT"1d"                             // Duration (1 day)
├─ DT"1w"                             // Duration (1 week)
├─ DT"1mo"                            // Duration (1 month)
├─ DT"1y"                             // Duration (1 year)
├─ DT"Friday 3:00PM"                  // Time + DayOfWeek
├─ DT"2025-11-30 15:00"               // DateTime
├─ DT"2025-11-30 Sunday"              // Date + DayOfWeek (validated)
└─ DT"2025-11-30 Sunday 15:00"        // Full specification (all validated)
```

---

## Reserved Enumeration Namespaces

**Extendable Enumerations** (users can define custom profiles):

```
#DT.*                                  // Reserved enumeration root
│
├─ #DT.Hijri.*                        // Islamic calendar profiles (EXTENDABLE)
│   │                                 // General form: DT.Hijri.{Profile}"..."
│   ├─ #DT.Hijri                      // Algorithmic (no profile)
│   ├─ #DT.Hijri.SaudiArabia
│   ├─ #DT.Hijri.UKMoonSighting
│   ├─ #DT.Hijri.Turkey
│   ├─ #DT.Hijri.Egypt
│   ├─ #DT.Hijri.ISNA
│   ├─ #DT.Hijri.MWL
│   └─ #DT.Hijri.{UserDefined}        // User adds via [#] definition
│
├─ #DT.Hebrew.*                       // Hebrew calendar profiles (EXTENDABLE)
│   │                                 // General form: DT.Hebrew.{Profile}"..."
│   ├─ #DT.Hebrew                     // Algorithmic (no profile)
│   ├─ #DT.Hebrew.Sephardic
│   ├─ #DT.Hebrew.Ashkenazi
│   ├─ #DT.Hebrew.Israel
│   └─ #DT.Hebrew.{UserDefined}
│
├─ #DT.Chinese.*                      // Chinese calendar profiles (EXTENDABLE)
│   │                                 // General form: DT.Chinese.{Profile}"..."
│   ├─ #DT.Chinese                    // Algorithmic (no profile)
│   ├─ #DT.Chinese.Mainland
│   ├─ #DT.Chinese.HongKong
│   ├─ #DT.Chinese.Taiwan
│   └─ #DT.Chinese.{UserDefined}
│
└─ #DT.Business.Week.*                // Business week definitions (EXTENDABLE)
    ├─ #DT.Business.Week.Standard
    ├─ #DT.Business.Week.TwoDayWeekend.SatSun
    ├─ #DT.Business.Week.TwoDayWeekend.FriSat
    ├─ #DT.Business.Week.TwoDayWeekend.ThuFri
    └─ #DT.Business.Week.{UserDefined}
```

---

## Operations Available in Calendar Contexts

**{Operations}** placeholder represents these operations available in any calendar context:

```
{Calendar}.{Operation}                 // Operations in calendar-specific context
│
├─ {Calendar}.Yearly                  // Yearly recurrence in this calendar
├─ {Calendar}.First                   // First weekday in month
├─ {Calendar}.Second                  // Second weekday in month
├─ {Calendar}.Third                   // Third weekday in month
├─ {Calendar}.Fourth                  // Fourth weekday in month
├─ {Calendar}.Fifth                   // Fifth weekday in month
├─ {Calendar}.Last                    // Last weekday in month
├─ {Calendar}.Every                   // Recurrence in this calendar
├─ {Calendar}.Ago                     // Relative past in this calendar
├─ {Calendar}.From                    // Relative future in this calendar
└─ {Calendar}.Range                   // Range in this calendar
```

**Example expansions**:
- `DT.Hijri.SaudiArabia.Last"Friday in Ramadan"` - Last Friday in Ramadan (Saudi profile)
- `DT.Hebrew.Sephardic.Yearly"07-15"` - Yearly Sukkot (Sephardic authority)
- `DT.Chinese.HongKong.First"Monday in Month 1"` - First Monday of Chinese New Year (HK variant)

---

## Pipeline Count Summary

**Total Namespaces**:
- **Core Operations**: ~25 (Now, Time, Day, Ago, From, Every, Daily, etc.)
- **Calendar Namespaces**: 9 base calendars (Gregorian, Hijri, Julian, Assyrian, Chinese, Hebrew, Buddhist, Persian, Ethiopian)
- **Profile Namespaces**: 3 extendable (Hijri.*, Hebrew.*, Chinese.*)
- **Built-in Profiles**: ~16 (7 Islamic + 4 Hebrew + 4 Chinese + 1 Business.Week)
- **User-Definable**: Unlimited (extendable enumerations)

**Trigger Mirroring**:
- Every `DT.*` pattern has corresponding `T.DT.*` trigger variant

**Shorthand Forms**:
- ~15 shorthand formats (`DT"{param}"`)

**Total Pipeline Patterns**: 100+ built-in, unlimited with user extensions

---

## Implementation Notes

### Pipeline Registration

**Built-in pipelines** are registered in standard library:
```rust
// Example pipeline registration
register_pipeline("DT.Now", dt_now_pipeline);
register_pipeline("DT.Hijri.SaudiArabia", dt_hijri_saudi_pipeline);
```

**User-defined profiles** are dynamically registered via enumeration definitions:
```polyglot
[#] DT.Hijri.MyCompanyHR
[s] |U.YAML.Load"\\FileDir\\MyCompanyHRHijri.yaml"
[s][!] *
[X]
```

### Namespace Resolution

**Lookup order**:
1. Check user-defined profiles (extendable enumerations)
2. Check built-in profiles
3. Check calendar-specific operations
4. Check core operations
5. Error: Unknown pipeline

### Profile Loading

**YAML configuration loaded at**:
- Enumeration definition time (`[#]` marker)
- Cached for performance (reload on file change)

---

**Related Documents**:
- DateTime String Literal Specification: `docs/technical/datetime-string-literal-specification.md`
- Reserved Enumeration Schema: `docs/user/audit/reserved-enumeration-schema-decisions.md`
