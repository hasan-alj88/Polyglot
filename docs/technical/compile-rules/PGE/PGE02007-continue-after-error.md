---
audience: developer
rule: "2.7"
code: PGE02007
name: Continue After Error
severity: error
split_from: PGE02005
---

### Rule 2.7 — Continue After Error
`PGE02007`

**Statement:** `*Continue` is a collector inside an `[!]` error block that provides a fallback value when an error occurs, allowing the pipeline to continue instead of terminating. Syntax: `[*] *Continue >FallBack << "value"`. The `>FallBack` value is pushed into the output variable as **Final** state — downstream pipelines trigger normally. The variable state is the flow control: Final (from success or `>FallBack`) triggers downstream, Failed (no handler) blocks downstream via IO implicit gate.
**Rationale:** The default `[!]` behavior terminates the pipeline on error (see PGE02005). `*Continue` provides an explicit opt-in to keep the pipeline running by supplying a known-good fallback value. No boolean flag is needed — the variable's lifecycle state (Final vs Failed) naturally controls whether downstream pipelines fire.
**Detection:** At compile time: validate that `[*] *Continue` appears only inside `[!]` blocks, and that `>FallBack` is provided with a value of the correct type for the output variable.

**VALID:**
```polyglot
[ ] ✓ *Continue with >FallBack — variable becomes Final, pipeline continues
[=] >data#string
[r] =Fetch
   [=] >payload >> >data
   [!] !FetchError
      [r] =LogError
         [=] <msg << "fetch failed"
      [*] *Continue >FallBack << ""
[r] =Process
   [=] <input << >data            [ ] ✓ >data is Final (success value or >FallBack "")
```

```polyglot
[ ] ✓ *Continue with typed >FallBack — int fallback
[=] >count#int
[r] =CountItems
   [=] >total >> >count
   [!] !CountError
      [*] *Continue >FallBack << 0
[r] =Report
   [=] <count << >count           [ ] ✓ >count is Final (actual count or 0)
```

```polyglot
[ ] ✓ *Continue with >FallBack from error handler pipeline
[=] >config#string
[r] =LoadConfig
   [=] >payload >> >config
   [!] !ConfigError
      [r] =LoadDefaultConfig
         [=] >default >> $defaultCfg
      [*] *Continue >FallBack << $defaultCfg
[r] =ApplyConfig
   [=] <cfg << >config            [ ] ✓ >config is Final (loaded or default)
```

**INVALID:**
```polyglot
[ ] ✗ PGE02007 — *Continue outside [!] block
[=] >data#string
[r] =Fetch
   [=] >payload >> >data
[*] *Continue >FallBack << ""     [ ] ✗ PGE02007 — *Continue only valid inside [!]
```

```polyglot
[ ] ✗ PGE02007 — *Continue without >FallBack value
[=] >data#string
[r] =Fetch
   [=] >payload >> >data
   [!] !FetchError
      [*] *Continue               [ ] ✗ PGE02007 — must provide >FallBack
```

**See also:**
- [PGE02005 — Failed Is Terminal](PGE02005-failed-is-terminal.md) — core Failed state semantics
- [PGW02004 — Pipeline Terminates on Error](../PGW/PGW02004-failed-variable-usage.md) — warning when no recovery is provided

### See Also

- [[stdlib/collectors/Continue|*Continue Collector]] — user-facing documentation for `*Continue` with usage examples
