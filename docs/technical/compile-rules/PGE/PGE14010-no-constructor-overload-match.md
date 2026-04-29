---
audience: developer
rule: "14.10"
code: PGE14010
name: No Constructor Overload Match
severity: error
---

# Rule 14.10 — No Constructor Overload Match
`PGE14010`

<!-- @u:syntax/constructors -->
<!-- @c:technical/ebnf/definition-blocks/09-13-constructor -->

**Statement:** A constructor call `$Constructor"string"` provides an argument that does not match any overload's compiled regex pattern. The compiler tests the literal argument against all overloads and finds zero matches.

**Rationale:** Constructors guarantee a valid Final value for every invocation. If no overload matches the argument, no value can be produced. Unlike pipeline calls where runtime errors are handled with `[!]`, constructor calls must succeed — so an unmatched argument is a compile error.

**Detection:** Compiler compiles each overload's full-pattern regex, tests the call-site argument against all of them. If zero overloads match, PGE14010 is raised. The error message lists available overloads and their patterns.

**See also:** PGE14001 (ambiguous overloads — the definition-side complement), PGE14012 (undefined constructor — no definitions at all), [[syntax/constructors]]

---

**Error Message:**
```
error[PGE14010]: no constructor overload matches input string
  --> src/pipeline.pg:8:20
   |
8  |    [-] $t << $DT"not-a-date"
   |                  ^^^^^^^^^^^^ does not match any $DT overload
   |
   = note: available overloads:
           $DT"{hours}:{min}:{seconds}"  regex: ^([0-2][0-9]):([0-5][0-9]):([0-5][0-9])$
           $DT"{year}-{month}-{day}"     regex: ^([0-9]{4})-([0-1][0-9])-([0-3][0-9])$
           $DT"Today"                    regex: ^Today$
           $DT"Now"                      regex: ^Now$
```

**VALID:**
```polyglot
[ ] ✓ argument matches the time overload pattern
{-} -MyPipeline
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot

   [ ]
   [-] $t << $DT"14:30:00"
```

**INVALID:**
```polyglot
[ ] ✗ PGE14010 — "not-a-date" does not match any $DT overload
{-} -MyPipeline
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot

   [ ]
   [-] $t << $DT"not-a-date"
```

**Open point:** None.
