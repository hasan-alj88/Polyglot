---
audience: design
type: reference
updated: 2026-04-09
---

<!-- @edge-cases/INDEX -->

## 18. Wrapper Structure (S18)

> **Note:** This section was originally "Wrapper & Macro Structure." Macros were retired in Issue #272 (parameterized ## schemas replace macros). EC-18.4 (zero-parameter macro) is retired — see PGE01023 redirect stub. Wrapper edge cases remain unchanged.

### EC-18.1: Minimal wrapper — `(-)` IO, `[\]` setup, `[/]` cleanup

<!-- @u:blocks:Scope -->
<!-- @u:pipelines:Wrappers -->
**EBNF:** `wrapper_def ::= "{W}" pipeline_id NEWLINE { indent wrapper_body_line NEWLINE }` (§9.5)

**What it tests:** Complete `{W}` structure with setup/cleanup and IO. No `[T]`, `[Q]`, or pipeline-level IO. See [[blocks#Scope]], [[concepts/pipelines/wrappers#Wrappers]].

```polyglot
{W} -W.DB.Transaction
   (-) <connectionString;string
   (-) >dbConn;serial

   [\]
      [-] -DB.Connect
         (-) <connStr << $connectionString
         (-) >conn >> $dbConn
      [-] -DB.Begin
         (-) <conn << $dbConn

   [/]
      [-] -DB.Commit
         (-) <conn << $dbConn
      [-] -DB.Disconnect
         (-) <conn << $dbConn
```

### EC-18.2: Wrapper usage site — wrapper IO wired with `(-)` using `$` variables

**EBNF:** `wrapper_line ::= "[W]" pipeline_ref NEWLINE { indent wrapper_io_line NEWLINE }` where `wrapper_io_line ::= "(-)" variable_io`

**What it tests:** `[W]` wires wrapper IO using `(-)` with `$` variables. Wrapper outputs become available in body. See [[concepts/pipelines/wrappers#Wrappers]].

```polyglot
{-} -Invoice.Save
   (-) <invoice#Invoice
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.DB.Transaction
      (-) $connectionString << $dbConnStr
      (-) $dbConn >> $dbConn
   [ ] $dbConn available from wrapper output
   [ ]
   [-] -DB.Insert
      (-) <conn << $dbConn
      (-) <data << $invoice
      (-) >id >> >savedId
```

### EC-18.3: `{W}` with no output — setup/cleanup only

**What it tests:** A wrapper that provides lifecycle scope but exposes no outputs to the pipeline.

```polyglot
{W} -W.AuditScope
   (-) <userId;string
   (-) <action;string

   [\]
      [-] -Audit.Open
         (-) <userId << $userId
         (-) <action << $action
         (-) >token >> $auditToken

   [/]
      [-] -Audit.Close
         (-) <token << $auditToken
```

### EC-18.4: *(Retired)* Zero-parameter macro

**Status:** Retired — macro block type removed in Issue #272. Parameterized `##` schemas with `[#]` inputs now handle type generation. See [[compile-rules/PGE/PGE01023-parameterless-macro|PGE01023 redirect stub]].
