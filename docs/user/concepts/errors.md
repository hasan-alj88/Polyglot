---
audience: user
type: spec
updated: 2026-03-22
---

# Error Handling

<!-- @pipelines:Error Handling -->
<!-- @variable-lifecycle:Failed -->
<!-- @data-is-trees -->

Errors in Polyglot Code use the `!` prefix and live at the `%!` branch of the metadata tree (see [[data-is-trees#How Concepts Connect]]). They follow the same [[identifiers]] rules as all Polyglot objects — `.` for fixed fields, `:` for flexible fields.

## Error Scoping

`[!]` error blocks are scoped to the specific `[r]` call that can produce them, indented under the call (after its `[=]` IO lines):

```polyglot
[r] @FS=File.Text.Read
   [=] <path << <filepath
   [=] >content >> >content
   [!] !File.NotFound
      [r] >content << "Error: file not found"
   [!] !File.ReadError
      [r] >content << "Error: could not read file"
```

By default, if an `[!]` handler does **not** push a replacement value into the output variable, the pipeline **terminates on error**. No downstream code runs. This is the safe default.

## Error Recovery

To continue after an error, place `[*] *Continue` inside the `[!]` block. `*Continue` is a collector that produces a boolean `>IsFailed` output:

```polyglot
[r] =Fetch
   [=] >payload >> >data
   [!] !FetchError
      [r] =LogError
         [=] <msg << "fetch failed"
      [*] *Continue >IsFailed >> $fetchFailed
[?] $fetchFailed =? true
   [r] =HandleMissing
[?] *?
   [r] =Process
      [=] <input << >data
```

Four patterns for error handling:

| Pattern | Pipeline continues? | Variable state |
|---------|-------------------|---------------|
| `[!]` pushes replacement (`<<`/`>>`) | Yes | Always Final |
| `[!]` without replacement (default) | No — ends on error | Never Failed |
| `[!]` with `[*] *Continue >IsFailed >> $var` | Yes | May be Failed — handle via `$var` boolean |
| `[>] <!` fallback on IO line | Yes | Always Final — fallback value used |

If the compiler cannot guarantee the `>IsFailed` output is handled, it emits PGW-205.

## Chain Error Addressing

In chain execution (`[r] =A >> =B >> =C`), errors are prefixed with a step reference:

**Prefer numeric indices** — always unambiguous:

```polyglot
[r] =File.Text.Read >> =Text.Parse.CSV
   [=] >0.path;path << $path
   [=] <1.rows;string >> >content
   [!] !0.File.NotFound
      [r] >content << "Error: file not found"
   [!] !1.Parse.InvalidFormat
      [r] >content << "Error: invalid CSV"
```

**Leaf name ambiguity:** When a leaf name shares a segment with the error name, extend the step reference by one level up to disambiguate:

```polyglot
[ ] Ambiguous — "Read" + "File.NotFound" looks like step "Read.File"
[!] !Read.File.NotFound

[ ] Unambiguous — extend step ref to "Text.Read"
[!] !Text.Read.File.NotFound

[ ] Always safe — numeric index
[!] !0.File.NotFound
```

See [[pipelines#Error Handling in Chains]] for the full chain execution context.

## Standard Error Trees

Every pipeline exposes an error tree — a structured list of every error it can raise. The stdlib defines four root namespaces:

| Namespace | Covers |
|-----------|--------|
| `!File` | File system operations (NotFound, ReadError, WriteError, ...) |
| `!No` | Missing resource errors (No.Input, No.Connection, ...) |
| `!Timeout` | Operation timeouts (Timeout.Connection, Timeout.Read, ...) |
| `!Validation` | Data validation failures |

See [[stdlib/errors/errors]] for the complete error tree listings.

## Failed State

When a pipeline responsible for producing a variable's value terminates with an error, that variable enters the **Failed** stage (see [[variable-lifecycle#Failed]]). A failed variable:

- Will **never resolve** — it cannot transition to any other stage
- Causes downstream pipelines waiting on it to **not fire** (IO implicit gate)
- Has its `live` metadata frozen and accessible in `[!]` error handlers

Query a variable's state via `$varName%state` — this reads from `%$:{name}:{instance}.state` in the metadata tree. The `#VarState` enum includes: Declared, Default, Final, Failed, Released. See [[metadata#Variable (`$`)]].

## Error Fallback Operators

<!-- @operators -->
<!-- @io:Fallback IO -->
<!-- @blocks:Data Flow -->
The `<!` and `!>` operators (see [[operators#Assignment Operators]]) provide inline fallback values on IO lines, preventing variables from entering the Failed state. Fallback lines use the `[>]` / `[<]` block markers (see [[blocks#Data Flow]]) scoped under `[=]` IO lines (see [[io#Fallback IO]]).

### Generic Fallback

A `[>] <! value` line catches **any** error not handled by an `[!]` block:

```polyglot
[r] =File.Text.Read
   [=] <path << $file
   [=] >content >> $out
      [>] <! "generic fallback"
```

If `=File.Text.Read` errors (any error), `$out` becomes Final with `"generic fallback"` instead of entering the Failed state.

### Error-Specific Fallback

`<!Error.Name` fuses the error name into the operator, providing a fallback only for that specific error:

```polyglot
[r] =File.Text.Read
   [=] <path << $file
   [=] >content >> $out
      [>] <! "generic fallback"
      [>] <!File.NotFound "file not found"
      [>] <!File.ReadError "read error"
```

Error-specific fallbacks take priority over the generic fallback.

### Fallback Values

Fallback accepts any `value_expr` — not just literals:

```polyglot
[=] >profile >> $profile
   [>] <! $defaultProfile
   [>] <! =LoadCached"{$userId}"
```

(Only ONE of the above per output — duplicates are PGE-703.)

### Precedence: `[!]` Before `<!`

When both `[!]` blocks and `<!` fallback exist on the same pipeline call:

1. Pipeline call errors
2. `[!]` blocks check — if a matching `[!]` exists, its body runs first
3. If `[!]` pushed a replacement value → variable is Final, done
4. If `[!]` did NOT push a replacement (or no `[!]` matched):
   - Error-specific `<!Error.Name` on `[>]` line → variable is Final with that value
   - Generic `<!` on `[>]` line → variable is Final with that value
   - No fallback exists → existing behavior (pipeline terminates or variable is Failed)
5. When any fallback activates: `$var%sourceError` is set to the error that occurred

```polyglot
[r] =File.Text.Read
   [=] <path << $file
   [=] >content >> $out
      [>] <! "last resort"
   [!] !File.NotFound
      [ ] Complex recovery — [!] handles this fully
      [r] =LogMissing
         [=] <path << $file
      [r] >content << "logged and handled"
   [!] !File.ReadError
      [ ] Simple fallback inside [!]
      [=] >content <! "read error"
```

Here `!File.NotFound` is fully handled by `[!]` (it pushes a replacement). `!File.ReadError` uses `<!` inside its `[!]` block. Any other error falls through to the generic `[>] <! "last resort"`.

### Metadata Exposure

When a fallback activates, the error that triggered it is accessible via `$var%sourceError` (`;live.error`). If no error occurred, `%sourceError` is `!NoError`. See [[metadata#Variable (`$`)]].

```polyglot
[?] $content%sourceError =!? !NoError
   [r] =LogWarning
      [=] <msg << "Used fallback for {$file}: {$content%sourceError}"
[?] *?
   [ ] Normal path — no error occurred
```

### Compiler Rules

- **PGE-703** — duplicate `<!` on same output for same error (or duplicate generic). See [[compile-rules/PGE/PGE-703-duplicate-fallback-assignment]].
