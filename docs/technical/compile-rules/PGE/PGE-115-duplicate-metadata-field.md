---
rule: "1.15"
code: PGE-115
name: Duplicate Metadata Field
severity: error
---

### Rule 1.15 — Duplicate Metadata Field
`PGE-115`

**Statement:** Each fixed metadata field (`.description`, `.version`, `.authors`, `.license`, `.deprecated`, `.deprecatedMessage`) must appear at most once within a single `[%]` section. Duplicate fixed field declarations are a compile error. Note: `%alias` is a flexible field (not fixed) and is covered by PGE-1002 instead.
**Rationale:** Fixed metadata fields have singular semantics — a pipeline has one description, one version, one license. Two `.description` lines create ambiguity about which value is authoritative. This is analogous to PGE-413 (duplicate data field name) and PGE-111 (duplicate IO parameter name).
**Detection:** The compiler collects all fixed-field names within each `[%]` section and rejects any name that appears more than once. Flexible `:info` keys are checked separately — duplicate keys under `:info` are also rejected.

**See also:** PGE-413 (duplicate data field name — analogous rule for `{#}` fields), PGE-111 (duplicate IO parameter name — analogous rule for pipeline IO), PGE-1001 (undefined metadata field access), PGE-1002 (duplicate alias)

**VALID:**
```polyglot
[ ] ✓ each fixed field appears once
{=} =Payments
   [%] .description << "Processes payments"
   [%] .version << "1.0.0"
   [%] .authors << ["Alice", "Bob"]
   [%] .license << "MIT"
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out#string
   [r] >out << "done"
```

```polyglot
[ ] ✓ flexible :info fields can have multiple distinct keys
{=} =Tagged
   [%] .description << "Tagged pipeline"
   [%] :info
      :team << "platform"
      :priority << "high"              [ ] ✓ different keys under :info
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out#string
   [r] >out << "done"
```

```polyglot
[ ] ✓ same field name in different definitions is fine
{#} #Invoice
   [%] .description << "Invoice record"
   [.] .amount#float

{#} #Payment
   [%] .description << "Payment record" [ ] ✓ different definitions
   [.] .total#float
```

**INVALID:**
```polyglot
[ ] ✗ PGE-115 — duplicate .description
{=} =BadMeta
   [%] .description << "First description"
   [%] .description << "Second description"  [ ] ✗ PGE-115 — .description appears twice
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out#string
   [r] >out << "done"
```

```polyglot
[ ] ✗ PGE-115 — duplicate .version
{=} =AlsoBad
   [%] .version << "1.0.0"
   [%] .version << "2.0.0"              [ ] ✗ PGE-115 — .version appears twice
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out#string
   [r] >out << "done"
```

```polyglot
[ ] ✗ PGE-115 — duplicate .deprecated
{#} #OldRecord
   [%] .deprecated << .True
   [%] .deprecated << .False             [ ] ✗ PGE-115 — .deprecated appears twice
   [.] .name#string
```

**Open point:** None.

### See Also

- [[concepts/pipelines/INDEX|Pipelines Overview]] — documents metadata uniqueness constraint, references PGE-115
- [[concepts/pipelines/inline-calls|Inline Calls]] — compile rule quick-reference table includes PGE-115
