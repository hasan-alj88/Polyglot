---
audience: design
type: reference
updated: 2026-04-23
---

<!-- @edge-cases/INDEX -->

## 10. Execution Statements (S10)

### EC-10.1: Pipeline call with IO and error

<!-- @u:pipelines:Error Handling -->
<!-- @u:io:Pipeline Call -->
**EBNF:** `pipeline_call ::= pipeline_ref NEWLINE { indent call_io_line NEWLINE } { indent error_block NEWLINE }`

**What it tests:** Full call structure: ref -> IO lines -> error blocks scoped under call. See [[concepts/pipelines/error-handling#Error Handling]], [[io#Pipeline Call]].

```aljam3
[-] @AD-Account.Create
   (-) <name << $fullName
   (-) <email << $email
   (-) >id >> $adId
   [!] !AD.CreateFailed
      [-] $adId << "FAILED"
   [!] !AD.Timeout
      [-] $adId << "TIMEOUT"
```

### EC-10.2: pglib pipeline call — no import needed

<!-- @u:EBNF:pipeline_ref -->
**EBNF:** `pipeline_ref ::= pipeline_id` — pglib uses `=` prefix like all pipelines.

**What it tests:** `-File.Text.Append` with `=` prefix (all identifiers have a prefix, no exceptions). No `[@]` import needed. See [[concepts/pipelines/INDEX|pipelines]], [[technical/ebnf/10-execution#10.2]].

```aljam3
[-] -File.Text.Append
   (-) <path << "/var/log/app.log"
   (-) <content << $message
   (-) >written >> $ok
   [!] !File.WriteError
```

### EC-10.3: Data load

**EBNF:** `data_load ::= "[#]" assignment_expr`

**What it tests:** `[#]` block element for loading serialized data into typed structures. In `{#}` definitions, file access is mediated through `{_}` permission objects. See [[blocks#Execution]], [[concepts/permissions/enforcement#Compile-Time File Binding]].

```aljam3
[ ] In execution: deserialize serial into typed data
[#] $hire#NewHire << $payload

[ ] In {#} definitions: permission-mediated file load
{_} _AppConfig
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/config/appsettings.json"
   [.] .path "/config/appsettings.json"
   [.] .format #JSON

{#} #Config
   (#) _AppConfig
   [#] #file1 << -Json.LoadFile
      (-) <source << _AppConfig
   [.] .dbConnection#string <~ #file1.db.connectionString
```

### EC-10.3b: Data load with permission template

**What it tests:** `[#]` file load using a `{_}` template with `(_)` inputs. Template is resolved at compile time; each instantiation produces its own content hash.

```aljam3
{_} _YAMLFile
   (_) <file#path
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "{<file}"
   [.] .path "{<file}"
   [.] .format #YAML

{#} #Secrets
   (#) _YAMLFile
      (_) <file << "/config/secrets.yaml"
   [#] #data << -Yaml.LoadFile
      (-) <source << _YAMLFile
   [.] .connectionString#string <~ #data.db.connectionString
   [.] .reportFolder#string <~ #data.report.folder
```

### EC-10.4: Parallel execution

<!-- @u:blocks:Execution -->
**What it tests:** `[=]` for parallel runs. See [[blocks#Execution]].

```aljam3
[=] @AD-Account.Create
   (-) <name << $name

[=] @Mail-Mailbox.Provision
   (-) <email << $email
```

### EC-10.5: Chain execution — explicit multi-IO wiring

<!-- @u:pipelines:Chain Execution -->
<!-- @u:io:Chain IO Addressing -->
**EBNF:** `chain_call ::= pipeline_ref "->" pipeline_ref { "->" pipeline_ref }`

**What it tests:** Multiple pipelines chained with `->`, IO wired via numeric step indices. See [[concepts/pipelines/chains#Chain Execution]], [[io#Chain IO Addressing]].

```aljam3
[-] -Pipeline1->-Pipeline2->-Pipeline3
   (-) >0.inputParam1#path << $file
   (-) >0.inputParam2#string << "Hello"
   (-) <0.outputResult1#string >> <1.inputParam1
   (-) <0.outputResult2#string >> <1.inputParam2
   (-) <1.outputResult#string >> <2.inputParam1
   (-) <2.outputResult#string >> >output
```

### EC-10.6: Chain execution — leaf name references

**EBNF:** `step_ref ::= step_index | step_leaf_name`

**What it tests:** Using pipeline leaf name instead of numeric index for readability.

```aljam3
[-] -File.List->-Data.Transform.Rows->-Report.Format
   (-) >List.folder#path << $folder
   (-) <List.files >> <Rows.input
   (-) <Rows.output >> <Format.content
   (-) <Format.result >> >report
```

### EC-10.7: Chain execution — auto-wire (single IO pair, same type)

**EBNF:** Auto-wire semantic rule — omit `chain_io_line` when adjacent steps have exactly one output and one input of matching type.

**What it tests:** Only entry and exit IO declared; intermediate wiring is implicit.

```aljam3
[-] -File.Text.Read->-Text.Transform->-Text.Format
   [ ] Each step: one output#string -> one input#string — auto-wired
   (-) >0.path#path << $path
   (-) <2.formatted#string >> >formatted
```

### EC-10.8: Chain execution — error handling with step index

**EBNF:** `chain_error_block ::= "[!]" '!' step_ref fixed_sep error_name`

**What it tests:** Errors scoped to specific chain steps using `!N.ErrorName`.

```aljam3
[-] -File.Text.Read->-Text.Parse.CSV
   (-) >0.path#path << $path
   (-) <1.rows#string >> >content
   [!] !0.File.NotFound
      [-] >content << "Error: file not found"
   [!] !0.File.ReadError
      [-] >content << "Error: could not read"
   [!] !1.Parse.InvalidFormat
      [-] >content << "Error: invalid CSV"
```

### EC-10.9: Chain execution — mixed numeric and leaf name references

**What it tests:** Numeric index and leaf name references used in the same chain, including in error blocks.

```aljam3
[-] -User.Fetch->-User.Validate->-User.Store
   (-) >0.id#int << $userId
   (-) <Fetch.profile >> <Validate.input
   (-) <1.validated >> <Store.record
   (-) <2.status >> >status
   [!] !0.No.Input
      [-] >status << "not found"
   [!] !Store.WriteError
      [-] >status << "save failed"
```

### EC-10.10: Chain execution — ambiguous leaf name (INVALID)

**What it tests:** Duplicate leaf names in a chain must use numeric indices.

```aljam3
[ ] INVALID — both steps have leaf name "Transform"
[ ] [-] -Text.Transform->-Data.Transform
[ ]    (-) >Transform.input << $val   <- ambiguous, compile error

[ ] VALID — use numeric indices instead
[-] -Text.Transform->-Data.Transform
   (-) >0.input << $val
   (-) <0.output >> <1.input
```

### EC-10.11: Chain execution — auto-wire type mismatch (INVALID)

**What it tests:** Auto-wire fails when types don't match between adjacent steps.

```aljam3
[ ] INVALID — step 0 outputs #string, step 1 expects #int
[ ] Auto-wire cannot infer: explicit (-) wiring required
[-] -ProduceString->-ConsumeInt
   (-) >0.input << $data
   (-) <0.output#string >> <1.input#int
   [ ] <- compile error: type mismatch string vs int
```

### EC-10.12: Bare literal as execution expression (INVALID)

<!-- @u:EBNF:exec_expr -->
**EBNF:** `exec_expr` — `literal` is not a valid alternative.

**What it tests:** A standalone literal under `[-]`/`[=]`/`[b]` — no assignment, no call, no effect. See [[technical/compile-rules/PGE/PGE01020-effectless-execution-expression|PGE01020]].

```aljam3
[ ] INVALID — bare integer literal
[-] 42                                    [ ] ✗ PGE01020 — bare literal, no effect

[ ] INVALID — bare string literal
[-] "orphaned string"                     [ ] ✗ PGE01020 — bare literal, no effect
```

### EC-10.13: Non-pipeline identifier as execution expression (INVALID)

<!-- @u:EBNF:exec_expr -->
**EBNF:** `exec_expr` — `identifier` removed from alternatives.

**What it tests:** Data type, variable, or package identifiers used as execution expressions — they aren't pipeline calls and produce no effect. See [[technical/compile-rules/PGE/PGE01020-effectless-execution-expression|PGE01020]].

```aljam3
[ ] INVALID — data type identifier (not a pipeline call)
[-] #UserRecord                           [ ] ✗ PGE01020 — data type, not a call

[ ] INVALID — variable identifier (not an assignment)
[=] $existingVar                          [ ] ✗ PGE01020 — variable, not a call

[ ] INVALID — package alias (not a pipeline reference)
[b] @AD                                   [ ] ✗ PGE01020 — package alias, not a call
```

### EC-10.14: Orphan `[~]` continuation line

**EBNF ref:** `continuation_line ::= "[~]" expression`
**What it tests:** `[~]` at start of block with no preceding incomplete expression. PGE01026 fires.

```aljam3
[ ] ✗ PGE01026 — [~] with no preceding expression
{-} -Bad
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3
   [~] "orphan continuation"
```

### EC-10.15: Self-chain — valid with numeric indexing

<!-- @u:pipelines:Chains -->
**EBNF ref:** `chain_call ::= pipeline_ref "->" pipeline_ref { "->" pipeline_ref }`
**What it tests:** `-A -> -A` is valid (runs twice) but requires numeric step indexing. PGE08012 fires without it.

```aljam3
[ ] ✓ self-chain with numeric indexing
[ ]
[-] -Process -> -Process
   (-) >0.input << $data
   (-) <1.result >> >output
```

```aljam3
[ ] ✗ PGE08012 — self-chain without indexing
[-] -Process -> -Process
   (-) >input << $data
   (-) <result >> >output
```

### EC-10.16: Empty foreign code block

**EBNF ref:** `foreign_code_block` — requires at least one `foreign_code_line`
**What it tests:** `[C]` block with no code lines. PGE01027 fires.

```aljam3
[ ] ✗ PGE01027 — empty foreign code block
[-] -RT.Python.Script
   (-) <env << $env
   (-) <script <<
   (-) >stdout >> $output
```

### EC-10.17: Wildcard auto-wire — valid bijective match

<!-- @u:syntax/io/auto-wire -->
**EBNF ref:** `call_io_line ::= ... | "(-)" wildcard_input "<<" wildcard_output`
**What it tests:** `<* << $Label>*` with a single unique-type pairing — the simplest successful wildcard auto-wire. PGW08001 fires (valid but explicit wiring preferred). See [[user/syntax/io/auto-wire]].

```aljam3
[ ] ✓ valid — one #string output bijectively maps to one #string input
[ ] ⚠ PGW08001 — auto-wire succeeded, explicit wiring preferred
[-] -File.Text.Read
   (-) $Read
   (-) <path#path << $path
   (-) >content#string

[-] -Text.Transform
   (-) <* << $Read>*
   (-) >formatted#string >> >output
```

### EC-10.18: Wildcard auto-wire — type mismatch (INVALID)

<!-- @u:syntax/io/auto-wire -->
**EBNF ref:** `wildcard_input`, `wildcard_output` — bijective type-identity required
**What it tests:** `<* << $A>*` where `$A` has an output with no corresponding input type on the target. PGE08001 fires. See [[technical/compile-rules/PGE/PGE08001-auto-wire-type-mismatch|PGE08001]].

```aljam3
[ ] ✗ PGE08001 — >total#int has no matching <…#int input
[-] -Count.Items
   (-) $A
   (-) <list#array:string << $items
   (-) >total#int

[-] -Format.Label
   (-) <* << $A>*
   (-) <text#string
   (-) >formatted#string >> >output
```

### EC-10.19: Wildcard auto-wire — ambiguous types (INVALID)

<!-- @u:syntax/io/auto-wire -->
**EBNF ref:** `wildcard_input`, `wildcard_output` — unique type-identity required per side
**What it tests:** Multiple outputs share a type-identity, making the bijection non-unique. PGE08002 fires. See [[technical/compile-rules/PGE/PGE08002-auto-wire-ambiguous-type|PGE08002]].

```aljam3
[ ] ✗ PGE08002 — two #string outputs compete for one #string input
[-] -Fetch.Both
   (-) $A
   (-) <url#string << $url
   (-) >name#string
   (-) >label#string

[-] -Process.Single
   (-) <* << $A>*
   (-) <text#string
   (-) >result#string >> >output
```

### EC-10.20: Wildcard auto-wire — port count mismatch (INVALID)

<!-- @u:syntax/io/auto-wire -->
**EBNF ref:** `wildcard_input`, `wildcard_output` — cardinality must match
**What it tests:** `$A` has more outputs than the target has inputs, so the bijection cannot be onto. PGE08003 fires. See [[technical/compile-rules/PGE/PGE08003-auto-wire-unmatched-parameter|PGE08003]].

```aljam3
[ ] ✗ PGE08003 — 3 outputs, 2 inputs
[-] -Fetch.Data
   (-) $A
   (-) <url#string << $url
   (-) >content#string
   (-) >count#int
   (-) >status#string

[-] -Transform.Text
   (-) <* << $A>*
   (-) <text#string
   (-) <flag#int
   (-) >result#string >> >output
```
