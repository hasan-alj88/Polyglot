---
audience: developer
rule: "14.1"
code: PGE14001
name: Ambiguous Constructor Overload
severity: error
---

# Rule 14.1 — Ambiguous Constructor Overload
`PGE14001`

<!-- @u:syntax/constructors -->
<!-- @c:technical/ebnf/definition-blocks/09-13-constructor -->

**Statement:** Two `{$}` definitions for the same constructor name have overlapping regex match sets. The compiler cannot guarantee unambiguous overload resolution.

**Rationale:** Overload resolution requires exactly one match per input. If two overloads' compiled regex patterns can both match the same input string, the compiler cannot determine which overload to select. This is detected at definition compile time, not at call sites.

**Detection:** Compiler compiles each overload's pattern (substituting `{capture}` with capture `.re`) into full regex, then checks for intersection between all overload pairs. If any intersection is non-empty, PGE14001 is raised.

**See also:** PGE14002 (duplicate keyword — special case), PGE14010 (no match at call site), [[syntax/constructors]]

---

**VALID:**
```aljam3
[ ] ✓ Two $DT overloads with non-overlapping patterns

{$} $DT"{hours}:{min}:{seconds}"
   ($) <hours.re << "[0-9][0-9]"
   ($) <min.re << "[0-9][0-9]"
   ($) <seconds.re << "[0-9][0-9]"
   [$] #DateTime
   [.] .hours << <hours
   [.] .minutes << <min
   [.] .seconds << <seconds

{$} $DT"Today"
   [$] #DateTime
   [.] .hours << 0
   [.] .minutes << 0
   [.] .seconds << 0
```

**INVALID:**
```aljam3
[ ] ✗ PGE14001 — both overloads match "12:30"

{$} $DT"{hours}:{min}"
   ($) <hours.re << "[0-9:]+"
   ($) <min.re << "[0-9:]+"
   [$] #DateTime
   [.] .hours << <hours
   [.] .minutes << <min

{$} $DT"{h}:{m}"
   ($) <h.re << "[0-9]+"
   ($) <m.re << "[0-9]+"
   [$] #DateTime
   [.] .hours << <h
   [.] .minutes << <m
```

**Open point:** None.
