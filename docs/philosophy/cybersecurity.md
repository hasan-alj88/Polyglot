---
audience: [automation-builder, integrator, design]
type: reference
updated: 2026-04-20
---

<!-- @c:vision -->
<!-- @u:concepts/permissions -->
<!-- @u:concepts/permissions/enforcement -->
<!-- @u:concepts/permissions/foreign-code -->
<!-- @u:technical/spec/job-sandbox -->
# Cybersecurity

> Polyglot orchestrates code across language boundaries — including compiled binaries and foreign scripts that the compiler cannot fully analyse. That power demands a security model built on distrust, not convenience. This page defines how Polyglot treats security as a design principle. See [[vision]] for the broader project context.

## Zero Trust on Executed Code

Polyglot's security model starts from a single premise: **no code is trusted until proven safe, and no proof lasts forever.**

The compiler performs exhaustive static analysis — type safety, permission coverage, error handling, concurrency correctness. For Polyglot-native code, if it compiles, the permissions are satisfied. But the compiler is only one gate. The Polyglot Service enforces permissions independently at runtime, using OS-level mechanisms that operate below the language layer. Even if code somehow bypasses compile-time checks, the runtime rejects unauthorized actions.

This is not defense-in-depth as a buzzword — it is a structural property. The compiler and the runtime enforce the same permission model through independent mechanisms. The compiler validates statically; the runtime constrains dynamically. Neither trusts the other's verdict. A compiled binary that attempts to read a file outside its declared `{_}` grant is blocked by the kernel, regardless of what the compiler approved.

The principle extends to every integration boundary. Foreign code in `-Run.*` pipelines runs inside an OS-level sandbox — Linux namespaces, Landlock filesystem rules, seccomp syscall filters, cgroups resource limits — all derived from the same `{_}` permission objects the compiler validated. The sandbox does not ask the code to cooperate. It constrains the process at the kernel level, where the code has no choice.

This is system-enforced security, not honour-system security. See [[user/concepts/permissions/enforcement]] for the technical enforcement model and [[technical/spec/job-sandbox]] for the sandbox specification.

## No Permission Inheritance

Permissions in Polyglot are never inherited, never passed down, and never assumed. Every package, pipeline, and task declares its own permissions independently through explicit `{_}` grant objects.

The package ceiling (`{@}`) sets an upper bound on what any definition in the package may request. But a ceiling is a limit, not a grant. A package ceiling that allows file access does not mean any pipeline in the package *has* file access — each pipeline must explicitly declare the specific `{_}` grants it needs, and those grants must fall within the ceiling. No pipeline can piggyback on another's permissions or inherit access from its package.

This design eliminates an entire class of privilege escalation vulnerabilities. In systems where permissions propagate — where a child process inherits its parent's access, or a module inherits its package's grants — a single compromised component can leverage permissions it never needed. Polyglot prevents this structurally: every component's IO footprint is explicit, auditable, and independently declared.

The hierarchy exists for constraint validation — the compiler checks that every grant falls within its ceiling. "No inheritance" means grants are never automatic. This is a security feature: the hierarchy provides bounds, explicit declaration prevents silent privilege escalation.

See [[user/concepts/permissions/hierarchical-scoping]] for the scoping rules.

## Black Box Monitoring Is Mandatory

Polyglot cannot analyse what it cannot see. Compiled binaries and AST-invisible constructs are opaque — the compiler cannot verify their internal behaviour. A Go binary referenced by `-Run.Go.CLI` might do anything: read unexpected files, open network connections, fork child processes. The compiler sees the binary as a black box.

Polyglot's answer is not to ban black boxes — they are essential for integrating legacy code and compiled binaries. The answer is to **watch what they do.** OpenTelemetry tracing is built into the Polyglot runtime as mandatory security infrastructure, not optional telemetry. Every job execution, every permission check, every sandbox event is recorded:

- **Sandbox setup** — confirmation that all isolation layers initialised correctly before foreign code runs
- **Permission violations** — when the kernel blocks a syscall because the code exceeded its declared permissions
- **Resource consumption** — when cgroup limits are breached (CPU, memory, IO, processes, duration)
- **AST-invisible suppression** — when compile-time errors were downgraded to warnings because the code is opaque

These are not debug logs. They are the security audit trail. When a black box misbehaves, the OTel events record exactly what it attempted, which sandbox layer blocked it, and what permission category was missing. This data feeds into compliance reports that connect runtime behaviour back to the compile-time permission model.

The principle is straightforward: if Polyglot cannot verify code statically, it monitors code dynamically. Verification and monitoring are complementary, not alternatives.

See [[technical/spec/otel-permission-events]] for the 8 security events and [[technical/spec/otel-foundation]] for the tracing infrastructure.

## Black Box Trust Metric

Every black box in the Polyglot ecosystem is tracked. Compiled binaries via `-Run.*`, code using AST-invisible functions from the [[technical/compiler/ast-invisible-registry|banned registry]], and any pipeline running under `_Unsafe.SandboxOnly` — all accumulate a runtime history.

The metrics that matter are:

- **Permission breach attempts** — how often the sandbox blocks unauthorised operations. A binary that repeatedly attempts to read files outside its grant is not just failing — it is revealing intent that was not declared.
- **Error rates** — abnormal failure patterns that diverge from the binary's historical baseline. A sudden spike in errors may indicate a changed binary, a changed environment, or a changed adversary.
- **Resource consumption patterns** — consistent resource usage within limits is expected. Erratic spikes or sustained high consumption against cgroup limits suggest the code is doing more than declared.
- **Anomalous behaviour** — any deviation from the binary's established operational pattern. The first hundred runs build a baseline; deviations from that baseline are flagged.

These metrics are not punitive — they are informational. A black box that operates cleanly builds trust over time. A black box that triggers violations builds a trust deficit that operators can act on. The data is available in the runtime compliance appendix, connected to the compile-time compliance report, and queryable through the OTel pipeline.

The accountability chain (see [[philosophy/accountability]]) ensures that every black box has a named author and a named auditor recorded in its metadata. When trust metrics indicate a problem, the metadata identifies who is responsible. Sloppy audits have consequences because a name is attached.

---

## Related Philosophy

- [[philosophy/core-philosophy]] — Mind-shift, values, and evolution
- [[philosophy/language-design]] — Design principles and safety model
- [[philosophy/accountability]] — Human inspection and no dynamic code
- [[philosophy/error-philosophy]] — Murphy's Law and exhaustive error handling
