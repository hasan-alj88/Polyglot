---
audience: designer
type: reference
updated: 2026-03-30
---

<!-- @edge-cases/INDEX -->

## 21. Third Registry Type (S21)

### EC-21.1: `Registry` type — public/global registry address format

<!-- @identifiers -->
<!-- @packages -->
**EBNF:** `registry_type ::= "Local" | "Community" | "Registry"`

**What it tests:** Third registry type `Registry` uses a different ID format from `Local` (numeric) and `Community` (username). See [[identifiers]], [[packages]].

```polyglot
{@} @Local:001.App:v1.0.0
   [@] @Slack << @Community:polyglot-tools.SlackAdmin:v1.3.0
   [@] @Payments << @Registry:stripe.PaymentsAPI:v3.0.0
```
