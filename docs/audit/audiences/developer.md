---
type: audience-rules
audience: developer
classification: internal
updated: 2026-04-22
---

# Audience: Developer

<!-- @c:vision:Core Philosophy -->
Authority: [[vision#Core Philosophy]]

## Who They Are

Implementers who build Aljam3: writing compiler passes, implementing compile rules, building pglib operators, creating tests, and coding the runtime services. They turn design specifications into working software.

## Writing Rules

| Rule | Directive |
|------|-----------|
| Tone | Implementation-focused, precise, code-centric |
| Voice | Active voice, present tense, imperative for instructions |
| Format | Code examples (Rust, Python, etc.), test cases, error code implementations, API documentation |
| Structure | Specification reference -> implementation -> tests -> edge cases |
| Vocabulary | Exact [[glossary]] terms. Implementation-level terminology (AST, IR, codegen) expected |
| Code | Working implementation examples. Test cases for every feature. Error code implementations |

## ASSUME

- Strong programming experience (Rust, Python, or equivalent)
- Compiler theory basics (parsing, AST, type checking, code generation)
- Familiarity with the Aljam3 design spec (from [[audiences/design]] docs)
- Testing and CI/CD experience
- Understanding of the Aljam3 service architecture at a high level

## NEVER

- Restate design philosophy or rationale (that's [[audiences/design]] scope)
- Include product-level requirements (that's [[audiences/product]] scope)
- Leave implementation behavior unspecified — every code path needs documentation
- Skip test cases — every implementation needs verification
- Contradict the design specification

## ALWAYS

- Reference the authoritative design spec for each feature being implemented
- Include test cases (unit, integration) for every implementation
- Document error codes and their implementation (PGE/PGW)
- Show actual code examples in the implementation language
- Specify performance considerations where relevant
- Cross-reference related implementations via [[wikilinks]]
