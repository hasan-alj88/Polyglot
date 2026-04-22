---
audience: automation-builder
type: specification
updated: 2026-04-22
---

# $Dur Constructor

<!-- @u:syntax/constructors -->
<!-- @c:pglib/types/datetime/core-components -->

The `$Dur` constructor produces `#Duration` values from human-readable duration strings. All overloads are compile-time guaranteed — no error surface, no `[!]` handling needed.

`#Duration` is defined in the datetime family (see [[pglib/types/datetime/core-components|core components]]) with `.seconds#int` and `.nanos#int` fields.

## String-Parsing Overloads

### Hours + Minutes + Seconds

```polyglot
{$} $Dur"{h}h{m}m{s}s"
   ($) <h.re << "[0-9]+"
   ($) <m.re << "[0-9]+"
   ($) <s.re << "[0-9]+"
   [-] -Dur.Convert
      (<) <hours << <h
      (<) <minutes << <m
      (<) <seconds << <s
      (-) >dur >> $computed
   [$] #Duration
   [.] << $computed
```

Matches duration strings like `"1h30m45s"`. The `[-] -Dur.Convert` is a native pipeline (pglib-only pattern) that converts hours, minutes, and seconds into the `#Duration` fields (`.seconds` and `.nanos`).

### Hours + Minutes

```polyglot
{$} $Dur"{h}h{m}m"
   ($) <h.re << "[0-9]+"
   ($) <m.re << "[0-9]+"
   [-] -Dur.Convert
      (<) <hours << <h
      (<) <minutes << <m
      (-) >dur >> $computed
   [$] #Duration
   [.] << $computed
```

### Minutes + Seconds

```polyglot
{$} $Dur"{m}m{s}s"
   ($) <m.re << "[0-9]+"
   ($) <s.re << "[0-9]+"
   [-] -Dur.Convert
      (<) <minutes << <m
      (<) <seconds << <s
      (-) >dur >> $computed
   [$] #Duration
   [.] << $computed
```

### Hours Only

```polyglot
{$} $Dur"{h}h"
   ($) <h.re << "[0-9]+"
   [-] -Dur.Convert
      (<) <hours << <h
      (-) >dur >> $computed
   [$] #Duration
   [.] << $computed
```

### Minutes Only

```polyglot
{$} $Dur"{m}m"
   ($) <m.re << "[0-9]+"
   [-] -Dur.Convert
      (<) <minutes << <m
      (-) >dur >> $computed
   [$] #Duration
   [.] << $computed
```

### Seconds Only

```polyglot
{$} $Dur"{s}s"
   ($) <s.re << "[0-9]+"
   [-] -Dur.Convert
      (<) <seconds << <s
      (-) >dur >> $computed
   [$] #Duration
   [.] << $computed
```

All single-unit overloads follow the same native pipeline pattern. `-Dur.Convert` accepts any combination of `<hours`, `<minutes`, and `<seconds` inputs (all optional) and produces a complete `#Duration` value.

## Overload Resolution

The six overloads are distinguished by their unit suffix literals:

| Overload | Distinguishing Feature |
|---|---|
| HMS `"{h}h{m}m{s}s"` | Contains `h`, `m`, and `s` suffixes |
| HM `"{h}h{m}m"` | Contains `h` and `m` suffixes, no `s` |
| MS `"{m}m{s}s"` | Contains `m` and `s` suffixes, no `h` |
| Hours `"{h}h"` | Single `h` suffix |
| Minutes `"{m}m"` | Single `m` suffix |
| Seconds `"{s}s"` | Single `s` suffix |

The unit suffix characters (`h`, `m`, `s`) are literal separators — the `[0-9]+` capture regex cannot match them, ensuring non-overlapping patterns. Resolution order: longest match first (HMS before HM, HM before H).

## Usage

```polyglot
[ ] compile-time guaranteed — no error handling
[-] $timeout << $Dur"30s"
[-] $interval << $Dur"5m"
[-] $longRunning << $Dur"2h30m"
[-] $precise << $Dur"1h15m30s"

[ ] for dynamic strings, use -Dur.Parse with error handling
[-] $userDur#duration << -Dur.Parse
   (<) <raw#string << $userInput
   [!] !Parse.Duration.InvalidFormat
      [-] $userDur << $Dur"30s"
```

## Related

- [[constructors/INDEX|pglib Constructors]] -- constructor registry
- [[syntax/constructors]] -- `{$}` block specification
- [[pglib/pipelines/Dur.Parse|-Dur.Parse]] -- runtime duration string parsing
- [[pglib/types/datetime/core-components|#Duration type]] -- duration type definition
- [[pglib/constructors/DT|$DT constructor]] -- companion DateTime constructor
