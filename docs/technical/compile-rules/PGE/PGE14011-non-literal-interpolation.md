---
audience: developer
rule: "14.11"
code: PGE14011
name: Non-Literal Interpolation
severity: error
---

# Rule 14.11 — Non-Literal Interpolation
`PGE14011`

<!-- @u:syntax/constructors -->
<!-- @c:technical/ebnf/definition-blocks/09-13-constructor -->

**Statement:** A constructor call `$Constructor"{$var}"` interpolates a variable that was not produced by another constructor. Only constructor-sourced variables may appear in constructor interpolation. Variables from IO inputs, trigger outputs, or pipeline call results are rejected.

**Rationale:** Analogous to SQL prepared statements — structure is fixed at compile time, only typed parameter slots accept data. A runtime string can never become constructor input because the compiler cannot prove it satisfies the regex constraints at compile time. If you need to parse a dynamic string, use a pipeline call with error handling (e.g., `-Type.Parse`).

**Detection:** Compiler traces the origin of each interpolated `$var`. The variable must have been assigned from a `$Constructor"..."` expression (constructor-sourced). If the variable's origin is: IO input (`(-) <param`), trigger output, pipeline call result (`[-] $var << -Pipeline`), or any non-constructor source, PGE14011 is raised.

**See also:** PGE14013 (interpolation source not Final — lifecycle check after origin check), [[syntax/constructors|Interpolation Rule]]

---

**Error Message:**
```
error[PGE14011]: constructor interpolation requires constructor-sourced variable
  --> src/pipeline.pg:12:25
   |
12 |    [-] $date << $DT"{$userInput}"
   |                      ^^^^^^^^^^ '$userInput' is from trigger IO input
   |
   = note: '$userInput' declared at line 4: (-) <userInput#string
   = help: constructor arguments only accept literals or variables
           produced by other constructors
   = help: use pipeline '-DT.Parse' for runtime string parsing:
           [-] $date#DT << -DT.Parse
              (<) <raw#string << $userInput
              [!] !InvalidFormat
                 [-] ...handle error...
```

**VALID:**
```polyglot
[ ] ✓ $basePath is constructor-sourced — interpolation allowed
{-} -MyPipeline
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot

   [-] $basePath << $Path"/reports"
   [-] $fullPath << $Path"{$basePath}/daily"
```

**INVALID:**
```polyglot
[ ] ✗ PGE14011 — $userInput is IO-sourced, not constructor-sourced
{-} -MyPipeline
   (-) <userInput#string
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot

   [-] $parsed << $Path"{$userInput}"
```

**Open point:** None.
