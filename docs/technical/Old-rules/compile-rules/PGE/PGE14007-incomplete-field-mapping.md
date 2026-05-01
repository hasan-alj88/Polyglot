---
audience: developer
rule: "14.7"
code: PGE14007
name: Incomplete Field Mapping
severity: error
---

# Rule 14.7 — Incomplete Field Mapping
`PGE14007`

<!-- @u:syntax/constructors -->
<!-- @c:technical/ebnf/definition-blocks/09-13-constructor -->

**Statement:** A {$} constructor does not map all required fields of the target {#} type declared by [$]. Every required field must receive a value from a capture, metadata source, or constructor-sourced variable.

**Rationale:** The constructor contract guarantees a complete, valid Final value. If required fields are left unmapped, the produced value would have holes — violating the guarantee that every constructor invocation produces a fully-formed typed tree. Fields with defaults (<~) in the {#} definition are not required to be mapped.

**Detection:** Compiler resolves the [$] target type, identifies all required fields (those without <~ defaults), then checks that each required field has a corresponding [.] assignment in the {$} block. If any required field is unmapped, PGE14007 is raised.

**See also:** PGE14005 (target type mismatch — the related but opposite problem: mapping to fields that don't exist)

---

**VALID:**
```aljam3
[ ] ✓ all required fields of #DT.Time are mapped
{$} $DT"(?<hours>\d{2}):(?<minutes>\d{2}):(?<seconds>\d{2})"
    ($) <hours.re << "\d{2}"
    ($) <minutes.re << "\d{2}"
    ($) <seconds.re << "\d{2}"
    [$] #DT.Time
    [.] .hours << <hours
    [.] .minutes << <minutes
    [.] .seconds << <seconds
```

**INVALID:**
```aljam3
[ ] ✗ PGE14007 — .minutes and .seconds are required but unmapped
{$} $DT"(?<hours>\d{2})"
    ($) <hours.re << "\d{2}"
    [$] #DT.Time
    [.] .hours << <hours
```

**Open point:** None.
