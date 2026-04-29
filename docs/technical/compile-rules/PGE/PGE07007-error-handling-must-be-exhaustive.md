---
audience: developer
rule: "7.7"
code: PGE07007
name: Error Handling Must Be Exhaustive
severity: error
---

# Rule 7.7 — Error Handling Must Be Exhaustive
`PGE07007`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->
<!-- @c:compile-rules/PGE/PGE06001-conditional-must-be-exhaustive -->

**Statement:** When calling a failable pipeline (one that declares `(-) !ErrorName`), the caller must address every declared error on every output port. If any declared error has no handler and no fallback for any output, PGE07007 fires. **No variable may compile if there is a non-zero probability it can reach Failed state without explicit handling.** This mirrors PGE06001 (Conditional Must Be Exhaustive) — just as every conditional branch must route every value, every failable call must route every error.
**Rationale:** Unaddressed errors cause silent pipeline termination — the caller believes downstream code will execute, but the pipeline ends without explanation. Explicit handling ensures the developer has acknowledged every failure mode, even if the chosen response is termination. This eliminates a class of "it just stopped" bugs. This is Polyglot's exhaustive coverage principle applied to error paths — the compiler demands that every scenario is accounted for before the pipeline runs, rather than discovering unhandled failures in production.

## Addressing Mechanisms

An error is "addressed" if any of the following cover it:

| Mechanism | Syntax | Scope |
|-----------|--------|-------|
| Specific handler | `[!] !ErrorName` block under `[-]` call | Addresses that specific error |
| Wildcard handler | `[!] !*` block under `[-]` call | Addresses all remaining errors |
| Specific fallback (scattered) | `(>) !Error.Name>` under `(-)` output line | Addresses that specific error on that output |
| Generic fallback (scattered) | `(>) !>` under `(-)` output line | Addresses all errors on that output |
| Specific fallback (grouped) | `($) >outputName !Error.Name>` under `(-) $label` | Addresses that specific error on that output |
| Generic fallback (grouped) | `($) >outputName !>` under `(-) $label` | Addresses all errors on that output |

If any declared error is not addressed by at least one mechanism, PGE07007 fires.

**Exemption:** `[b]` (fire-and-forget) calls are exempt from PGE07007. The `[b]` marker is an explicit acknowledgment that the caller does not participate in the called pipeline's error handling — the called pipeline handles its own errors internally. See [PGW03002](../PGW/PGW03002-error-handler-on-fire-and-forget.md).

## Exhaustiveness Algorithm

The compiler checks exhaustiveness per `[-]` call in the following steps:

### Step 1 — Collect declared errors

Collect the set **E** of all errors declared by the called pipeline's `(-) !ErrorName` lines:

```
E = { !E1, !E2, ..., !En }
```

If **E** is empty (non-failable pipeline), PGE07007 does not apply.

### Step 2 — Collect handler coverage

Collect the set **H** of errors addressed by `[!]` blocks under the call:

- Each `[!] !ErrorName` adds that error to **H**
- If `[!] !*` is present, **H = E** (wildcard covers all declared errors)

### Step 3 — Collect fallback coverage per output

For each output port **O** of the called pipeline, collect **F(O)** — the set of errors with fallbacks on that output:

- Each `(>) !ErrorName>` on that output's `(-)` line adds that error to **F(O)**
- If `(>) !>` is present on that output, **F(O) = E** (generic catch-all)
- Each `($) >outputName !ErrorName>` under `(-) $label` adds that error to **F(O)**
- If `($) >outputName !>` under `(-) $label`, **F(O) = E**

### Step 4 — Compute coverage per output

For each output port **O**:

```
Coverage(O) = H ∪ F(O)
```

`[!]` handlers contribute to all outputs (a handler that pushes a replacement value resolves the error for every output). Fallbacks only contribute to the specific output they are declared on.

### Step 5 — Check completeness

For each output port **O**: if **Coverage(O) ⊊ E** (strict subset), emit PGE07007 for each error in **E \ Coverage(O)**, naming the uncovered output port.

### Summary Table

| Has `[!]` for all errors? | Has fallback on all outputs? | Result |
|---------------------------|------------------------------|--------|
| Yes | — | Pass (handlers cover all) |
| — | Yes (all outputs have `!>`) | Pass (fallbacks cover all) |
| Partial | Partial (union = E) | Pass (union covers all) |
| Partial | Partial (union ⊊ E) | **PGE07007** |
| No | No | **PGE07007** |

## Multi-Output Coverage

When a failable call has multiple outputs, **every output port must have coverage for every declared error**. `[!]` handler blocks that push replacement values cover all outputs (since the handler body can write to any output port). Scattered `(>)` fallbacks and grouped `($)` fallbacks only cover the output they are declared on.

```polyglot
[ ] ✗ PGE07007 — >status has no coverage for !File.NotFound or !File.ReadError
{-} -ProcessMultiOutput
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <path#string
   (-) >content#string
   (-) >status#string
   [ ]
   [-] -File.Text.Read
      (-) <path << $path
      (-) >content >> $content
         (>) !> "unavailable"          [ ] ✓ >content covered
      (-) >status >> $status
                                        [ ] ✗ PGE07007 — >status: !File.NotFound unaddressed
                                        [ ] ✗ PGE07007 — >status: !File.ReadError unaddressed
```

Fix with grouped fallback or `[!]` blocks that write to both outputs:

```polyglot
[ ] ✓ grouped fallback covers all outputs
{-} -ProcessMultiOutputFixed
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <path#string
   (-) >content#string
   (-) >status#string
   [ ]
   [-] -File.Text.Read
      (-) <path << $path
      (-) >content >> $content
      (-) >status >> $status
      (-) $Read
         ($) >content !> "unavailable"
         ($) >status !> "error"
```

```polyglot
[ ] ✓ [!] blocks push to all outputs — covers everything
{-} -ProcessMultiOutputHandlers
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <path#string
   (-) >content#string
   (-) >status#string
   [ ]
   [-] -File.Text.Read
      (-) <path << $path
      (-) >content >> $content
      (-) >status >> $status
      [!] !*
         [-] >content << "unavailable"
         [-] >status << "error"
```

## Diagnostic Format

| Context | Format |
|---------|--------|
| Single output | `Unaddressed error '!ErrorName' from failable call '-PipelineName' at line N — add [!] !ErrorName handler, [!] !* wildcard, or (>) !> fallback` |
| Multi-output | `Output '>portName' has unaddressed error '!ErrorName' from failable call '-PipelineName' at line N` |

**See also:**
- [PGE06001 — Conditional Must Be Exhaustive](PGE06001-conditional-must-be-exhaustive.md) — the analogous rule for conditionals
- [PGE07001 — Error Block Scoping](PGE07001-error-block-scoping.md) — `[!]` blocks must be under their producing `[-]`
- [PGE07005 — Undeclared Error Raise](PGE07005-undeclared-error-raise.md) — pipeline-side: can't raise undeclared errors
- [PGW07001 — Error Handler on Non-Failable Call](../PGW/PGW07001-error-handler-on-non-failable-call.md) — inverse: handler on non-failable call
- [PGE02005 — Failed Must Resolve](PGE02005-failed-is-terminal.md) — variable lifecycle consequence of unhandled errors

---

## Examples

### VALID

```polyglot
[ ] ✓ all declared errors handled with specific [!] blocks
{-} -Process
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <path#string
   (-) >content#string
   [ ]
   [-] -File.Text.Read
      (-) <path << $path
      (-) >content >> $content
      [!] !File.NotFound
         [-] >content << "Error: file not found"
      [!] !File.ReadError
         [-] >content << "Error: could not read file"
   [-] >content << $content
```

```polyglot
[ ] ✓ [!] !* wildcard covers all errors — like [?] *? for conditionals
{-} -ProcessWildcard
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <path#string
   (-) >content#string
   [ ]
   [-] -File.Text.Read
      (-) <path << $path
      (-) >content >> $content
      [!] !*
         [-] >content << "Error: operation failed"
   [-] >content << $content
```

```polyglot
[ ] ✓ generic (>) !> fallback addresses all errors
{-} -ProcessFallback
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <path#string
   (-) >content#string
   [ ]
   [-] -File.Text.Read
      (-) <path << $path
      (-) >content >> $content
         (>) !> "unavailable"
   [-] >content << $content
```

```polyglot
[ ] ✓ mixed — specific [!] + [!] !* wildcard for the rest
{-} -ProcessMixed
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <path#string
   (-) >content#string
   [ ]
   [-] -File.Text.Read
      (-) <path << $path
      (-) >content >> $content
      [!] !File.NotFound
         [-] >content << "not found — using default"
      [!] !*
         [ ] all other errors (e.g., !File.ReadError) handled here
         [-] >content << "Error: read failed"
   [-] >content << $content
```

```polyglot
[ ] ✓ error-specific fallbacks cover each declared error
{-} -ProcessSpecificFallbacks
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <path#string
   (-) >content#string
   [ ]
   [-] -File.Text.Read
      (-) <path << $path
      (-) >content >> $content
         (>) !File.NotFound> "missing"
         (>) !File.ReadError> "unreadable"
   [-] >content << $content
```

```polyglot
[ ] ✓ grouped fallback under (-) $label
{-} -ProcessGrouped
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <path#string
   (-) >content#string
   [ ]
   [-] -File.Text.Read
      (-) <path << $path
      (-) >content >> $content
      (-) $Read
         ($) >content !> "unavailable"
```

```polyglot
[ ] ✓ grouped with error-specific and [!] blocks
{-} -ProcessGroupedMixed
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <path#string
   (-) >content#string
   [ ]
   [-] -File.Text.Read
      (-) <path << $path
      (-) >content >> $content
      (-) $Read
         [!] !File.NotFound
            [-] $Read>content << "not found"
         [!] !*
            [-] $Read>content << "error"
```

### INVALID

```polyglot
[ ] ✗ PGE07007 — no error handling on failable call
{-} -ProcessNone
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <path#string
   (-) >content#string
   [ ]
   [-] -File.Text.Read                              [ ] declares !File.NotFound, !File.ReadError
      (-) <path << $path
      (-) >content >> $content
                                                     [ ] ✗ PGE07007 — !File.NotFound unaddressed
                                                     [ ] ✗ PGE07007 — !File.ReadError unaddressed
   [-] >content << $content
```

```polyglot
[ ] ✗ PGE07007 — partial handling — !File.ReadError not addressed
{-} -ProcessPartial
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <path#string
   (-) >content#string
   [ ]
   [-] -File.Text.Read
      (-) <path << $path
      (-) >content >> $content
      [!] !File.NotFound
         [-] >content << "not found"
                                                     [ ] ✗ PGE07007 — !File.ReadError unaddressed
   [-] >content << $content
```

```polyglot
[ ] ✗ PGE07007 — specific fallback covers one error but not the other
{-} -ProcessPartialFallback
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <path#string
   (-) >content#string
   [ ]
   [-] -File.Text.Read
      (-) <path << $path
      (-) >content >> $content
         (>) !File.NotFound> "missing"
                                                     [ ] ✗ PGE07007 — !File.ReadError unaddressed
   [-] >content << $content
```

## See Also

- [[user/concepts/errors|Errors]] — references PGE07007 in declaring pipeline errors
