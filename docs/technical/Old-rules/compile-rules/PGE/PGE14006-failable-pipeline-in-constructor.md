---
audience: developer
rule: "14.6"
code: PGE14006
name: Failable Pipeline In Constructor
severity: error
---

# Rule 14.6 — Failable Pipeline In Constructor
`PGE14006`

<!-- @u:syntax/constructors -->
<!-- @c:technical/ebnf/definition-blocks/09-13-constructor -->

**Statement:** A user-defined {$} constructor contains a [-] pipeline call. Only jm3lib constructors (backed by {N} native definitions) may use [-] calls inside {$} blocks.

**Rationale:** The constructor contract guarantees no error surface — every invocation produces a valid Final value. User-defined pipeline calls introduce async complexity and potential failure modes that the compiler cannot prove infallible. Only jm3lib's {N} native operations carry the compiler's trust for infallibility. User constructors must use regex-based string parsing exclusively.

**Detection:** Compiler scans {$} block body for [-] lines. If found, checks whether the containing {$} is a jm3lib definition (backed by {N} native). If not jm3lib, PGE14006 is raised.

**See also:** PGE14003 (missing capture regex — the mechanism user constructors must use instead)

---

**VALID:**
```aljam3
[ ] ✓ jm3lib constructor may use [-] native pipeline calls
{$} $DT"Now"
    [$] #DT.Time
    [-] -DT.Current
    [.] .hours << >hours
    [.] .minutes << >minutes
    [.] .seconds << >seconds
```

**INVALID:**
```aljam3
[ ] ✗ PGE14006 — user-defined constructor cannot contain [-] pipeline calls
{$} $MyType"(?<data>.+)"
    ($) <data.re << ".+"
    [$] #MyType
    [-] -SomeApi.Fetch
    [.] .result << >response
```

**Open point:** None.
