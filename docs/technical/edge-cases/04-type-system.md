---
audience: design
type: reference
updated: 2026-03-30
---

<!-- @edge-cases/INDEX -->

## 4. Type System (S4)

### EC-4.1: Element-typed array — basic type

<!-- @u:types:Element-Typed Arrays -->
**EBNF:** `array_type ::= "array" [ fixed_sep element_type ]`

**What it tests:** `array.string`, `array.int`, `array.path` — dot separates array from element type. See [[syntax/types/arrays#Element-Typed Arrays]].

```aljam3
(-) <names#array:string
(-) <scores#array:int
(-) <files#array:path
```

### EC-4.2: Element-typed array — user-defined type (no # prefix)

**EBNF:** `element_type ::= basic_type | name` — user types drop `#` inside `array.`

**What it tests:** `array.UserRecord` not `array.#UserRecord`. See [[syntax/types/basic-types#User-Defined Types]].

```aljam3
(-) <users#array:UserRecord
```

### EC-4.3: Serial type

**EBNF:** `serial_type ::= "serial"`

**What it tests:** `serial` as a type annotation on IO parameters. See [[syntax/types/basic-types#Basic Types]], [[concepts/collections/INDEX#Collection Hierarchy]].

```aljam3
(-) <payload#serial
```

### EC-4.4: User-defined type reference

**EBNF:** `user_type ::= '#' dotted_name`

**What it tests:** `#DataName` as type annotation. See [[syntax/types/basic-types#User-Defined Types]].

```aljam3
[-] $hire#NewHire << <payload
```

### EC-4.5: `$Path"..."` constructor path creation

**What it tests:** `$Path"..."` constructor call creating `#path` values in execution body. On infrastructure lines (`[T]`/`[Q]`/`[W]`), the inline form `-Path"..."` remains valid. See [[syntax/types/strings#$Path Constructor Notation]], [[syntax/constructors]], [[aj3lib/pipelines/Path|-Path]].

```aljam3
[ ] Basic usage — constructor in execution body
[-] $dir#path << $Path"/tmp/MyApp"

[ ] With {.} shorthand
[-] $logDir#path << $Path"{.}/logs"

[ ] Separator equivalence — both resolve identically
[-] $a#path << $Path"{.}\MyApp\logs"
[-] $b#path << $Path"{.}/MyApp/logs"

[ ] Interpolation with constructor-sourced path variable
[-] $root#path << $Path"/opt"
[-] $appDir#path << $Path"{$root}/MyApp"

[ ] Literal braces in path string
[-] $weird#path << $Path"/tmp/{{backup}}/files"

[ ] Infrastructure line — inline form still valid
[W] -W.Env
   (-) <scriptDir#path << -Path"./scripts"
```

### EC-4.6: Single-platform path (PGW04001 / PGE04008)

**What it tests:** Warning when only one OS subfield assigned; error when current OS subfield missing. See [[syntax/types/strings#Explicit Subfield Assignment]].

```aljam3
[ ] PGW04001 — single platform, but matches current OS (Unix)
[ ]
[-] $dir#path
   [.] .Unix << "/tmp/MyApp"

[ ] PGE04008 — .Unix missing, compiling on Unix
[-] $dir#path
   [.] .Windows << "C:\MyApp"

[ ] suppressed warning
[ ] Ignore PGW04001
[-] $dir#path
   [.] .Unix << "/tmp/MyApp"

[ ] no warning — both platforms
[-] $dir#path
   [.] .Unix << "/tmp/MyApp"
   [.] .Windows << "C:\MyApp"
```

### EC-4.7: Plain string to `#path` type mismatch (PGE04001)

**What it tests:** Assigning a plain string to a `#path` variable is a type mismatch. See [[syntax/types/strings#Explicit Subfield Assignment]].

```aljam3
[ ] PGE04001 — string != path, no implicit coercion
[-] $dir#path << "/tmp/MyApp"

[ ] correct — use $Path"..." constructor
[-] $dir#path << $Path"/tmp/MyApp"
```

### EC-4.8: Inline pipeline call — single output (infrastructure context)

**EBNF:** `inline_pipeline_call ::= pipeline_ref string_literal`

**What it tests:** An inline pipeline call on an infrastructure line with one output evaluates to that output's type directly. In execution body, use constructors for known values.

```aljam3
[ ] infrastructure line — inline pipeline config
[W] -W.Env
   (-) <dir#path << -Path"/tmp/MyApp"

[ ] execution body — use $Path constructor instead
[ ]
[-] $dir#path << $Path"/tmp/MyApp"
```

### EC-4.9: Inline pipeline call — multiple outputs (infrastructure context)

**What it tests:** An inline pipeline call with multiple outputs evaluates to `#serial` with output parameter names as keys. This is an infrastructure-context pattern — `-ParsePair` uses `%InlineString` for infrastructure configuration.

```aljam3
( ) -ParsePair is an infrastructure pipeline with %InlineString
{-} -ParsePair
   (-) %InlineString << "{key}-{value}"
   (-) <key#string
   (-) <value#string
   (-) >key#string
   (-) >value#string
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3
   [ ] ... parsing logic ...

[ ] infrastructure line usage — multiple outputs -> #serial
[W] -W.Custom
   (-) <config#serial << -ParsePair"name-Alice"
```

### EC-4.10: Inline pipeline call — type mismatch (infrastructure context)

**What it tests:** Target type must match the inline pipeline's output type on infrastructure lines.

```aljam3
[ ] PGE04001 — -Path returns #path, not #string
[W] -W.Env
   (-) <name#string << -Path"/tmp"

[ ] matching types
[W] -W.Env
   (-) <dir#path << -Path"/tmp"

[ ] execution body — use $Path constructor
[ ]
[-] $dir#path << $Path"/tmp"
```

### EC-4.11: Inline pipeline call — user-defined infrastructure pipeline

**What it tests:** User-defined pipelines can accept inline calls on infrastructure lines by declaring `%InlineString` with a template. In execution body, use constructors for known values or `[-]` pipeline calls for dynamic values.

```aljam3
{-} -Greeting
   (-) %InlineString << "{name}"
   (-) <name#string <~ "World"
   (-) >message#string
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3
   [ ]
   [-] >message << "Hello {$name}"

[ ] infrastructure line — inline call valid
[W] -W.Custom
   (-) <greeting << -Greeting"Alice"

[ ] execution body — normal call (inline calls are infrastructure-only)
[ ]
[-] -Greeting
   (-) <name << "Alice"
   (-) >message >> $msg
```

### EC-4.12: Pipeline without `%InlineString` called inline (infrastructure context)

**What it tests:** Calling a pipeline inline on an infrastructure line when it has not declared a `%InlineString` template. This rule (PGE12003) applies to infrastructure inline calls only — for constructor errors in execution body, see PGE14xxx.

```aljam3
{-} -NormalPipeline
   (-) <input#string
   (-) >output#string
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3
   [ ]
   [-] >output << $input

[ ] PGE12003 — -NormalPipeline has no %InlineString declaration (infrastructure line)
[W] -W.Custom
   (-) <result << -NormalPipeline"test"
```

### EC-4.13: Typed flexible wildcard — basic inference

**EBNF:** `typed_flex_wildcard ::= "[:]" flex_sep "*" type_annotation`

**What it tests:** New `:key` at a typed flexible level inherits the wildcard type. No explicit annotation needed. See [[syntax/types/flexible-fields#Typed Flexible Fields]].

```aljam3
{#} #Handler
   [.] .endpoint#string
   [.] .method#string

{#} #Registry
   [.] .plugins
      [:] :*#Handler

[ ] compiler infers :myPlugin is #Handler
[-] $reg.plugins:myPlugin.endpoint << "/api/data"
[-] $reg.plugins:myPlugin.method << "GET"
```

### EC-4.14: Typed flexible wildcard — contradicting annotation (PGE04001)

**What it tests:** Explicit type annotation that contradicts the wildcard type is a compile error.

```aljam3
{#} #Registry
   [.] .plugins
      [:] :*#Handler

[ ] PGE04001 — :myPlugin is #Handler (from wildcard), not #string
[-] $reg.plugins:myPlugin#string << "not a handler"
```

### EC-4.15: Typed flexible wildcard — multi-level resolution

**What it tests:** Nested typed flexible levels — compiler resolves one level at a time. See [[syntax/types/flexible-fields#Constraints]].

```aljam3
{#} #Setting
   [.] .value#string
   [.] .default#string

{#} #Section
   [:] :*#Setting

{#} #Config
   [.] .sections
      [:] :*#Section

[ ] :auth -> #Section, :timeout -> #Setting, .value -> #string
[-] $cfg.sections:auth:timeout.value << "30s"
[-] $cfg.sections:auth:timeout.default << "60s"
```

### EC-4.16: Typed flexible wildcard — untyped level (no wildcard)

**What it tests:** Flexible level without `[:] :*#Type` is untyped — treated as `#serial`.

```aljam3
{#} #OpenConfig
   [.] .data
      [:] :key1#string
      [:] :key2#int

[ ] individually declared flex fields — matched by name
[-] $cfg.data:key1 << "hello"
[-] $cfg.data:key2 << 42

[ ] :unknown has no wildcard, no individual declaration — treated as #serial
[-] $cfg.data:unknown << "anything"
```

### EC-4.17: Typed flexible wildcard — individual override before wildcard fallback

**What it tests:** Named flex field matches first; wildcard is fallback.

```aljam3
{#} #MixedRegistry
   [.] .entries
      [:] :default#SpecialHandler
      [:] :*#Handler

[ ] :default matches the named declaration -> #SpecialHandler
[-] $reg.entries:default.specialField << "value"

[ ] :other falls back to wildcard -> #Handler
[-] $reg.entries:other.endpoint << "/api"
```

### EC-4.18: Multidimensional array — `:ND` dimension specifier

**EBNF:** `array_type ::= "array" [ fixed_sep element_type ] [ flex_sep dimension ]` with `dimension ::= digit { digit } "D"`

**What it tests:** Arrays support a dimension specifier via flexible field notation. Omitting `:<N>D` defaults to 1D. Access depth must match declared dimension count.

```aljam3
[ ] 1D array — default (no :ND specified)
(-) <items#array:string
[-] $first << $items.0

[ ] 2D matrix — :2D dimension specifier
(-) <matrix#array:float:2D
[-] $val << $matrix.0.1

[ ] 3D cube — :3D dimension specifier
(-) <cube#array:int:3D
[-] $val << $cube.2.3.0

[ ] 4D with user-defined element type
(-) <hyper#array:UserRecord:4D
[-] $cell << $hyper.0.1.2.3

[ ] PGE04017 — too many indices for :2D
(-) <matrix#array:float:2D
[ ] [-] $val << $matrix.0.1.2                 <- 3 indices on :2D

[ ] PGE04017 — too few indices for :3D
(-) <cube#array:int:3D
[ ] [-] $val << $cube.2                        <- 1 index on :3D

[ ] PGE04017 — :0D is not valid
[ ] (-) <nothing#array:float:0D                <- dimension must be positive

[ ] PGE04013 — nested array still banned
[ ] (-) >matrix#array:array.float              <- use #array:float:2D instead
```

### EC-4.19: `#array` without element type

**What it tests:** `#array` type annotation without element type specifier. The grammar requires `element_type_param` — `#array` alone is a grammar error (previously PGE04025, now grammar-enforced).

```aljam3
[ ] ✗ grammar error — no element type
(-) <items#array

[ ] ✓ typed array
(-) <items#array:int
```

### EC-4.20: Wildcard type `#*` in non-generic context (X.37)

**EBNF ref:** `wildcard_type` removed from `type_expr` and `type_param` (§4.1)

**What it tests:** `#*` is no longer valid in any type annotation position. The grammar does not include a wildcard type. Multi-type constraints use `##` schemas instead (e.g., `##Scalar`, `##Leaf`).

```aljam3
[ ] ✗ grammar error — wildcard type removed
[-] $x#* << 42

[ ] ✗ grammar error — no wildcard in field declarations
{#} #Loose
   [.] .anything#*

[ ] ✗ grammar error — no wildcard in IO params
(-) <input#*

[ ] ✓ correct — use ## schema for multi-type constraints
(#) <#T
   [<] ##Scalar

[ ] ✓ correct — specific type annotation
[-] $x#int << 42
```

### EC-4.21: Dimension without element type in array (X.38)

**EBNF ref:** `array_type ::= "array" flex_sep element_type_param [ flex_sep dimension ]` (§4.1), `element_type_param ::= basic_type \| user_type`

**What it tests:** `element_type_param` excludes `dimension`, so `#array:2D` no longer parses — the first slot only accepts element types, not dimension literals.

```aljam3
[ ] ✗ grammar error — dimension in element type slot
[-] $matrix#array:2D

[ ] ✗ grammar error — dimension in both slots
[-] $weird#array:3D:2D

[ ] ✓ correct — element type then dimension
[-] $matrix#array:float:2D
[-] $list#array:string
[-] $cube#array:int:3D
```

### EC-4.22: Double `live` type annotation (X.39)

**EBNF ref:** `live_type ::= "live" concrete_type_expr` (��4.1) — `concrete_type_expr` excludes `live_type`

**What it tests:** Nested `live` is a grammar error. `live` wraps `concrete_type_expr` (basic, collection, or user type), not `type_expr`. Double or deeper nesting no longer parses. Note: `live` is an internal compiler/metadata concept, not user-facing.

```aljam3
[ ] ✗ grammar error — double live
[-] $x#live live string

[ ] ✗ grammar error — triple live
[-] $y#live live live int

[ ] ✓ correct — single live
[-] $z#live string
```

### EC-4.23: Multi-digit version segments (X.40)

**EBNF ref:** `version ::= 'v' digit { digit } '.' digit { digit } '.' digit { digit } [ '.' digit { digit } ]` (§3.3)

**What it tests:** Version segments now support multi-digit numbers. Previously each segment was limited to a single `digit` (0-9), capping versions at v9.9.9.9.

```aljam3
[ ] ✓ single-digit segments (unchanged)
[@] @Registry:Acme.Analytics:v1.2.3

[ ] ✓ multi-digit — now valid
[@] @Registry:Acme.Analytics:v12.3.4
[@] @Registry:Acme.Analytics:v1.23.456

[ ] ✓ four-segment with multi-digit
[@] @Registry:Acme.Analytics:v10.0.0.1
```
