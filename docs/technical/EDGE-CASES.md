---
type: specification
audience: developer
updated: 2026-03-16
status: draft
---

# EBNF Edge Cases

<!-- @EBNF -->
Structured edge case catalog for validating the Polyglot Code grammar ([[EBNF]]). Each case maps to an EBNF production rule and tests a boundary condition, ambiguity, or interaction between rules.

## How to Use

Review in batches by section. Each edge case includes:
- **EBNF ref** — production rule being tested
- **What it tests** — the boundary or ambiguity
- **Cross-refs** — related spec files that govern the behavior
- **Example** — minimal Polyglot Code snippet

---

## 1. File Structure (§1)

### EC-1.1: Multiple definitions in one file

<!-- @EBNF:file -->
**EBNF:** `file ::= package_block { definition }` — the `{ }` repetition allows zero or more definitions after the package block.

**What it tests:** A single `.pg` file containing `{@}`, `{#}`, `{=}`, and `{ }` definitions together.

**Cross-refs:** [[packages]] (package block), [[blocks]] (definition elements)

```polyglot
{@} @Local:001.Multi:v1.0.0

{#} #Status
   [.] .Active

{=} =First
   [t] =T.Call
   [W] =W.Polyglot

{=} =Second
   [t] =T.Call
   [W] =W.Polyglot
```

### EC-1.2: File with only package block (no definitions)

**EBNF:** `file ::= package_block { definition }` — zero definitions is valid.

**What it tests:** Minimal valid `.pg` file.

```polyglot
{@} @Local:001.Empty:v1.0.0
```

---

## 2. Lexical Elements (§2)

### EC-2.1: Indentation depth — deeply nested scopes

<!-- @line-structure -->
**EBNF:** `indent ::= { "   " }` — unlimited nesting via 3-space repetition.

**What it tests:** 4+ levels of indentation (package → pipeline → expand → conditional → error). See [[line-structure]].

```polyglot
{=} =Deep
   [t] =T.Call
   [W] =W.Polyglot

   [r] ~ForEach.Array
   [~] <Array << $items
   [~] >item >> $item

      [?] $item >? 0
         [r] =SomeCall
         [=] <val << $item
         [=] >out >> $result
            [!] !Some.Error
               [r] $result << -1
```

### EC-2.2: Bool literals

<!-- @types -->
**EBNF:** `bool_literal ::= "#Boolean.True" | "#Boolean.False"` — booleans are data references, not keywords.

**What it tests:** Bool values are `#Boolean.True` / `#Boolean.False` (not `true`/`false`). See [[types]].

```polyglot
[r] $flag;bool << #Boolean.True
[=] >enabled;bool ~> #Boolean.False
```

### EC-2.3: Negative numeric literals

**EBNF:** `int_literal ::= [ '-' ] digit { digit }` and `float_literal` — optional leading minus.

**What it tests:** Negative integers and floats as literal values.

```polyglot
[r] $offset;int << -1
[r] $threshold;float << -0.5
```

### EC-2.4: Empty string literal

**EBNF:** `string_literal ::= '"' { any_char - '"' } '"'` — zero characters between quotes is valid.

**What it tests:** `""` as a valid string literal.

```polyglot
[.] .name;string <~ ""
```

---

## 3. Identifiers (§3)

### EC-3.1: Package address — all components present

<!-- @packages -->
<!-- @identifiers -->
**EBNF:** `package_address ::= registry_type flex_sep registry_id fixed_sep package_name { fixed_sep sub_package } [ flex_sep version ]`

**What it tests:** Full address with subpackage and 4-segment version. See [[packages]], [[identifiers]].

```polyglot
{@} @Local:999.MyPackage.Sub:v1.2.3.2
```

### EC-3.2: Community registry type

**EBNF:** `registry_type ::= "Local" | "Community"`

**What it tests:** `Community` registry with username-style ID.

```polyglot
[@] @Slack << @Community:tools.SlackAdmin:v1.3.0
```

### EC-3.3: Package address — minimal (no subpackage, no version)

**What it tests:** Whether version and subpackage are truly optional.

```polyglot
{@} @Local:001.Minimal
```

### EC-3.4: Cross-package enum reference

<!-- @types:Enum Fields -->
**EBNF:** `cross_pkg_enum ::= '@' name '#' dotted_name`

**What it tests:** Referencing an enum value from an imported package: `@alias#DataName.EnumField`. See [[types#Enum Fields vs Value Fields]].

```polyglot
[?] $status =? @HR#EmployeeStatus.Active
```

### EC-3.5: Cross-package pipeline reference

**EBNF:** `cross_pkg_pipeline ::= '@' name pipeline_id`

**What it tests:** Calling an imported pipeline. See [[packages#Usage]].

```polyglot
[r] @Mail=Mailbox.Provision
[=] <email << $email
```

### EC-3.6: Flexible-field variable paths

<!-- @identifiers:Serialized Identifiers -->
**EBNF:** `field_path ::= name { field_separator name }` with `flex_sep ::= ':'`

**What it tests:** Variables with `:` flexible field separators. See [[identifiers#Serialized Identifiers]].

```polyglot
[r] $config:timeout:value;int << 30
[r] $user:name;string << "Alice"
```

### EC-3.7: Sibling homogeneity violation (INVALID)

<!-- @identifiers:Serialization Rules -->
**EBNF:** Semantic rule — all siblings must use same separator. See [[identifiers#Serialization Rules]].

**What it tests:** Mixing `.` and `:` at same level is rejected.

```polyglot
[ ] INVALID — mixed separators at same sibling level
[r] $point.x << 10
[r] $point:y << 20
```

---

## 4. Type System (§4)

### EC-4.1: Element-typed array — basic type

<!-- @types:Element-Typed Arrays -->
**EBNF:** `array_type ::= "array" [ fixed_sep element_type ]`

**What it tests:** `array.string`, `array.int`, `array.path` — dot separates array from element type. See [[types#Element-Typed Arrays]].

```polyglot
[=] <names;array.string
[=] <scores;array.int
[=] <files;array.path
```

### EC-4.2: Element-typed array — user-defined type (no # prefix)

**EBNF:** `element_type ::= basic_type | name` — user types drop `#` inside `array.`

**What it tests:** `array.UserRecord` not `array.#UserRecord`. See [[types#User-Defined Types]].

```polyglot
[=] <users;array.UserRecord
```

### EC-4.3: Serial type

**EBNF:** `serial_type ::= "serial"`

**What it tests:** `serial` as a type annotation on IO parameters. See [[types#Basic Types]], [[collections#Collection Types]].

```polyglot
[=] <payload;serial
```

### EC-4.4: User-defined type reference

**EBNF:** `user_type ::= '#' dotted_name`

**What it tests:** `#DataName` as type annotation. See [[types#User-Defined Types]].

```polyglot
[r] $hire;#NewHire << <payload
```

### EC-4.5: `=Path"..."` inline path creation

**What it tests:** `=Path"..."` inline pipeline call creating `;path` values. See [[types#=Path Inline Notation]], [[STDLIB#=Path]].

```polyglot
[ ] Basic usage
[r] $dir;path << =Path"/tmp/MyApp"

[ ] With {.} shorthand
[r] $logDir;path << =Path"{.}/logs"

[ ] Separator equivalence — both resolve identically
[r] $a;path << =Path"{.}\MyApp\logs"
[r] $b;path << =Path"{.}/MyApp/logs"

[ ] Interpolation with user-defined path variable
[r] $root;path
   [.] .Unix << "/opt"
   [.] .Windows << "D:"
[r] $appDir;path << =Path"{$root}/MyApp"

[ ] Literal braces in path string
[r] $weird;path << =Path"/tmp/{{backup}}/files"
```

### EC-4.6: Single-platform path (PGW-408 / PGE-408)

**What it tests:** Warning when only one OS subfield assigned; error when current OS subfield missing. See [[types#Explicit Subfield Assignment]].

```polyglot
[ ] ⚠ PGW-408 — single platform, but matches current OS (Unix)
[r] $dir;path
   [.] .Unix << "/tmp/MyApp"

[ ] ✗ PGE-408 — .Unix missing, compiling on Unix
[r] $dir;path
   [.] .Windows << "C:\MyApp"

[ ] ✓ suppressed warning
[ ] Ignore PGW-408
[r] $dir;path
   [.] .Unix << "/tmp/MyApp"

[ ] ✓ no warning — both platforms
[r] $dir;path
   [.] .Unix << "/tmp/MyApp"
   [.] .Windows << "C:\MyApp"
```

### EC-4.7: Plain string to `;path` type mismatch (PGE-401)

**What it tests:** Assigning a plain string to a `;path` variable is a type mismatch. See [[types#Explicit Subfield Assignment]].

```polyglot
[ ] ✗ PGE-401 — string ≠ path, no implicit coercion
[r] $dir;path << "/tmp/MyApp"

[ ] ✓ correct — use =Path"..." instead
[r] $dir;path << =Path"/tmp/MyApp"
```

### EC-4.8: Inline pipeline call — single output

**EBNF:** `inline_pipeline_call ::= pipeline_ref string_literal`

**What it tests:** An inline pipeline call with one output evaluates to that output's type directly.

```polyglot
[ ] ✓ =Path has one output >result;path — value is ;path
[r] $dir;path << =Path"/tmp/MyApp"

[ ] ✓ inline call as comparison operand
[?] $dir =? =Path"/expected"
```

### EC-4.9: Inline pipeline call — multiple outputs

**What it tests:** An inline pipeline call with multiple outputs evaluates to `;serial` with output parameter names as keys.

```polyglot
{=} =ParsePair
   [=] <InlineStringLiteral;string <~ ""
   [=] >key;string
   [=] >value;string
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [ ] ... parsing logic ...

[ ] ✓ multiple outputs → ;serial with keys "key" and "value"
[r] $result;serial << =ParsePair"name=Alice"
```

### EC-4.10: Inline pipeline call — type mismatch

**What it tests:** Target type must match the inline pipeline's output type.

```polyglot
[ ] ✗ PGE-401 — =Path returns ;path, not ;string
[r] $name;string << =Path"/tmp"

[ ] ✓ matching types
[r] $dir;path << =Path"/tmp"
```

### EC-4.11: Inline pipeline call — user-defined pipeline

**What it tests:** User-defined pipelines can accept inline calls by declaring `<InlineStringLiteral;string`.

```polyglot
{=} =Greeting
   [=] <InlineStringLiteral;string <~ ""
   [=] >message;string
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [?] $InlineStringLiteral =!? ""
      [r] >message << "Hello {$InlineStringLiteral}"
   [?] *?
      [r] >message << "Hello World"

[ ] ✓ inline call
[r] $msg;string << =Greeting"Alice"

[ ] ✓ normal call — $InlineStringLiteral is "" (default)
[r] =Greeting
   [=] >message >> $msg
```

### EC-4.12: Pipeline without `<InlineStringLiteral;string` called inline

**What it tests:** Calling a pipeline inline when it has not declared the reserved parameter.

```polyglot
{=} =NormalPipeline
   [=] <input;string
   [=] >output;string
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] >output << $input

[ ] ✗ compile error — =NormalPipeline has no <InlineStringLiteral;string
[r] $result;string << =NormalPipeline"test"
```

### EC-4.13: Typed flexible wildcard — basic inference

**EBNF:** `typed_flex_wildcard ::= "[:]" flex_sep "*" type_annotation`

**What it tests:** New `:key` at a typed flexible level inherits the wildcard type. No explicit annotation needed. See [[types#Typed Flexible Fields]].

```polyglot
{#} #Handler
   [.] .endpoint;string
   [.] .method;string

{#} #Registry
   [.] .plugins
      [:] :*;Handler

[ ] ✓ compiler infers :myPlugin is ;Handler
[r] $reg.plugins:myPlugin.endpoint << "/api/data"
[r] $reg.plugins:myPlugin.method << "GET"
```

### EC-4.14: Typed flexible wildcard — contradicting annotation (PGE-401)

**What it tests:** Explicit type annotation that contradicts the wildcard type is a compile error.

```polyglot
{#} #Registry
   [.] .plugins
      [:] :*;Handler

[ ] ✗ PGE-401 — :myPlugin is ;Handler (from wildcard), not ;string
[r] $reg.plugins:myPlugin;string << "not a handler"
```

### EC-4.15: Typed flexible wildcard — multi-level resolution

**What it tests:** Nested typed flexible levels — compiler resolves one level at a time. See [[types#Constraints]].

```polyglot
{#} #Setting
   [.] .value;string
   [.] .default;string

{#} #Section
   [:] :*;Setting

{#} #Config
   [.] .sections
      [:] :*;Section

[ ] ✓ :auth → ;Section, :timeout → ;Setting, .value → ;string
[r] $cfg.sections:auth:timeout.value << "30s"
[r] $cfg.sections:auth:timeout.default << "60s"
```

### EC-4.16: Typed flexible wildcard — untyped level (no wildcard)

**What it tests:** Flexible level without `[:] :*;Type` is untyped — treated as `;serial`.

```polyglot
{#} #OpenConfig
   [.] .data
      [:] :key1;string
      [:] :key2;int

[ ] ✓ individually declared flex fields — matched by name
[r] $cfg.data:key1 << "hello"
[r] $cfg.data:key2 << 42

[ ] ✗ :unknown has no wildcard, no individual declaration — treated as ;serial
[r] $cfg.data:unknown << "anything"
```

### EC-4.17: Typed flexible wildcard — individual override before wildcard fallback

**What it tests:** Named flex field matches first; wildcard is fallback.

```polyglot
{#} #MixedRegistry
   [.] .entries
      [:] :default;SpecialHandler
      [:] :*;Handler

[ ] ✓ :default matches the named declaration → ;SpecialHandler
[r] $reg.entries:default.specialField << "value"

[ ] ✓ :other falls back to wildcard → ;Handler
[r] $reg.entries:other.endpoint << "/api"
```

### EC-4.18: Multidimensional array — `:ND` dimension specifier

**EBNF:** `array_type ::= "array" [ fixed_sep element_type ] [ flex_sep dimension ]` with `dimension ::= digit { digit } "D"`

**What it tests:** Arrays support a dimension specifier via flexible field notation. Omitting `:<N>D` defaults to 1D. Access depth must match declared dimension count.

```polyglot
[ ] ✓ 1D array — default (no :ND specified)
[=] <items;array.string
[r] $first << $items.0

[ ] ✓ 2D matrix — :2D dimension specifier
[=] <matrix;array.float:2D
[r] $val << $matrix.0.1

[ ] ✓ 3D cube — :3D dimension specifier
[=] <cube;array.int:3D
[r] $val << $cube.2.3.0

[ ] ✓ 4D with user-defined element type
[=] <hyper;array.UserRecord:4D
[r] $cell << $hyper.0.1.2.3

[ ] ✗ PGE-417 — too many indices for :2D
[=] <matrix;array.float:2D
[ ] [r] $val << $matrix.0.1.2                 ← 3 indices on :2D

[ ] ✗ PGE-417 — too few indices for :3D
[=] <cube;array.int:3D
[ ] [r] $val << $cube.2                        ← 1 index on :3D

[ ] ✗ PGE-417 — :0D is not valid
[ ] [=] <nothing;array.float:0D                ← dimension must be positive

[ ] ✗ PGE-412 — nested array still banned
[ ] [=] >matrix;array.array.float              ← use ;array.float:2D instead
```

---

## 5. Block Elements (§5)

### EC-5.1: All block element categories used

<!-- @blocks -->
**What it tests:** A file exercising every block element category. See [[blocks]].

| Category | Elements | Where tested |
|----------|----------|-------------|
| Registry | `[@]` | Package imports |
| Data Flow | `[=]` `[~]` `[*]` | IO lines, expand IO, collect invocation |
| Execution | `[r]` `[p]` `[b]` `[s]` | Run, parallel, background, serial load |
| Control Flow | `[?]` `[!]` `[t]` `[Q]` `[W]` | Conditionals, errors, trigger, queue, wrapper |
| Data Access | `[.]` `[:]` | Data definitions |
| Logical | `[&]` `[+]` `[-]` `[^]` | Conditional compound logic |
| Comment | `[ ]` | Inline comments |

### EC-5.2: Background execution

<!-- @blocks:Execution -->
**EBNF:** `background_line ::= "[b]" exec_expr`

**What it tests:** Fire-and-forget execution. See [[blocks#Execution]].

```polyglot
[b] =Logging.SendMetric
   [=] <event << "user_created"
```

---

## 6. Operators (§6)

### EC-6.1: All four assignment operators

<!-- @operators -->
<!-- @variable-lifecycle -->
**What it tests:** Each operator used in its correct context. See [[operators]], [[variable-lifecycle]].

```polyglot
[.] .name;string <~ "default"
[=] >count;int ~> 0
[r] $x;int << 42
[=] >item >> $result
```

### EC-6.2: All comparison operators

**EBNF:** `comparison_op ::= "=?" | ">?" | "<?" | ">=?" | "<=?" | "=!?"`

**What it tests:** Each comparison in a conditional.

```polyglot
[?] $a =? 0
[?] $b >? 10
[?] $c <? 5
[?] $d >=? 100
[?] $e <=? -1
[?] $f =!? ""
```

### EC-6.3: Range operators

**EBNF:** `range_expr ::= value_expr range_open value_expr ',' value_expr range_close`

**What it tests:** All four range combinations — mixing `[` (inclusive) and `(` (exclusive) on each bound. Mathematical interval notation.

```polyglot
[ ] Inclusive-inclusive: 1 ≤ val ≤ 10
[?] $val ?[1,10]
[ ] Exclusive-exclusive: 0 < val < 100
[?] $val ?(0,100)
[ ] Inclusive-exclusive: 1 ≤ val < 10
[?] $val ?[1,10)
[ ] Exclusive-inclusive: 0 < val ≤ 10
[?] $val ?(0,10]
```

### EC-6.4: Arithmetic in assignment

**EBNF:** `arithmetic_expr ::= value_expr arithmetic_op value_expr`

**What it tests:** `+`, `-`, `*`, `/` used in assignments.

```polyglot
[r] $total;int << $price * $quantity
[r] $name;string << "{$first} {$last}"
[r] $avg;float << $sum / $count
[r] $diff;int << $a - $b
```

---

## 7. IO Parameters (§7)

### EC-7.1: IO with field separators

<!-- @io -->
**EBNF:** `input_param ::= '<' name { field_separator name }` — IO params can have sub-fields.

**What it tests:** Dot-navigated IO parameters. See [[io]].

```polyglot
[=] <config.timeout;int << 30
[=] >result.status;string >> $status
```

---

## 8. Expressions (§8)

### EC-8.1: Inline data — multiple elements

<!-- @types:Inline Data Shorthand -->
**EBNF:** `inline_data ::= '{' value_expr { ',' value_expr } '}'`

**What it tests:** Non-empty inline data with mixed types. See [[types#Inline Data Shorthand]].

```polyglot
[r] $nums;array << {1, 2, 3, 4, 5}
[r] $services;array.string << {"AD", "Email", "Slack"}
```

### EC-8.2: Inline data — empty collection

**EBNF:** `inline_data ::= '{' '}'`

**What it tests:** Empty `{}` as valid collection initializer.

```polyglot
[=] >results;array.string ~> {}
```

### EC-8.3: String interpolation

<!-- @types:String Interpolation -->
**EBNF:** `interpolation ::= '{' variable_id '}'`

**What it tests:** Variable interpolation inside string literals using `{$var}` syntax. See [[types#String Interpolation]].

```polyglot
[r] $msg;string << "Hello {$first} {$last}!"
[r] $path;string << "/users/{$userId}/profile"
[ ] Escaped literal braces
[r] $json;string << "{{\"key\": \"{$val}\"}}"
```

---

## 9. Definition Blocks (§9)

### EC-9.1: Package with multiple imports

<!-- @packages -->
**EBNF:** `package_block ::= "{@}" package_id NEWLINE { indent import_line NEWLINE }`

**What it tests:** Multiple `[@]` imports in one package block. See [[packages]].

```polyglot
{@} @Local:001.App:v1.0.0
   [@] @AD << @Local:001.ActiveDirectory:v2.0.0
   [@] @Mail << @Local:001.EmailSystem:v1.2.0
   [@] @HR << @Local:001.HRSystem:v2.1.0
```

### EC-9.2: Enum fields — pure enum (no value sub-fields)

<!-- @types:Enum Fields -->
**EBNF:** `enum_field ::= "[.]" fixed_sep name`

**What it tests:** All-enum siblings, no `;type`, no assignment. See [[types#Enum Fields vs Value Fields]].

```polyglot
{#} #Direction
   [.] .North
   [.] .South
   [.] .East
   [.] .West
```

### EC-9.3: Enum field with nested value sub-fields

**EBNF:** `enum_field ::= "[.]" fixed_sep name NEWLINE { indent data_field NEWLINE }`

**What it tests:** Enum variant carrying typed data underneath.

```polyglot
{#} #Status
   [.] .Failed
      [.] .reason;string <~ "unknown"
      [.] .retries;int <~ 0
   [.] .Success
```

### EC-9.4: Value field data definition — all siblings assigned

<!-- @identifiers:Serialization Rules -->
**EBNF:** `value_field ::= "[.]" fixed_sep name type_annotation [ assignment_op value_expr ]`

**What it tests:** All-or-none assignment rule — all siblings have defaults. See [[identifiers#Serialization Rules]].

```polyglot
{#} #Config
   [.] .timeout;int <~ 30
   [.] .retries;int <~ 3
   [.] .verbose;bool <~ #Boolean.False
```

### EC-9.5: Flexible-field data definition

**EBNF:** `flex_data_field ::= "[:]" flex_sep name type_annotation [ assignment_op value_expr ]`

**What it tests:** `[:]` with `:` separator for open-schema data.

```polyglot
{#} #Metadata
   [:] :author;string <~ ""
   [:] :version;string <~ "0.0.0"
```

### EC-9.6: Pipeline — mandatory structure ordering

<!-- @pipelines -->
**EBNF:** `pipeline_body ::= trigger_section [ io_section ] [ queue_section ] wrapper_section execution_section`

**What it tests:** Correct order: trigger → IO → queue → wrapper → execution. See [[pipelines]].

```polyglot
{=} =Ordered
   [t] =T.Call
   [=] <input;string
   [=] >output;string ~> ""
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] >output << <input
```

### EC-9.7: Pipeline — minimal (no IO, no queue)

**What it tests:** IO and Queue are optional per EBNF.

```polyglot
{=} =Minimal
   [t] =T.Call
   [W] =W.Polyglot
   [r] $x;int << 1
```

### EC-9.8: Trigger with string argument

**EBNF:** `trigger_ref ::= pipeline_ref [ string_literal ]`

**What it tests:** Triggers that take configuration strings.

```polyglot
[t] =T.Daily"3AM"
[t] =T.Webhook"/api/onboarding"
[t] =T.Folder.NewFiles"/inbox/"
```

### EC-9.9: IO as implicit triggers — all three modes

<!-- @pipelines:IO as Implicit Triggers -->
**What it tests:** Constant, default, and required IO. See [[pipelines#IO as Implicit Triggers]].

```polyglot
[=] <constant;string << "locked"
[=] <fallback;string <~ "default"
[=] <required;string
```

### EC-9.10: `[p]` in `[\]` — parallel fork outlives setup, collected in `[/]`

<!-- @pipelines:Parallel Forking in Setup -->
**EBNF:** `scope_setup ::= "[\]" NEWLINE { indent exec_line NEWLINE }` — `exec_line` includes `parallel_line`.

**What it tests:** `[p]` at end of `[\]` with no `[*] *All` — forked path runs concurrently with body; `[/]` collects via `[*] *All` with `[*] <<` wait inputs. See [[pipelines#Parallel Forking in Setup]].

```polyglot
{M} =W.Tracing
   [{] $traceId;string
   [}] $duration;string
   [\]
      [r] =Tracer.Open
         [=] <id << $traceId
         [=] >session >> $session
      [ ] No *All after [p] — timer runs concurrently with body
      [p] =Tracer.StartTimer
         [=] <session << $session
         [=] >handle >> $timerHandle
   [/]
      [*] *All
         [*] << $timerHandle
      [r] =Tracer.StopTimer
         [=] <handle << $timerHandle
         [=] >elapsed >> $duration
      [r] =Tracer.Close
         [=] <session << $session
```

### EC-9.11: `[b]` in `[\]` — fire-and-forget, no collection in `[/]`

**EBNF:** `scope_setup ::= "[\]" NEWLINE { indent exec_line NEWLINE }` — `exec_line` includes `background_line`.

**What it tests:** `[b]` in setup fires and is never collected — no `[*] *All` in `[/]` for it.

```polyglot
{M} =W.AuditLog
   [{] $userId;string
   [\]
      [r] =Session.Open
         [=] <id << $userId
         [=] >session >> $session
      [ ] Fire audit event — no result needed, no collection
      [b] =Audit.LogEntry
         [=] <userId << $userId
   [/]
      [r] =Session.Close
         [=] <session << $session
```

---

## 10. Execution Statements (§10)

### EC-10.1: Pipeline call with IO and error

<!-- @pipelines:Error Handling -->
<!-- @io:Pipeline Call -->
**EBNF:** `pipeline_call ::= pipeline_ref NEWLINE { indent call_io_line NEWLINE } { indent error_block NEWLINE }`

**What it tests:** Full call structure: ref → IO lines → error blocks scoped under call. See [[pipelines#Error Handling]], [[io#Pipeline Call]].

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

**What it tests:** `=File.Text.Append` with `=` prefix (all identifiers have a prefix, no exceptions). No `[@]` import needed. See [[pipelines]], [[EBNF#10.2]].

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
[#] $hire;NewHire << $payload

[ ] In {#} definitions: load external config files
{#} #Config
   [#] #file1 << =Json.LoadFile"/config/appsettings.json"
   [.] .dbConnection;string <~ #file1.db.connectionString
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

**What it tests:** Multiple pipelines chained with `=>`, IO wired via numeric step indices. See [[pipelines#Chain Execution]], [[io#Chain IO Addressing]].

```polyglot
[r] =Pipeline1=>=Pipeline2=>=Pipeline3
   [=] >0.inputParam1;path << $file
   [=] >0.inputParam2;string << "Hello"
   [=] <0.outputResult1;string >> <1.inputParam1
   [=] <0.outputResult2;string >> <1.inputParam2
   [=] <1.outputResult;string >> <2.inputParam1
   [=] <2.outputResult;string >> >output
```

### EC-10.6: Chain execution — leaf name references

**EBNF:** `step_ref ::= step_index | step_leaf_name`

**What it tests:** Using pipeline leaf name instead of numeric index for readability.

```polyglot
[r] =File.List=>=Data.Transform.Rows=>=Report.Format
   [=] >List.folder;path << $folder
   [=] <List.files >> <Rows.input
   [=] <Rows.output >> <Format.content
   [=] <Format.result >> >report
```

### EC-10.7: Chain execution — auto-wire (single IO pair, same type)

**EBNF:** Auto-wire semantic rule — omit `chain_io_line` when adjacent steps have exactly one output and one input of matching type.

**What it tests:** Only entry and exit IO declared; intermediate wiring is implicit.

```polyglot
[r] =File.Text.Read=>=Text.Transform=>=Text.Format
   [ ] Each step: one output;string → one input;string — auto-wired
   [=] >0.path;path << $path
   [=] <2.formatted;string >> >formatted
```

### EC-10.8: Chain execution — error handling with step index

**EBNF:** `chain_error_block ::= "[!]" '!' step_ref fixed_sep error_name`

**What it tests:** Errors scoped to specific chain steps using `!N.ErrorName`.

```polyglot
[r] =File.Text.Read=>=Text.Parse.CSV
   [=] >0.path;path << $path
   [=] <1.rows;string >> >content
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
   [=] >0.id;int << $userId
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
[ ]    [=] >Transform.input << $val   ← ambiguous, compile error

[ ] VALID — use numeric indices instead
[r] =Text.Transform=>=Data.Transform
   [=] >0.input << $val
   [=] <0.output >> <1.input
```

### EC-10.11: Chain execution — auto-wire type mismatch (INVALID)

**What it tests:** Auto-wire fails when types don't match between adjacent steps.

```polyglot
[ ] INVALID — step 0 outputs ;string, step 1 expects ;int
[ ] Auto-wire cannot infer: explicit [=] wiring required
[r] =ProduceString=>=ConsumeInt
   [=] >0.input << $data
   [=] <0.output;string >> <1.input;int
   [ ] ← compile error: type mismatch string vs int
```

---

## 11. Control Flow (§11)

### EC-11.1: Conditional chain — multiple [?] branches

<!-- @blocks:Control Flow -->
**EBNF:** `conditional_line ::= "[?]" comparison_expr`

**What it tests:** Sequential `[?]` blocks acting as switch-case. See [[blocks#Control Flow]].

```polyglot
{#} #Status
   [.] .Ok
   [.] .Warn
   [.] .Fail

[ ] ...in pipeline execution...
[?] $status =? #Status.Ok
   [r] >result << "Success"

[?] $status =? #Status.Warn
   [r] >result << "Warning"

[?] $status =? #Status.Fail
   [r] >result << "Failure"
```

### EC-11.2: Error block — scoped under [r], not pipeline-level

**What it tests:** `[!]` indentation must be under the specific `[r]` call, after its `[=]` IO lines. See [[pipelines#Error Handling]].

```polyglot
[ ] CORRECT — error scoped under call
[r] =SomeCall
[=] <in << $val
[=] >out >> $result
   [!] !Some.Error
      [r] $result << "fallback"

[ ] WRONG — error at pipeline level (NOT valid)
```

### EC-11.3: Logical operators in conditionals

**EBNF:** `logical_and ::= "[&]" comparison_expr` etc.

**What it tests:** Compound conditions using `[&]`, `[+]`, `[-]`, `[^]`.

```polyglot
[ ] AND: both conditions must be true
[?] $age >=? 18
[&] $verified =? #Boolean.True
   [r] $access << #AccessLevel.Granted

[ ] OR: either condition
[?] $role =? #Role.Admin
[+] $role =? #Role.Superuser
   [r] $elevated << #Boolean.True

[ ] Negation: insert ! before ? in comparison operator
[ ] <!? means "not less than", >=!? means "not greater-or-equal"
[?] $banned =? #Boolean.False
[&] $age <!? 13
   [r] $allowed << #Boolean.True
```

---

## 12. Collection Operations (§12)

### EC-12.1: Expand with sequential vs parallel

<!-- @collections:Expand Operators -->
**EBNF:** `expand_line ::= ( "[r]" | "[p]" ) expand_invocation`

**What it tests:** `[r]` = sequential mini-pipelines, `[p]` = parallel. See [[collections#Expand Operators]].

```polyglot
[ ] Sequential — order matters
[r] ~ForEach.Array
   [~] <Array << $orderedItems
   [~] >item >> $item

[ ] Parallel — order doesn't matter
[p] ~ForEach.Array
   [~] <Array << $independentItems
   [~] >item >> $item
```

### EC-12.2: ForEach.Array.Enumerate — index + item

**EBNF:** `expand_operator ::= "ForEach.Array.Enumerate"`

**What it tests:** Enumerate provides both `>index` and `>item`.

```polyglot
[r] ~ForEach.Array.Enumerate
   [~] <Array << $items
   [~] >index >> $idx
   [~] >item >> $val
```

### EC-12.3: ForEach.Serial — key/item pairs

**EBNF:** `expand_operator ::= "ForEach.Serial"`

**What it tests:** Serial iteration with `>key` and `>item`.

```polyglot
[r] ~ForEach.Serial
   [~] <Serial << $config
   [~] >key >> $k
   [~] >item >> $v
```

### EC-12.4: ForEach.Level — tilde suffix marks iteration point

**EBNF:** Special input syntax `<level << #SomeData.SubField.~`

**What it tests:** The `~` suffix on the input path. See [[collections#ForEach.Level]].

```polyglot
[r] ~ForEach.Level
   [~] <level << #UserData.Preferences.~
   [~] >key >> $prefKey
   [~] >item >> $prefValue
```

### EC-12.5: Collector invocation with execution marker + [*] IO

<!-- @io:Collection Operators -->
**EBNF:** `collect_line ::= ( "[r]" | "[p]" ) collect_invocation NEWLINE { indent collect_io_line NEWLINE }` where `collect_io_line ::= "[*]" ...`

**What it tests:** `[r]`/`[p]` execution marker for invocation, `[*]` for IO — consistent with expand (`[r]`/`[p]` + `[~]`). See [[io#Collection Operators]].

```polyglot
[r] *Into.Array
   [*] <item << $value
   [*] >Array >> $collected
```

### EC-12.6: Direct output port write from collector

**EBNF:** `assign_target ::= output_param` — collector output writes to `>pipelineOutput`.

**What it tests:** `>> >pipelineOutput` syntax. See [[io#Direct Output Port Writing]].

```polyglot
[r] *Agg.Count
   [*] <item << $service
   [*] >count >> >successCount
[ ] Target >successCount is now Final — no other push allowed
```

### EC-12.7: Multiple collectors in same expand scope

**What it tests:** Two `*` collectors operating within one `~ForEach` body.

```polyglot
[p] ~ForEach.Array
   [~] <Array << $items
   [~] >item >> $item

   [r] *Into.Array
      [*] <item << $item
      [*] >Array >> >results

   [r] *Agg.Sum
      [*] <number << $item.value
      [*] >sum >> >total
```

### EC-12.8: All *Agg operators

**What it tests:** Every aggregate collector variant.

```polyglot
[r] *Agg.Sum
   [*] <number << $n
   [*] >sum >> $s

[r] *Agg.Count
   [*] <item << $x
   [*] >count >> $c

[r] *Agg.Average
   [*] <number << $n
   [*] >average >> $avg

[r] *Agg.Max
   [*] <number << $n
   [*] >max >> $mx

[r] *Agg.Min
   [*] <number << $n
   [*] >min >> $mn

[r] *Agg.Concatenate
   [*] <string << $s
   [*] >result >> $concat
```

### EC-12.9: `*All` — sync barrier with `[*] <<` only

**EBNF:** `sync_operator ::= "All"` ; `wait_input ::= "[*]" "<<" variable_ref`

**What it tests:** `*All` with `[*] <<`-only lines outside expand scope. Variables remain accessible after.

```polyglot
[p] =Fetch.A
   [=] <id << $id
   [=] >resultA >> $resultA

[p] =Fetch.B
   [=] <id << $id
   [=] >resultB >> $resultB

[*] *All
   [*] << $resultA
   [*] << $resultB

[ ] $resultA and $resultB are accessible here
[r] =Process
   [=] <a << $resultA
   [=] <b << $resultB
```

### EC-12.10: `*First` — race collector with `[*] <<` inputs and `[*] >>` output

**EBNF:** `race_operator ::= "First"` ; `collect_output ::= "[*]" ">>" variable_ref`

**What it tests:** `*First` cancels losing `[*] <<` inputs; only `[*] >>` output survives. All `[*] <<` inputs same type.

```polyglot
[p] =Search.A
   [=] <q << $query
   [=] >result >> $rA

[p] =Search.B
   [=] <q << $query
   [=] >result >> $rB

[*] *First
   [*] << $rA
   [*] << $rB
   [*] >> $fastest

[ ] Only $fastest is accessible here — $rA and $rB are cancelled
```

### EC-12.11: `*Nth` — generic race with `<n;int` IO

**EBNF:** `race_operator ::= "Nth"` ; `collect_io_line ::= "[*]" "<n;int" assignment_op value_expr`

**What it tests:** `*Nth` takes `<n;int` position parameter. `*First`/`*Second` are sugar for n=1/n=2.

```polyglot
[p] =Search.A
   [=] <q << $query
   [=] >result >> $rA

[p] =Search.B
   [=] <q << $query
   [=] >result >> $rB

[p] =Search.C
   [=] <q << $query
   [=] >result >> $rC

[*] *Nth
   [*] <n;int << 2
   [*] << $rA
   [*] << $rB
   [*] << $rC
   [*] >> $second
```

### EC-12.12: Multi-wave parallel pattern with multiple `*All` barriers

**What it tests:** `[*] *All` used twice in a pipeline body to form sequential parallel waves.

```polyglot
[p] =Fetch.Profile
   [=] <id << $id
   [=] >profile >> $profile

[p] =Fetch.Prefs
   [=] <id << $id
   [=] >prefs >> $prefs

[*] *All
   [*] << $profile
   [*] << $prefs

[p] =Enrich.A
   [=] <profile << $profile
   [=] >enriched >> $enriched

[p] =Enrich.B
   [=] <prefs << $prefs
   [=] >recs >> $recs

[*] *All
   [*] << $enriched
   [*] << $recs

[r] =Assemble
   [=] <enriched << $enriched
   [=] <recs << $recs
```

### EC-12.13: `[*] <<` vs `[*] >>` — wait input keeps variable, collect output cancels inputs

**What it tests:** Contrast: `[*] <<` alone on `*All` leaves vars accessible; `[*] <<`+`[*] >>` on `*First` cancels `[*] <<` vars.

```polyglot
[ ] *All: [*] << only — $a and $b accessible after
[*] *All
   [*] << $a
   [*] << $b
[r] =UseAB
   [=] <x << $a
   [=] <y << $b

[ ] *First: [*] <<+[*] >> — $a and $b cancelled; only $winner accessible
[*] *First
   [*] << $a
   [*] << $b
   [*] >> $winner
[r] =UseWinner
   [=] <x << $winner
```

---

## 13. Comments (§13)

### EC-13.1: Single-line square bracket comment

**EBNF:** `comment_line ::= "[ ]" comment_text`

```polyglot
[ ] This is a comment
```

### EC-13.2: Definition-level curly bracket comment

**EBNF:** `comment_curly ::= "{ }" comment_text`

```polyglot
{ } This is a top-level comment between definitions
```

### EC-13.3: Multiline comment block

**EBNF:** `multiline_comment ::= "[ ]<" NEWLINE { any_text NEWLINE } "[ ]>"`

```polyglot
[ ]<
This is a multiline comment.
It can span multiple lines.
No bracket prefix needed inside.
[ ]>
```

---

## 14. Variable Lifecycle (§14)

### EC-14.1: Default then Final — one reassignment

<!-- @variable-lifecycle -->
**What it tests:** Default allows exactly one promotion to Final. See [[variable-lifecycle]].

```polyglot
[=] >output;string ~> "fallback"
[ ] ... later in execution ...
[r] >output << "actual value"
```

### EC-14.2: Final — no further assignment (INVALID if reassigned)

**What it tests:** Once `<<` or `>>` is used, no more assignments.

```polyglot
[r] $x;int << 42
[ ] INVALID: $x is Final, cannot reassign
[ ] [r] $x << 99   ← would be rejected
```

### EC-14.3: Leaf-only assignment

<!-- @identifiers:Serialization Rules -->
**What it tests:** Only leaf fields (no children) can be assigned. See [[identifiers#Serialization Rules]].

```polyglot
[ ] VALID — assign to leaf
[r] $user:name << "Alice"
[r] $user:age << 30

[ ] INVALID — assign to branch that has children
[ ] [r] $user << "Alice"
[ ]    [r] $user:name << "Alice"
```

### EC-14.4: Sibling kind homogeneity

**What it tests:** All siblings must be the same kind (all enum or all value). Assignment within value fields is individually optional. See [[identifiers#Serialization Rules]].

```polyglot
[ ] VALID — all value fields, all assigned
[.] .timeout;int <~ 30
[.] .retries;int <~ 3

[ ] VALID — all value fields, mixed assignment (some assigned, some declared)
[.] .timeout;int <~ 30
[.] .retries;int

[ ] VALID — all value fields, none assigned
[.] .timeout;int
[.] .retries;int

[ ] INVALID — mixed kinds (enum + value at same level)
[ ] [.] .Active
[ ] [.] .count;int <~ 0
```

---

## 15. Metadata Blocks (§15)

### EC-15.1: `[%]` user-declared fields in pipeline

<!-- @blocks:Metadata -->
**EBNF:** `metadata_line ::= "[%]" fixed_sep name [ type_annotation ] [ "<<" value_expr ]`

**What it tests:** `[%]` lines appear before `[t]` (position 0). Fixed fields `.description`, `.version`, `.authors` assigned via `<<`. See [[blocks#Metadata]].

```polyglot
{=} =Invoice.Process
   [%] .description << "Processes incoming invoices and routes to accounting"
   [%] .version << "2.1.0"
   [%] .authors;array.string << {"alice@corp.com", "bob@corp.com"}
   [%] .license << "MIT"
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] $done;bool << #Boolean.True
```

### EC-15.2: `[%]` alias field — resolves to fully qualified path

**EBNF:** `alias_field ::= "[%]" ".alias" "<<" '#' name | '=' name`

**What it tests:** `.alias` field makes a definition reachable via short name. Alias preserves type prefix.

```polyglot
{#} #SystemConfig
   [%] .alias << #Config
   [.] .timeout;int <~ 30
   [.] .retries;int <~ 3

{=} =Provision.User
   [%] .alias << =ProvisionUser
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] $x;int << 1
```

### EC-15.3: `.info;serial` flexible metadata

**EBNF:** `info_field ::= "[%]" ".info;serial" NEWLINE { indent flex_data_field NEWLINE }`

**What it tests:** `.info;serial` opens a `:` flexible scope for arbitrary tooling metadata. See [[blocks#Metadata]].

```polyglot
{=} =Report.Generate
   [%] .description << "Generates monthly report"
   [%] .info;serial
      [:] :owner << "platform-team"
      [:] :ticket << "INFRA-42"
      [:] :runbook << "https://wiki/runbooks/report"
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] $x;int << 1
```

### EC-15.4: `live` metadata accessor — `%` on pipeline, variable, data

<!-- @types:Live Type Modifier -->
**EBNF:** `metadata_access ::= identifier '%' name`

**What it tests:** `%` accessor reads live runtime fields. Read-only — no assignment. See [[types#Live Type Modifier]], [[identifiers]].

```polyglot
[ ] Pipeline live fields
[?] =Invoice.Process%status
   [?] #AwaitTrigger
      [r] $ready;bool << #Boolean.True
   [?] #Running
      [r] $ready;bool << #Boolean.False
   [?] #Failed
      [b] =Audit.Log
         [=] <event << "pipeline_failed"
   [?] *?
      [r] $ready;bool << #Boolean.False

[ ] Variable lifecycle state
[?] $myVar%state
   [?] #Ready
      [r] $safe;bool << #Boolean.True
   [?] *?
      [r] $safe;bool << #Boolean.False

[ ] Data definition metadata (read-only counters)
[r] $uses;int << #Config%usageCount
```

---

## 16. Trigger IO Wiring (§16)

### EC-16.1: Trigger that produces outputs — IO declared before trigger, wired inside

<!-- @pipelines:Triggers -->
**EBNF:** `trigger_section ::= { io_line } trigger_line { indent trigger_io_line }`

**What it tests:** IO must be declared **before** the trigger that pushes into it. Trigger outputs wired via indented `[=]` lines. See [[pipelines#Triggers]].

```polyglot
{=} =Inbox.Monitor
   [=] <NewFiles;array.path
   [t] =T.Folder.NewFiles"/inbox/"
      [=] >NewFiles >> <NewFiles
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] $count;int << 0
```

### EC-16.2: Multiple trigger outputs wired to multiple inputs

**What it tests:** A trigger with two outputs, each wired to a declared input. Order of `[=]` declarations before `[t]` matters.

```polyglot
{=} =Webhook.Receiver
   [=] <payload;serial
   [=] <headers;serial
   [t] =T.Webhook"/api/v2/events"
      [=] >payload >> <payload
      [=] >headers >> <headers
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] $type;string << $payload:eventType
```

### EC-16.3: Mixed trigger modes — some inputs from trigger, some from caller

**What it tests:** Inputs can be filled by trigger wiring **or** left unfilled (must be provided by caller). No mixing of assignment modes on the same param.

```polyglot
{=} =File.Processor
   [=] <file;path
   [=] <options;serial <~ {}
   [t] =T.Folder.NewFiles"/watch/"
      [=] >NewFiles >> <file
   [Q] =Q.Default
   [W] =W.Polyglot
   [ ] $options uses default {}; $file comes from trigger
   [r] $name;string << "{$file}"
```

---

## 17. Negation Operators (§17)

### EC-17.1: All four negation comparison operators

<!-- @operators -->
**EBNF:** negation inserts `!` before `?` in base comparison: `<!?`, `>!?`, `<=!?`, `>=!?`

**What it tests:** Each negated form used correctly in `[?]` conditionals. `<!?` = not-less-than (≥), `>!?` = not-greater-than (≤), `<=!?` = not-less-or-equal (>), `>=!?` = not-greater-or-equal (<). See [[operators#Comparison Operators]].

```polyglot
[ ] Not less than — equivalent to >=
[?] $age <!? 18
   [r] $eligible;bool << #Boolean.True
[?] *?
   [r] $eligible;bool << #Boolean.False

[ ] Not greater than — equivalent to <=
[?] $score >!? 100
   [r] $capped;bool << #Boolean.True
[?] *?
   [r] $capped;bool << #Boolean.False

[ ] Not less-or-equal — equivalent to >
[?] $priority <=!? 3
   [r] $urgent;bool << #Boolean.True
[?] *?
   [r] $urgent;bool << #Boolean.False

[ ] Not greater-or-equal — equivalent to <
[?] $retries >=!? 5
   [r] $giveUp;bool << #Boolean.True
[?] *?
   [r] $giveUp;bool << #Boolean.False
```

### EC-17.2: Negation in compound logical condition

**What it tests:** Negation operators combined with `[&]` / `[+]` logical markers. See [[operators#Comparison Operators]], [[blocks#Logical]].

```polyglot
[ ] Active user who is not banned and age is not less than 13
[?] $active =? #Boolean.True
[&] $banned =!? #Boolean.True
[&] $age <!? 13
   [r] $allowed;bool << #Boolean.True
[?] *?
   [r] $allowed;bool << #Boolean.False
```

---

## 18. Macro Structure (§18)

### EC-18.1: Minimal macro — `[{]` input, `[}]` output, `[\]` setup, `[/]` cleanup

<!-- @blocks:Scope -->
<!-- @pipelines:Wrappers -->
**EBNF:** `macro_def ::= "{M}" pipeline_id NEWLINE { indent macro_body_line NEWLINE }`

**What it tests:** Complete `{M}` structure with all four scope markers. No `[t]`, `[Q]`, or `[=]` IO. See [[blocks#Scope]], [[pipelines#Wrappers]].

```polyglot
{M} =W.DB.Transaction
   [{] $connectionString;string
   [}] $dbConn;serial

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

**What it tests:** `[W]` wires macro IO using `[=]` with `$` variables. `[}]` outputs become available in body. See [[pipelines#Wrappers]].

```polyglot
{=} =Invoice.Save
   [=] <invoice;Invoice
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
   [{] $userId;string
   [{] $action;string

   [\]
      [r] =Audit.Open
         [=] <userId << $userId
         [=] <action << $action
         [=] >token >> $auditToken

   [/]
      [r] =Audit.Close
         [=] <token << $auditToken
```

---

## 19. Collections — Gaps (§19)

### EC-19.1: `*Into.Serial` — uses `<key` and `<value` (not `<item`)

<!-- @collections:Collect Operators -->
**EBNF:** `collect_operator ::= "Into.Serial"` with IO `<key`, `<value`, `>Serial`

**What it tests:** `*Into.Serial` takes separate key and value inputs, not a single `<item`. See [[collections#Collect Operators]].

```polyglot
[r] ~ForEach.Array
   [~] <Array << $pairs
   [~] >item >> $pair

   [r] *Into.Serial
      [*] <key << $pair:key
      [*] <value << $pair:value
      [*] >Serial >> $result
```

### EC-19.2: `*Into.Level` — collects siblings at a specific level

**EBNF:** `collect_operator ::= "Into.Level"` with IO `<key`, `<value`, `>Serial`

**What it tests:** Level-targeted collect, parallel to `~ForEach.Level`.

```polyglot
[r] ~ForEach.Level
   [~] <level << #UserData.Preferences.~
   [~] >key >> $k
   [~] >item >> $v

   [r] *Into.Level
      [*] <key << $k
      [*] <value << $v
      [*] >Serial >> >preferencesOut
```

### EC-19.3: `*Second` — sugar for `*Nth` n=2

**EBNF:** `race_operator ::= "Second"` — syntactic sugar, equivalent to `*Nth` with `<n << 2`

**What it tests:** `*Second` used like `*First` but captures 2nd-to-finish. Same `[*] <<`/`[*] >>` semantics.

```polyglot
[p] =Search.Fast
   [=] <q << $query
   [=] >result >> $rFast

[p] =Search.Accurate
   [=] <q << $query
   [=] >result >> $rAccurate

[p] =Search.Deep
   [=] <q << $query
   [=] >result >> $rDeep

[*] *Second
   [*] << $rFast
   [*] << $rAccurate
   [*] << $rDeep
   [*] >> $backup

[ ] Only $backup is accessible — others cancelled
```

### EC-19.4: `[b]` collector inside expand — fire-and-forget per item

<!-- @blocks:Execution -->
**What it tests:** `[b]` execution marker on a collector invocation — fires without waiting for result. See [[blocks#Execution]], [[collections#Collect Operators]].

```polyglot
[p] ~ForEach.Array
   [~] <Array << $events
   [~] >item >> $event

   [ ] Fire metric per item — no output needed
   [b] =Metrics.Emit
      [=] <event << $event
```

---

## 20. Variable Lifecycle — Gaps (§20)

### EC-20.1: Declared state — value field without assignment cannot be pulled

<!-- @variable-lifecycle -->
<!-- @identifiers:Serialization Rules -->
**What it tests:** A value field with no assignment is in **Declared** state. Pulling from it before assignment is a compile error. Assignment within value siblings is individually optional. See [[identifiers#Serialization Rules]].

```polyglot
[ ] VALID — declared field, pushed to later
{#} #Request
   [.] .id;string
   [.] .method;string <~ "GET"

[ ] In execution: .id is Declared, must be pushed before use
[r] $req;Request
   [.] .id << $incomingId
[ ] .method uses default; .id is now Final
[r] >requestOut << $req

[ ] INVALID — pulling from Declared variable is a compile error
[ ] [r] =Pipeline.Call
[ ]    [=] <x << $req.id   ← compile error if .id never pushed
```

### EC-20.2: Released state — variable in mini-pipeline cannot be used outside expand scope

**What it tests:** Variables declared inside `~ForEach` body are Released when the mini-pipeline ends. Accessing them outside is a compile error. See [[collections#Expand Operators]].

```polyglot
[p] ~ForEach.Array
   [~] <Array << $items
   [~] >item >> $item

   [r] $doubled;int << $item * 2

   [r] *Agg.Sum
      [*] <number << $doubled
      [*] >sum >> >total

[ ] VALID — $total was written to output port, accessible here
[r] =Log.Value
   [=] <n << >total

[ ] INVALID — $doubled is Released after expand scope ends
[ ] [r] =Log.Value
[ ]    [=] <n << $doubled   ← compile error: variable released
```

### EC-20.3: `~>` default operator on output parameters

<!-- @operators -->
**What it tests:** `~>` sets a default on an **output** parameter — if execution does not push a value, the default is used. See [[operators#Assignment Operators]].

```polyglot
{=} =Safe.Lookup
   [=] <key;string
   [=] >result;string ~> "not_found"
   [=] >found;bool ~> #Boolean.False
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot

   [r] =Cache.Get
      [=] <key << $key
      [=] >value >> $value
      [!] !Cache.Miss
         [ ] No push to >result — default "not_found" used
         [ ] No push to >found — default #Boolean.False used

   [ ] Cache hit path
   [r] >result << $value
   [r] >found << #Boolean.True
```

---

## 21. Third Registry Type (§21)

### EC-21.1: `Registry` type — public/global registry address format

<!-- @identifiers -->
<!-- @packages -->
**EBNF:** `registry_type ::= "Local" | "Community" | "Registry"`

**What it tests:** Third registry type `Registry` uses a different ID format from `Local` (numeric) and `Community` (username). See [[identifiers]], [[packages]].

```polyglot
{@} @Local:001.App:v1.0.0
   [@] @Slack << @Community:polyglot-tools.SlackAdmin:v1.3.0
   [@] @Payments << @Registry:stripe.PaymentsAPI:v3.0.0
```

---

## 22. Control Flow — Gaps (§22)

### EC-22.1: Exhaustiveness — `[?] *?` catch-all is mandatory when conditions are non-exhaustive

<!-- @operators -->
**EBNF:** `conditional_chain ::= { conditional_branch } [ wildcard_branch ]` — wildcard required if set is non-exhaustive.

**What it tests:** A conditional on a string/int value (open set) requires `*?`. Missing `*?` is a compile error. See [[operators#Comparison Operators]].

```polyglot
[ ] VALID — open set needs *?
[?] $code =? 200
   [r] $status;string << "ok"
[?] $code =? 404
   [r] $status;string << "not_found"
[?] $code =? 500
   [r] $status;string << "error"
[?] *?
   [r] $status;string << "unknown"

[ ] VALID — exhaustive enum: all variants covered, no *? needed
[?] $dir =? #Direction.North
   [r] $label;string << "N"
[?] $dir =? #Direction.South
   [r] $label;string << "S"
[?] $dir =? #Direction.East
   [r] $label;string << "E"
[?] $dir =? #Direction.West
   [r] $label;string << "W"
```

### EC-22.2: Nested conditionals inside a branch

**What it tests:** A `[?]` block inside another `[?]` branch — each nesting level is independently exhaustive.

```polyglot
[?] $role =? #Role.Admin
   [?] $region =? #Region.EU
      [r] $policy;string << "GDPR"
   [?] $region =? #Region.US
      [r] $policy;string << "CCPA"
   [?] *?
      [r] $policy;string << "Global"
[?] $role =? #Role.User
   [r] $policy;string << "Standard"
[?] *?
   [r] $policy;string << "None"
```

### EC-22.3: Switching on pipeline `%status` — nested enum switch

**What it tests:** `[?]` on a live metadata field; inner `[?]` checks enum variants. All branches plus `*?`. See [[types#Live Type Modifier]], [[pipelines#Querying Pipeline Status]].

```polyglot
[?] =DataSync%status
   [?] #AwaitTrigger
      [r] $msg;string << "idle"
   [?] #Running
      [r] $msg;string << "in progress — instances: {$count}"
   [?] #Failed
      [r] $msg;string << "failed — check errors"
      [b] =Alert.Send
         [=] <msg << "DataSync failed"
   [?] #Disabled
      [r] $msg;string << "pipeline disabled"
   [?] *?
      [r] $msg;string << "unknown state"
```

### EC-22.4: `[^]` XOR logical operator

<!-- @blocks:Logical -->
**What it tests:** XOR condition — true when exactly one of two conditions holds. See [[blocks#Logical]].

```polyglot
[ ] Exactly one of $isAdmin or $isSudo — not both, not neither
[?] $isAdmin =? #Boolean.True
[^] $isSudo =? #Boolean.True
   [r] $elevated;bool << #Boolean.True
[?] *?
   [r] $elevated;bool << #Boolean.False
```

---

## 23. Stress Tests (§23)

### ST-1: Full employee onboarding — imports, trigger, parallel, collect, errors, chain

**What it tests:** Full production-grade pipeline combining package imports, `=T.Call` trigger, parallel execution, `*All` sync barrier, chain execution, and per-call error handling.

```polyglot
{@} @Local:001.HR.Onboarding:v2.0.0
   [@] @AD << @Local:001.ActiveDirectory:v2.1.0
   [@] @Mail << @Local:001.EmailSystem:v1.5.0
   [@] @HR << @Local:001.HRSystem:v3.0.0
   [@] @Slack << @Community:polyglot-tools.SlackAdmin:v1.3.0

{#} #NewHire
   [.] .id;string
   [.] .name;string
   [.] .email;string
   [.] .department;string
   [.] .startDate;string

{=} =Onboard.Employee
   [%] .description << "Provisions all accounts for a new hire"
   [%] .version << "2.0.0"
   [=] <hire;NewHire
   [=] >report;string ~> "incomplete"
   [=] >success;bool ~> #Boolean.False
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot

   [ ] Parallel provisioning — all three fire at once
   [p] @AD=Account.Create
      [=] <name << $hire.name
      [=] <email << $hire.email
      [=] >adId >> $adId
      [!] !AD.CreateFailed
         [r] $adId << "AD_FAILED"
      [!] !AD.Timeout
         [r] $adId << "AD_TIMEOUT"

   [p] @Mail=Mailbox.Provision
      [=] <email << $hire.email
      [=] <displayName << $hire.name
      [=] >mailboxId >> $mailboxId
      [!] !Mail.ProvisionFailed
         [r] $mailboxId << "MAIL_FAILED"

   [p] @Slack=User.Invite
      [=] <email << $hire.email
      [=] <team << $hire.department
      [=] >slackId >> $slackId
      [!] !Slack.InviteFailed
         [r] $slackId << "SLACK_FAILED"

   [ ] Wait for all three to complete
   [*] *All
      [*] << $adId
      [*] << $mailboxId
      [*] << $slackId

   [ ] Record to HR system — chain: build record then save
   [r] @HR=Record.Build=>@HR=Record.Save
      [=] >Build.hireId << $hire.id
      [=] >Build.adAccount << $adId
      [=] >Build.mailbox << $mailboxId
      [=] >Build.slack << $slackId
      [=] <Save.status >> >report
      [!] .0!Build.ValidationError
         [r] >report << "record build failed"
      [!] .1!Save.WriteError
         [r] >report << "record save failed"

   [ ] Mark success only if none of the IDs are failure markers
   [?] $adId =!? "AD_FAILED"
   [&] $adId =!? "AD_TIMEOUT"
   [&] $mailboxId =!? "MAIL_FAILED"
   [&] $slackId =!? "SLACK_FAILED"
      [r] >success << #Boolean.True
   [?] *?
      [r] >success << #Boolean.False
```

### ST-2: Complex conditional branching — range, logical, negation, exhaustive

**What it tests:** Nested range checks, all logical operators, negation operators, XOR, and mandatory `*?`. Every branch path is non-trivial.

```polyglot
{=} =Risk.Classify
   [=] <score;int
   [=] <flags;int
   [=] <verified;bool
   [=] >tier;string ~> "unknown"
   [=] >action;string ~> "review"
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot

   [ ] High risk: score > 80 AND (not verified OR flags > 2)
   [?] $score >? 80
   [&] $verified =!? #Boolean.True
      [r] >tier << "high"
      [r] >action << "block"

   [ ] Also high risk: score > 80 AND flags > 2 (even if verified)
   [?] $score >? 80
   [&] $flags >? 2
      [r] >tier << "high"
      [r] >action << "escalate"

   [ ] Medium: score in [50,80], flags not excessive
   [?] $score ?[50,80]
   [&] $flags <=? 2
      [r] >tier << "medium"
      [r] >action << "monitor"

   [ ] Low: score strictly in (0,50), verified, no flags
   [?] $score ?(0,50)
   [&] $verified =? #Boolean.True
   [&] $flags =? 0
      [r] >tier << "low"
      [r] >action << "pass"

   [ ] Suspicious: high score XOR high flags (one but not both)
   [?] $score >? 80
   [^] $flags >? 2
      [r] >tier << "suspicious"
      [r] >action << "investigate"

   [ ] Zero or negative score — anomalous
   [?] $score <=? 0
      [r] >tier << "invalid"
      [r] >action << "reject"

   [ ] Catch-all for any uncovered combination
   [?] *?
      [r] >tier << "unknown"
      [r] >action << "review"
```

### ST-3: Race collector feeding a chain — `*First` winner into chain execution

**What it tests:** Three parallel pipelines racing, winner fed directly into a chain via `[*] >>` collect output, then chain processed with explicit IO wiring and error handling.

```polyglot
{=} =Search.BestResult
   [=] <query;string
   [=] >result;serial ~> {}
   [=] >source;string ~> "none"
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot

   [ ] Fire three search engines in parallel
   [p] =Search.Engine.Fast
      [=] <q << $query
      [=] >result >> $fast

   [p] =Search.Engine.Semantic
      [=] <q << $query
      [=] >result >> $semantic

   [p] =Search.Engine.Archive
      [=] <q << $query
      [=] >result >> $archive

   [ ] Take whoever finishes first
   [*] *First
      [*] << $fast
      [*] << $semantic
      [*] << $archive
      [*] >> $winner

   [ ] Enrich and format the winner — chain
   [r] =Result.Enrich=>=Result.Format
      [=] >Enrich.raw << $winner
      [=] >Enrich.query << $query
      [=] <Enrich.enriched >> <Format.input
      [=] <Format.output >> >result
      [=] <Format.source >> >source
      [!] .Enrich!Enrich.Failed
         [r] >result << {}
         [r] >source << "enrich_error"
      [!] .Format!Format.Failed
         [r] >source << "format_error"
```

### ST-4: Multi-wave parallel with macro wrapper and nested expand

**What it tests:** Two parallel waves separated by `*All` barrier, followed by an expand+collect pipeline, all inside a DB transaction wrapper.

```polyglot
{=} =Batch.Process
   [=] <items;array.serial
   [=] >summary;serial ~> {}
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.DB.Transaction
      [=] $connectionString << $dbConnStr
      [=] $dbConn >> $dbConn

   [ ] Wave 1: fetch metadata for the batch in parallel
   [p] =Batch.FetchMetadata
      [=] <conn << $dbConn
      [=] <items << $items
      [=] >meta >> $meta

   [p] =Batch.FetchPermissions
      [=] <conn << $dbConn
      [=] <items << $items
      [=] >perms >> $perms

   [*] *All
      [*] << $meta
      [*] << $perms

   [ ] Wave 2: validate and enrich in parallel using wave 1 results
   [p] =Batch.Validate
      [=] <items << $items
      [=] <meta << $meta
      [=] <perms << $perms
      [=] >valid >> $validItems

   [p] =Batch.EnrichAll
      [=] <items << $items
      [=] <meta << $meta
      [=] >enriched >> $enrichedItems

   [*] *All
      [*] << $validItems
      [*] << $enrichedItems

   [ ] Process each valid+enriched item sequentially, collect results
   [r] ~ForEach.Array
      [~] <Array << $validItems
      [~] >item >> $item

      [r] =Item.Process
         [=] <item << $item
         [=] <conn << $dbConn
         [=] >status >> $itemStatus
         [!] !Item.ProcessFailed
            [r] $itemStatus;string << "failed"

      [r] *Into.Serial
         [*] <key << $item:id
         [*] <value << $itemStatus
         [*] >Serial >> >summary
```

### ST-5: Deep nesting — expand inside conditional inside expand with collectors

**What it tests:** Expand nested inside a conditional branch, which is itself inside another expand. Tests 4+ levels of indentation, per-level collector scoping, and `*?` at each conditional level.

```polyglot
{=} =Tree.Flatten
   [=] <categories;array.serial
   [=] >flat;array.string ~> {}
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot

   [ ] Outer expand — one mini-pipeline per category
   [p] ~ForEach.Array
      [~] <Array << $categories
      [~] >item >> $category

      [?] $category:enabled =? #Boolean.True
         [ ] Inner expand — each category's items
         [r] ~ForEach.Array
            [~] <Array << $category:items
            [~] >item >> $leaf

            [r] $label;string << "{$category:name}/{$leaf:name}"

            [r] *Into.Array
               [*] <item << $label
               [*] >Array >> >flat
      [?] *?
         [ ] Disabled category — skip silently
```

### ST-6: Macro with parallel timer in setup, body, and collect in cleanup

**What it tests:** The `[p]` in `[\]` with no `*All` — timer runs concurrently with body. `[/]` uses `*All` with `[*] <<` wait input to collect timer handle before stopping it. See [[pipelines#Parallel Forking in Setup]].

```polyglot
{M} =W.Traced
   [{] $operationId;string
   [}] $durationMs;int
   [}] $spanId;string

   [\]
      [ ] Sequential: open trace session before body
      [r] =Tracer.Open
         [=] <opId << $operationId
         [=] >session >> $session
         [=] >spanId >> $spanId

      [ ] Parallel: start timer — no *All, so it runs with body
      [p] =Tracer.StartTimer
         [=] <session << $session
         [=] >handle >> $timerHandle

   [ ] ... body executes while timer runs ...

   [/]
      [ ] Collect timer handle — must be Final before we stop it
      [*] *All
         [*] << $timerHandle

      [r] =Tracer.StopTimer
         [=] <handle << $timerHandle
         [=] >elapsed >> $durationMs

      [r] =Tracer.Close
         [=] <session << $session

{=} =Invoice.Parse
   [=] <raw;string
   [=] >invoice;serial ~> {}
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Traced
      [=] $operationId << "invoice.parse"
      [=] $durationMs >> $parseDuration
      [=] $spanId >> $spanId

   [ ] $spanId and $parseDuration from macro [}] outputs
   [r] =JSON.Parse
      [=] <input << $raw
      [=] >parsed >> >invoice
      [!] !JSON.ParseError
         [r] >invoice << {}

   [b] =Metrics.Record
      [=] <span << $spanId
      [=] <duration << $parseDuration
```

---

## Coverage Matrix

| EBNF Section | Edge Cases | Covered Productions |
|-------------|-----------|-------------------|
| §1 File Structure | EC-1.1, EC-1.2 | `file`, `definition` |
| §2 Lexical | EC-2.1–2.4 | `indent`, `bool_literal`, `int_literal`, `float_literal`, `string_literal` |
| §3 Identifiers | EC-3.1–3.7 | `package_address`, `cross_pkg_enum`, `cross_pkg_pipeline`, `field_path`, sibling homogeneity |
| §4 Types | EC-4.1–4.18 | `array_type`, `element_type`, `serial_type`, `user_type`, `inline_pipeline_call`, path types, multidimensional arrays |
| §5 Blocks | EC-5.1–5.2 | All block element categories, `[b]` background |
| §6 Operators | EC-6.1–6.4 | All assignment ops, all comparison ops, range ops, arithmetic |
| §7 IO | EC-7.1 | `input_param` with field separators |
| §8 Expressions | EC-8.1–8.3 | `inline_data`, empty `{}`, chained arithmetic |
| §9 Definitions | EC-9.1–9.11 | Package imports, enum/value fields, pipeline structure, triggers, IO modes, macro parallel fork |
| §10 Execution | EC-10.1–10.11 | Pipeline call + error, stdlib call, chain execution, chain IO, chain auto-wire, chain errors, serial load, parallel |
| §11 Control Flow | EC-11.1–11.3 | Conditional chains, error scoping, logical operators |
| §12 Collections | EC-12.1–12.13 | All expand variants, all collect variants, direct output, multiple collectors, sync/race collectors, multi-wave, [*] <</>>/semantics |
| §13 Comments | EC-13.1–13.3 | Square, curly, multiline |
| §14 Lifecycle | EC-14.1–14.4 | Default→Final, Final immutability, leaf-only, all-or-none |
| §15 Metadata Blocks | EC-15.1–15.4 | `[%]` user fields, alias, `.info;serial`, `%` live accessor |
| §16 Trigger IO Wiring | EC-16.1–16.3 | Trigger outputs, multi-output wiring, mixed fill modes |
| §17 Negation Operators | EC-17.1–17.2 | `<!?`, `>!?`, `<=!?`, `>=!?`, negation in compound logic |
| §18 Macro Structure | EC-18.1–18.3 | `{M}` full structure, `[W]` usage wiring, no-output macro |
| §19 Collections — Gaps | EC-19.1–19.4 | `*Into.Serial`, `*Into.Level`, `*Second`, `[b]` collector |
| §20 Lifecycle — Gaps | EC-20.1–20.3 | Declared state, Released state, `~>` on output params |
| §21 Third Registry Type | EC-21.1 | `Registry` address format |
| §22 Control Flow — Gaps | EC-22.1–22.4 | `*?` exhaustiveness, nested conditionals, `%status` switch, `[^]` XOR |
| §23 Stress Tests | ST-1–ST-6 | Full onboarding, complex conditionals, race+chain, multi-wave+expand, deep nesting, macro+timer |

**Total: 51 original + 33 new = 84 edge cases across 23 sections.**
