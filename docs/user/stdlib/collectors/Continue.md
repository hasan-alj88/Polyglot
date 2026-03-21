---
audience: user
type: specification
updated: 2026-03-20
status: draft
---

# *Continue — Error Recovery

Used inside `[!]` error blocks to continue the pipeline with a fallback value. Without `*Continue`, an `[!]` block terminates the pipeline on error (PGE-205).

No `[@]` import needed.

```
*Continue
   >FallBack           (required — the fallback value)
   >IsFailed           (optional — boolean flag for downstream checks)
```

## Usage: >FallBack

Provides a known-good fallback value. The failed variable becomes Final with the fallback value. Pipeline continues.

```polyglot
[r] =Fetch
   [=] >payload >> >data
   [!] !FetchError
      [*] *Continue >FallBack << ""
```

## Usage: >IsFailed

Provides a boolean flag without replacing the failed value. Downstream can check the flag.

```polyglot
[r] =Fetch
   [=] >payload >> >data
   [!] !FetchError
      [*] *Continue >IsFailed >> $fetchFailed
```

## Rules

- `*Continue` is only valid inside `[!]` blocks (PGE-207)
- `>FallBack` must match the type of the failed variable
- When used for serial→struct conversion, `[!]` + `*Continue >FallBack` is mandatory if the compiler cannot prove the match (PGE-409)

See also: [PGE-207](../../technical/compile-rules/PGE/PGE-207-continue-after-error.md), [PGE-409](../../technical/compile-rules/PGE/PGE-409-unhandled-serial-struct-conversion.md)
