---
audience: ai
type: audit-tracking
scope: issue-resolution-order
updated: 2026-04-16
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

## Cross-Milestone Dependencies

These dependencies connect milestones to each other and to pre-existing issues.

| Source | Target | Relationship |
|--------|--------|-------------|
| NATS Security (M5, all) | #267 Implement TM | Blocks — TM signal dispatch must use secure NATS |
| NATS Security (M5, all) | #268 Implement QH | Blocks — QH must validate payloads and verify senders |
| NATS Security (M5, all) | #269 Implement Runner | Blocks — Runner ACKs must use authenticated NATS |
| #301 Fault domains | #264 Finalize TM design | Informs — crash detection is a TM responsibility |
| #301 Fault domains | #266 Finalize Runner design | Informs — crash recovery must be in Runner spec |
| #264 TM design | #267 Implement TM | Blocks — design before implementation |
| #266 Runner design | #269 Implement Runner | Blocks — design before implementation |
| #270 Syntax spec | #271 Compiler architecture | Blocks — parser needs finalized syntax |

## Recommended Overall Sequence

```text
Phase 1: Language Spec
  #270 Finalize syntax spec
  #271 Finalize compiler architecture

Phase 2: Service Design
  #264 Finalize TM design (+ #301 fault domain input)
  #266 Finalize Runner design (+ #301 fault domain input)

Phase 3: NATS Security (M5)
  Wave 1: #289, #294, #296
  Wave 2: #290, #292, #293
  Wave 3: #291, #297
  Wave 4: #295

Phase 4: Service Implementation
  #267 Implement TM
  #268 Implement QH
  #269 Implement Runner
  #186–#263 Implement pglib operations (parallel with above)

Phase 5: Deployment & Operations (M6)
  Wave 1: #300, #301, #303
  Wave 2: #298, #299
  Wave 3: #302
```

---

See also: [[coverage-gaps]], [[inconsistencies]], [[decisions]]
