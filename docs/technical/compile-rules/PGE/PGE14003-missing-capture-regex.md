---
audience: developer
rule: "14.3"
code: PGE14003
name: Missing Capture Regex
severity: error
---

# Rule 14.3 — Missing Capture Regex
`PGE14003`

<!-- @u:syntax/constructors -->
<!-- @c:technical/ebnf/definition-blocks/09-13-constructor -->

**Statement:** A capture slot in a `{$}` constructor is declared without a `.re` validation pattern. Every capture parameter must have a regex constraint.

**Rationale:** The `.re` field is mandatory because the compiler uses it for: (1) overload resolution — building the full-pattern regex, (2) structural integrity checks — ensuring captures cannot match separators, (3) compile-time proof that all possible inputs produce valid trees. Without `.re`, none of these guarantees hold.

**Detection:** Compiler checks all `($)` lines in `{$}` blocks for the `.re` field. If any `($)` line declares a capture without `.re << "pattern"`, PGE14003 is raised.

**See also:** PGE14004 (structural integrity — validates what `.re` contains), [[syntax/constructors]]

---

**VALID:**
```aljam3
[ ] ✓ All captures have .re constraints

{$} $DT"{hours}:{min}"
   ($) <hours.re << "[0-9][0-9]"
   ($) <min.re << "[0-9][0-9]"
   [$] #DateTime
   [.] .hours << <hours
   [.] .minutes << <min
```

**INVALID:**
```aljam3
[ ] ✗ PGE14003 — <hours has no .re field

{$} $DT"{hours}:{min}"
   ($) <hours
   ($) <min.re << "[0-9][0-9]"
   [$] #DateTime
   [.] .hours << <hours
   [.] .minutes << <min
```

**Open point:** None.
