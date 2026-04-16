---
audience: developer
rule: "7.9"
code: PGE07009
name: Unterminated Fallback Chain
severity: error
---

# Rule 7.9 — Unterminated Fallback Chain
`PGE07009`

<!-- @u:syntax/operators -->
<!-- @c:compile-rules/PGE/PGE07008-fallback-on-non-failable-source -->
<!-- @c:compile-rules/PGE/PGE07003-duplicate-fallback-assignment -->

**Statement:** A fallback chain — a sequence of `!<`/`!>` operators — must terminate at a non-failable expression (literal value or variable reference). If the chain ends at a pipeline call, `PGE07009` fires. There is no final recovery value if the last expression in the chain can itself fail.
**Rationale:** Fallback chains express progressive error recovery: "try pipeline A; if it fails, try pipeline B; if that fails, use literal C." The terminal expression must be something that cannot fail — otherwise the entire chain can exhaust all options and still produce no value. The compiler walks the chain and verifies the terminal is non-failable.
**Detection:** The compiler walks the assignment chain from left to right. Each `!<`/`!>` whose RHS is a pipeline call must itself have a subsequent fallback. The final RHS in the chain must be a literal or variable reference (non-failable). If the final RHS is a pipeline call, PGE07009 fires.

## Chain Structure

A fallback chain is syntactically a sequence of fallback operators on the same assignment:

```text
$target !< -Pipeline.A !< -Pipeline.B !< "literal-terminal"
         ↑               ↑               ↑
     failable         failable      non-failable (valid terminal)
```

The compiler validates:
1. Each intermediate `!<`/`!>` has a failable RHS (pipeline call) — valid, needs further fallback
2. The final `!<`/`!>` has a non-failable RHS (literal or variable) — valid terminal
3. If the final RHS is a pipeline call — **PGE07009**

## Diagnostic Format

`Fallback chain at line N does not terminate at a non-failable value — final expression '-PipelineName' can itself fail; add a literal or variable as the terminal fallback`

---

## Examples

### VALID

```polyglot
[ ] ✓ chain terminates at literal
[-] -File.Text.Read
   (-) <path << $primary
   (-) >content >> $data
      (<) !< -File.Text.Read"/backup.txt" !< "no content available"
```

```polyglot
[ ] ✓ chain terminates at variable
[-] -Fetch.Config
   (-) <url << $primaryUrl
   (-) >config >> $cfg
      (<) !< -Fetch.Config"/fallback" !< $defaultConfig
```

```polyglot
[ ] ✓ single fallback to literal — trivially terminated
[-] -File.Text.Read
   (-) <path << $path
   (-) >content >> $data
      (<) !< "unavailable"
```

### INVALID

```polyglot
[ ] ✗ PGE07009 — chain ends at pipeline call
[-] -File.Text.Read
   (-) <path << $primary
   (-) >content >> $data
      (<) !< -File.Text.Read"/backup.txt" !< -File.Text.Read"/last-resort.txt"
```

```polyglot
[ ] ✗ PGE07009 — single fallback to pipeline with no further fallback
[-] -Fetch.Config
   (-) <url << $primaryUrl
   (-) >config >> $cfg
      (<) !< -Fetch.Config"/fallback"
```

### See Also

- [[compile-rules/PGE/PGE07008-fallback-on-non-failable-source|PGE07008]] — fallback on non-failable source (the per-expression rule)
- [[compile-rules/PGE/PGE07003-duplicate-fallback-assignment|PGE07003]] — duplicate fallback on same output
- [[compile-rules/PGE/PGE07007-error-handling-must-be-exhaustive|PGE07007]] — exhaustive error handling (the broader principle)
