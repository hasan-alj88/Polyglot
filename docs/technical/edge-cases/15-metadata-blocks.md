---
audience: design
type: reference
updated: 2026-03-30
---

<!-- @edge-cases/INDEX -->

## 15. Metadata Blocks (S15)

### EC-15.1: `[%]` user-declared fields in pipeline

<!-- @u:blocks:Metadata -->
**EBNF:** `metadata_line ::= "[%]" fixed_sep name [ type_annotation ] [ "<<" value_expr ]`

**What it tests:** `[%]` lines appear before `[T]` (position 0). Fixed fields `.description`, `.version`, `.authors` assigned via `<<`. See [[blocks#Metadata]].

```polyglot
{-} -Invoice.Process
   [%] .description << "Processes incoming invoices and routes to accounting"
   [%] .version << "2.1.0"
   [%] .authors#array:string << {"alice@corp.com", "bob@corp.com"}
   [%] .license << "MIT"
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] $done#bool << #Boolean.True
```

### EC-15.2: `[%]` %alias field — multiple aliases per definition

**EBNF:** `metadata_alias ::= "%" "alias" NEWLINE { indent flex_sep string_literal NEWLINE }`

**What it tests:** `%alias` field makes a definition reachable via multiple shorthand names. Each alias is a `#NestedKeyString` — allows `.` and `:` for nested paths. All aliases must be globally unique (PGE12002).

```polyglot
{#} #SystemConfig
   [%] %alias
      [:] "Config"
      [:] "SysConfig"
   [.] .timeout#int <~ 30
   [.] .retries#int <~ 3

{-} -Provision.User
   [%] %alias
      [:] "ProvisionUser"
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] $x#int << 1
```

### EC-15.2b: `[%]` %alias with nested key path

**What it tests:** Alias values can contain `.` and `:` to reference nested paths in the definition tree. Useful for cross-tree aliases (e.g., error aliases reachable from multiple namespaces).

```polyglot
{!} !Permission
   [.] .File
      [.] .Denied#Error
         [%] %alias
            [:] "File.Permission.Denied"
            [:] "FileDenied"
```

### EC-15.3: `.info#serial` flexible metadata

**EBNF:** `info_field ::= "[%]" ".info#serial" NEWLINE { indent flex_data_field NEWLINE }`

**What it tests:** `.info#serial` opens a `:` flexible scope for arbitrary tooling metadata. See [[blocks#Metadata]].

```polyglot
{-} -Report.Generate
   [%] .description << "Generates monthly report"
   [%] .info#serial
      [:] :owner << "platform-team"
      [:] :ticket << "INFRA-42"
      [:] :runbook << "https://wiki/runbooks/report"
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] $x#int << 1
```

### EC-15.4: `live` metadata accessor — `%` on pipeline, variable, data

<!-- @u:types:Live Type Modifier -->
**EBNF:** `metadata_access ::= identifier '%' name`

**What it tests:** `%` accessor reads live runtime fields. Read-only — no assignment. See [[syntax/types/hierarchy#Live Type Modifier]], [[identifiers]].

```polyglot
[ ] Pipeline live fields
[?] -Invoice.Process%status
   [?] #AwaitTrigger
      [-] $ready#bool << #Boolean.True
   [?] #Running
      [-] $ready#bool << #Boolean.False
   [?] #Failed
      [b] -Audit.Log
         (-) <event << "pipeline_failed"
   [?] *?
      [-] $ready#bool << #Boolean.False

[ ] Variable lifecycle state
[?] $myVar%state
   [?] #Ready
      [-] $safe#bool << #Boolean.True
   [?] *?
      [-] $safe#bool << #Boolean.False

[ ] Data definition metadata (read-only counters)
[-] $uses#int << #Config%usageCount
```

### EC-15.5: Empty `%alias` declaration

**EBNF ref:** `metadata_alias` — requires at least one `flex_sep string_literal`
**What it tests:** `[%] %alias` with no `:` children. PGE12004 fires.

```polyglot
[ ] ✗ PGE12004 — empty alias declaration
{#} #MyType
   [%] %alias
   [.] .field#string
```
