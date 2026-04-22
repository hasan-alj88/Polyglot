---
audience: developer
rule: "14.2"
code: PGE14002
name: Duplicate Constructor Keyword
severity: error
---

# Rule 14.2 — Duplicate Constructor Keyword
`PGE14002`

<!-- @u:syntax/constructors -->
<!-- @c:technical/ebnf/definition-blocks/09-13-constructor -->

**Statement:** Two keyword overloads for the same constructor name use the same literal string. Keyword overloads compile to exact-match regex — identical literals always overlap.

**Rationale:** Special case of PGE14001 detected via exact string comparison rather than regex intersection. Simpler and faster to detect, produces a clearer error message.

**Detection:** Compiler collects all keyword overloads (those with no `($)` capture lines) per constructor name and checks for duplicate literal strings. Exact string comparison — no regex needed.

**See also:** PGE14001 (general overload ambiguity), [[syntax/constructors]]

---

**VALID:**
```polyglot
[ ] ✓ Two keyword overloads with different literals

{$} $DT"Today"
   [$] #DateTime
   [.] .hours << 0
   [.] .minutes << 0
   [.] .seconds << 0

{$} $DT"Now"
   [$] #DateTime
   [-] $now << -Time.Now
   [.] .hours << $now>hours
   [.] .minutes << $now>minutes
   [.] .seconds << $now>seconds
```

**INVALID:**
```polyglot
[ ] ✗ PGE14002 — duplicate keyword "Today"

{$} $DT"Today"
   [$] #DateTime
   [.] .hours << 0
   [.] .minutes << 0
   [.] .seconds << 0

{$} $DT"Today"
   [$] #DateTime
   [.] .hours << 12
   [.] .minutes << 0
   [.] .seconds << 0
```

**Open point:** None.
