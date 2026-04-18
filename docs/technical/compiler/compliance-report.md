---
audience: developer
type: specification
status: complete
updated: 2026-04-18
---

# Compliance Report

<!-- @u:concepts/permissions/enforcement -->
<!-- @c:technical/algorithms/foreign-code-analysis -->
<!-- @c:technical/spec/otel-permission-events -->
<!-- @c:technical/spec/otel-foundation -->
<!-- @c:technical/spec/otel-config -->

The compiler generates a compliance report as part of the Behavior Contract. This document specifies the report structure, verdict types, and privacy considerations.

## Overview

After analyzing all foreign code in a package, the compiler produces a compliance report summarizing the IO analysis results for each pipeline. The report is emitted alongside the Behavior Contract and informs runtime sandbox configuration.

## Report Structure

Each pipeline with foreign code gets a per-pipeline section:

```text
Pipeline: -ProcessData
Permissions: _DataRead (#File.#Read, scope: /data/reports/*)
Foreign code: -Run.Python.Script (inline, 12 lines)
Analysis:
  | Call | Category | Resource | Verdict |
  |------|----------|----------|---------|
  | pandas.read_csv | #File.#Read | "/data/reports/q1.csv" | PASS |
  | builtins.open | #File.#Read | "/etc/shadow" | FAIL (PGE10013) |
  | custom_lib.process | unknown | — | UNKNOWN (PGW10005) |
Warnings: 1
Errors: 1
```

## Verdict Types

| Verdict | Meaning | Compile Rule |
|---------|---------|-------------|
| PASS | Resource within declared `{_}` scope | — |
| FAIL | Resource definitively outside scope | PGE10013 |
| UNVERIFIABLE | IO detected but resource not fully resolvable | PGW10002 |
| UNKNOWN | Function not in sink table or known-pure list | PGW10005 |
| OPAQUE | Pipeline uses `-Run.*.Bind` mode — no analysis possible | PGW10003 |
| BANNED | AST-invisible construct detected | PGE10014 |

**Note:** FAIL and BANNED verdicts halt compilation. UNVERIFIABLE, UNKNOWN, and OPAQUE are warnings — the pipeline compiles but the report documents the gap for auditing.

## Privacy Considerations

The compliance report must not leak sensitive information:

- **No source code** — reports contain call signatures and verdicts only, never the foreign code itself
- **No credential values** — `.credentials` paths appear as file paths, not their contents
- **Path patterns only** — `.scope` patterns are shown, not the full list of matched files
- **No variable values** — when `trace_assignment` resolves a variable to a literal, the report shows the literal path but not intermediate variable names or assignments

## Integration with Behavior Contract

The compliance report is a section within the Behavior Contract:

1. **Compile-time** — the report is generated during compilation and embedded in the contract
2. **Sandbox derivation** — runtime sandbox restrictions (Landlock/seccomp) are derived from the same `{_}` permission data that the report validates
3. **Audit trail** — the report provides evidence that compile-time analysis was performed, documenting what was verified and what relies on sandbox enforcement

## Example Report

```text
═══════════════════════════════════════
COMPLIANCE REPORT — @MyCompany::DataPipeline<1.0.0
═══════════════════════════════════════

Pipeline: -ProcessData
  Permissions: _DataRead (#File.#Read, /data/reports/*)
  Foreign code: -Run.Python.Script;PythonUV (file: /scripts/etl/transform.py)
  Calls analyzed: 4
  ┌──────────────────────┬────────────┬──────────────────────┬──────────────┐
  │ Call                 │ Category   │ Resource             │ Verdict      │
  ├──────────────────────┼────────────┼──────────────────────┼──────────────┤
  │ pandas.read_csv      │ #File.#Read│ /data/reports/q1.csv │ PASS         │
  │ pandas.read_csv      │ #File.#Read│ /data/reports/q2.csv │ PASS         │
  │ json.loads           │ (pure)     │ —                    │ (skipped)    │
  │ custom_lib.transform │ unknown    │ —                    │ UNKNOWN      │
  └──────────────────────┴────────────┴──────────────────────┴──────────────┘
  Result: PASS (1 warning)

Pipeline: -FetchExternal
  Permissions: _ApiAccess (#Web.#Request, api.internal.com:443)
  Foreign code: -Run.Python.Script;PythonUV (inline, 8 lines)
  Calls analyzed: 2
  ┌──────────────────────┬────────────┬──────────────────────┬──────────────┐
  │ Call                 │ Category   │ Resource             │ Verdict      │
  ├──────────────────────┼────────────┼──────────────────────┼──────────────┤
  │ requests.get         │ #Web       │ api.internal.com/data│ PASS         │
  │ requests.get         │ #Web       │ (variable: url)      │ UNVERIFIABLE │
  └──────────────────────┴────────────┴──────────────────────┴──────────────┘
  Result: PASS (1 warning)

Pipeline: -BindProcessor
  Permissions: _DataAccess (#File.#Read, /data/*)
  Foreign code: -Run.Python.Bind;PythonUV (file: /scripts/processor.py)
  Calls analyzed: 0 (opaque)
  Result: OPAQUE — sandbox enforcement only

═══════════════════════════════════════
Summary: 3 pipelines, 0 errors, 2 warnings, 1 opaque
═══════════════════════════════════════
```

## Runtime Appendix

The compile-time compliance report is static — it records AST analysis verdicts produced during compilation. At runtime, sandbox violations and resource limit breaches generate OTel events that the Runner collects into a runtime compliance appendix.

The runtime appendix extends the compile-time report with actual execution data:

- **Violation count and warning count** per job execution
- **Per-violation details:** timestamp, event type, job UID, structured attributes
- **Storage:** the runtime appendix is stored alongside the Behavior Contract in the NoSQL DB, keyed by job UID

The compile-time report remains immutable. The runtime appendix is a separate record that references the same pipeline and package, enabling operators to compare "what was analyzed" against "what actually happened."

See [[otel-permission-events]] for the full event format, attribute definitions, and appendix schema.

## Related

- [[algorithms/foreign-code-analysis]] — the algorithm that produces these verdicts
- [[permissions/enforcement]] — how the Behavior Contract uses compliance data
- [[otel-permission-events]] — runtime OTel events for sandbox and permission operations
- [[otel-foundation]] — tracing infrastructure, span hierarchy, NATS trace context propagation
- [[otel-config]] — exporter configuration, sampling strategy, fallback logging
- PGE10013, PGE10014, PGW10002, PGW10003, PGW10005 — compile rules that generate verdicts
