---
rule: "8.9w"
code: PGW-809
name: Uncaptured Output With Default/Fallback
severity: warning
---

### Rule 8.9w — Uncaptured Output With Default/Fallback
`PGW-809`

**Statement:** A declared `>output` with a default value (`<~`) or fallback (`[>] <!`) that the caller does not capture triggers a warning. The diagnostic message includes the default or fallback value(s) so the caller can confirm they are acceptable. This warning is suppressible. Fallback values must cover all cases (success and error paths), not just the success case. Applies to sequential pipeline (`[r]`), parallel pipeline (`[p]`), expand operator (`[~]`), collect operator (`[*]`), and wrapper (`[W]`) calls.
**Rationale:** Default and fallback values are set by the pipeline author — but callers may not be aware of what values they are implicitly discarding. Surfacing the value in the warning gives the caller the information to either accept it (and suppress the warning) or capture explicitly.
**Detection:** The compiler resolves the called pipeline's `[=]` declarations. For each `>output` that has a default (`<~`) or fallback (`[>] <!`) and no corresponding capture from the caller, PGW-809 fires with the default/fallback value(s) in the message.

**See also:**
- PGE-809 (uncaptured required output — the error counterpart for outputs without defaults/fallbacks)
- PGW-808 (unaddressed input with default — analogous for inputs)
- PGW-203 (unpushed output port — pipeline-body side, not call-site)

**VALID:**
```polyglot
[ ] ✓ output with default captured explicitly — no warning
{=} =Lookup
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <key;string
   [=] >value;string
   [r] $value <~ "unknown"                 [ ] default value
   [=] >found;bool
   [r] $found <~ .False                    [ ] default value
   [r] >value << $value
   [r] >found << .True

{=} =UseLookup
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out;string
   [r] =Lookup
      [=] <key << "user.name"
      [=] >value >> $val                   [ ] ✓ captured
      [=] >found >> $flag                  [ ] ✓ captured
   [r] >out << $val
```

**WARNING:**
```polyglot
[ ] ⚠ PGW-809 — >found not captured, has default .False
{=} =UseLookupPartial
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out;string
   [r] =Lookup
      [=] <key << "user.name"
      [=] >value >> >out
                                            [ ] ⚠ PGW-809 — >found has default .False
```

```polyglot
[ ] ⚠ PGW-809 — >content not captured, has fallback "unavailable"
{=} =ReadFile
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <path;string
   [=] >content;string
   [=] !File.NotFound
   [=] !File.ReadError
   [r] =File.Text.Read
      [=] <path << $path
      [=] >content >> $content
         [>] <! "unavailable"
   [r] >content << $content

{=} =UseReadFile
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out;string
   [r] =ReadFile
      [=] <path << "/tmp/data.txt"
                                            [ ] ⚠ PGW-809 — >content has fallback "unavailable"
      [!] !*
         [r] >out << "error"
   [r] >out << "done"
```

**Diagnostic:** "Output `>{name}` on {operator type} call to `={PipelineName}` at line {N} has {default/fallback} value `{value}` — capture explicitly or suppress this warning"

**Open point:** None.
