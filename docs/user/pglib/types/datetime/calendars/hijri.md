---
audience: automation-builder
type: reference
updated: 2026-04-09
metadata_definition: "%definition.#:HijriDate"
metadata_instance: "%#:HijriDate:N"
---

# Hijri Calendar Types

The Hijri calendar is regional -- month starts depend on moon sighting authority. Saudi uses Umm al-Qura (astronomical), Pakistan/India often differ by 1-2 days. The design supports regional authorities, multiple methods, and fully custom Hijri configurations.

## #HijriDate

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

## #HijriMonth

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

## #HijriAuthority

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

## #HijriMethod

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

## #HijriLeap

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

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:HijriDate` | Compile-time type template |
| Instance | `%#:HijriDate:N` | Runtime instance (N = instance number) |
| Definition | `%definition.#:HijriMonth` | Compile-time type template |
| Definition | `%definition.#:HijriAuthority` | Compile-time type template |
| Definition | `%definition.#:HijriMethod` | Compile-time type template |
| Definition | `%definition.#:HijriLeap` | Compile-time type template |

See [[metadata-tree/FULL-TREE|Full Metadata Tree]] for the authoritative tree.
