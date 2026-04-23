---
audience: developer
rule: "3.4"
code: PGE03004
name: Section-Boundary Pairing
severity: error
---

# Rule 3.4 — Section-Boundary Pairing
`PGE03004`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->

**Statement:** A `[=]` parallel and its `(*)` collector must be paired within valid section boundaries. Valid pairings: within the same section (`[\]` internal, body internal, `[/]` internal) and across `[\]`↔`[/]`. Invalid pairings: across `[\]`↔body or body↔`[/]` — because the execution body runs concurrently with in-flight `[\]` parallels, and `[/]` cleanup must not depend on body-scoped parallels still running.
**Rationale:** Setup (`[\]`) runs before the body, cleanup (`[/]`) runs after. A parallel started in `[\]` can be collected in `[/]` because the body has completed by then. But a parallel started in `[\]` cannot be collected in the body — the body may start before the parallel finishes. Similarly, a body parallel cannot be collected in `[/]` because the body's scope is closed.
**Detection:** The compiler maps each `[=]` to its containing section and each `(*)` to its containing section, then validates the pairing.

**VALID:**
```polyglot
[ ] ✓ [=] and (*) both in execution body
[=] -Fetch.A
   (-) >result >> $a

[=] -Fetch.B
   (-) >result >> $b

[*] *All
   (*) << $a
   (*) << $b
```

```polyglot
[ ] ✓ [=] in [\] setup, (*) in [/] cleanup
[\]
   [=] -Warmup.Cache
      (-) >status >> $cacheReady

[ ] execution body runs concurrently with Warmup.Cache
[-] -DoWork
   (-) <input << $data

[/]
   [*] *All
      (*) << $cacheReady             [ ] ✓ [\]↔[/] pairing is valid
```

```polyglot
[ ] ✓ [=] and (*) both within [\] — internal pairing
[\]
   [=] -Init.Config
      (-) >config >> $config

   [*] *All
      (*) << $config                 [ ] ✓ same-section pairing
   [-] -Init.WithConfig
      (-) <config << $config
```

**INVALID:**
```polyglot
[ ] ✗ PGE03004 — [=] in [\], (*) in body
[\]
   [=] -Warmup.Cache
      (-) >status >> $cacheReady

[ ] body — Warmup.Cache may still be running
[*] *All
   (*) << $cacheReady                   [ ] ✗ PGE03004 — [\]↔body pairing not allowed
```

```polyglot
[ ] ✗ PGE03004 — [=] in body, (*) in [/]
[=] -Fetch.Data
   (-) >result >> $data

[/]
   [*] *All
      (*) << $data                   [ ] ✗ PGE03004 — body↔[/] pairing not allowed
```

**Note:** `[\]` setup and `[/]` cleanup form one scope with a mandatory sequential body execution between them. Parallel branches launched in either section must be collected somewhere within that scope (setup, cleanup, or `[\]`↔`[/]` pairing) before the scope ends — otherwise it's a compile error. Same-section internal pairing (e.g., `[=]` and `(*)` both in `[\]`) is valid. See resolved design issue 004 (git history: `docs/technical/compiler_issues/004-setup-internal-parallel-collection.md`).

## See Also

- [[concepts/collections/collect|Collect]] — documents section-boundary pairing constraint (references PGE03004)
- [[concepts/collections/examples|Collection Examples]] — error table mapping PGE03004 to example scenarios
