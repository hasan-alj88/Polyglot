---
audience: developer
type: reference
updated: 2026-03-30
---

<!-- @edge-cases/INDEX -->

## 18. Macro Structure (S18)

### EC-18.1: Minimal macro — `[{]` input, `[}]` output, `[\]` setup, `[/]` cleanup

<!-- @blocks:Scope -->
<!-- @pipelines:Wrappers -->
**EBNF:** `macro_def ::= "{M}" pipeline_id NEWLINE { indent macro_body_line NEWLINE }`

**What it tests:** Complete `{M}` structure with all four scope markers. No `[t]`, `[Q]`, or `[=]` IO. See [[blocks#Scope]], [[concepts/pipelines/wrappers#Wrappers]].

```polyglot
{M} =W.DB.Transaction
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

### EC-18.2: Wrapper usage site — macro IO wired with `[=]` using `$` variables

**EBNF:** `wrapper_line ::= "[W]" pipeline_ref NEWLINE { indent wrapper_io_line NEWLINE }` where `wrapper_io_line ::= "[=]" variable_io`

**What it tests:** `[W]` wires macro IO using `[=]` with `$` variables. `[}]` outputs become available in body. See [[concepts/pipelines/wrappers#Wrappers]].

```polyglot
{=} =Invoice.Save
   [=] <invoice#Invoice
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.DB.Transaction
      [=] $connectionString << $dbConnStr
      [=] $dbConn >> $dbConn
   [ ] $dbConn available from macro [}] output
   [r] =DB.Insert
      [=] <conn << $dbConn
      [=] <data << $invoice
      [=] >id >> >savedId
```

### EC-18.3: `{M}` with no `[}]` output — setup/cleanup only

**What it tests:** A macro that provides lifecycle scope but exposes no outputs to the pipeline.

```polyglot
{M} =W.AuditScope
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
