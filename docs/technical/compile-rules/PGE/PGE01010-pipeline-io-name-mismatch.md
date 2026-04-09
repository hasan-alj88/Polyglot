---
audience: developer
rule: "1.10"
code: PGE01010
name: Pipeline IO Name Mismatch
severity: error
---

### Rule 1.10 — Pipeline IO Name Mismatch
`PGE01010`

**Statement:** Every `(-)` IO line under an `[-]` or `[=]` pipeline call must reference a parameter name declared in the target pipeline's IO section. An input (`<name`) or output (`>name`) that does not match any declared parameter is a compile error. This applies to both direct calls and parallel fork calls.
**Rationale:** Typos and stale parameter names silently fail at runtime. Catching name mismatches at compile time prevents wiring bugs and makes refactoring safe — renaming a pipeline parameter immediately surfaces all call sites that need updating.
**Detection:** The compiler resolves the target pipeline definition, enumerates its declared `<input` and `>output` parameters, and checks each `(-)` line's parameter name against that list. Any name not found in the declaration triggers PGE01010.

**See also:** PGE01009 (wrapper IO mismatch — same concept for `[W]` wrappers), PGE04001 (type mismatch — types match but names assumed correct), PGE08005 (unresolved step reference — same concept for chain step names)

**VALID:**
```polyglot
{-} -Greet
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <name#string
   (-) >greeting#string
   [-] >greeting << "Hello, {$name}!"

[ ] ✓ IO names match the pipeline declaration
[-] -Greet
   (-) <name#string << "Alice"           [ ] ✓ <name exists in -Greet
   (-) >greeting#string >> $result        [ ] ✓ >greeting exists in -Greet
```

```polyglot
{-} -Transform
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <input#string
   (-) >output#string
   [-] >output << $input

[ ] ✓ parallel fork with correct IO names
[=] -Transform
   (-) <input#string << $data             [ ] ✓ <input exists in -Transform
   (-) >output#string >> $transformed     [ ] ✓ >output exists in -Transform
```

**INVALID:**
```polyglot
{-} -Greet
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <name#string
   (-) >greeting#string
   [-] >greeting << "Hello, {$name}!"

[ ] ✗ PGE01010 — input name doesn't exist in target pipeline
[-] -Greet
   (-) <username#string << "Alice"        [ ] ✗ PGE01010 — -Greet has <name, not <username
   (-) >greeting#string >> $result
```

```polyglot
[ ] ✗ PGE01010 — output name doesn't exist in target pipeline
[-] -Greet
   (-) <name#string << "Alice"
   (-) >message#string >> $result         [ ] ✗ PGE01010 — -Greet has >greeting, not >message
```

```polyglot
{-} -Process
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <data#string
   (-) >result#string
   [-] >result << $data

[ ] ✗ PGE01010 — typo in parameter name
[-] -Process
   (-) <dta#string << $input              [ ] ✗ PGE01010 — -Process has <data, not <dta
   (-) >result#string >> $output
```

**Open point:** None.

### See Also

- [[concepts/pipelines/inline-calls|Inline Calls]] — documents IO name matching rules, references PGE01010
