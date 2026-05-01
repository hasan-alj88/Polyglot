---
type: audience-rules
audience: integrator
classification: external
updated: 2026-04-13
---

# Audience: Integrators

<!-- @c:vision:Core Philosophy -->
Authority: [[vision#Core Philosophy]]

## Who They Are

Developers who integrate their existing codebases (Python, Rust, JavaScript, etc.) with Aljam3 without writing .jm3 code. They use Aljam3 as a service — connecting their applications through SDKs, APIs, and cross-language bridges. They care about how Aljam3 fits into their stack, not its internal syntax.

## Writing Rules

| Rule | Directive |
|------|-----------|
| Tone | Practical, SDK/API-focused, solution-oriented |
| Voice | Active voice, present tense, imperative for instructions |
| Format | Host-language code snippets (Python, Rust, JS, etc.), API signatures, integration diagrams |
| Structure | Goal -> integration code -> expected behavior -> troubleshooting |
| Vocabulary | Use host-language terminology where appropriate. Define Aljam3 terms via [[glossary]] |
| Code | Host-language examples showing SDK/API usage. No .jm3 syntax required |

## ASSUME

- Proficiency in their host programming language
- Basic understanding of async/concurrent concepts and IPC
- Familiarity with package managers, build systems, and dependency management
- No knowledge of Aljam3 code syntax

## NEVER

- Require .jm3 syntax knowledge — they don't write Aljam3 code
- Assume they have read the language spec or syntax docs
- Use Aljam3-internal jargon without definition
- Show .jm3 code as the primary example — always lead with host-language code
- Contradict [[vision#Evolution]] integration strategy

## ALWAYS

- Show host-language code snippets (Python, Rust, JS, etc.)
- Document SDK/API surfaces with signatures and return types
- Include expected behavior and output for every example
- Cross-reference [[audiences/automation-builder]] docs when deeper Aljam3 understanding would help
- Note the integration evolution phase ([[vision#Evolution]]) for cross-language features
- Provide troubleshooting for common integration failures
