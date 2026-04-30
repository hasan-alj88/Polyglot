---
type: audience-rules
audience: automation-builder
classification: external
updated: 2026-04-13
---

# Audience: Automation Builders

<!-- @c:vision:Core Philosophy -->
Authority: [[vision#Core Philosophy]]

## Who They Are

Developers who write Aljam3 code (.aj3 files). They author pipelines, define data structures, and use the standard library to build automated workflows. They are programmers experienced in at least one language, learning Aljam3 as a new tool.

## Writing Rules

| Rule | Directive |
|------|-----------|
| Tone | Technical but approachable, tutorial-friendly |
| Voice | Active voice, present tense, second person ("you") |
| Format | Code-first: .aj3 examples with annotations, progressive complexity |
| Structure | Concept -> syntax -> example -> try-it |
| Vocabulary | Technical terms expected. Define Aljam3-specific terms on first use via [[glossary]] |
| Code | Annotated .aj3 examples for every feature. Complete, runnable snippets |

## ASSUME

- Programming experience in at least one language
- Comfort reading code examples
- Willingness to learn new syntax
- No prior Aljam3 knowledge

## NEVER

- Expose runtime architecture internals (Queue Manager, Trigger Monitor implementation — that's [[audiences/design]] scope)
- Assume Aljam3-specific knowledge upfront — introduce each concept
- Skip code examples — every concept needs a .aj3 snippet
- Leave syntax ambiguous — every construct must have a clear definition
- Contradict [[vision#Language Design]] principles

## ALWAYS

- Show canonical .aj3 examples for every language feature
- Link to [[SPEC-INDEX]] and [[aj3lib/INDEX]] for reference
- Include edge cases and error states
- Provide "try-it" sections where the reader can modify and experiment
- Cross-reference related features via [[wikilinks]]
- Build concepts progressively — simple first, complex later
