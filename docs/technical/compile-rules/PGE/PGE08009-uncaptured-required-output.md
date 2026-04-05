---
audience: developer
rule: "8.9"
code: PGE08009
name: Uncaptured Required Output at Call Site
severity: error
---

### Rule 8.9 — Uncaptured Required Output at Call Site
`PGE08009`

**Statement:** Every declared `>output` on the called pipeline that has no default value and no fallback must be captured by the caller — into a variable (`$name`), an output port (`>name`), or the discard pattern (`$*`). If the caller omits a required output, PGE08009 fires. This applies to sequential pipeline (`[r]`), parallel pipeline (`[p]`), expand operator (`[~]`), collect operator (`[*]`), and wrapper (`[W]`) calls. Fire-and-forget pipeline (`[b]`) calls are exempt.
**Rationale:** The compiler knows the full IO contract from the pipeline definition. An uncaptured output with no fallback means the pipeline produces a value that goes nowhere — this is either a wiring oversight or a logic error. If the caller intentionally discards the output, `$*` makes that explicit. Fallback values must cover all cases (success and error paths), not just the success path.
**Detection:** The compiler resolves the called pipeline's `[=]` declarations, enumerates all `>output` parameters, and checks each against the caller's wiring lines. Any `>output` that has no default (`<~`), no fallback (`[>] <!`), and no corresponding capture from the caller triggers PGE08009. `[b]` calls are exempt (see PGE07007 exemption).

**See also:**
- PGW02003 (unpushed output port — pipeline-body side: output declared but never pushed internally)
- PGW08003 (uncaptured output with default/fallback — the warning counterpart)
- PGE07007 (error handling must be exhaustive — analogous for errors)
- PGE01010 (pipeline IO name mismatch — wrong name, not missing capture)

**VALID:**
```polyglot
[ ] ✓ all outputs captured — sequential pipeline call
{=} =Process
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <data#string
   [=] >result#string
   [=] >count#int
   [r] >result << $data
   [r] >count << 1

{=} =UseProcess
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out#string
   [r] =Process
      [=] <data << "hello"
      [=] >result >> $res                  [ ] ✓ captured
      [=] >count >> $cnt                   [ ] ✓ captured
   [r] >out << $res
```

```polyglot
[ ] ✓ output captured into discard — intentionally unused
{=} =UseProcessDiscard
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out#string
   [r] =Process
      [=] <data << "hello"
      [=] >result >> >out
      [=] >count >> $*                     [ ] ✓ explicit discard — no error
```

```polyglot
[ ] ✓ fire-and-forget exempt from output capture
{=} =FireAndForget
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <data#string
   [b] =Logger                             [ ] ✓ [b] exempt — outputs not required
      [=] <message << $data
```

**INVALID:**
```polyglot
[ ] ✗ PGE08009 — uncaptured output on sequential pipeline call
{=} =UseProcessBad
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out#string
   [r] =Process
      [=] <data << "hello"
      [=] >result >> >out
                                            [ ] ✗ PGE08009 — >count not captured, has no fallback
```

```polyglot
[ ] ✗ PGE08009 — uncaptured output on parallel pipeline call
{=} =BatchBad
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out#string
   [p] =Process
      [=] <data << "hello"
      [=] >result >> $res
                                            [ ] ✗ PGE08009 — >count not captured, has no fallback
   [r] >out << $res
```

**Diagnostic:** "Uncaptured output `>{name}` on {operator type} call to `={PipelineName}` at line {N} — output has no fallback and must be wired (use `$*` to discard)"

### See Also

- [[user/concepts/pipelines/inline-calls|Inline Calls]] — references PGE08009 in call site rules

**Open point:** None.
