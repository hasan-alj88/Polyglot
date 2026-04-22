---
audience: design
type: spec
status: complete
updated: 2026-04-18
---

# OTel Configuration: Exporter and Telemetry Settings

<!-- @c:technical/spec/otel-foundation -->
<!-- @c:technical/spec/otel-permission-events -->
Related: [[otel-foundation]], [[otel-permission-events]]

This specification defines how users configure where Polyglot telemetry goes — exporters, sampling, fallback logging, and batch processing. All configuration is runtime-controlled; no recompilation is needed to change telemetry destinations.

**Scope boundary:** This document covers telemetry *configuration* (where data goes, how it's filtered). The tracing infrastructure (crate stack, span hierarchy, NATS propagation) is defined in [[otel-foundation]]. Event specifications (what is logged and when) are defined per-domain (e.g., [[otel-permission-events]]).

## Configuration Methods

Polyglot supports four configuration methods, evaluated in priority order:

| Priority | Method | Scope | Example |
|---|---|---|---|
| 1 (highest) | Environment variables | Per-process override | `OTEL_EXPORTER_OTLP_ENDPOINT=http://jaeger:4317` |
| 2 | CLI flags | Per-invocation override | `--otel-exporter=stdout` |
| 3 | `polyglot.toml` | Project-level default | `[telemetry]` section in project root |
| 4 (lowest) | Built-in defaults | Fallback | OTLP exporter, localhost:4317, always-on sampling |

Higher-priority methods override lower-priority ones field by field. For example, setting `OTEL_EXPORTER_OTLP_ENDPOINT` overrides `polyglot.toml`'s `endpoint` field but leaves `sampling.strategy` from `polyglot.toml` intact.

## polyglot.toml \[telemetry\] Section

The `[telemetry]` section in `polyglot.toml` is the primary configuration surface. This is the first defined section of `polyglot.toml` — other sections (e.g., `[compiler]`, `[runtime]`) will be defined by future specifications as needed.

```toml
[telemetry]
enabled = true
exporter = "otlp"
endpoint = "http://localhost:4317"
protocol = "grpc"

[telemetry.sampling]
strategy = "tail-based"
ratio = 1.0
keep_errors = true

[telemetry.fallback]
enabled = true
target = "stderr"
format = "json"
path = "/var/log/polyglot/otel-fallback.json"

[telemetry.batch]
max_queue_size = 2048
scheduled_delay_ms = 5000
max_export_batch_size = 512
```

### Field Reference

#### \[telemetry\]

| Field | Type | Default | Valid Values | Description |
|---|---|---|---|---|
| `enabled` | bool | `true` | `true`, `false` | Master switch. When `false`, no telemetry is emitted and the OTel SDK is not initialized. |
| `exporter` | string | `"otlp"` | `"otlp"`, `"stdout"`, `"none"` | Primary exporter type. `"otlp"` sends to an OTLP-compatible endpoint. `"stdout"` prints human-readable spans to terminal. `"none"` disables export (spans are still created for in-process use). |
| `endpoint` | string | `"http://localhost:4317"` | URL | OTLP endpoint address. Used only when `exporter = "otlp"`. |
| `protocol` | string | `"grpc"` | `"grpc"`, `"http"` | OTLP transport protocol. `"grpc"` uses tonic; `"http"` uses reqwest with OTLP/HTTP JSON encoding. |

#### \[telemetry.sampling\]

| Field | Type | Default | Valid Values | Description |
|---|---|---|---|---|
| `strategy` | string | `"tail-based"` | `"always-on"`, `"ratio"`, `"tail-based"` | Sampling strategy. `"always-on"` keeps all spans. `"ratio"` keeps a configured fraction. `"tail-based"` makes keep/drop decisions after span completion based on content. |
| `ratio` | float | `1.0` | `0.0` to `1.0` | Sampling ratio for `"ratio"` strategy. `1.0` = keep all, `0.1` = keep 10%. Ignored for other strategies. |
| `keep_errors` | bool | `true` | `true`, `false` | For `"tail-based"` strategy: always keep spans that contain error or violation events, regardless of sampling ratio. Ensures `permission.sandbox.violation`, `permission.resource.kill`, and `permission.sandbox.setup_failed` events are never dropped. |

#### \[telemetry.fallback\]

| Field | Type | Default | Valid Values | Description |
|---|---|---|---|---|
| `enabled` | bool | `true` | `true`, `false` | Enable fallback logging when the primary exporter is unavailable (network failure, endpoint misconfiguration, export timeout). |
| `target` | string | `"stderr"` | `"stderr"`, `"file"` | Where fallback output goes. `"stderr"` writes to standard error. `"file"` writes to the path specified in `path`. |
| `format` | string | `"json"` | `"json"` | Fallback output format. Currently only structured JSON is supported. |
| `path` | string | `"/var/log/polyglot/otel-fallback.json"` | File path | Output file path for `target = "file"`. Parent directory must exist. Ignored when `target = "stderr"`. |

#### \[telemetry.batch\]

| Field | Type | Default | Valid Values | Description |
|---|---|---|---|---|
| `max_queue_size` | int | `2048` | Positive integer | Maximum number of spans held in the export queue before dropping. |
| `scheduled_delay_ms` | int | `5000` | Positive integer | Interval in milliseconds between batch export attempts. |
| `max_export_batch_size` | int | `512` | Positive integer, ≤ `max_queue_size` | Maximum number of spans sent in a single export request. |

## Environment Variable Mapping

Standard OTel environment variables and Polyglot-specific variables override `polyglot.toml` fields:

| Environment Variable | Maps To | Standard |
|---|---|---|
| `OTEL_EXPORTER_OTLP_ENDPOINT` | `telemetry.endpoint` | OTel standard |
| `OTEL_EXPORTER_OTLP_PROTOCOL` | `telemetry.protocol` | OTel standard |
| `OTEL_TRACES_SAMPLER` | `telemetry.sampling.strategy` | OTel standard |
| `OTEL_TRACES_SAMPLER_ARG` | `telemetry.sampling.ratio` | OTel standard |
| `POLYGLOT_OTEL_EXPORTER` | `telemetry.exporter` | Polyglot-specific |
| `POLYGLOT_OTEL_ENABLED` | `telemetry.enabled` | Polyglot-specific |
| `POLYGLOT_OTEL_FALLBACK` | `telemetry.fallback.enabled` | Polyglot-specific |

**Naming convention:** Standard OTel variables use the `OTEL_` prefix. Polyglot-specific variables use the `POLYGLOT_OTEL_` prefix to avoid namespace collision.

**Value mapping for `OTEL_TRACES_SAMPLER`:**

| Env Value | Maps To |
|---|---|
| `always_on` | `strategy = "always-on"` |
| `traceidratio` | `strategy = "ratio"` |
| `parentbased_traceidratio` | `strategy = "tail-based"` |

## Exporter Destinations

Polyglot supports several telemetry destinations, all via standard OTel protocols:

### OTLP Endpoint (default)

Any OTel-compatible backend accepting OTLP over gRPC or HTTP:
- **Jaeger** — `http://jaeger:4317` (gRPC) or `http://jaeger:4318` (HTTP)
- **Grafana Tempo** — `http://tempo:4317`
- **Datadog** — via Datadog Agent's OTLP receiver
- **Honeycomb** — `https://api.honeycomb.io:443` with API key header

No Polyglot-specific integration is needed. Any backend that speaks OTLP receives full span and event data.

### stdout (dev mode)

Human-readable span output to terminal. Useful for local development and debugging:

```text
POLYGLOT_OTEL_EXPORTER=stdout polyglot run myproject/
```

Prints span names, durations, attributes, and events as they complete. Not suitable for production.

### JSON File

Structured JSON span output to a file for offline analysis. Uses the fallback mechanism with `target = "file"`:

```toml
[telemetry]
exporter = "none"

[telemetry.fallback]
enabled = true
target = "file"
path = "/var/log/polyglot/traces.json"
```

### none (disabled)

Telemetry disabled entirely. No OTel SDK initialization, no span creation overhead. Use for performance-sensitive environments where observability is handled externally.

### Composable Exporters

Multiple destinations are supported by running an OTel Collector as an intermediary:

```text
Polyglot → OTLP → OTel Collector → Jaeger
                                  → Grafana Tempo
                                  → File export
```

The OTel Collector is an external process, not a Polyglot component. Polyglot sends to a single OTLP endpoint; the Collector fans out to multiple backends. This keeps Polyglot's exporter logic simple while supporting arbitrarily complex routing.

## Fallback Logging

**Resolves open question from #315:** When the OTel exporter is unavailable (network failure, misconfigured endpoint, export timeout), the fallback logger ensures telemetry data — especially security-relevant events — is never silently lost.

### Behavior

1. The primary exporter attempts to send a batch of spans
2. If the export fails (connection refused, timeout, HTTP 5xx), the batch is routed to the fallback logger
3. The fallback logger writes each span as a structured JSON line to the configured target (stderr or file)
4. The primary exporter continues retrying on subsequent batches

### Fallback Output Format

Each line is a self-contained JSON object with the same attributes as the OTel span:

```json
{"timestamp":"2026-04-18T14:30:05.789Z","severity":"ERROR","name":"permission.sandbox.violation","trace_id":"abc123","span_id":"exec-789","attributes":{"polyglot.job.uid":"job-a1b2c3d4","polyglot.sandbox.syscall":"open","polyglot.sandbox.resource":"/etc/shadow"}}
```

### Design Rationale

- **Never lose violations:** `permission.sandbox.violation` and `permission.resource.kill` events are security-critical. Silent loss during an exporter outage is unacceptable.
- **Single format:** JSON-lines only. Adding multiple fallback formats creates maintenance burden without proportional value.
- **No second export pipeline:** The fallback is a simple write, not a queued exporter. It does not retry, batch, or buffer beyond the primary exporter's queue.

## Sampling Strategy

**Resolves open question from #315:** Sandbox setup spans are high-volume for frequently-running pipelines. The default sampling strategy balances observability cost against the requirement that security events are never dropped.

### Tail-Based Sampling (default)

Tail-based sampling makes keep/drop decisions after a span completes, based on its content:

| Span Content | Decision |
|---|---|
| Contains error event (`permission.sandbox.violation`, `permission.resource.kill`, `permission.sandbox.setup_failed`) | **Always keep** |
| Contains warning event (`permission.ast.suppressed`, `permission.opaque.activated`, `permission.resource.exceeded` with throttle) | **Always keep** (when `keep_errors = true`) |
| Normal completion, no violations | **Sample** at configured ratio |

This means:
- A routine `permission.sandbox.setup` (INFO) on a pipeline that runs 1000 times/hour may be sampled down to 10%
- A `permission.sandbox.violation` (ERROR) is always kept, regardless of sampling ratio
- The full trace containing a violation is kept (tail-based sampling keeps the entire trace, not just the violating span)

### Other Strategies

| Strategy | Use Case |
|---|---|
| `always-on` | Development, debugging, low-traffic environments. Keeps all spans. |
| `ratio` | Simple head-based sampling. Drops spans at creation time based on probability. Cannot guarantee error retention. |
| `tail-based` | Production default. Cost-effective with guaranteed error visibility. |

### Sandbox Span Sampling

Specifically addressing the #315 open question about sandbox setup spans:

- **Sandbox Setup Span** — sampled at the pipeline's configured rate (may be dropped if routine)
- **Foreign Code Execution Span** — sampled at the pipeline's configured rate
- **Any span containing a violation/error event** — always retained, overriding the sampling rate

This ensures operators always see security events while avoiding excessive storage costs from high-volume routine sandbox instrumentation.
