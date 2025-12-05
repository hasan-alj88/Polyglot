# DateTime Formatted String Grammar (BNF)

**Document Status**: Specification
**Last Updated**: 2025-11-30
**Related**: DateTime String Literal Specification, DT Pipeline Tree

---

## Overview

This document defines the **complete formal grammar** for all datetime formatted strings in Polyglot using **Backus-Naur Form (BNF)**.

**Key Principles**:
1. **ISO 8601 compliance** - All date/time formats follow ISO 8601 standard
2. **No decimals in durations** - Avoid ambiguity
3. **Ordered duration units** - Units must appear in descending order
4. **AM/PM leftmost is hours** - When using AM/PM, leftmost number is always hours
5. **Wildcard support** - `*` for any value in date/time patterns

---

## Complete BNF Grammar

```bnf
(* ============================================ *)
(* DATE FORMATS                                 *)
(* ============================================ *)

<date> ::= <year> "-" <month> "-" <day>
         | <year> "-" <month> "-" <day> ":"
         | <wildcard-date>

<year> ::= <digit> <digit> <digit> <digit>
         | "*"

<month> ::= "01" | "02" | "03" | "04" | "05" | "06"
          | "07" | "08" | "09" | "10" | "11" | "12"
          | "*"

<day> ::= "01" | "02" | ... | "28" | "29" | "30" | "31"
        | "*"

<wildcard-date> ::= "*" "-" <month> "-" <day>
                  | <year> "-" "*" "-" <day>
                  | <year> "-" <month> "-" "*"
                  | "*" "-" "*" "-" <day>
                  | "*" "-" <month> "-" "*"
                  | <year> "-" "*" "-" "*"

(* At least one non-wildcard component required *)
(* "*-*-*" is INVALID *)


(* ============================================ *)
(* TIME FORMATS                                 *)
(* ============================================ *)

<time> ::= <time-24hour>
         | <time-12hour>

(* 24-hour format *)
<time-24hour> ::= <hour-24> ":" <minute>
                | <hour-24> ":" <minute> ":" <second>
                | <hour-24> ":" <minute> ":" <second> "." <millisecond>
                | <hour-24> ":" "*"
                | "*" ":" <minute>

<hour-24> ::= "00" | "01" | ... | "23" | "24"
            | "*"

(* 12-hour format *)
<time-12hour> ::= <hour-12> <am-pm>
                | <hour-12> ":" <minute> <am-pm>
                | <hour-12> ":" <minute> ":" <second> <am-pm>
                | <hour-12> ":" <minute> ":" <second> "." <millisecond> <am-pm>

<hour-12> ::= "1" | "2" | "3" | "4" | "5" | "6"
            | "7" | "8" | "9" | "10" | "11" | "12"

<am-pm> ::= "AM" | "PM" | "am" | "pm"

(* Common components *)
<minute> ::= "00" | "01" | ... | "59"
           | "*"

<second> ::= "00" | "01" | ... | "59"
           | "*"

<millisecond> ::= <digit> <digit> <digit>

(* Midnight special cases - all valid *)
(* 00:00, 24:00, 12:00AM, 12AM *)


(* ============================================ *)
(* DATETIME FORMATS                             *)
(* ============================================ *)

<datetime> ::= <date> " " <time>
             | <date> " " <time> " " <timezone-ref>
             | <date> "T" <time>
             | <date> "T" <time> "Z"
             | <date> "T" <time> <timezone-offset>

<timezone-ref> ::= <iana-timezone>
                 | <timezone-offset>
                 | "UTC"

<iana-timezone> ::= <region> "/" <location>

<region> ::= "US" | "Europe" | "Asia" | "Africa" | "Australia" | ...

<location> ::= "Eastern" | "Central" | "Pacific" | "London" | "Tokyo" | ...

<timezone-offset> ::= "+" <hour-offset> ":" <minute-offset>
                    | "-" <hour-offset> ":" <minute-offset>

<hour-offset> ::= "00" | "01" | ... | "14"

<minute-offset> ::= "00" | "15" | "30" | "45"


(* ============================================ *)
(* DURATION FORMATS                             *)
(* ============================================ *)

<duration> ::= <duration-component>
             | <duration-component> " " <duration>

<duration-component> ::= <number> <duration-unit>

<number> ::= <digit>+

<duration-unit> ::= "y"   (* years *)
                  | "mo"  (* months *)
                  | "w"   (* weeks *)
                  | "d"   (* days *)
                  | "h"   (* hours *)
                  | "m"   (* minutes *)
                  | "s"   (* seconds *)

(* STRICT ORDERING REQUIRED: y > mo > w > d > h > m > s *)
(* Valid:   "2h 30m 15s" *)
(* Invalid: "30m 2h"     *)
(* Invalid: "2.5h"       *)
(* Invalid: "-2h"        *)


(* ============================================ *)
(* DAY OF WEEK FORMATS                          *)
(* ============================================ *)

<day-of-week> ::= <day-full-name>
                | <day-abbrev>

<day-full-name> ::= "Monday" | "Tuesday" | "Wednesday" | "Thursday"
                  | "Friday" | "Saturday" | "Sunday"

<day-abbrev> ::= "Mon" | "Tue" | "Wed" | "Thu" | "Fri" | "Sat" | "Sun"


(* ============================================ *)
(* COMBINED FORMATS                             *)
(* ============================================ *)

<day-and-time> ::= <day-of-week> " " <time>

<date-and-day> ::= <date> " " <day-of-week>

<date-day-time> ::= <date> " " <day-of-week> " " <time>


(* ============================================ *)
(* RELATIVE DATE PATTERNS (Hierarchy-based)     *)
(* ============================================ *)

(* Pattern: DT.{Calendar}.{Month}.{Occurrence}.{DayOfWeek}"" *)

<relative-date> ::= ""  (* Empty string - hierarchy defines the pattern *)

<occurrence> ::= "First" | "Second" | "Third" | "Fourth" | "Fifth" | "Last"


(* ============================================ *)
(* MONTH NAMES (Calendar-specific)              *)
(* ============================================ *)

(* Gregorian months *)
<gregorian-month> ::= "January" | "February" | "March" | "April"
                    | "May" | "June" | "July" | "August"
                    | "September" | "October" | "November" | "December"

(* Islamic months (Hijri calendar) *)
<hijri-month> ::= "Muharram" | "Safar" | "RabiAlAwwal" | "RabiAlThani"
                | "JumadaAlAwwal" | "JumadaAlThani" | "Rajab" | "Shaban"
                | "Ramadan" | "Shawwal" | "DhulQadah" | "DhulHijjah"

(* Hebrew months *)
<hebrew-month> ::= "Nisan" | "Iyar" | "Sivan" | "Tammuz"
                 | "Av" | "Elul" | "Tishrei" | "Heshvan"
                 | "Kislev" | "Tevet" | "Shevat" | "Adar"

(* Chinese months - use numbers 1-12 *)


(* ============================================ *)
(* HELPER DEFINITIONS                           *)
(* ============================================ *)

<digit> ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"

<letter> ::= "A" | "B" | ... | "Z" | "a" | "b" | ... | "z"
```

---

## Format Examples by Category

### 1. Date Formats

```bnf
<date> productions:

"2025-11-30"          (* ISO 8601 date *)
"2025-11-30:"         (* Full day: 00:00 to 23:59:59 *)
"*-11-30"             (* Wildcard year: every Nov 30 *)
"2025-*-15"           (* Wildcard month: 15th of every month in 2025 *)
"2025-11-*"           (* Wildcard day: every day in Nov 2025 *)
"*-03-01"             (* March 1 every year *)
```

### 2. Time Formats

```bnf
<time> productions:

(* 24-hour format *)
"15:00"               (* 3:00 PM *)
"15:00:30"            (* With seconds *)
"15:00:30.500"        (* With milliseconds *)
"00:00"               (* Midnight option 1 *)
"24:00"               (* Midnight option 2 - end of day *)

(* 12-hour format *)
"3PM"                 (* Hour only *)
"3:00PM"              (* With minutes *)
"3:00:30PM"           (* With seconds *)
"12AM"                (* Midnight option 3 *)
"12:00AM"             (* Midnight option 4 *)

(* Wildcards *)
"15:*"                (* Every minute at 3 PM *)
"*:00"                (* Every hour on the hour *)
```

### 3. DateTime Formats

```bnf
<datetime> productions:

"2025-11-30 15:00"                    (* Space separator *)
"2025-11-30T15:00:00"                 (* ISO 8601 T separator *)
"2025-11-30T15:00:00Z"                (* UTC with Z suffix *)
"2025-11-30 15:00 US/Eastern"         (* With timezone *)
"2025-11-30 3:00PM EST"               (* 12-hour + abbreviation *)
"2025-11-30 15:00 +05:00"             (* With offset *)
```

### 4. Duration Formats

```bnf
<duration> productions:

(* Single unit *)
"30s"                 (* 30 seconds *)
"5m"                  (* 5 minutes *)
"2h"                  (* 2 hours *)
"1d"                  (* 1 day *)
"1w"                  (* 1 week *)
"1mo"                 (* 1 month *)
"1y"                  (* 1 year *)

(* Compound (ordered) *)
"2h 30m"              (* 2 hours 30 minutes *)
"1d 6h"               (* 1 day 6 hours *)
"2h 30m 15s"          (* 2 hours 30 minutes 15 seconds *)
"1y 3mo 2w"           (* 1 year 3 months 2 weeks *)

(* INVALID examples *)
"30m 2h"              (* ❌ Wrong order *)
"2.5h"                (* ❌ No decimals *)
"-2h"                 (* ❌ No negative - use DT.Ago"2h" *)
```

### 5. Day of Week Formats

```bnf
<day-of-week> productions:

"Monday"              (* Full name *)
"Mon"                 (* Abbreviated *)
"Friday"              (* Full name *)
"Fri"                 (* Abbreviated *)
```

### 6. Combined Formats

```bnf
<combined> productions:

(* Day + Time *)
"Friday 3:00PM"       (* Friday at 3 PM *)
"Mon 15:00"           (* Monday at 3 PM 24-hour *)

(* Date + Day (validated) *)
"2025-11-30 Sunday"   (* Must validate: Nov 30, 2025 IS Sunday *)

(* Date + Day + Time (all validated) *)
"2025-11-30 Sunday 15:00"         (* Full specification *)
"2025-11-30 Sunday 3:00PM EST"    (* With 12-hour + timezone *)
```

---

## Validation Rules

### Date Validation

1. **Month Range**: 01-12
2. **Day Range**: 01-31 (depends on month and leap year)
3. **Leap Year**: February 29 only in leap years
4. **Month Lengths**:
   - Jan, Mar, May, Jul, Aug, Oct, Dec: 31 days
   - Apr, Jun, Sep, Nov: 30 days
   - Feb: 28 days (29 in leap year)

### Time Validation

1. **Hour Range (24-hour)**: 00-23 (24:00 allowed as end-of-day)
2. **Hour Range (12-hour)**: 1-12 (with AM/PM)
3. **Minute Range**: 00-59
4. **Second Range**: 00-59
5. **Millisecond Range**: 000-999
6. **Midnight Special Cases**:
   - `00:00` = Start of day
   - `24:00` = End of day (same as next day 00:00)
   - `12:00AM` = Start of day
   - `12AM` = Start of day

### Duration Validation

1. **Unit Order**: Must be descending (y > mo > w > d > h > m > s)
2. **No Duplicates**: Each unit can appear at most once
3. **Positive Only**: No negative values (use DT.Ago for past)
4. **Integer Only**: No decimal values

### Wildcard Validation

1. **At Least One Non-Wildcard**: Cannot have all components as wildcards
2. **Valid Wildcards**:
   - `*-03-01` ✅ (specific month + day)
   - `2025-*-15` ✅ (specific year + day)
   - `*-*-*` ❌ (all wildcards - too ambiguous)

### Day-of-Week Validation

1. **Date + Day Matching**: When both specified, date must fall on that day
2. **Example**: `"2025-11-30 Sunday"` ✅ (Nov 30, 2025 IS Sunday)
3. **Example**: `"2025-11-30 Monday"` ❌ (Nov 30, 2025 is NOT Monday)

---

## Parser Implementation Hints

### Lexer Tokens

```
TOKEN_YEAR          // 4 digits or *
TOKEN_MONTH         // 2 digits (01-12) or *
TOKEN_DAY           // 2 digits (01-31) or *
TOKEN_HOUR          // 2 digits (00-24) or 1-2 digits (1-12)
TOKEN_MINUTE        // 2 digits (00-59) or *
TOKEN_SECOND        // 2 digits (00-59) or *
TOKEN_MILLISECOND   // 3 digits (000-999)
TOKEN_AM_PM         // AM, PM, am, pm
TOKEN_DURATION_NUM  // Integer
TOKEN_DURATION_UNIT // y, mo, w, d, h, m, s
TOKEN_DAY_OF_WEEK   // Monday-Sunday or Mon-Sun
TOKEN_TIMEZONE      // Region/Location or offset
TOKEN_WILDCARD      // *
TOKEN_SEPARATOR     // -, :, T, Z, space
TOKEN_COLON_SUFFIX  // : (for full day)
```

### Parse Order

1. **Lexical Analysis**: Tokenize string
2. **Syntax Check**: Match against BNF grammar
3. **Semantic Validation**: Check value ranges
4. **Calendar-Specific Validation**: Month lengths, leap years
5. **Day-of-Week Validation**: ICU4X calculation
6. **Profile Validation** (if applicable): Moon sighting, regional variations

### Error Messages

```
"Invalid date: February 30 does not exist"
"Invalid time: Hour must be 00-23 or 1-12 with AM/PM"
"Invalid duration: Units must be in descending order (got 'm' after 's')"
"Day-of-week mismatch: 2025-11-30 is Sunday, not Monday"
"Wildcard ambiguity: At least one non-wildcard component required"
"Duration decimals not allowed: Use '2h 30m' instead of '2.5h'"
```

---

## Extended Examples

### ISO 8601 Week Dates

```bnf
<iso-week-date> ::= <year> "-W" <week> "-" <weekday>

<week> ::= "01" | "02" | ... | "52" | "53"

<weekday> ::= "1" | "2" | "3" | "4" | "5" | "6" | "7"

Example:
"2025-W48-6"      (* Week 48, day 6 (Saturday) *)
```

### ISO 8601 Ordinal Dates

```bnf
<iso-ordinal-date> ::= <year> "-" <day-of-year>

<day-of-year> ::= "001" | "002" | ... | "365" | "366"

Example:
"2025-334"        (* Day 334 of 2025 *)
```

---

## Formal Grammar Summary

**Total Productions**: 15+
**Token Types**: 15+
**Validation Levels**: 4 (Lexical, Syntax, Semantic, Runtime)

**Grammar Categories**:
1. Date Formats (6 productions)
2. Time Formats (9 productions)
3. DateTime Formats (5 productions)
4. Duration Formats (2 productions + ordering rules)
5. Day of Week Formats (2 productions)
6. Combined Formats (3 productions)
7. Wildcard Patterns (6 productions)

**Extension Points**:
- Calendar-specific month names
- Timezone references
- Custom format macros (via DT.Format.*)

---

**Related Documents**:
- DateTime String Literal Specification
- DT Pipeline Tree
- DateTime Extension Guide
