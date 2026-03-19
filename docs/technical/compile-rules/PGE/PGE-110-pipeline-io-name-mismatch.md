---
rule: "1.10"
code: PGE-110
name: Pipeline IO Name Mismatch
severity: error
---

### Rule 1.10 — Pipeline IO Name Mismatch
`PGE-110`

**Statement:** Every `[=]` IO line under an `[r]` or `[p]` pipeline call must reference a parameter name declared in the target pipeline's IO section. An input (`<name`) or output (`>name`) that does not match any declared parameter is a compile error. This applies to both direct calls and parallel fork calls.
**Rationale:** Typos and stale parameter names silently fail at runtime. Catching name mismatches at compile time prevents wiring bugs and makes refactoring safe — renaming a pipeline parameter immediately surfaces all call sites that need updating.
**Detection:** The compiler resolves the target pipeline definition, enumerates its declared `<input` and `>output` parameters, and checks each `[=]` line's parameter name against that list. Any name not found in the declaration triggers PGE-110.

**See also:** PGE-109 (wrapper IO mismatch — same concept for `[W]` wrappers and `{M}` macros), PGE-401 (type mismatch — types match but names assumed correct), PGE-805 (unresolved step reference — same concept for chain step names)

**VALID:**
```polyglot
{=} =Greet
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <name;string
   [=] >greeting;string
   [r] >greeting << "Hello, {$name}!"

[ ] ✓ IO names match the pipeline declaration
[r] =Greet
   [=] <name;string << "Alice"           [ ] ✓ <name exists in =Greet
   [=] >greeting;string >> $result        [ ] ✓ >greeting exists in =Greet
```

```polyglot
{=} =Transform
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input;string
   [=] >output;string
   [r] >output << $input

[ ] ✓ parallel fork with correct IO names
[p] =Transform
   [=] <input;string << $data             [ ] ✓ <input exists in =Transform
   [=] >output;string >> $transformed     [ ] ✓ >output exists in =Transform
```

**INVALID:**
```polyglot
{=} =Greet
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <name;string
   [=] >greeting;string
   [r] >greeting << "Hello, {$name}!"

[ ] ✗ PGE-110 — input name doesn't exist in target pipeline
[r] =Greet
   [=] <username;string << "Alice"        [ ] ✗ PGE-110 — =Greet has <name, not <username
   [=] >greeting;string >> $result
```

```polyglot
[ ] ✗ PGE-110 — output name doesn't exist in target pipeline
[r] =Greet
   [=] <name;string << "Alice"
   [=] >message;string >> $result         [ ] ✗ PGE-110 — =Greet has >greeting, not >message
```

```polyglot
{=} =Process
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <data;string
   [=] >result;string
   [r] >result << $data

[ ] ✗ PGE-110 — typo in parameter name
[r] =Process
   [=] <dta;string << $input              [ ] ✗ PGE-110 — =Process has <data, not <dta
   [=] >result;string >> $output
```

**Open point:** None.
