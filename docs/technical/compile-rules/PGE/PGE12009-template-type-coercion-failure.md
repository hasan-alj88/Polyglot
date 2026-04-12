---
audience: developer
rule: "10.2"
code: PGE12009
name: Template Type Coercion Failure
severity: error
---

# Rule 10.2 — Template Type Coercion Failure
`PGE12009`

<!-- @u:concepts/pipelines/inline-calls -->

**Statement:** When the compiler extracts a value from a `%InlineString` template placeholder and the extracted string cannot be coerced to the target input's declared type, the call is a compile error.
**Rationale:** Template extraction always produces strings. If the target input expects a non-string type (e.g., `#int`, `#float`, `#path`), the compiler applies type coercion. When coercion fails (e.g., extracting `"abc"` for an `#int` input), the error is caught at compile time rather than causing a runtime failure.
**Detection:** After template extraction, the compiler attempts to coerce each extracted string value to the corresponding input's declared type. If coercion fails, PGE12009 is raised at the call site. Coercion follows the standard `#String` subtype rules.

**See also:** PGE12005 (format mismatch — string doesn't match template pattern), PGE04001 (general type mismatch)

---

**VALID:**
```polyglot
{-} -Repeat
   (-) %InlineString << "{text}:{count}"
   (-) <text#string
   (-) <count#int
   (-) >result#string
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [ ] ...

[ ] ✓ "3" coerces to #int
[-] $out << -Repeat"hello:3"
```

**INVALID:**
```polyglot
[ ] ✗ PGE12009 — "abc" cannot coerce to #int for <count
[-] $out << -Repeat"hello:abc"

[ ] ✗ PGE12009 — "3.5" cannot coerce to #int for <count
[-] $out << -Repeat"hello:3.5"
```

**Open point:** None.
