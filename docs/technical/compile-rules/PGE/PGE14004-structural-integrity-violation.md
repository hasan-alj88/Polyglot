---
audience: developer
rule: "14.4"
code: PGE14004
name: Structural Integrity Violation
severity: error
---

# Rule 14.4 — Structural Integrity Violation
`PGE14004`

<!-- @u:syntax/constructors -->
<!-- @c:technical/ebnf/definition-blocks/09-13-constructor -->

**Statement:** A capture slot's `.re` regex pattern can match the constructor pattern's literal separator characters. This would allow a single input value to span multiple capture slots, breaking pattern structure.

**Rationale:** Analogous to SQL injection prevention. Constructor patterns rely on literal separators to divide the input into capture regions. If a capture's regex admits separator characters, an input value could "escape" its slot and corrupt adjacent captures. The compiler proves structural safety at definition time — every call site inherits the guarantee.

**Detection:** Compiler identifies all literal characters in the pattern string (characters outside `{capture}` placeholders). For each capture's `.re` pattern, the compiler tests whether the regex can match any of these literal separator characters. If so, PGE14004 is raised.

**See also:** PGE14001 (ambiguous overloads — related structural concern), [[syntax/constructors]]

---

**VALID:**
```aljam3
[ ] ✓ Capture regex cannot match separator ":"

{$} $DT"{hours}:{min}"
   ($) <hours.re << "[0-9][0-9]"
   ($) <min.re << "[0-9][0-9]"
   [$] #DateTime
   [.] .hours << <hours
   [.] .minutes << <min
```

**INVALID:**
```aljam3
[ ] ✗ PGE14004 — hours regex can match separator ":"

{$} $DT"{hours}:{min}:{seconds}"
   ($) <hours.re << "[0-9:]+"
   ($) <min.re << "[0-9][0-9]"
   ($) <seconds.re << "[0-9][0-9]"
   [$] #DateTime
   [.] .hours << <hours
   [.] .minutes << <min
   [.] .seconds << <seconds
```

```
error[PGE14004]: constructor slot regex can match pattern separator
  --> src/datetime.aj3:3:4
   |
1  | {$} $DT"{hours}:{min}:{seconds}"
   |                ^     ^           separator characters: ':'
2  |    ($) <hours.re << "[0-9:]+"
   |                      ^^^^^^^ regex '[0-9:]+' can match ':'
   |
   = help: slot regex must not match separator characters to prevent
           structural ambiguity (SQL injection prevention)
   = help: change regex to '[0-9]+' or '[0-9][0-9]'
```

**Open point:** None.
