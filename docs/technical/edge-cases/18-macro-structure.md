---
audience: designer
type: reference
updated: 2026-03-31
---

<!-- @edge-cases/INDEX -->

## 18. Wrapper & Macro Structure (S18)

### EC-18.1: Minimal wrapper — `[{]` input, `[}]` output, `[\]` setup, `[/]` cleanup

<!-- @blocks:Scope -->
<!-- @pipelines:Wrappers -->
**EBNF:** `wrapper_def ::= "{W}" pipeline_id NEWLINE { indent wrapper_body_line NEWLINE }` (§9.4b)

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

### EC-18.4: Zero-parameter macro — should be `{#}` instead

**EBNF ref:** `macro_def` (§9.4) — requires at least one `macro_param` or `macro_type_param`
**What it tests:** A `{M}` with no `[#]` parameters. PGE01023 fires. See [[concepts/macros|macros]].

```polyglot
[ ] ✗ PGE01023 — no parameters, use {#} instead
{M} #Singleton
   {#} #Singleton
      [.] .instance#string << "only"
```
