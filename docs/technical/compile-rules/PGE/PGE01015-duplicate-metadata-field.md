---
rule: "1.15"
code: PGE01015
name: Duplicate Metadata Field
severity: error
---

### Rule 1.15 — Duplicate Metadata Field
`PGE01015`

**Statement:** Each fixed metadata field (`.description`, `.version`, `.authors`, `.license`, `.deprecated`, `.deprecatedMessage`) must appear at most once within a single `[%]` section. Duplicate fixed field declarations are a compile error. Note: `%alias` is a flexible field (not fixed) and is covered by PGE12002 instead.
**Rationale:** Fixed metadata fields have singular semantics — a pipeline has one description, one version, one license. Two `.description` lines create ambiguity about which value is authoritative. This is analogous to PGE05003 (duplicate data field name) and PGE01011 (duplicate IO parameter name).
**Detection:** The compiler collects all fixed-field names within each `[%]` section and rejects any name that appears more than once. Flexible `:info` keys are checked separately — duplicate keys under `:info` are also rejected.

**See also:** PGE05003 (duplicate data field name — analogous rule for `{#}` fields), PGE01011 (duplicate IO parameter name — analogous rule for pipeline IO), PGE12001 (undefined metadata field access), PGE12002 (duplicate alias)

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
[ ] ✗ PGE01015 — duplicate .description
{=} =BadMeta
   [%] .description << "First description"
   [%] .description << "Second description"  [ ] ✗ PGE01015 — .description appears twice
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out#string
   [r] >out << "done"
```

```polyglot
[ ] ✗ PGE01015 — duplicate .version
{=} =AlsoBad
   [%] .version << "1.0.0"
   [%] .version << "2.0.0"              [ ] ✗ PGE01015 — .version appears twice
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out#string
   [r] >out << "done"
```

```polyglot
[ ] ✗ PGE01015 — duplicate .deprecated
{#} #OldRecord
   [%] .deprecated << .True
   [%] .deprecated << .False             [ ] ✗ PGE01015 — .deprecated appears twice
   [.] .name#string
```

**Open point:** None.

### See Also

- [[concepts/pipelines/INDEX|Pipelines Overview]] — documents metadata uniqueness constraint, references PGE01015
- [[concepts/pipelines/inline-calls|Inline Calls]] — compile rule quick-reference table includes PGE01015
