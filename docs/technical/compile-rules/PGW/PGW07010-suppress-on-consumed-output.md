---
audience: developer
rule: "7.10w"
code: PGW07010
name: Suppress on Consumed Output
severity: warning
---

# Rule 7.10w — Suppress on Consumed Output
`PGW07010`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:concepts/errors -->

**Statement:** `[!] !*-` suppresses all errors from a `[-]` call without pushing replacement values. If any output of the suppressed call is consumed downstream (wired to a variable that appears in a later expression, piped to another call, or collected), those outputs remain Failed with no recovery. PGW07010 warns that downstream code will receive Failed values.

**Rationale:** `[!] !*-` is intentional fire-and-forget at the error level — the developer explicitly chose to dismiss all errors. But if the call's outputs are consumed downstream, the developer likely expects usable values. The warning flags this mismatch so they can add explicit `(>) !>` fallbacks on consumed outputs, or switch to a full `[!] !*` block with replacement pushes.

**Detection:** The compiler checks each `[!] !*-` usage. For the parent `[-]` call, it inspects all `(-) >output >> $target` lines. If any `$target` variable is referenced in a downstream expression (assignment, pipeline input, conditional, collector), PGW07010 fires on that output.

**See also:**
- [PGE07007 — Error Handling Must Be Exhaustive](../PGE/PGE07007-error-handling-must-be-exhaustive.md) — `[!] !*-` satisfies exhaustive handling (wildcard covers all errors)
- [PGW07001 — Error Handler on Non-Failable Call](PGW07001-error-handler-on-non-failable-call.md) — dead handler on non-failable call
- [PGW03002 — Error Handler on Fire-and-Forget](PGW03002-error-handler-on-fire-and-forget.md) — `[b]` fire-and-forget is a different mechanism (detached execution, no error propagation)

**VALID:**
```polyglot
[ ] ✓ [!] !*- on pure side-effect call — no outputs consumed
[-] -File.Text.Write
   (-) <path << $logPath
   (-) <content << $event
   [!] !*-
```

```polyglot
[ ] ✓ [!] !*- with (>) !> fallback on consumed output — fallback provides recovery
[-] -File.Text.Read
   (-) <path << $configPath
   (-) >content >> $config
      (>) !> "{}"
   [!] !*-
```

**WARNING:**
```polyglot
[ ] ⚠ PGW07010 — !*- on call with consumed output $data, no fallback
[-] -File.Text.Read
   (-) <path << $file
   (-) >content >> $data
   [!] !*-

[ ] $data used downstream — will be Failed if -File.Text.Read errors
[-] -Transform.JSON
   (-) <input << $data
```

```polyglot
[ ] ⚠ PGW07010 — !*- on call with consumed output $result
[-] -API.Call
   (-) <url << $endpoint
   (-) >response >> $result
   (-) >status >> $code
   [!] !*-

[ ] Both $result and $code used downstream — both Failed if call errors
[?] $code =? 200
   [-] $output << $result
[?] *?
   [-] $output << "error"
```

**Fix:** Add `(>) !>` fallback on each consumed output to provide recovery values, or replace `[!] !*-` with a full `[!] !*` block that pushes replacements:

```polyglot
[ ] ✓ Fix option 1: add (>) !> fallback per consumed output
[-] -File.Text.Read
   (-) <path << $file
   (-) >content >> $data
      (>) !> ""
   [!] !*-
```

```polyglot
[ ] ✓ Fix option 2: use full [!] !* block with replacement pushes
[-] -File.Text.Read
   (-) <path << $file
   (-) >content >> $data
   (-) $Read
      [!] !*
         [-] $Read>content << ""
```

**Diagnostic:** "Error suppression `[!] !*-` at line N under `[-]` call with consumed output `>outputName` — downstream code will receive Failed value. Add `(>) !>` fallback or use `[!] !*` with replacement push"

**Open point:** None.
