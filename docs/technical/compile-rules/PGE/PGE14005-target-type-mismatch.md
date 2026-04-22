---
audience: developer
rule: "14.5"
code: PGE14005
name: Target Type Mismatch
severity: error
---

# Rule 14.5 — Target Type Mismatch
`PGE14005`

<!-- @u:syntax/constructors -->
<!-- @c:technical/ebnf/definition-blocks/09-13-constructor -->

**Statement:** A [.] field mapping in a {$} constructor references a field that does not exist on the target {#} type declared by [$].

**Rationale:** Field assignments must map to actual fields of the target type. Mapping to a nonexistent field indicates a typo or misunderstanding of the target type's topology. The compiler cross-references all [.] assignments against the target type's declared fields.

**Detection:** Compiler resolves the [$] target type, enumerates its declared fields (from the {#} definition), then checks each [.] assignment's field name against that enumeration. If any [.] references a field not in the target type, PGE14005 is raised.

**See also:** PGE14007 (incomplete field mapping — the inverse problem)

---

**VALID:**
```polyglot
[ ] ✓ all [.] fields exist on the [$] target type
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
```polyglot
[ ] ✗ PGE14005 — .nonexistent is not a field of #DT.Time
{$} $DT"(?<hours>\d{2}):(?<minutes>\d{2}):(?<seconds>\d{2})"
    ($) <hours.re << "\d{2}"
    ($) <minutes.re << "\d{2}"
    ($) <seconds.re << "\d{2}"
    [$] #DT.Time
    [.] .nonexistent << <hours
    [.] .minutes << <minutes
    [.] .seconds << <seconds
```

**Open point:** None.
