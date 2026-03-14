---
type: audience-rules
audience: developer
updated: 2026-03-14
---

# Audience: Developers

<!-- @vision:Core Philosophy -->
Authority: [[vision#Core Philosophy]]

## Who They Are

Software engineers building with and on Polyglot: writing .pg files, integrating Polyglot into existing codebases, and contributing to the platform itself.

## Writing Rules

| Rule | Directive |
|------|-----------|
| Tone | Precise, technical, reference-oriented |
| Voice | Active voice, present tense, imperative for instructions |
| Format | Code examples with annotations, API signatures, specification language |
| Structure | Concept -> syntax -> example -> edge cases |
| Vocabulary | Technical terms expected. Use [[glossary]] definitions exactly |
| Code | Annotated .pg examples, cross-language integration snippets |

## ASSUME

- Programming experience in at least one language
- Familiarity with async/concurrent concepts
- Comfort reading formal syntax definitions
- Understanding of package managers and build systems

## NEVER

- Over-explain basic programming concepts
- Provide hand-holding walkthroughs (that's user audience)
- Use marketing language in specs
- Leave syntax ambiguous — every construct must have a formal definition
- Contradict [[vision#Language Design]] principles

## ALWAYS

- Show canonical .pg examples for every language feature
- Include edge cases and error states
- Specify behavior precisely (what happens, not what "should" happen)
- Cross-reference related features via [[wikilinks]]
- Note integration evolution phase ([[vision#Evolution]]) for cross-language features
