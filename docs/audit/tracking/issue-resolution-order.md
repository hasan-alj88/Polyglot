---
audience: ai-finder
type: audit-tracking
scope: issue-resolution-order
updated: 2026-04-21
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

### Wave 1 — Finalize Designs (depends on M6 #301)

| # | Title | Priority | Blocked By |
|---|-------|----------|------------|
| #270 | Finalize Aljam3 syntax specification | P1-critical | ~~#321~~ — UNBLOCKED (#321 closed 2026-04-19) |
| #264 | Finalize Trigger Monitor design specification | P1-critical | #301 (fault domains) — #320 closed 2026-04-19 |
| #266 | Finalize Runner design specification | P1-critical | #301 (fault domains) — #321 closed 2026-04-19 |

### Wave 2 — Compiler Architecture (depends on Wave 1)

| # | Title | Priority | Blocked By |
|---|-------|----------|------------|
| #271 | Finalize compiler architecture and algorithm | P1-critical | #270 (parser needs finalized syntax) |

## Milestone: SDK & Bridge Design (unassigned) — COMPLETE

### Wave 1 — SDK Documentation (COMPLETE)

| # | Title | Priority | Notes |
|---|-------|----------|-------|
| ~~#328~~ | ~~Add #bytes and #dt binding types to marshalling tables~~ | — | CLOSED 2026-04-19 |
| ~~#329~~ | ~~Document per-language SDK encode/decode algorithms~~ | — | CLOSED 2026-04-19 |
| ~~#330~~ | ~~Document canonical float/null/boolean wire format conventions~~ | — | CLOSED 2026-04-19 |

## Milestone: Language (M4)

### Wave 1 — Cross-Language Integration (COMPLETE)

| # | Title | Priority | Notes |
|---|-------|----------|-------|
| ~~#321~~ | ~~-Run.Bridge pipeline~~ | ~~P3-medium~~ | CLOSED 2026-04-19 |

### Wave 2 — Service Implementation (depends on M2 Wave 2 + M5)

| # | Title | Priority | Blocked By |
|---|-------|----------|------------|
| #267 | Implement Trigger Monitor | P1-critical | #264 (TM design) + NATS Security (M5) |
| #268 | Implement Queue Handler | P1-critical | NATS Security (M5, all) |
| #269 | Implement Runner | P1-critical | #266 (Runner design) + NATS Security (M5) |

### Wave 3 — stdlib Implementation (parallel with Wave 2)

| # | Title | Priority | Notes |
|---|-------|----------|-------|
| #38–#45, #59 | File stdlib operations (8 + 1 issues) | — | No design dependencies; can start after compiler architecture |
| #164–#185 | Trigger/queue jm3lib operations (22 issues) | — | Parallel with service implementation |
| #186–#263 | Remaining jm3lib operations (78 issues) | — | Parallel with service implementation |

### Wave 4 — Bridge Implementation (depends on Wave 2 + compiler)

| # | Title | Priority | Blocked By |
|---|-------|----------|------------|
| #322 | Python-Rust Bridge implementation | — | #271 (compiler) + #269 (Runner) |
| #323 | Python-Go Bridge implementation | — | #271 (compiler) + #269 (Runner) |
| #324 | Python-JavaScript Bridge implementation | — | #271 (compiler) + #269 (Runner) |
| #325 | Rust-Go Bridge implementation | — | #271 (compiler) + #269 (Runner) |
| #326 | Rust-JavaScript Bridge implementation | — | #271 (compiler) + #269 (Runner) |
| #327 | Go-JavaScript Bridge implementation | — | #271 (compiler) + #269 (Runner) |

## Standalone Documentation (no milestone) — COMPLETE

### Wave 1 — Philosophy Pages (COMPLETE)

| # | Title | Priority | Notes |
|---|-------|----------|-------|
| ~~#331~~ | ~~Restructure vision.md into docs/philosophy/~~ | — | CLOSED 2026-04-20 |
| ~~#332~~ | ~~Add philosophy/symbology.md — Symbol design rationale~~ | — | CLOSED 2026-04-20 |
| ~~#333~~ | ~~Add philosophy/accountability.md — Human inspection and no dynamic code~~ | — | CLOSED 2026-04-20 |
| ~~#334~~ | ~~Add philosophy/cybersecurity.md — Zero trust and black box monitoring~~ | — | CLOSED 2026-04-20 |
| ~~#335~~ | ~~Add philosophy/error-philosophy.md — Murphy's Law and exhaustive error handling~~ | — | CLOSED 2026-04-20 |
| ~~#336~~ | ~~Add remaining philosophy files (data-trees, behavioral-contract, DX, extensibility, how-differs)~~ | — | CLOSED 2026-04-20 |

## Cross-Milestone Dependencies

These dependencies connect milestones to each other and to pre-existing issues.

| Source | Target | Relationship |
|--------|--------|-------------|
| NATS Security (M5, all) | #267 Implement TM | Blocks — TM signal dispatch must use secure NATS |
| NATS Security (M5, all) | #268 Implement QH | Blocks — QH must validate payloads and verify senders |
| NATS Security (M5, all) | #269 Implement Runner | Blocks — Runner ACKs must use authenticated NATS |
| ~~#319 SDK spec~~ | #320 NATS -T.Call signal path | ~~Blocks~~ — RESOLVED (closed 2026-04-18) |
| ~~#319 SDK spec~~ | #321 -Run.Bridge syntax | ~~Blocks~~ — RESOLVED (closed 2026-04-18) |
| ~~#321 -Run.Bridge syntax~~ | #270 Finalize syntax | ~~Blocks~~ — RESOLVED (closed 2026-04-19) |
| ~~#320 -T.Call signal path~~ | #264 Finalize TM design | ~~Blocks~~ — RESOLVED (closed 2026-04-19) |
| ~~#321 -Run.Bridge syntax~~ | #266 Finalize Runner design | ~~Blocks~~ — RESOLVED (closed 2026-04-19) |
| #301 Fault domains | #264 Finalize TM design | Blocks — crash detection answers open TM design questions |
| #301 Fault domains | #266 Finalize Runner design | Blocks — crash recovery answers open Runner design questions |
| #270 Syntax spec | #271 Compiler architecture | Blocks — parser needs finalized syntax |
| #264 TM design | #267 Implement TM | Blocks — design before implementation |
| #266 Runner design | #269 Implement Runner | Blocks — design before implementation |
| #320 NATS -T.Call | NATS Security (M5) | Informs — call signal topics need security hardening |

## Recommended Overall Sequence

```text
Phase 1: Foundations (no dependencies — can work in parallel)
  ✓ #319 Aljam3 SDK spec (CLOSED 2026-04-18)
  #301 Design fault domain behavior (crash recovery)

Phase 2: New Syntax + Signal Architecture (COMPLETE)
  ✓ #321 -Run.Bridge pipeline syntax (CLOSED 2026-04-19)
  ✓ #320 -T.Call NATS signal path (CLOSED 2026-04-19)

Phase 3: Finalize Designs (depends on #301 — #320/#321 both closed)
  #270 Finalize syntax spec (UNBLOCKED — #321 closed)
  #264 Finalize TM design (blocked by #301)
  #266 Finalize Runner design (blocked by #301)

Phase 3.5: SDK Documentation (COMPLETE)
  ✓ #328 Add #bytes and #dt binding types (CLOSED 2026-04-19)
  ✓ #329 Document per-language SDK encode/decode algorithms (CLOSED 2026-04-19)
  ✓ #330 Document canonical float/null/boolean wire format (CLOSED 2026-04-19)

Phase 4: Compiler Architecture (depends on Phase 3)
  #271 Finalize compiler architecture (depends on #270)

Phase 5: NATS Security (M5) — can start in parallel with Phases 1–4
  Wave 1: #289, #294, #296
  Wave 2: #290, #292, #293
  Wave 3: #291, #297
  Wave 4: #295

Phase 6: Service Implementation (depends on Phase 4 + Phase 5)
  #267 Implement TM
  #268 Implement QH
  #269 Implement Runner
  #38–#45, #59 Implement File stdlib operations (parallel with above)
  #164–#185 Implement trigger/queue jm3lib operations (parallel with above)
  #186–#263 Implement remaining jm3lib operations (parallel with above)

Phase 6.5: Bridge Implementation (depends on Phase 6)
  #322–#327 Six language-pair Bridge implementations

Phase 7: Deployment & Operations (M6)
  Wave 1: #300, #303
  Wave 2: #298, #299
  Wave 3: #302
```

---

See also: [[coverage-gaps]], [[inconsistencies]], [[decisions]]
