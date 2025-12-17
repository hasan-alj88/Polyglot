# DateTime Extension Guide

**Document Status**: User Guide
**Last Updated**: 2025-11-30
**Related**: DateTime String Literal Specification, DT Pipeline Tree, DateTime Formatted String Grammar

---

## Overview

This guide explains how to extend Polyglot's DateTime system with custom timezones, calendar profiles, aliases, and holiday definitions using **extendable reserved enumerations**.

**What You Can Extend**:
- 🌍 **Timezones** - Custom timezone definitions with DST rules
- 📅 **Calendar Profiles** - Organization-specific calendar observations
- 🔖 **Aliases** - Shortcuts for commonly used patterns
- 🎉 **Holidays** - Named holidays and observances
- 📐 **Custom Formats** - Reusable format macros

---

## Extension Pattern

All DateTime extensions follow the same enumeration pattern:

```polyglot
[#] DT.{Namespace}.{YourName}
[s] |{pipeline to initialize}
[s][!] *
[X]
```

**Key Components**:
1. `[#]` - Enumeration definition
2. Namespace: `DT.TimeZone.*`, `DT.Hijri.*`, `DT.Alias.*`, etc.
3. Your custom name (must be valid identifier)
4. Initialization pipeline (loads YAML, sets properties, etc.)
5. `[!]` - Exports enumeration for use in DT.* pipelines

---

## 1. Custom Timezones

### Basic Timezone Definition

**Use Case**: Define a custom timezone for your organization or region.

**File**: `timezones/MyCompanyTimezone.yaml`
```yaml
timezone:
  name: "MyCompany"
  description: "Company timezone with custom DST rules"
  gmt_offset: "-03:00"      # 3 hours behind UTC
  dst_enabled: true
  dst_offset: "+01:00"      # 1 hour forward during DST
  dst_start: "*-03-01"      # March 1 every year
  dst_end: "*-09-01"        # September 1 every year
```

**Polyglot Enumeration**:
```polyglot
[#] DT.TimeZone.MyCompany
[<] .config: pg\yaml << U.YAML.Load"\\FileDir\\timezones\\MyCompanyTimezone.yaml"
[<] .gmt_offset: pg\dt << DT.Parse(.config.timezone.gmt_offset)
[<] .dst_offset: pg\dt << DT.Parse(.config.timezone.dst_offset)
[<] .dst_start: pg\dt << DT.Parse(.config.timezone.dst_start)
[<] .dst_end: pg\dt << DT.Parse(.config.timezone.dst_end)
[s][!] *
[X]
```

**Usage**:
```polyglot
[<] meeting_time: pg\dt << DT.TimeZone.MyCompany"2025-11-30 15:00"
[<] deadline: pg\dt << DT.TimeZone.MyCompany.Weekly"Friday 5:00PM"
```

---

### Advanced: Dynamic Timezone (API-Based)

**Use Case**: Fetch timezone data from an external API.

```polyglot
[#] DT.TimeZone.Dynamic
[<] .api_response: pg\yaml << U.HTTP.Get"https://api.timeapi.io/api/TimeZone/zone?timeZone=America/Chicago"
[<] .gmt_offset: pg\dt << DT.Parse(.api_response.currentUtcOffset.seconds | U.Duration.FromSeconds)
[s][!] *
[X]
```

**Usage**:
```polyglot
[<] current_time: pg\dt << DT.TimeZone.Dynamic.Now""
```

---

## 2. Calendar Profiles

### Islamic Calendar Profile (Moon Sighting)

**Use Case**: Your organization follows a specific Islamic moon sighting authority.

**File**: `calendars/MyCompanyHijri.yaml`
```yaml
calendar:
  type: "Hijri"
  profile_name: "MyCompanyHR"
  description: "Company HR Hijri calendar with observed dates"

  # Manual overrides (Priority 1)
  observed_dates:
    # Format: ISO_DATE: HIJRI_DATE
    "2025-03-30": "1446-09-01"  # Ramadan 1446 observed start
    "2025-03-31": "1446-09-02"
    "2025-04-28": "1446-10-01"  # Shawwal 1446 observed start

  # API configuration (Priority 2)
  api:
    enabled: true
    url: "https://api.aladhan.com/v1/hijriCalendar"
    method: "Saudi"
    cache_days: 30

  # Calculation method (Priority 3 fallback)
  calculation:
    method: "ICU4X"
    algorithm: "Umm al-Qura"
```

**Polyglot Enumeration**:
```polyglot
[#] DT.Hijri.MyCompanyHR
[<] .config: pg\yaml << U.YAML.Load"\\FileDir\\calendars\\MyCompanyHijri.yaml"

[~] Priority 1: Check manual overrides
[<] .overrides: pg\yaml << .config.calendar.observed_dates
[?] .overrides | U.YAML.HasKey(@DT.Now"" | DT.Format.ISO8601)
  [@] .hijri_date: pg\string << .overrides[@DT.Now"" | DT.Format.ISO8601]
  [>]

[~] Priority 2: Check API cache
[?] .config.calendar.api.enabled == true
  [<] .api_result: pg\yaml << U.HTTP.Get(.config.calendar.api.url)
  [?] .api_result.status == 200
    [@] .hijri_date: pg\string << .api_result.data.hijri.date
    [>]

[~] Priority 3: Fallback to ICU4X calculation
[@] .hijri_date: pg\string << @DT.Now"" | DT.Convert.Hijri | DT.Format.ISO8601

[s][!] *
[X]
```

**Usage**:
```polyglot
[~] Get current Hijri date using company profile
[<] today_hijri: pg\dt << DT.Hijri.MyCompanyHR.Now""

[~] Get Ramadan start for company
[<] ramadan_start: pg\dt << DT.Hijri.MyCompanyHR.Ramadan.First""

[~] Recurring Friday prayer times
[<] friday_prayer: pg\dt << DT.Hijri.MyCompanyHR.Weekly"Friday 1:00PM"
```

---

### Hebrew Calendar Profile

**Use Case**: Define Sephardic vs Ashkenazi holiday observations.

**File**: `calendars/SephardicProfile.yaml`
```yaml
calendar:
  type: "Hebrew"
  profile_name: "Sephardic"

  # Holiday offset days
  holidays:
    Purim:
      observed_offset: 0      # No offset
    LagBaOmer:
      observed_offset: 0
    TuBishvat:
      observed_offset: 0

  # Custom holidays
  custom_holidays:
    - name: "Mimouna"
      date: "Nisan 15"        # Day after Passover
      enabled: true
```

**Polyglot Enumeration**:
```polyglot
[#] DT.Hebrew.Sephardic
[<] .config: pg\yaml << U.YAML.Load"\\FileDir\\calendars\\SephardicProfile.yaml"
[s][!] *
[X]
```

**Usage**:
```polyglot
[<] purim: pg\dt << DT.Hebrew.Sephardic"Adar 14"
[<] mimouna: pg\dt << DT.Hebrew.Sephardic"Nisan 15"
```

---

## 3. Aliases

### Basic Alias Definition

**Use Case**: Create shortcuts for frequently used datetime patterns.

```polyglot
[#] DT.Alias.EOD
[s] |DT.Time"23:59:59"
[s][!] *
[X]

[#] DT.Alias.StartOfDay
[s] |DT.Time"00:00:00"
[s][!] *
[X]

[#] DT.Alias.BusinessHoursStart
[s] |DT.Time"9:00AM"
[s][!] *
[X]

[#] DT.Alias.BusinessHoursEnd
[s] |DT.Time"5:00PM"
[s][!] *
[X]
```

**Usage**:
```polyglot
[<] deadline: pg\dt << DT.Alias.EOD
[<] work_start: pg\dt << DT.Alias.BusinessHoursStart
```

---

### Advanced: Parameterized Alias

**Use Case**: Create reusable patterns with parameters.

```polyglot
[#] DT.Alias.NextBusinessDay
[p] input_date: pg\dt
[<] .tomorrow: pg\dt << @input_date | DT.Add"1d"
[?] .tomorrow | DT.DayOfWeek == "Saturday"
  [<] .result: pg\dt << .tomorrow | DT.Add"2d"
[:] .tomorrow | DT.DayOfWeek == "Sunday"
  [<] .result: pg\dt << .tomorrow | DT.Add"1d"
[|]
  [<] .result: pg\dt << .tomorrow
[X]
[s] |@.result
[s][!] *
[X]
```

**Usage**:
```polyglot
[<] today: pg\dt << DT.Now""
[<] next_work_day: pg\dt << DT.Alias.NextBusinessDay(@today)
```

---

## 4. Holidays

### National Holidays

**File**: `holidays/USHolidays.yaml`
```yaml
holidays:
  country: "US"
  year: 2025

  fixed_dates:
    - name: "IndependenceDay"
      date: "2025-07-04"
      observed: "2025-07-04"    # Falls on Friday

    - name: "Christmas"
      date: "2025-12-25"
      observed: "2025-12-25"    # Falls on Thursday

  relative_dates:
    - name: "Thanksgiving"
      pattern: "November.Fourth.Thursday"
      year: 2025

    - name: "LaborDay"
      pattern: "September.First.Monday"
      year: 2025
```

**Polyglot Enumeration**:
```polyglot
[#] DT.Holiday.US
[<] .config: pg\yaml << U.YAML.Load"\\FileDir\\holidays\\USHolidays.yaml"

[~] Fixed holidays
[<] .independence_day: pg\dt << DT"2025-07-04"
[<] .christmas: pg\dt << DT"2025-12-25"

[~] Relative holidays
[<] .thanksgiving: pg\dt << DT.Gregorian.November.Fourth.Thursday""
[<] .labor_day: pg\dt << DT.Gregorian.September.First.Monday""

[s][!] *
[X]
```

**Usage**:
```polyglot
[<] independence: pg\dt << DT.Holiday.US.IndependenceDay
[<] thanksgiving: pg\dt << DT.Holiday.US.Thanksgiving

[~] Check if today is a holiday
[?] DT.Now"" == DT.Holiday.US.Christmas
  [o] "Merry Christmas!"
```

---

### Company Holidays

**Use Case**: Define your organization's specific holidays.

```polyglot
[#] DT.Holiday.MyCompany
[<] .founder_day: pg\dt << DT"2025-03-15"
[<] .summer_shutdown_start: pg\dt << DT"2025-07-01"
[<] .summer_shutdown_end: pg\dt << DT"2025-07-15"
[<] .all_hands_meeting: pg\dt << DT.Monthly"First Monday 9:00AM"
[s][!] *
[X]
```

**Usage**:
```polyglot
[?] DT.Now"" | DT.IsBetween(@DT.Holiday.MyCompany.summer_shutdown_start, @DT.Holiday.MyCompany.summer_shutdown_end)
  [o] "Office closed for summer shutdown"
```

---

## 5. Custom Format Macros

### Reusable Format Definitions

**Use Case**: Standardize datetime formatting across your codebase.

```polyglot
[#] DT.Format.ISO8601
[s] |"%Y-%m-%d"
[s][!] *
[X]

[#] DT.Format.USStandard
[s] |"%m/%d/%Y"
[s][!] *
[X]

[#] DT.Format.EuropeanStandard
[s] |"%d/%m/%Y"
[s][!] *
[X]

[#] DT.Format.LogTimestamp
[s] |"%Y-%m-%d %H:%M:%S.%f"
[s][!] *
[X]

[#] DT.Format.HumanReadable
[s] |"%A, %B %d, %Y at %I:%M %p"
[s][!] *
[X]
```

**Usage**:
```polyglot
[<] now: pg\dt << DT.Now""
[o] @now | DT.Format(@DT.Format.ISO8601)           [~] Output: "2025-11-30"
[o] @now | DT.Format(@DT.Format.USStandard)        [~] Output: "11/30/2025"
[o] @now | DT.Format(@DT.Format.LogTimestamp)      [~] Output: "2025-11-30 15:00:00.000"
[o] @now | DT.Format(@DT.Format.HumanReadable)     [~] Output: "Sunday, November 30, 2025 at 3:00 PM"
```

---

## 6. Multi-Extension Example

### Complete Company DateTime Configuration

This example shows how to combine multiple extensions for a complete organizational datetime setup.

**File Structure**:
```
company-datetime/
├── timezones/
│   └── CompanyTimezone.yaml
├── calendars/
│   ├── CompanyHijri.yaml
│   └── CompanyHebrew.yaml
├── holidays/
│   └── CompanyHolidays.yaml
└── datetime-config.pg
```

**File**: `datetime-config.pg`
```polyglot
[~] ======================================
[~] COMPANY DATETIME CONFIGURATION
[~] ======================================

[~] 1. TIMEZONE CONFIGURATION
[~] ======================================
[#] DT.TimeZone.Company
[<] .config: pg\yaml << U.YAML.Load"\\FileDir\\timezones\\CompanyTimezone.yaml"
[<] .gmt_offset: pg\dt << DT.Parse(.config.timezone.gmt_offset)
[<] .dst_offset: pg\dt << DT.Parse(.config.timezone.dst_offset)
[<] .dst_start: pg\dt << DT.Parse(.config.timezone.dst_start)
[<] .dst_end: pg\dt << DT.Parse(.config.timezone.dst_end)
[s][!] *
[X]


[~] 2. CALENDAR PROFILES
[~] ======================================

[~] Hijri calendar for Muslim employees
[#] DT.Hijri.CompanyHR
[<] .config: pg\yaml << U.YAML.Load"\\FileDir\\calendars\\CompanyHijri.yaml"
[<] .overrides: pg\yaml << .config.calendar.observed_dates
[s][!] *
[X]

[~] Hebrew calendar for Jewish employees
[#] DT.Hebrew.CompanyHR
[<] .config: pg\yaml << U.YAML.Load"\\FileDir\\calendars\\CompanyHebrew.yaml"
[s][!] *
[X]


[~] 3. COMPANY HOLIDAYS
[~] ======================================
[#] DT.Holiday.Company
[<] .config: pg\yaml << U.YAML.Load"\\FileDir\\holidays\\CompanyHolidays.yaml"

[~] Fixed holidays
[<] .new_year: pg\dt << DT"2025-01-01"
[<] .founder_day: pg\dt << DT"2025-03-15"
[<] .independence_day: pg\dt << DT"2025-07-04"

[~] Relative holidays
[<] .thanksgiving: pg\dt << DT.Gregorian.November.Fourth.Thursday""

[~] Religious holidays (multi-calendar)
[<] .eid_al_fitr: pg\dt << DT.Hijri.CompanyHR"1446-10-01"
[<] .eid_al_adha: pg\dt << DT.Hijri.CompanyHR"1446-12-10"
[<] .rosh_hashanah: pg\dt << DT.Hebrew.CompanyHR"Tishrei 1"

[s][!] *
[X]


[~] 4. COMPANY ALIASES
[~] ======================================
[#] DT.Alias.WorkDay
[s] |DT.TimeZone.Company"Monday-Friday"
[s][!] *
[X]

[#] DT.Alias.BusinessHours
[<] .start: pg\dt << DT.Time"9:00AM"
[<] .end: pg\dt << DT.Time"5:00PM"
[s][!] *
[X]

[#] DT.Alias.Standup
[s] |DT.TimeZone.Company.Daily"9:30AM"
[s][!] *
[X]

[#] DT.Alias.AllHandsMeeting
[s] |DT.TimeZone.Company.Monthly"First Monday 10:00AM"
[s][!] *
[X]


[~] 5. CUSTOM FORMATS
[~] ======================================
[#] DT.Format.CompanyLog
[s] |"%Y-%m-%d %H:%M:%S %Z"
[s][!] *
[X]

[#] DT.Format.CompanyEmail
[s] |"%A, %B %d, %Y"
[s][!] *
[X]

[#] DT.Format.CompanyReport
[s] |"%m/%d/%Y %I:%M %p"
[s][!] *
[X]
```

**Usage Example**:
```polyglot
[~] Load company datetime configuration
[=] U.File.Run"\\CompanyDir\\datetime-config.pg"

[~] Use company timezone
[<] meeting_time: pg\dt << DT.TimeZone.Company"2025-11-30 15:00"

[~] Check if today is a company holiday
[?] DT.Now"" == DT.Holiday.Company.Founder_Day
  [o] "Happy Founder's Day!"

[~] Schedule standup (uses company alias)
[<] next_standup: pg\dt << DT.Alias.Standup

[~] Format log timestamp
[<] log_entry: pg\string << "ERROR: " + (@DT.Now"" | DT.Format(@DT.Format.CompanyLog))
[o] @log_entry  [~] Output: "ERROR: 2025-11-30 15:00:00 CST"

[~] Get religious holiday dates
[<] eid: pg\dt << DT.Holiday.Company.eid_al_fitr
[<] rosh_hashanah: pg\dt << DT.Holiday.Company.rosh_hashanah

[o] "Eid al-Fitr: " + (@eid | DT.Format(@DT.Format.CompanyEmail))
[o] "Rosh Hashanah: " + (@rosh_hashanah | DT.Format(@DT.Format.CompanyEmail))
```

---

## Best Practices

### 1. Naming Conventions

**DO**:
```polyglot
[#] DT.TimeZone.MyCompany              // Clear, descriptive
[#] DT.Hijri.SaudiArabia               // Follows namespace
[#] DT.Alias.EOD                       // Common abbreviation
[#] DT.Holiday.USFederal               // Specific scope
```

**DON'T**:
```polyglot
[#] DT.TimeZone.tz1                    // ❌ Non-descriptive
[#] DT.Hijri.SA                        // ❌ Unclear abbreviation
[#] DT.Alias.x                         // ❌ Meaningless name
[#] DT.Holiday.Holidays                // ❌ Redundant
```

---

### 2. Configuration Management

**Use YAML for Data**:
```yaml
# Good: Separate configuration from logic
timezone:
  name: "MyCompany"
  gmt_offset: "-03:00"
  dst_enabled: true
```

**Load in Polyglot**:
```polyglot
[#] DT.TimeZone.MyCompany
[<] .config: pg\yaml << U.YAML.Load"\\FileDir\\config.yaml"
[<] .gmt_offset: pg\dt << DT.Parse(.config.timezone.gmt_offset)
[s][!] *
[X]
```

**Benefits**:
- Non-programmers can update YAML
- Easier version control
- Can reload without code changes

---

### 3. Validation

**Always Validate Configuration**:
```polyglot
[#] DT.TimeZone.MyCompany
[<] .config: pg\yaml << U.YAML.Load"\\FileDir\\timezones\\MyCompany.yaml"

[~] Validate required fields
[?] !(.config | U.YAML.HasKey("timezone.gmt_offset"))
  [E] "Configuration error: Missing gmt_offset"

[?] !(.config | U.YAML.HasKey("timezone.dst_enabled"))
  [E] "Configuration error: Missing dst_enabled"

[~] Validate offset format
[<] .offset_parsed: pg\dt << DT.Parse(.config.timezone.gmt_offset)
[?] @.offset_parsed == DT.Error
  [E] "Invalid offset format: " + .config.timezone.gmt_offset

[<] .gmt_offset: pg\dt << @.offset_parsed
[s][!] *
[X]
```

---

### 4. Documentation

**Document Each Extension**:
```polyglot
[~] ======================================
[~] DT.Hijri.MyCompanyHR
[~] ======================================
[~] Profile for company HR Hijri calendar
[~]
[~] Priority order:
[~]   1. Manual overrides from HR
[~]   2. Moon sighting API cache
[~]   3. ICU4X algorithmic calculation
[~]
[~] Configuration: calendars/MyCompanyHijri.yaml
[~] Maintained by: HR Department
[~] Last updated: 2025-11-30
[~] ======================================

[#] DT.Hijri.MyCompanyHR
[~] ... implementation ...
[X]
```

---

### 5. Testing

**Test Your Extensions**:
```polyglot
[~] Test timezone definition
[<] test_time: pg\dt << DT.TimeZone.MyCompany"2025-11-30 15:00"
[?] @test_time == DT.Error
  [E] "Timezone definition failed"

[~] Test holiday dates
[<] test_holiday: pg\dt << DT.Holiday.Company.Thanksgiving
[?] @test_holiday == DT.Error
  [E] "Holiday definition failed"

[~] Test alias
[<] test_alias: pg\dt << DT.Alias.EOD
[?] @test_alias == DT.Error
  [E] "Alias definition failed"

[o] "All datetime extensions validated successfully"
```

---

## Validation Reference

### Extension Validation Checklist

Before deploying datetime extensions, validate:

**Timezone Extensions**:
- [ ] GMT offset is valid (-12:00 to +14:00)
- [ ] DST offset is valid (typically +01:00)
- [ ] DST start/end dates use valid wildcard patterns
- [ ] Configuration YAML is well-formed
- [ ] Required fields are present

**Calendar Profile Extensions**:
- [ ] Calendar type matches namespace (Hijri, Hebrew, Chinese)
- [ ] Observed dates are in correct format
- [ ] API URLs are accessible (if using API)
- [ ] Fallback calculation method is specified
- [ ] Configuration YAML is well-formed

**Alias Extensions**:
- [ ] Alias produces valid DateTime value
- [ ] Dependencies on other DT.* pipelines are met
- [ ] Parameterized aliases validate inputs
- [ ] Return value is documented

**Holiday Extensions**:
- [ ] Fixed dates use ISO 8601 format
- [ ] Relative dates use valid DT.* patterns
- [ ] Multi-calendar holidays specify correct profile
- [ ] Holiday names are unique within namespace

**Format Extensions**:
- [ ] Format string uses valid strftime-compatible specifiers
- [ ] Format produces expected output
- [ ] Edge cases handled (leap years, DST transitions)

---

## Troubleshooting

### Common Issues

**Issue**: Enumeration not found
```
ERROR: DT.TimeZone.MyCompany is not defined
```

**Solution**: Ensure enumeration is exported with `[!]`
```polyglot
[#] DT.TimeZone.MyCompany
[~] ... implementation ...
[s][!] *              ← Must export!
[X]
```

---

**Issue**: YAML parsing fails
```
ERROR: Failed to parse YAML configuration
```

**Solution**: Validate YAML syntax
```bash
# Use yamllint or online validator
yamllint timezones/MyCompany.yaml
```

---

**Issue**: Date validation fails
```
ERROR: Day-of-week mismatch: 2025-11-30 is Sunday, not Monday
```

**Solution**: Verify date calculations
```polyglot
[~] Check what day it actually is
[<] check_day: pg\string << DT"2025-11-30" | DT.DayOfWeek
[o] @check_day  [~] Output: "Sunday"
```

---

**Issue**: Timezone offset incorrect
```
ERROR: Time conversion produced unexpected result
```

**Solution**: Verify offset format and DST rules
```polyglot
[~] Test timezone offset
[<] utc_time: pg\dt << DT"2025-11-30T15:00:00Z"
[<] local_time: pg\dt << @utc_time | DT.Convert.TimeZone(@DT.TimeZone.MyCompany)
[o] "UTC: " + (@utc_time | DT.Format.ISO8601)
[o] "Local: " + (@local_time | DT.Format.ISO8601)
```

---

## Advanced Patterns

### 1. Chained Extensions

**Use Case**: Create extension that depends on another extension.

```polyglot
[~] Base timezone
[#] DT.TimeZone.US.Eastern
[<] .gmt_offset: pg\dt << DT.Ago"5h"
[<] .dst_offset: pg\dt << DT"1h"
[s][!] *
[X]

[~] Derived timezone (reuses base but different DST rules)
[#] DT.TimeZone.CustomEastern
[<] .base: pg\dt << @DT.TimeZone.US.Eastern
[<] .dst_start: pg\dt << DT"*-04-01"     [~] Different DST start
[<] .dst_end: pg\dt << DT"*-10-01"       [~] Different DST end
[s][!] *
[X]
```

---

### 2. Conditional Extensions

**Use Case**: Extension behavior changes based on environment.

```polyglot
[#] DT.TimeZone.Auto
[<] .env: pg\string << U.Env.Get"DEPLOYMENT_ENV"

[?] @.env == "production"
  [<] .config_file: pg\string << "timezones/production.yaml"
[:] @.env == "staging"
  [<] .config_file: pg\string << "timezones/staging.yaml"
[|]
  [<] .config_file: pg\string << "timezones/development.yaml"
[X]

[<] .config: pg\yaml << U.YAML.Load(@.config_file)
[<] .gmt_offset: pg\dt << DT.Parse(.config.timezone.gmt_offset)
[s][!] *
[X]
```

---

### 3. Cached Extensions

**Use Case**: Avoid repeated expensive operations.

```polyglot
[#] DT.Holiday.US
[<] .cache_file: pg\string << "\\CacheDir\\us_holidays_2025.cache"

[~] Check if cache exists and is fresh
[?] U.File.Exists(@.cache_file) && (U.File.Age(@.cache_file) < DT"24h")
  [<] .holidays: pg\yaml << U.File.Read(@.cache_file)
[|]
  [~] Fetch from API
  [<] .api_result: pg\yaml << U.HTTP.Get"https://api.holidays.com/us/2025"
  [<] .holidays: pg\yaml << .api_result.data

  [~] Cache the result
  [=] U.File.Write(@.cache_file, @.holidays)
[X]

[~] Build holiday definitions from cached data
[<] .thanksgiving: pg\dt << DT.Parse(.holidays.thanksgiving.date)
[<] .christmas: pg\dt << DT.Parse(.holidays.christmas.date)
[s][!] *
[X]
```

---

## Related Documentation

- [DateTime String Literal Specification](../technical/datetime-string-literal-specification.md) - Complete syntax reference
- [DT Pipeline Tree](../technical/dt-pipeline-tree.md) - Full namespace hierarchy
- [DateTime Formatted String Grammar](../technical/datetime-formatted-string-grammar.md) - BNF grammar specification
- [Type System Documentation](../technical/type-system.md) - Polyglot type system overview
- [Extendable Enumeration Guide](../technical/extendable-enumerations.md) - General enumeration patterns

---

**Version**: 1.0
**Contributors**: Polyglot Core Team
**License**: Same as Polyglot Language
