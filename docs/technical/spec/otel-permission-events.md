---
audience: design
type: spec
status: complete
updated: 2026-04-18
---

# OTel Permission and Sandbox Events

<!-- @c:technical/spec/job-sandbox -->
<!-- @c:technical/compiler/compliance-report -->
<!-- @c:technical/spec/behavior-contract#Permission Manifest -->
<!-- @c:technical/plan/queue-manager/resource-controls -->
<!-- @c:technical/spec/otel-foundation -->
<!-- @c:technical/spec/otel-config -->
<!-- @u:concepts/permissions/enforcement -->
<!-- @u:concepts/permissions/foreign-code -->
Related: [[job-sandbox]], [[compliance-report]], [[behavior-contract]], [[resource-controls]], [[enforcement]], [[foreign-code]], [[otel-foundation]], [[otel-config]]

This specification defines the 8 OpenTelemetry log events emitted by the Runner and Queue Handler for permission and sandbox operations. It covers event names, trigger conditions, severity levels, structured attributes, span hierarchy, and runtime compliance report integration.

**Scope boundary:** This document defines *what* is logged and *when*. Where logs go (OTLP endpoint, stdout, JSON files) is configured by the user in [[otel-config]]. The tracing infrastructure (crate stack, span hierarchy, NATS propagation) is defined in [[otel-foundation]]. The Runner emits OTel data; the exporter routes it. This spec is exporter-independent.

## Event Summary

| Event Name | Severity | Emitter | When |
|---|---|---|---|
| `permission.sandbox.setup` | INFO | Runner | Sandbox layers configured successfully before foreign code execution |
| `permission.sandbox.setup_failed` | CRITICAL | Runner | One or more sandbox layers failed to initialize |
| `permission.sandbox.violation` | ERROR | Runner | Kernel blocks a syscall (EACCES from Landlock, EPERM from seccomp) |
| `permission.resource.exceeded` | WARN/ERROR | Queue Handler | cgroup resource limit breached — WARN for throttle, ERROR for kill |
| `permission.resource.kill` | ERROR | Queue Handler | Job killed after exceeding resource limit and grace period |
| `permission.ast.suppressed` | WARN | Runner | AST-invisible error (PGE10014) suppressed to warning under `_Unsafe.SandboxOnly` |
| `permission.opaque.activated` | WARN | Runner | `_Unsafe.SandboxOnly` activated — maximum containment mode engaged |
| `permission.compliance.generated` | INFO | Runner | Runtime compliance appendix generated at job completion |

## Attribute Registry

All attributes use the `aljam3.*` namespace following OTel semantic conventions.

| Attribute | Type | Description | Example |
|---|---|---|---|
| `aljam3.job.uid` | string | Unique job identifier assigned by the Trigger Monitor | `"job-a1b2c3d4"` |
| `aljam3.pipeline.name` | string | Pipeline name including prefix | `"-ProcessData"` |
| `aljam3.package.name` | string | Fully qualified package address | `"@MyCompany::DataPipeline<1.0.0"` |
| `aljam3.permission.category` | string | `{_}` permission category involved | `"#File.#Read"` |
| `aljam3.sandbox.layer` | string | Sandbox enforcement layer that acted | `"landlock"`, `"seccomp"`, `"namespace"`, `"cgroup"` |
| `aljam3.sandbox.syscall` | string | Blocked or intercepted syscall name | `"open"`, `"connect"`, `"fork"` |
| `aljam3.sandbox.resource` | string | Attempted resource path, host\:port, or byte value | `"/etc/shadow"`, `"evil.com:443"`, `"1073741824"` |
| `aljam3.sandbox.action` | string | Enforcement action taken | `"blocked"`, `"#Kill"`, `"#Throttle"`, `"#Retry"` |
| `aljam3.sandbox.opaque` | bool | Whether `_Unsafe.SandboxOnly` is active for this job | `true` |

## Event Specifications

### permission.sandbox.setup

**Trigger:** Runner completes sandbox configuration (namespaces, Landlock, seccomp, cgroups) before calling `exec()` on the foreign code process.

**Severity:** INFO

**Rationale:** Normal operational event. Confirms the sandbox is correctly applied for auditing and debugging.

**Required attributes:**

| Attribute | Source |
|---|---|
| `aljam3.job.uid` | Trigger Monitor assignment |
| `aljam3.pipeline.name` | Behavior Contract |
| `aljam3.package.name` | Behavior Contract |
| `aljam3.sandbox.opaque` | Permission Manifest |

**Optional attributes:**

| Attribute | When present |
|---|---|
| `aljam3.sandbox.layer` | One event per layer successfully initialized (multiple events possible) |

**Parent span:** Sandbox Setup Span (child of Job Span)

**Example OTel log record:**

```json
{
  "Timestamp": "2026-04-18T14:30:01.123Z",
  "SeverityText": "INFO",
  "Body": "Sandbox setup complete for -ProcessData",
  "Attributes": {
    "aljam3.job.uid": "job-a1b2c3d4",
    "aljam3.pipeline.name": "-ProcessData",
    "aljam3.package.name": "@MyCompany::DataPipeline<1.0.0",
    "aljam3.sandbox.opaque": false
  },
  "TraceId": "abc123...",
  "SpanId": "setup-span-456"
}
```

### permission.sandbox.setup\_failed

**Trigger:** One or more sandbox layers fail to initialize. The Runner cannot safely spawn the foreign code process. The job enters Failed state without executing.

**Severity:** CRITICAL

**Rationale:** Sandbox setup failure means the job runs without protection or does not run at all. This is a system-level failure requiring operator attention.

**Required attributes:**

| Attribute | Source |
|---|---|
| `aljam3.job.uid` | Trigger Monitor assignment |
| `aljam3.pipeline.name` | Behavior Contract |
| `aljam3.package.name` | Behavior Contract |
| `aljam3.sandbox.layer` | Which layer failed |

**Optional attributes:**

| Attribute | When present |
|---|---|
| `aljam3.sandbox.resource` | Path or resource that caused the failure (e.g., missing mount target) |

**Parent span:** Sandbox Setup Span (child of Job Span)

**Example OTel log record:**

```json
{
  "Timestamp": "2026-04-18T14:30:01.456Z",
  "SeverityText": "CRITICAL",
  "Body": "Sandbox setup failed: Landlock ruleset creation returned ENOSYS",
  "Attributes": {
    "aljam3.job.uid": "job-a1b2c3d4",
    "aljam3.pipeline.name": "-ProcessData",
    "aljam3.package.name": "@MyCompany::DataPipeline<1.0.0",
    "aljam3.sandbox.layer": "landlock",
    "aljam3.sandbox.resource": "/data/reports"
  },
  "TraceId": "abc123...",
  "SpanId": "setup-span-456"
}
```

### permission.sandbox.violation

**Trigger:** The kernel blocks a syscall during foreign code execution. The Runner detects this through the job's exit status or via seccomp USER\_NOTIF supervisor interception.

**Severity:** ERROR

**Rationale:** A violation means the foreign code attempted an operation outside its declared permissions. This is a security event.

**Required attributes:**

| Attribute | Source |
|---|---|
| `aljam3.job.uid` | Trigger Monitor assignment |
| `aljam3.pipeline.name` | Behavior Contract |
| `aljam3.package.name` | Behavior Contract |
| `aljam3.sandbox.layer` | Which enforcement layer blocked the call |
| `aljam3.sandbox.syscall` | Blocked syscall name |
| `aljam3.sandbox.resource` | Attempted resource |
| `aljam3.permission.category` | Which `{_}` category would have been needed |

**Optional attributes:**

| Attribute | When present |
|---|---|
| `aljam3.sandbox.opaque` | Always present; highlights opaque vs analyzed code |
| `aljam3.sandbox.action` | Always `"blocked"` for violations |

**Parent span:** Foreign Code Execution Span (child of Job Span)

**Example OTel log record:**

```json
{
  "Timestamp": "2026-04-18T14:30:05.789Z",
  "SeverityText": "ERROR",
  "Body": "Sandbox violation: open(/etc/shadow) blocked by Landlock — missing #File.#Read for /etc/shadow",
  "Attributes": {
    "aljam3.job.uid": "job-a1b2c3d4",
    "aljam3.pipeline.name": "-ProcessData",
    "aljam3.package.name": "@MyCompany::DataPipeline<1.0.0",
    "aljam3.sandbox.layer": "landlock",
    "aljam3.sandbox.syscall": "open",
    "aljam3.sandbox.resource": "/etc/shadow",
    "aljam3.permission.category": "#File.#Read",
    "aljam3.sandbox.action": "blocked",
    "aljam3.sandbox.opaque": false
  },
  "TraceId": "abc123...",
  "SpanId": "exec-span-789"
}
```

### permission.resource.exceeded

**Trigger:** A cgroup resource limit is breached. The Queue Handler detects this through cgroup event notifications or polling.

**Severity:** WARN when the action is throttle; ERROR when the action is kill.

**Rationale:** WARN for throttle because the job continues with degraded performance. ERROR for kill because the job terminates.

**Required attributes:**

| Attribute | Source |
|---|---|
| `aljam3.job.uid` | Trigger Monitor assignment |
| `aljam3.pipeline.name` | Behavior Contract |
| `aljam3.package.name` | Behavior Contract |
| `aljam3.permission.category` | Resource category exceeded (e.g., `#RAM.#Limit`, `#CPU.#Limit`) |
| `aljam3.sandbox.resource` | Current value that exceeded the limit |
| `aljam3.sandbox.action` | `"#Throttle"` or `"#Kill"` |

**Optional attributes:**

| Attribute | When present |
|---|---|
| `aljam3.sandbox.layer` | Always `"cgroup"` for resource events |

**Parent span:** Foreign Code Execution Span (child of Job Span)

**Example OTel log record:**

```json
{
  "Timestamp": "2026-04-18T14:31:12.345Z",
  "SeverityText": "WARN",
  "Body": "Resource limit exceeded: CPU throttled for -ProcessData",
  "Attributes": {
    "aljam3.job.uid": "job-a1b2c3d4",
    "aljam3.pipeline.name": "-ProcessData",
    "aljam3.package.name": "@MyCompany::DataPipeline<1.0.0",
    "aljam3.permission.category": "#CPU.#Limit",
    "aljam3.sandbox.resource": "2.5 cores",
    "aljam3.sandbox.action": "#Throttle",
    "aljam3.sandbox.layer": "cgroup"
  },
  "TraceId": "abc123...",
  "SpanId": "exec-span-789"
}
```

### permission.resource.kill

**Trigger:** The Queue Handler kills a job after a resource limit is exceeded and the grace period (from `#Duration.#Limit` or `{Q}` configuration) expires. The process receives SIGTERM followed by SIGKILL.

**Severity:** ERROR

**Rationale:** The job is terminated. This is an error condition requiring investigation.

**Required attributes:**

| Attribute | Source |
|---|---|
| `aljam3.job.uid` | Trigger Monitor assignment |
| `aljam3.pipeline.name` | Behavior Contract |
| `aljam3.package.name` | Behavior Contract |
| `aljam3.permission.category` | Resource category that triggered the kill |
| `aljam3.sandbox.resource` | Value at time of kill |
| `aljam3.sandbox.action` | `"#Kill"` |

**Optional attributes:**

| Attribute | When present |
|---|---|
| `aljam3.sandbox.layer` | Always `"cgroup"` |

**Parent span:** Job Span (directly, since this terminates the job)

**Example OTel log record:**

```json
{
  "Timestamp": "2026-04-18T14:31:45.678Z",
  "SeverityText": "ERROR",
  "Body": "Job killed: -ProcessData exceeded #RAM.#Limit (512MB, used 1073741824 bytes)",
  "Attributes": {
    "aljam3.job.uid": "job-a1b2c3d4",
    "aljam3.pipeline.name": "-ProcessData",
    "aljam3.package.name": "@MyCompany::DataPipeline<1.0.0",
    "aljam3.permission.category": "#RAM.#Limit",
    "aljam3.sandbox.resource": "1073741824",
    "aljam3.sandbox.action": "#Kill",
    "aljam3.sandbox.layer": "cgroup"
  },
  "TraceId": "abc123...",
  "SpanId": "job-span-123"
}
```

### permission.ast.suppressed

**Trigger:** The compiler detects an AST-invisible construct (eval, exec, dynamic import) that would normally produce PGE10014, but `_Unsafe.SandboxOnly` is active, so the error is suppressed to a warning. This event fires at job startup when the Runner loads the compliance report.

**Severity:** WARN

**Rationale:** The suppressed error means the sandbox is the sole enforcement mechanism for this construct. Operators need visibility into what was suppressed.

**Required attributes:**

| Attribute | Source |
|---|---|
| `aljam3.job.uid` | Trigger Monitor assignment |
| `aljam3.pipeline.name` | Behavior Contract |
| `aljam3.package.name` | Behavior Contract |
| `aljam3.sandbox.opaque` | Always `true` (suppression only happens under opaque mode) |

**Optional attributes:**

| Attribute | When present |
|---|---|
| `aljam3.permission.category` | Category of the suppressed violation, if determinable |

**Parent span:** Foreign Code Execution Span (child of Job Span)

**Example OTel log record:**

```json
{
  "Timestamp": "2026-04-18T14:30:02.000Z",
  "SeverityText": "WARN",
  "Body": "AST-invisible error suppressed: PGE10014 (eval detected) — _Unsafe.SandboxOnly active",
  "Attributes": {
    "aljam3.job.uid": "job-e5f6g7h8",
    "aljam3.pipeline.name": "-LegacyProcessor",
    "aljam3.package.name": "@MyCompany::Legacy<2.0.0",
    "aljam3.sandbox.opaque": true
  },
  "TraceId": "def456...",
  "SpanId": "exec-span-012"
}
```

### permission.opaque.activated

**Trigger:** The Runner activates maximum containment mode because the pipeline declares `_Unsafe.SandboxOnly`. This fires after sandbox setup, before foreign code execution begins.

**Severity:** WARN

**Rationale:** Opaque execution is a security-relevant state. All isolation layers activate, and AST analysis is best-effort only. Operators must be aware.

**Required attributes:**

| Attribute | Source |
|---|---|
| `aljam3.job.uid` | Trigger Monitor assignment |
| `aljam3.pipeline.name` | Behavior Contract |
| `aljam3.package.name` | Behavior Contract |
| `aljam3.sandbox.opaque` | Always `true` |

**Parent span:** Sandbox Setup Span (child of Job Span)

**Example OTel log record:**

```json
{
  "Timestamp": "2026-04-18T14:30:01.500Z",
  "SeverityText": "WARN",
  "Body": "_Unsafe.SandboxOnly activated for -LegacyProcessor — maximum containment engaged",
  "Attributes": {
    "aljam3.job.uid": "job-e5f6g7h8",
    "aljam3.pipeline.name": "-LegacyProcessor",
    "aljam3.package.name": "@MyCompany::Legacy<2.0.0",
    "aljam3.sandbox.opaque": true
  },
  "TraceId": "def456...",
  "SpanId": "setup-span-789"
}
```

### permission.compliance.generated

**Trigger:** The Runner generates the runtime compliance appendix at job completion. This happens regardless of whether violations occurred — the appendix records either a clean run or a list of violations.

**Severity:** INFO

**Rationale:** Normal operational event. The compliance appendix is generated for every job with foreign code.

**Required attributes:**

| Attribute | Source |
|---|---|
| `aljam3.job.uid` | Trigger Monitor assignment |
| `aljam3.pipeline.name` | Behavior Contract |
| `aljam3.package.name` | Behavior Contract |

**Optional attributes:**

| Attribute | When present |
|---|---|
| `aljam3.sandbox.opaque` | Present when opaque mode was active |

**Parent span:** Job Completion Span (child of Job Span)

**Example OTel log record:**

```json
{
  "Timestamp": "2026-04-18T14:32:00.000Z",
  "SeverityText": "INFO",
  "Body": "Runtime compliance appendix generated for -ProcessData (2 violations, 1 warning)",
  "Attributes": {
    "aljam3.job.uid": "job-a1b2c3d4",
    "aljam3.pipeline.name": "-ProcessData",
    "aljam3.package.name": "@MyCompany::DataPipeline<1.0.0"
  },
  "TraceId": "abc123...",
  "SpanId": "completion-span-999"
}
```

## Span Hierarchy

Sandbox-related spans nest inside the Job Span. The Job Span itself nests inside the Pipeline Span defined by #318 (OTel foundation).

```text
Pipeline Span (pipeline:{name})                          [#318 scope]
  └── Job Span (job:{UID})                               [#318 scope]
        ├── Sandbox Setup Span (permission.sandbox.setup)
        │     ├── permission.sandbox.setup (INFO)
        │     ├── permission.opaque.activated (WARN, if applicable)
        │     └── permission.sandbox.setup_failed (CRITICAL, if any)
        ├── Foreign Code Execution Span
        │     ├── permission.ast.suppressed (WARN, if applicable)
        │     ├── permission.sandbox.violation (ERROR, if any)
        │     └── permission.resource.exceeded (WARN/ERROR, if any)
        ├── permission.resource.kill (ERROR, if any — attaches to Job Span directly)
        └── Job Completion Span
              └── permission.compliance.generated (INFO)
```

The Sandbox Setup Span and Foreign Code Execution Span are children of the Job Span. Events attach to their parent span, providing temporal context for when in the job lifecycle each event occurred.

**Compatibility with #318:** The Pipeline Span and Job Span are defined by the OTel foundation specification (#318). This document defines only the sandbox-specific child spans and events. The Runner creates these child spans when executing foreign code jobs.

## Runtime Compliance Appendix Format

The compile-time [[compliance-report]] is static — it records what the compiler analyzed before execution. At runtime, the Runner appends violation data to produce a complete compliance picture.

### Appendix Structure

```json
{
  "runtime_appendix": {
    "job_uid": "job-a1b2c3d4",
    "pipeline": "-ProcessData",
    "package": "@MyCompany::DataPipeline<1.0.0",
    "timestamp_start": "2026-04-18T14:30:00.000Z",
    "timestamp_end": "2026-04-18T14:32:00.000Z",
    "violation_count": 2,
    "warning_count": 1,
    "violations": [
      {
        "timestamp": "2026-04-18T14:30:05.789Z",
        "event": "permission.sandbox.violation",
        "attributes": {
          "aljam3.sandbox.layer": "landlock",
          "aljam3.sandbox.syscall": "open",
          "aljam3.sandbox.resource": "/etc/shadow",
          "aljam3.permission.category": "#File.#Read",
          "aljam3.sandbox.action": "blocked"
        }
      },
      {
        "timestamp": "2026-04-18T14:31:45.678Z",
        "event": "permission.resource.kill",
        "attributes": {
          "aljam3.permission.category": "#RAM.#Limit",
          "aljam3.sandbox.resource": "1073741824",
          "aljam3.sandbox.action": "#Kill"
        }
      }
    ],
    "warnings": [
      {
        "timestamp": "2026-04-18T14:30:02.000Z",
        "event": "permission.ast.suppressed",
        "attributes": {
          "aljam3.sandbox.opaque": true
        }
      }
    ]
  }
}
```

### Storage

The runtime appendix is stored alongside the Behavior Contract in the NoSQL DB. Each job execution produces one appendix entry keyed by `job_uid`. The compile-time compliance report remains unchanged — the runtime appendix is a separate record that references the same pipeline and package.

### Relationship to Compile-Time Report

| Aspect | Compile-Time Report | Runtime Appendix |
|---|---|---|
| When produced | Compilation | Job completion |
| What it records | AST analysis verdicts (PASS, FAIL, UNKNOWN, etc.) | Actual sandbox events during execution |
| Mutability | Immutable after compilation | One entry per job execution |
| Storage | Part of Behavior Contract | Alongside Behavior Contract, keyed by job UID |

## Resolved by #318

The following open questions were resolved by the OTel foundation specification (#318):

- **Fallback logging:** Resolved — stderr JSON fallback activates when the primary exporter is unavailable. Violations are never silently lost. See [[otel-config#Fallback Logging]] for configuration and format.
- **Sampling:** Resolved — tail-based sampling is the default strategy. Spans containing error or violation events are always retained (`keep_errors = true`), while routine spans (e.g., `permission.sandbox.setup` INFO) are sampled at the configured ratio. See [[otel-config#Sampling Strategy]] for full details.
- **NATS trace context:** Resolved — W3C `traceparent` headers are injected into all 3 NATS message hops (TM→QH, QH→Runner, Runner→QH), enabling single distributed traces across all services. See [[otel-foundation#NATS Trace Context Propagation]] for the inject/extract pattern.
