---
audience: developer
type: reference
updated: 2026-03-30
---

<!-- @edge-cases/INDEX -->

## 10. Execution Statements (S10)

### EC-10.1: Pipeline call with IO and error

<!-- @pipelines:Error Handling -->
<!-- @io:Pipeline Call -->
**EBNF:** `pipeline_call ::= pipeline_ref NEWLINE { indent call_io_line NEWLINE } { indent error_block NEWLINE }`

**What it tests:** Full call structure: ref -> IO lines -> error blocks scoped under call. See [[concepts/pipelines/error-handling#Error Handling]], [[io#Pipeline Call]].

```polyglot
[r] @AD=Account.Create
   [=] <name << $fullName
   [=] <email << $email
   [=] >id >> $adId
   [!] !AD.CreateFailed
      [r] $adId << "FAILED"
   [!] !AD.Timeout
      [r] $adId << "TIMEOUT"
```

### EC-10.2: Stdlib pipeline call — no import needed

<!-- @EBNF:pipeline_ref -->
**EBNF:** `pipeline_ref ::= pipeline_id` — stdlib uses `=` prefix like all pipelines.

**What it tests:** `=File.Text.Append` with `=` prefix (all identifiers have a prefix, no exceptions). No `[@]` import needed. See [[concepts/pipelines/INDEX|pipelines]], [[technical/ebnf/10-execution#10.2]].

```polyglot
[r] =File.Text.Append
   [=] <path << "/var/log/app.log"
   [=] <content << $message
   [=] >written >> $ok
   [!] !File.WriteError
```

### EC-10.3: Data load

**EBNF:** `data_load ::= "[#]" assignment_expr`

**What it tests:** `[#]` block element for loading serialized data into typed structures. See [[blocks#Execution]].

```polyglot
[ ] In execution: deserialize serial into typed data
[#] $hire#NewHire << $payload

[ ] In {#} definitions: load external config files
{#} #Config
   [#] #file1 << =Json.LoadFile"/config/appsettings.json"
   [.] .dbConnection#string <~ #file1.db.connectionString
```

### EC-10.4: Parallel execution

<!-- @blocks:Execution -->
**What it tests:** `[p]` for parallel runs. See [[blocks#Execution]].

```polyglot
[p] @AD=Account.Create
   [=] <name << $name

[p] @Mail=Mailbox.Provision
   [=] <email << $email
```

### EC-10.5: Chain execution — explicit multi-IO wiring

<!-- @pipelines:Chain Execution -->
<!-- @io:Chain IO Addressing -->
**EBNF:** `chain_call ::= pipeline_ref "=>" pipeline_ref { "=>" pipeline_ref }`

**What it tests:** Multiple pipelines chained with `=>`, IO wired via numeric step indices. See [[concepts/pipelines/chains#Chain Execution]], [[io#Chain IO Addressing]].

```polyglot
[r] =Pipeline1=>=Pipeline2=>=Pipeline3
   [=] >0.inputParam1#path << $file
   [=] >0.inputParam2#string << "Hello"
   [=] <0.outputResult1#string >> <1.inputParam1
   [=] <0.outputResult2#string >> <1.inputParam2
   [=] <1.outputResult#string >> <2.inputParam1
   [=] <2.outputResult#string >> >output
```

### EC-10.6: Chain execution — leaf name references

**EBNF:** `step_ref ::= step_index | step_leaf_name`

**What it tests:** Using pipeline leaf name instead of numeric index for readability.

```polyglot
[r] =File.List=>=Data.Transform.Rows=>=Report.Format
   [=] >List.folder#path << $folder
   [=] <List.files >> <Rows.input
   [=] <Rows.output >> <Format.content
   [=] <Format.result >> >report
```

### EC-10.7: Chain execution — auto-wire (single IO pair, same type)

**EBNF:** Auto-wire semantic rule — omit `chain_io_line` when adjacent steps have exactly one output and one input of matching type.

**What it tests:** Only entry and exit IO declared; intermediate wiring is implicit.

```polyglot
[r] =File.Text.Read=>=Text.Transform=>=Text.Format
   [ ] Each step: one output#string -> one input#string — auto-wired
   [=] >0.path#path << $path
   [=] <2.formatted#string >> >formatted
```

### EC-10.8: Chain execution — error handling with step index

**EBNF:** `chain_error_block ::= "[!]" '!' step_ref fixed_sep error_name`

**What it tests:** Errors scoped to specific chain steps using `!N.ErrorName`.

```polyglot
[r] =File.Text.Read=>=Text.Parse.CSV
   [=] >0.path#path << $path
   [=] <1.rows#string >> >content
   [!] !0.File.NotFound
      [r] >content << "Error: file not found"
   [!] !0.File.ReadError
      [r] >content << "Error: could not read"
   [!] !1.Parse.InvalidFormat
      [r] >content << "Error: invalid CSV"
```

### EC-10.9: Chain execution — mixed numeric and leaf name references

**What it tests:** Numeric index and leaf name references used in the same chain, including in error blocks.

```polyglot
[r] =User.Fetch=>=User.Validate=>=User.Store
   [=] >0.id#int << $userId
   [=] <Fetch.profile >> <Validate.input
   [=] <1.validated >> <Store.record
   [=] <2.status >> >status
   [!] !0.No.Input
      [r] >status << "not found"
   [!] !Store.WriteError
      [r] >status << "save failed"
```

### EC-10.10: Chain execution — ambiguous leaf name (INVALID)

**What it tests:** Duplicate leaf names in a chain must use numeric indices.

```polyglot
[ ] INVALID — both steps have leaf name "Transform"
[ ] [r] =Text.Transform=>=Data.Transform
[ ]    [=] >Transform.input << $val   <- ambiguous, compile error

[ ] VALID — use numeric indices instead
[r] =Text.Transform=>=Data.Transform
   [=] >0.input << $val
   [=] <0.output >> <1.input
```

### EC-10.11: Chain execution — auto-wire type mismatch (INVALID)

**What it tests:** Auto-wire fails when types don't match between adjacent steps.

```polyglot
[ ] INVALID — step 0 outputs #string, step 1 expects #int
[ ] Auto-wire cannot infer: explicit [=] wiring required
[r] =ProduceString=>=ConsumeInt
   [=] >0.input << $data
   [=] <0.output#string >> <1.input#int
   [ ] <- compile error: type mismatch string vs int
```

### EC-10.12: Bare literal as execution expression (INVALID)

<!-- @EBNF:exec_expr -->
**EBNF:** `exec_expr` — `literal` is not a valid alternative.

**What it tests:** A standalone literal under `[r]`/`[p]`/`[b]` — no assignment, no call, no effect. See [[technical/compile-rules/PGE/PGE01020-effectless-execution-expression|PGE01020]].

```polyglot
[ ] INVALID — bare integer literal
[r] 42                                    [ ] ✗ PGE01020 — bare literal, no effect

[ ] INVALID — bare string literal
[r] "orphaned string"                     [ ] ✗ PGE01020 — bare literal, no effect
```

### EC-10.13: Non-pipeline identifier as execution expression (INVALID)

<!-- @EBNF:exec_expr -->
**EBNF:** `exec_expr` — `identifier` removed from alternatives.

**What it tests:** Data type, variable, or package identifiers used as execution expressions — they aren't pipeline calls and produce no effect. See [[technical/compile-rules/PGE/PGE01020-effectless-execution-expression|PGE01020]].

```polyglot
[ ] INVALID — data type identifier (not a pipeline call)
[r] #UserRecord                           [ ] ✗ PGE01020 — data type, not a call

[ ] INVALID — variable identifier (not an assignment)
[p] $existingVar                          [ ] ✗ PGE01020 — variable, not a call

[ ] INVALID — package alias (not a pipeline reference)
[b] @AD                                   [ ] ✗ PGE01020 — package alias, not a call
```

### EC-10.14: Orphan `[+]` continuation line

**EBNF ref:** `continuation_line ::= "[+]" expression`
**What it tests:** `[+]` at start of block with no preceding incomplete expression. PGE01026 fires.

```polyglot
[ ] ✗ PGE01026 — [+] with no preceding expression
{=} =Bad
   [T] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [+] "orphan continuation"
```

### EC-10.15: Self-chain — valid with numeric indexing

<!-- @pipelines:Chains -->
**EBNF ref:** `chain_call ::= pipeline_ref "=>" pipeline_ref { "=>" pipeline_ref }`
**What it tests:** `=A => =A` is valid (runs twice) but requires numeric step indexing. PGE08012 fires without it.

```polyglot
[ ] ✓ self-chain with numeric indexing
[r] =Process => =Process
   [=] >0.input << $data
   [=] <1.result >> >output
```

```polyglot
[ ] ✗ PGE08012 — self-chain without indexing
[r] =Process => =Process
   [=] >input << $data
   [=] <result >> >output
```

### EC-10.16: Empty foreign code block

**EBNF ref:** `foreign_code_block` — requires at least one `foreign_code_line`
**What it tests:** `[c]` header with no body lines. PGE01027 fires.

```polyglot
[ ] ✗ PGE01027 — empty foreign code block
[c] #Code:Python:3
```
