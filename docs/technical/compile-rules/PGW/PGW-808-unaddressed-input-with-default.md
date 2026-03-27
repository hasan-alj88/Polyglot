---
rule: "8.8w"
code: PGW-808
name: Unaddressed Input With Default
severity: warning
---

### Rule 8.8w — Unaddressed Input With Default
`PGW-808`

**Statement:** A declared `<input` with a default value (`<~`) that the caller does not wire triggers a warning. The diagnostic message includes the default value so the caller can confirm it is acceptable. This warning is suppressible. Applies to sequential pipeline (`[r]`), parallel pipeline (`[p]`), expand operator (`[~]`), collect operator (`[*]`), and wrapper (`[W]`) calls.
**Rationale:** Default values are set by the pipeline author as sensible fallbacks — but callers may not be aware of what default they are implicitly accepting. Surfacing the default value in the warning gives the caller the information to either accept it (and suppress the warning) or wire an explicit value.
**Detection:** The compiler resolves the called pipeline's `[=]` declarations. For each `<input` that has a default (`<~`) and no corresponding wiring line from the caller, PGW-808 fires with the default value in the message.

**See also:**
- PGE-808 (missing required input — the error counterpart for inputs without defaults)
- PGW-809 (uncaptured output with default — analogous for outputs)

**VALID:**
```polyglot
[ ] ✓ default input explicitly wired — no warning
{=} =Format
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <text#string
   [=] <locale#string
   [r] $locale <~ "en-US"                  [ ] default value
   [=] >formatted#string
   [r] >formatted << "{$text} ({$locale})"

{=} =UseFormat
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out#string
   [r] =Format
      [=] <text << "Hello"
      [=] <locale << "fr-FR"               [ ] ✓ explicitly overrides default — no warning
      [=] >formatted >> >out
```

**WARNING:**
```polyglot
[ ] ⚠ PGW-808 — <locale not wired, using default "en-US"
{=} =UseFormatDefault
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out#string
   [r] =Format
      [=] <text << "Hello"
                                            [ ] ⚠ PGW-808 — <locale uses default "en-US"
      [=] >formatted >> >out
```

```polyglot
[ ] ⚠ PGW-808 — <timeout not wired on parallel pipeline call, using default 30
{=} =Fetch
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <url#string
   [=] <timeout#int
   [r] $timeout <~ 30
   [=] >body#string
   [r] >body << "response"

{=} =BatchFetch
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out#string
   [p] =Fetch
      [=] <url << "https://example.com"
                                            [ ] ⚠ PGW-808 — <timeout uses default 30
      [=] >body >> $body
   [r] >out << $body
```

**Diagnostic:** "Input `<{name}` on {operator type} call to `={PipelineName}` at line {N} uses default value `{default}` — wire explicitly or suppress this warning"

**Open point:** None.
