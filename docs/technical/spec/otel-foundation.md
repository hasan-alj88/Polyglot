---
audience: architect
type: spec
status: complete
updated: 2026-04-18
---

# OTel Foundation: Tracing Infrastructure

<!-- @c:technical/spec/otel-permission-events -->
<!-- @c:technical/spec/behavior-contract -->
<!-- @c:technical/spec/job-sandbox -->
<!-- @c:technical/spec/otel-config -->
Related: [[otel-permission-events]], [[behavior-contract]], [[job-sandbox]], [[otel-config]]

This specification defines the shared OpenTelemetry tracing infrastructure for all Polyglot services — the Rust crate stack, span hierarchy, semantic conventions, and cross-service trace context propagation.

**Scope boundary:** This document covers the tracing infrastructure (*how* telemetry flows). What is logged and when is defined per-domain in event specification documents (e.g., [[otel-permission-events]] for permission/sandbox events). Where telemetry goes (exporters, configuration) is defined in [[otel-config]].

## Rust Crate Stack

Polyglot uses the standard Rust observability ecosystem. All services (Trigger Monitor, Queue Handler, Runner, Compiler) share this crate stack.

| Crate | Role | Feature Flags | Notes |
|---|---|---|---|
| `tracing` | Instrumentation API — spans, events, structured fields | — | All Polyglot code instruments via `tracing` macros (`#[instrument]`, `tracing::info!`, `tracing::error!`) |
| `tracing-opentelemetry` | Bridges `tracing` spans to OTel spans | — | Installed as a `tracing` layer; converts `tracing::Span` into OTel `SpanData` |
| `opentelemetry-sdk` | OTel SDK — batch span processor, sampling, resource configuration | — | Configures service identity (`polyglot.service.role`), batch export intervals, sampling strategy |
| `opentelemetry-otlp` | OTLP exporter — sends spans/logs to OTel-compatible backends | `grpc-tonic`, `http-reqwest` | gRPC (tonic) is the default transport; HTTP (reqwest) available as fallback |
| `opentelemetry-stdout` | Dev mode exporter — prints spans to stdout | — | Human-readable output for local development; enabled via `POLYGLOT_OTEL_EXPORTER=stdout` |
| `opentelemetry-nats` | NATS trace context propagation | — | Injects/extracts W3C `traceparent` headers in NATS messages for cross-service correlation |

### Integration Pattern

The crates compose as a layered pipeline. Each service initializes this stack at startup:

```text
Application Code
    │
    ▼
tracing (instrumentation API)
    │  #[instrument], info!(), error!()
    ▼
tracing-opentelemetry (bridge layer)
    │  converts tracing::Span → OTel SpanData
    ▼
opentelemetry-sdk (batch processor + sampler)
    │  batches spans, applies sampling strategy
    ▼
Exporter (selected at startup from config)
    ├── opentelemetry-otlp  → OTLP endpoint (Jaeger, Tempo, Datadog)
    ├── opentelemetry-stdout → terminal (dev mode)
    └── (fallback)          → stderr JSON (when primary exporter fails)
```

The bridge layer means all Polyglot code uses `tracing` — the standard Rust instrumentation crate — without importing OTel types directly. The OTel integration is a deployment concern configured at service startup.

## Semantic Convention Registry

All attributes use the `polyglot.*` namespace following [OTel semantic conventions](https://opentelemetry.io/docs/specs/semconv/). The registry is split into two groups: permission/sandbox attributes (defined by #315) and service-wide attributes (defined here).

### Permission/Sandbox Attributes (from #315)

These 9 attributes are defined authoritatively in [[otel-permission-events#Attribute Registry]]. They are listed here for completeness — see that document for full descriptions and examples.

| Attribute | Type | Defined In |
|---|---|---|
| `polyglot.job.uid` | string | [[otel-permission-events]] |
| `polyglot.pipeline.name` | string | [[otel-permission-events]] |
| `polyglot.package.name` | string | [[otel-permission-events]] |
| `polyglot.permission.category` | string | [[otel-permission-events]] |
| `polyglot.sandbox.layer` | string | [[otel-permission-events]] |
| `polyglot.sandbox.syscall` | string | [[otel-permission-events]] |
| `polyglot.sandbox.resource` | string | [[otel-permission-events]] |
| `polyglot.sandbox.action` | string | [[otel-permission-events]] |
| `polyglot.sandbox.opaque` | bool | [[otel-permission-events]] |

### Service-Wide Attributes (this document)

| Attribute | Type | Description | Example | Attached To |
|---|---|---|---|---|
| `polyglot.service.role` | string | Which Polyglot service emitted the span | `"tm"`, `"qh"`, `"runner"`, `"compiler"` | Service Span (root) |
| `polyglot.trigger.name` | string | Trigger definition name from the pipeline | `"-T.Timer.Cron"`, `"-T.Git.Push"` | Trigger Evaluation Span |
| `polyglot.queue.name` | string | Queue name from the `{Q}` definition | `"default"`, `"high-priority"` | Queue Dispatch Span |

### Registry Extensibility

The `polyglot.*` namespace is extensible. Future event specifications (pipeline lifecycle, trigger monitor, job lifecycle — see the Future Event Sets section below) will register additional attributes in this namespace. Each event spec document owns its attribute definitions; this document maintains the cross-reference index.

**Current total: 12 attributes** (9 permission/sandbox + 3 service-wide).

## Span Hierarchy

The full span tree across all Polyglot services. Each span is owned by one service and created/closed at defined lifecycle points.

```text
Service Span (polyglot.service.role)
  └── Pipeline Span (polyglot.pipeline.name, polyglot.package.name)
        ├── Trigger Evaluation Span (polyglot.trigger.name)
        ├── Job Span (polyglot.job.uid)
        │     ├── Queue Dispatch Span (polyglot.queue.name)
        │     ├── Wrapper Setup Span ([\])
        │     ├── Execution Body Span
        │     │     ├── Foreign Code Span (if -Run.*)
        │     │     │     ├── Sandbox Setup Span          ← #315
        │     │     │     ├── Foreign Code Execution Span ← #315
        │     │     │     └── (sandbox events attach here) ← #315
        │     │     └── (native execution — no sandbox)
        │     ├── Wrapper Cleanup Span ([/])
        │     └── Job Completion Span
        └── Collector Span (*All / *First / *Nth)
```

### Span Details

| Span | Owner | Created | Closed | Key Attributes |
|---|---|---|---|---|
| Service Span | Each service | Service startup | Service shutdown | `polyglot.service.role` |
| Pipeline Span | Trigger Monitor | Pipeline enabled and first trigger evaluates | All jobs for this pipeline execution complete | `polyglot.pipeline.name`, `polyglot.package.name` |
| Trigger Evaluation Span | Trigger Monitor | Trigger condition begins evaluation | Trigger fires or condition not met | `polyglot.trigger.name` |
| Job Span | Trigger Monitor | Job UID assigned, TOON dispatched | Job completion ACK received from Runner | `polyglot.job.uid` |
| Queue Dispatch Span | Queue Handler | Job enters queue | Job dispatched to Runner | `polyglot.queue.name` |
| Wrapper Setup Span | Runner | `[\]` setup block begins | Setup block completes | — |
| Execution Body Span | Runner | Pipeline body execution begins | Body execution completes | — |
| Foreign Code Span | Runner | `-Run.*` pipeline spawns foreign process | Foreign process exits | — |
| Wrapper Cleanup Span | Runner | `[/]` cleanup block begins | Cleanup block completes | — |
| Job Completion Span | Runner | Job result packaging begins | Result transmitted to Queue Handler | — |
| Collector Span | Trigger Monitor | Collector begins waiting for sub-jobs | All required sub-jobs collected or race winner determined | — |

### Compatibility with #315

The sandbox-specific spans defined in [[otel-permission-events#Span Hierarchy]] nest inside the Foreign Code Span:

- **Sandbox Setup Span** — child of Foreign Code Span, contains `permission.sandbox.setup` and `permission.opaque.activated` events
- **Foreign Code Execution Span** — child of Foreign Code Span, contains `permission.sandbox.violation` and `permission.resource.exceeded` events
- **Job Completion Span** — child of Job Span (directly), contains `permission.compliance.generated` event

The `permission.resource.kill` event attaches to the Job Span directly because it terminates the job.

### Cross-Service Span Ownership

Spans cross service boundaries via NATS trace context propagation. The Pipeline Span is created by the Trigger Monitor but receives child spans from all three services:

```text
Trigger Monitor          Queue Handler          Runner
──────────────          ──────────────          ──────
Pipeline Span ─────────────────────────────────────────
  │
  ├── Trigger Eval Span
  │
  ├── Job Span ──────── Queue Dispatch ──────── Wrapper Setup
  │                     Span                    Execution Body
  │                                             Foreign Code Span
  │                                               └── Sandbox spans
  │                                             Wrapper Cleanup
  │                                             Job Completion
  │
  └── Collector Span
```

The trace context (W3C `traceparent`) is injected into NATS messages at each service boundary, allowing the distributed trace to be reconstructed as a single tree in the backend (Jaeger, Tempo, etc.).

## NATS Trace Context Propagation

NATS messages between Polyglot services carry W3C Trace Context headers for cross-service span correlation. Without this, each service would produce isolated traces with no way to connect a trigger evaluation to its resulting job execution.

### Propagation Points

Three NATS message exchanges carry trace context:

| Hop | From | To | NATS Subject | Payload Contains |
|---|---|---|---|---|
| 1. Trigger Fire | Trigger Monitor | Queue Handler | `polyglot.trigger.{pipeline}` | TOON + `traceparent` header |
| 2. Job Dispatch | Queue Handler | Runner | `polyglot.job.{uid}` | Job config + `traceparent` header |
| 3. Completion ACK | Runner | Queue Handler | `polyglot.ack.{uid}` | Result + `traceparent` header |

### Inject/Extract Pattern

Each NATS message includes a `traceparent` header following the W3C Trace Context specification:

```text
traceparent: 00-<trace-id>-<span-id>-<trace-flags>
```

**On send (inject):**
1. Get current span context from `tracing::Span::current()`
2. Use `TraceContextPropagator` to inject `traceparent` into NATS message headers
3. Publish message with headers

**On receive (extract):**
1. Extract `traceparent` from NATS message headers using `TraceContextPropagator`
2. Create new span as child of the extracted context
3. Process message within this child span

### End-to-End Trace Flow

A complete pipeline execution produces a single distributed trace:

```text
TM: Pipeline Span created (trace-id=abc123)
TM:   Trigger Eval Span
TM:   Job Span created (span-id=001)
 │
 │  NATS publish: traceparent: 00-abc123-001-01
 ▼
QH: Extract traceparent → child of span 001
QH:   Queue Dispatch Span (span-id=002)
 │
 │  NATS publish: traceparent: 00-abc123-002-01
 ▼
Runner: Extract traceparent → child of span 002
Runner:   Wrapper Setup Span
Runner:   Execution Body Span
Runner:     Foreign Code Span
Runner:       Sandbox Setup Span (permission.sandbox.setup event)
Runner:       Foreign Code Execution Span
Runner:     Wrapper Cleanup Span
Runner:   Job Completion Span (permission.compliance.generated event)
 │
 │  NATS publish: traceparent: 00-abc123-007-01
 ▼
QH: Extract traceparent → completion processing
TM: Job Span closed (all child spans visible in trace)
TM: Collector Span (if parallel jobs)
TM: Pipeline Span closed
```

All spans share `trace-id=abc123`, making the entire pipeline execution — from trigger evaluation through foreign code sandbox setup to job completion — visible as a single trace in the backend.

## Future Event Sets

This foundation supports additional event specifications beyond the permission/sandbox events defined in #315. Each event set will be a separate specification document following the pattern established by [[otel-permission-events]].

**Planned event sets (not in scope for #318):**

| Event Set | Events | Primary Emitter |
|---|---|---|
| Pipeline Lifecycle | compiled, compile.failed, registered, enabled, disabled, recompiled, version.replaced | Compiler, Trigger Monitor |
| Trigger Monitor | partial, fired, retrigger.blocked, retrigger.queued, condition.error | Trigger Monitor |
| Job Lifecycle | enqueued, started, completed, failed, paused, resumed, throttled, killed, subjob.spawned | Queue Handler, Runner |

Each event set will:
- Define its own events with severity, trigger conditions, and attributes
- Register new `polyglot.*` attributes in this foundation document's registry
- Specify which spans its events attach to (using the hierarchy defined above)
- Follow the same structured format as [[otel-permission-events]]
