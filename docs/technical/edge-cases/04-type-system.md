---
audience: designer
type: reference
updated: 2026-03-30
---

<!-- @edge-cases/INDEX -->

## 4. Type System (S4)

### EC-4.1: Element-typed array — basic type

<!-- @types:Element-Typed Arrays -->
**EBNF:** `array_type ::= "array" [ fixed_sep element_type ]`

**What it tests:** `array.string`, `array.int`, `array.path` — dot separates array from element type. See [[syntax/types/arrays#Element-Typed Arrays]].

```polyglot
(-) <names#array:string
(-) <scores#array:int
(-) <files#array:path
```

### EC-4.2: Element-typed array — user-defined type (no # prefix)

**EBNF:** `element_type ::= basic_type | name` — user types drop `#` inside `array.`

**What it tests:** `array.UserRecord` not `array.#UserRecord`. See [[syntax/types/basic-types#User-Defined Types]].

```polyglot
(-) <users#array:UserRecord
```

### EC-4.3: Serial type

**EBNF:** `serial_type ::= "serial"`

**What it tests:** `serial` as a type annotation on IO parameters. See [[syntax/types/basic-types#Basic Types]], [[concepts/collections/INDEX#Collection Hierarchy]].

```polyglot
(-) <payload#serial
```

### EC-4.4: User-defined type reference

**EBNF:** `user_type ::= '#' dotted_name`

**What it tests:** `#DataName` as type annotation. See [[syntax/types/basic-types#User-Defined Types]].

```polyglot
[-] $hire#NewHire << <payload
```

### EC-4.5: `-Path"..."` inline path creation

**What it tests:** `-Path"..."` inline pipeline call creating `#path` values. See [[syntax/types/strings#-Path Inline Notation]], [[pglib/pipelines/Path|-Path]].

```polyglot
[ ] Basic usage
[-] $dir#path << -Path"/tmp/MyApp"

[ ] With {.} shorthand
[-] $logDir#path << -Path"{.}/logs"

[ ] Separator equivalence — both resolve identically
[-] $a#path << -Path"{.}\MyApp\logs"
[-] $b#path << -Path"{.}/MyApp/logs"

[ ] Interpolation with user-defined path variable
[-] $root#path
   [.] .Unix << "/opt"
   [.] .Windows << "D:"
[-] $appDir#path << -Path"{$root}/MyApp"

[ ] Literal braces in path string
[-] $weird#path << -Path"/tmp/{{backup}}/files"
```

### EC-4.6: Single-platform path (PGW04001 / PGE04008)

**What it tests:** Warning when only one OS subfield assigned; error when current OS subfield missing. See [[syntax/types/strings#Explicit Subfield Assignment]].

```polyglot
[ ] PGW04001 — single platform, but matches current OS (Unix)
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

```polyglot
[ ] PGE04001 — string != path, no implicit coercion
[-] $dir#path << "/tmp/MyApp"

[ ] correct — use -Path"..." instead
[-] $dir#path << -Path"/tmp/MyApp"
```

### EC-4.8: Inline pipeline call — single output

**EBNF:** `inline_pipeline_call ::= pipeline_ref string_literal`

**What it tests:** An inline pipeline call with one output evaluates to that output's type directly.

```polyglot
[ ] -Path has one output >result#path — value is #path
[-] $dir#path << -Path"/tmp/MyApp"

[ ] inline call as comparison operand
[?] $dir =? -Path"/expected"
```

### EC-4.9: Inline pipeline call — multiple outputs

**What it tests:** An inline pipeline call with multiple outputs evaluates to `#serial` with output parameter names as keys.

```polyglot
{-} -ParsePair
   (-) <InlineStringLiteral#string <~ ""
   (-) >key#string
   (-) >value#string
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [ ] ... parsing logic ...

[ ] multiple outputs -> #serial with keys "key" and "value"
[-] $result#serial << -ParsePair"name-Alice"
```

### EC-4.10: Inline pipeline call — type mismatch

**What it tests:** Target type must match the inline pipeline's output type.

```polyglot
[ ] PGE04001 — -Path returns #path, not #string
[-] $name#string << -Path"/tmp"

[ ] matching types
[-] $dir#path << -Path"/tmp"
```

### EC-4.11: Inline pipeline call — user-defined pipeline

**What it tests:** User-defined pipelines can accept inline calls by declaring `<InlineStringLiteral#string`.

```polyglot
{-} -Greeting
   (-) <InlineStringLiteral#string <~ ""
   (-) >message#string
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [?] $InlineStringLiteral =!? ""
      [-] >message << "Hello {$InlineStringLiteral}"
   [?] *?
      [-] >message << "Hello World"

[ ] inline call
[-] $msg#string << -Greeting"Alice"

[ ] normal call — $InlineStringLiteral is "" (default)
[-] -Greeting
   (-) >message >> $msg
```

### EC-4.12: Pipeline without `<InlineStringLiteral#string` called inline

**What it tests:** Calling a pipeline inline when it has not declared the reserved parameter.

```polyglot
{-} -NormalPipeline
   (-) <input#string
   (-) >output#string
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] >output << $input

[ ] compile error — -NormalPipeline has no <InlineStringLiteral#string
[-] $result#string << -NormalPipeline"test"
```

### EC-4.13: Typed flexible wildcard — basic inference

**EBNF:** `typed_flex_wildcard ::= "[:]" flex_sep "*" type_annotation`

**What it tests:** New `:key` at a typed flexible level inherits the wildcard type. No explicit annotation needed. See [[syntax/types/flexible-fields#Typed Flexible Fields]].

```polyglot
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

```polyglot
{#} #Registry
   [.] .plugins
      [:] :*#Handler

[ ] PGE04001 — :myPlugin is #Handler (from wildcard), not #string
[-] $reg.plugins:myPlugin#string << "not a handler"
```

### EC-4.15: Typed flexible wildcard — multi-level resolution

**What it tests:** Nested typed flexible levels — compiler resolves one level at a time. See [[syntax/types/flexible-fields#Constraints]].

```polyglot
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

```polyglot
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

```polyglot
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

```polyglot
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

### EC-4.19: `{Array}` without element type

**EBNF ref:** `array_def ::= "{Array}" variable_id type_annotation NEWLINE ...`
**What it tests:** `#array` without element type specifier. PGE04025 fires — element type is mandatory. All elements must share the same schema.

```polyglot
[ ] ✗ PGE04025 — no element type
{Array} $items#array
   [-] $items << {1, "mixed", #Boolean.True}
```

```polyglot
[ ] ✓ typed array
{Array} $items#array.int
   [-] $items << {1, 2, 3}
```
