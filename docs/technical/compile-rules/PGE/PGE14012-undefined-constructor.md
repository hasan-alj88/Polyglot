---
audience: developer
rule: "14.12"
code: PGE14012
name: Undefined Constructor
severity: error
---

# Rule 14.12 — Undefined Constructor
`PGE14012`

<!-- @u:syntax/constructors -->
<!-- @c:technical/ebnf/definition-blocks/09-13-constructor -->

**Statement:** A constructor call references a constructor name for which no `{$}` definition exists in the current scope. The compiler cannot find any `{$}` block defining the referenced `$Constructor`.

**Rationale:** Simple undefined-reference error. Constructors must be defined before use — either in the same package, in an `[@]`-imported package, or in pglib. Without a definition, the compiler has no overloads to match against.

**Detection:** Compiler looks up the constructor name in the current scope (local package definitions, `[@]`-imported packages, pglib). If no `{$}` definition exists for that name, PGE14012 is raised.

**See also:** PGE14010 (no overload match — constructor exists but no pattern matches), PGE09003 (unresolved pipeline reference — analogous for pipelines), [[syntax/constructors]]

---

**VALID:**
```aljam3
[ ] ✓ $DT is defined by pglib — constructor call accepted
{-} -MyPipeline
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3

   [ ]
   [-] $t << $DT"Today"
```

**INVALID:**
```aljam3
[ ] ✗ PGE14012 — no {$} $Foo definition exists in scope
{-} -MyPipeline
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3

   [ ]
   [-] $x << $Foo"bar"
```

**Open point:** None.
