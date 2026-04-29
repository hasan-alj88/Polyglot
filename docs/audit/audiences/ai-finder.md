---
type: audience-rules
audience: ai-finder
classification: internal
updated: 2026-04-22
---

# Audience: AI Finder

<!-- @c:vision:Core Philosophy -->
Authority: [[vision#Core Philosophy]]

## Who They Are

The discoverability layer for AI agents navigating Aljam3 documentation. AI-finder content provides indexes, summaries, retrieval metadata, and structured navigation aids that help AI tools locate and consume the right documentation efficiently.

## Writing Rules

| Rule | Directive |
|------|-----------|
| Tone | Structured, unambiguous, machine-parseable |
| Voice | Declarative, specification-style |
| Format | YAML/JSON schemas, indexes, summary tables, typed cross-references, navigation metadata |
| Structure | Index -> classification -> cross-references -> retrieval hints |
| Vocabulary | Exact [[glossary]] terms only. No synonyms |
| Code | Complete, valid examples. No pseudocode or elisions |

## REQUIRE

- Every index has a machine-readable structure (YAML, tables, or typed lists)
- Every entry has explicit classification and cross-references
- Every summary is concise and self-contained for retrieval
- All examples are valid — no "..." or placeholder elisions

## NEVER

- Use natural language where a structured format would be clearer
- Leave implicit relationships undocumented
- Use ambiguous terms ("may", "might", "usually")
- Provide partial indexes — always show complete coverage
- Duplicate content — reference it with typed cross-references (@c:, @u:, @d:)

## ALWAYS

- Define input/output types formally in schemas
- Document all cross-reference types and their meaning
- Include retrieval hints (keywords, related topics, prerequisite reading)
- Maintain coverage completeness — every documented feature appears in indexes
- Version all structured formats explicitly
- Use typed cross-references (@c:, @u:, @d:) for all links
