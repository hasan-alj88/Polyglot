---
type: audience-rules
audience: pg-coder
updated: 2026-04-03
---

# Audience: PG-Coders

<!-- @vision:Core Philosophy -->
Authority: [[vision#Core Philosophy]]

## Who They Are

Developers who write Polyglot code (.pg files). They author pipelines, define data structures, and use the standard library to build automated workflows. They are programmers experienced in at least one language, learning Polyglot as a new tool.

## Writing Rules

| Rule | Directive |
|------|-----------|
| Tone | Technical but approachable, tutorial-friendly |
| Voice | Active voice, present tense, second person ("you") |
| Format | Code-first: .pg examples with annotations, progressive complexity |
| Structure | Concept -> syntax -> example -> try-it |
| Vocabulary | Technical terms expected. Define Polyglot-specific terms on first use via [[glossary]] |
| Code | Annotated .pg examples for every feature. Complete, runnable snippets |

## ASSUME

- Programming experience in at least one language
- Comfort reading code examples
- Willingness to learn new syntax
- No prior Polyglot knowledge

## NEVER

- Expose runtime architecture internals (Queue Manager, Trigger Monitor implementation — that's [[audiences/architect]] scope)
- Assume Polyglot-specific knowledge upfront — introduce each concept
- Skip code examples — every concept needs a .pg snippet
- Leave syntax ambiguous — every construct must have a clear definition
- Contradict [[vision#Language Design]] principles

## ALWAYS

- Show canonical .pg examples for every language feature
- Link to [[SPEC-INDEX]] and [[stdlib/INDEX]] for reference
- Include edge cases and error states
- Provide "try-it" sections where the reader can modify and experiment
- Cross-reference related features via [[wikilinks]]
- Build concepts progressively — simple first, complex later
