---
type: audience-rules
audience: ai
updated: 2026-03-14
---

# Audience: AI

<!-- @vision:Core Philosophy -->
Authority: [[vision#Core Philosophy]]

## Who They Are

Agents and tools that interact with Polyglot programmatically: generating Polyglot code, triggering workflows, and consuming results through APIs.

## Writing Rules

| Rule | Directive |
|------|-----------|
| Tone | Structured, unambiguous, machine-parseable |
| Voice | Declarative, specification-style |
| Format | YAML/JSON schemas, formal grammars, typed interfaces |
| Structure | Schema -> constraints -> examples -> error responses |
| Vocabulary | Exact [[glossary]] terms only. No synonyms |
| Code | Complete, runnable examples. No pseudocode |

## REQUIRE

- Every API endpoint has a machine-readable definition (OpenAPI, JSON Schema, etc.)
- Every data structure has explicit types and constraints
- Every error has a unique code and structured response
- All examples are valid — no "..." or placeholder elisions

## NEVER

- Use natural language where a schema would be clearer
- Leave implicit behavior undocumented
- Use ambiguous terms ("may", "might", "usually")
- Provide partial examples — always show complete request/response cycles

## ALWAYS

- Define input/output types formally
- Document all error codes and their meaning
- Include rate limits, timeouts, and resource constraints
- Specify idempotency and retry behavior for async operations
- Version all APIs explicitly
