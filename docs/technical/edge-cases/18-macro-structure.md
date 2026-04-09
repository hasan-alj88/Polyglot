---
audience: designer
type: reference
updated: 2026-04-09
---

<!-- @edge-cases/INDEX -->

## 18. Wrapper Structure (S18)

> **Note:** This section was originally "Wrapper & Macro Structure." The `{M}` macro block type was retired in Issue #272 (parameterized ## schemas replace macros). EC-18.4 (zero-parameter macro) is retired — see PGE01023 redirect stub. Wrapper edge cases remain unchanged.

### EC-18.1: Minimal wrapper — `[{]` input, `[}]` output, `[\]` setup, `[/]` cleanup

<!-- @blocks:Scope -->
<!-- @pipelines:Wrappers -->
**EBNF:** `wrapper_def ::= "{W}" pipeline_id NEWLINE { indent wrapper_body_line NEWLINE }` (§9.5)

**What it tests:** Complete `{W}` structure with all four scope markers. No `[T]`, `[Q]`, or `[=]` IO. See [[blocks#Scope]], [[concepts/pipelines/wrappers#Wrappers]].

```polyglot
{W} =W.DB.Transaction
   [{] $connectionString#string
   [}] $dbConn#serial

   [\]
      [r] =DB.Connect
         [=] <connStr << $connectionString
         [=] >conn >> $dbConn
      [r] =DB.Begin
         [=] <conn << $dbConn

   [/]
      [r] =DB.Commit
         [=] <conn << $dbConn
      [r] =DB.Disconnect
         [=] <conn << $dbConn
```

### EC-18.2: Wrapper usage site — wrapper IO wired with `[=]` using `$` variables

**EBNF:** `wrapper_line ::= "[W]" pipeline_ref NEWLINE { indent wrapper_io_line NEWLINE }` where `wrapper_io_line ::= "[=]" variable_io`

**What it tests:** `[W]` wires wrapper IO using `[=]` with `$` variables. `[}]` outputs become available in body. See [[concepts/pipelines/wrappers#Wrappers]].

```polyglot
{=} =Invoice.Save
   [=] <invoice#Invoice
   [T] =T.Call
   [Q] =Q.Default
   [W] =W.DB.Transaction
      [=] $connectionString << $dbConnStr
      [=] $dbConn >> $dbConn
   [ ] $dbConn available from wrapper [}] output
   [r] =DB.Insert
      [=] <conn << $dbConn
      [=] <data << $invoice
      [=] >id >> >savedId
```

### EC-18.3: `{W}` with no `[}]` output — setup/cleanup only

**What it tests:** A wrapper that provides lifecycle scope but exposes no outputs to the pipeline.

```polyglot
{W} =W.AuditScope
   [{] $userId#string
   [{] $action#string

   [\]
      [r] =Audit.Open
         [=] <userId << $userId
         [=] <action << $action
         [=] >token >> $auditToken

   [/]
      [r] =Audit.Close
         [=] <token << $auditToken
```

### EC-18.4: *(Retired)* Zero-parameter macro

**Status:** Retired — `{M}` macro block type removed in Issue #272. See [[compile-rules/PGE/PGE01023-parameterless-macro|PGE01023 redirect stub]].
