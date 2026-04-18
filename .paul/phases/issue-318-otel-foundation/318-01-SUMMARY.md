---
phase: issue-318-otel-foundation
plan: 01
subsystem: infra
tags: [opentelemetry, tracing, otel, nats, rust]

requires:
  - phase: issue-315-otel-permission-sandbox
    provides: 8 OTel events, 9 polyglot.* attributes, sandbox span hierarchy
provides:
  - OTel tracing infrastructure specification (crate stack, span hierarchy, semantic conventions)
  - OTel exporter configuration specification (polyglot.toml [telemetry], env vars, fallback, sampling)
  - Resolution of 3 open questions from #315
affects: [issue-318 plan 02, future pipeline/trigger/job event specs]

tech-stack:
  added: []
  patterns: [tracing-layer-composition, tail-based-sampling, nats-traceparent-propagation]

key-files:
  created:
    - docs/technical/spec/otel-foundation.md
    - docs/technical/spec/otel-config.md
  modified: []

key-decisions:
  - "Tail-based sampling as default: always keep spans with errors/violations"
  - "Fallback logging to stderr JSON when exporter unavailable"
  - "W3C traceparent in NATS messages for cross-service correlation"
  - "OTel Collector as composable exporter pattern (not built into Polyglot)"
  - "polyglot.toml [telemetry] as first defined config section"

patterns-established:
  - "Split spec pattern: foundation (infrastructure) + config (user-facing) as separate docs"
  - "Attribute registry cross-reference: each event spec owns its attributes, foundation indexes them"

duration: ~15min
started: 2026-04-18
completed: 2026-04-18
---

# Issue #318 Plan 01: OTel Foundation Spec Documents Summary

**Two new specification documents defining the OTel tracing infrastructure and exporter configuration for all Polyglot services — crate stack, span hierarchy, 12 semantic convention attributes, NATS trace propagation, polyglot.toml [telemetry] schema, and resolution of all 3 open questions from #315.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~15min |
| Started | 2026-04-18 |
| Completed | 2026-04-18 |
| Tasks | 2 completed |
| Files created | 2 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Rust Crate Stack Documented | Pass | 6 crates with role, feature flags, integration pattern diagram |
| AC-2: Full Span Hierarchy Documented | Pass | 11 spans from Service through Collector, compatible with #315 sandbox children |
| AC-3: Semantic Conventions Extended | Pass | 12 total attributes (9 from #315 + 3 new service-wide) |
| AC-4: NATS Trace Context Propagation | Pass | 3 hops with W3C traceparent inject/extract, end-to-end trace flow |
| AC-5: Exporter Configuration Documented | Pass | polyglot.toml [telemetry] schema, env var mapping, 4 destinations |
| AC-6: Fallback and Sampling Documented | Pass | stderr JSON fallback, tail-based sampling with keep_errors |

## Accomplishments

- Created complete OTel tracing infrastructure specification with layered crate composition pattern
- Defined full span hierarchy across TM/QH/Runner with cross-service ownership model
- Created first polyglot.toml section specification ([telemetry]) with full field reference
- Resolved all 3 open questions from #315: fallback logging, sampling strategy, NATS trace context

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| docs/technical/spec/otel-foundation.md | Created | Tracing infrastructure: crate stack, span hierarchy, semantic conventions, NATS propagation |
| docs/technical/spec/otel-config.md | Created | Exporter configuration: polyglot.toml [telemetry], env vars, fallback, sampling |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| Tail-based sampling default | Balances cost (drop routine spans) with safety (never drop violations) | All security events guaranteed visible |
| stderr JSON fallback | Simple write, no second export pipeline; violations never silently lost | Operators always have audit trail |
| NATS traceparent propagation | W3C standard; enables single distributed trace across TM/QH/Runner | Full request tracing in Jaeger/Tempo |
| OTel Collector for composable export | Keeps Polyglot's exporter simple (single OTLP endpoint); Collector fans out | No multi-exporter complexity in Polyglot code |
| polyglot.toml as first config section | No existing config file spec; [telemetry] establishes the pattern | Future specs add [compiler], [runtime], etc. |

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- Foundation and config docs complete; plan 318-02 can now add cross-references to existing docs
- Open questions from #315 resolved; otel-permission-events.md ready for update

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: issue-318-otel-foundation, Plan: 01*
*Completed: 2026-04-18*
