---
audience: developer
rule: "14.13"
code: PGE14013
name: Interpolation Source Not Final
severity: error
---

# Rule 14.13 — Interpolation Source Not Final
`PGE14013`

<!-- @u:syntax/constructors -->
<!-- @c:technical/ebnf/definition-blocks/09-13-constructor -->
<!-- @c:technical/compile-rules/PGE/PGE02003 -->

**Statement:** A constructor interpolation `{$var}` references a constructor-sourced variable that is not in Final state at the point of use. The interpolated variable must be Final — not Declared or Default.

**Rationale:** Constructor interpolation substitutes the variable's value into the pattern before regex matching. If the variable is not Final, its value is not yet determined — the compiler cannot prove the composed string will match any overload. This check runs after PGE14011 (origin check) — the variable is known to be constructor-sourced but may not yet have reached Final state.

**Detection:** Compiler performs lifecycle analysis on the interpolated variable at the point of the constructor call. The variable must be in Final state (assigned via `<<` from a constructor expression). If it is still Declared (no assignment) or Default (`<~` but not yet pushed to Final), PGE14013 is raised.

**See also:** PGE14011 (non-literal interpolation — origin check), PGE02003 (Final is push-once — lifecycle rule), [[syntax/constructors|Interpolation Rule]]

---

**VALID:**
```polyglot
[ ] ✓ $base is Final after constructor assignment — interpolation accepted
{-} -MyPipeline
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot

   [ ]
   [-] $base << $Path"/reports"
   [-] $full << $Path"{$base}/daily"
```

**INVALID:**
```polyglot
[ ] ✗ PGE14013 — $base is Declared (no assignment yet) at point of use
{-} -MyPipeline
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot

   [ ]
   [-] $base#path
   [-] $full << $Path"{$base}/daily"
   [-] $base << $Path"/reports"
```

**Open point:** None.
