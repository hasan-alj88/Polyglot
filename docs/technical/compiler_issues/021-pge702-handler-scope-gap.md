---
issue: "021"
title: PGE-702 chain error handler scope not fully specified
related: PGE-702 (Rule 7.2), PGE-701 (Rule 7.1)
priority: enhancement
status: resolved
created: 2026-03-19
---

# 021 — PGE-702 chain error handler scope not fully specified

## Problem

PGE-702 (Chain Error Scoping) states that an error handler "sees only its step's IO." However, the rule doesn't explicitly address:

### 1. Prime pipeline variable access

Can a chain error handler read `$variables` from the enclosing pipeline scope?

```polyglot
{=} =Process
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   >path;path
   >fallbackPath;path
   [r] =File.Text.Read >> =Text.Parse.CSV
      [=] >0.path;path << $path
      [=] <1.rows;string >> >content
      [!] .0!File.NotFound
         [r] =File.Text.Read              [ ] can this handler read $fallbackPath?
            [=] <path << $fallbackPath
            [=] >content >> >content
```

### 2. Prime pipeline output writes

Can a chain error handler push to the enclosing pipeline's output ports directly?

The design principle "each chain step behaves as a separate `[r]` call" suggests handlers should have access to the enclosing scope (just as a standalone `[r]`'s `[!]` handler can access the pipeline scope), but this isn't stated.

## Affected Rules

- `compile-rules/PGE/PGE-702-chain-error-scoping.md`

## Proposed Resolution

Clarify in PGE-702 that:
1. Chain error handlers CAN read `$variables` from the enclosing pipeline (same as non-chain `[!]` handlers under PGE-701)
2. Chain error handlers CAN write to the chain's declared output variable (already stated — "provide replacement value")
3. Chain error handlers CANNOT write to intermediate step IO (they only see their own step)

Add a VALID example showing prime scope variable access in a handler.

## See also

- [PGE-702 — Chain Error Scoping](../compile-rules/PGE/PGE-702-chain-error-scoping.md)
- [PGE-701 — Error Block Scoping](../compile-rules/PGE/PGE-701-error-block-scoping.md)
