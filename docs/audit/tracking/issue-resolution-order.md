---
audience: ai
type: audit-tracking
scope: issue-resolution-order
updated: 2026-04-19
---

# Issue Resolution Order

<!-- @c:audit/README -->

Recommended execution order for open GitHub issues based on dependency analysis. Issues within the same wave can be worked in parallel. Issues in later waves are blocked by earlier ones.

## Milestone: NATS Security Hardening (M5)

### Wave 1 — Foundations (no dependencies)

| # | Title | Priority | Notes |
|---|-------|----------|-------|
| #289 | Add TLS encryption to all NATS connections | P1-critical | Foundation — all other security layers build on encrypted transport |
| #294 | Sanitize jobId to prevent NATS subject injection | P1-critical | Independent — compile-time + runtime validation |
| #296 | Add input validation rules to NATS signal payloads | P2-high | Independent — schema work, no infrastructure dependency |

### Wave 2 — Identity (depends on Wave 1)

| # | Title | Priority | Blocked By |
|---|-------|----------|------------|
| #290 | Add service authentication to NATS | P1-critical | #289 (TLS required for client certificates) |
| #292 | Add message integrity verification (HMAC/signatures) | P2-high | #289 (TLS transport before message-level integrity) |
| #293 | Add replay protection to NATS signals | P2-high | #296 (payload schema must include timestamp/nonce fields) |

### Wave 3 — Authorization and Audit (depends on Wave 2)

| # | Title | Priority | Blocked By |
|---|-------|----------|------------|
| #291 | Add NATS subject-level ACLs | P1-critical | #290 (accounts required before ACL assignment) |
| #297 | Add audit logging for NATS signal sources | P3-medium | #290 (sender_id requires identity system) |

### Wave 4 — Rate Control (depends on Wave 3)

| # | Title | Priority | Blocked By |
|---|-------|----------|------------|
| #295 | Add rate limiting and backpressure | P3-medium | #291 (account-level limits need NATS accounts) |

## Milestone: Deployment & Operations (M6)

### Wave 1 — Foundations (no dependencies)

| # | Title | Priority | Notes |
|---|-------|----------|-------|
| #300 | Design standard health-check pattern | P3-medium | Foundation — discovery and fault handling depend on knowing what "healthy" means |
| #301 | Design fault domain behavior when Runner crashes | P2-high | Foundation — defines crash recovery, informs TM and Runner design |
| #303 | Determine non-Linux development strategy | P4-low | Independent — WSL2/Lima/Multipass decision |

### Wave 2 — Service Coordination (depends on Wave 1)

| # | Title | Priority | Blocked By |
|---|-------|----------|------------|
| #298 | Design multi-version deployment | P4-low | #301 (fault domains define how versions coexist) |
| #299 | Design service discovery mechanism | P3-medium | #300 (health checks inform live/dead detection) |

### Wave 3 — Integration Patterns (depends on Wave 2)

| # | Title | Priority | Blocked By |
|---|-------|----------|------------|
| #302 | Design shared state pattern between services | P4-low | #299 (services must discover each other first) |

## Milestone: Design & Architecture Spec (M2)

### Wave 1 — Signal Architecture (no open dependencies)

| # | Title | Priority | Notes |
|---|-------|----------|-------|
| #320 | Document -T.Call signal path via NATS request-reply | P3-medium | Unblocked — #319 (SDK spec) closed 2026-04-18 |

### Wave 2 — Finalize Designs (depends on Wave 1 + M6 #301)

| # | Title | Priority | Blocked By |
|---|-------|----------|------------|
| #264 | Finalize Trigger Monitor design specification | P1-critical | #320 (-T.Call signal path) + #301 (fault domains) |
| #270 | Finalize Polyglot syntax specification | P1-critical | #321 (bridge syntax must be designed first) |
| #266 | Finalize Runner design specification | P1-critical | #321 (bridge pipelines) + #301 (fault domains) |

### Wave 3 — Compiler Architecture (depends on Wave 2)

| # | Title | Priority | Blocked By |
|---|-------|----------|------------|
| #271 | Finalize compiler architecture and algorithm | P1-critical | #270 (parser needs finalized syntax) |

## Milestone: Language (M4)

### Wave 1 — Cross-Language Integration (no open dependencies)

| # | Title | Priority | Notes |
|---|-------|----------|-------|
| #321 | -Run.Bridge pipeline — pairwise cross-language binding syntax | P3-medium | Unblocked — #319 (SDK spec) closed 2026-04-18 |

### Wave 2 — Service Implementation (depends on M2 Wave 3 + M5)

| # | Title | Priority | Blocked By |
|---|-------|----------|------------|
| #267 | Implement Trigger Monitor | P1-critical | #264 (TM design) + NATS Security (M5) |
| #268 | Implement Queue Handler | P1-critical | NATS Security (M5, all) |
| #269 | Implement Runner | P1-critical | #266 (Runner design) + NATS Security (M5) |

### Wave 3 — stdlib Implementation (parallel with Wave 2)

| # | Title | Priority | Notes |
|---|-------|----------|-------|
| #38–#45, #59 | File stdlib operations (8 + 1 issues) | — | No design dependencies; can start after compiler architecture |
| #164–#185 | Trigger/queue pglib operations (22 issues) | — | Parallel with service implementation |
| #186–#263 | Remaining pglib operations (78 issues) | — | Parallel with service implementation |

## Cross-Milestone Dependencies

These dependencies connect milestones to each other and to pre-existing issues.

| Source | Target | Relationship |
|--------|--------|-------------|
| NATS Security (M5, all) | #267 Implement TM | Blocks — TM signal dispatch must use secure NATS |
| NATS Security (M5, all) | #268 Implement QH | Blocks — QH must validate payloads and verify senders |
| NATS Security (M5, all) | #269 Implement Runner | Blocks — Runner ACKs must use authenticated NATS |
| ~~#319 SDK spec~~ | #320 NATS -T.Call signal path | ~~Blocks~~ — RESOLVED (closed 2026-04-18) |
| ~~#319 SDK spec~~ | #321 -Run.Bridge syntax | ~~Blocks~~ — RESOLVED (closed 2026-04-18) |
| #321 -Run.Bridge syntax | #270 Finalize syntax | Blocks — bridge syntax must be designed before syntax finalization |
| #320 -T.Call signal path | #264 Finalize TM design | Blocks — TM handles -T.Call signals, needs signal path defined |
| #321 -Run.Bridge syntax | #266 Finalize Runner design | Blocks — Runner executes bridge pipelines, needs syntax defined |
| #301 Fault domains | #264 Finalize TM design | Blocks — crash detection answers open TM design questions |
| #301 Fault domains | #266 Finalize Runner design | Blocks — crash recovery answers open Runner design questions |
| #270 Syntax spec | #271 Compiler architecture | Blocks — parser needs finalized syntax |
| #264 TM design | #267 Implement TM | Blocks — design before implementation |
| #266 Runner design | #269 Implement Runner | Blocks — design before implementation |
| #320 NATS -T.Call | NATS Security (M5) | Informs — call signal topics need security hardening |

## Recommended Overall Sequence

```text
Phase 1: Foundations (no dependencies — can work in parallel)
  ✓ #319 Polyglot SDK spec (CLOSED 2026-04-18)
  #301 Design fault domain behavior (crash recovery)

Phase 2: New Syntax + Signal Architecture (depends on Phase 1)
  #321 -Run.Bridge pipeline syntax (UNBLOCKED — #319 closed)
  #320 -T.Call NATS signal path (UNBLOCKED — #319 closed)

Phase 3: Finalize Designs (depends on Phase 2 + #301)
  #270 Finalize syntax spec (after #321 bridge syntax designed)
  #264 Finalize TM design (after #301 + #320)
  #266 Finalize Runner design (after #301 + #321)

Phase 4: Compiler Architecture (depends on Phase 3)
  #271 Finalize compiler architecture (depends on #270)

Phase 5: NATS Security (M5) — can start in parallel with Phases 1–4
  Wave 1: #289, #294, #296
  Wave 2: #290, #292, #293
  Wave 3: #291, #297
  Wave 4: #295

Phase 6: Service Implementation (depends on Phase 3–5)
  #267 Implement TM
  #268 Implement QH
  #269 Implement Runner
  #38–#45, #59 Implement File stdlib operations (parallel with above)
  #164–#185 Implement trigger/queue pglib operations (parallel with above)
  #186–#263 Implement remaining pglib operations (parallel with above)

Phase 7: Deployment & Operations (M6)
  Wave 1: #300, #303
  Wave 2: #298, #299
  Wave 3: #302
```

---

See also: [[coverage-gaps]], [[inconsistencies]], [[decisions]]
