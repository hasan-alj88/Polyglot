---
audience: [automation-builder, integrator, architect, designer]
type: reference
updated: 2026-04-20
---

<!-- @c:vision -->
<!-- @u:concepts/data-is-trees -->
<!-- @u:technical/spec/metadata-tree/INDEX -->
<!-- @u:syntax/types/basic-types#RawString -->
# Everything Is a Tree

> Polyglot's entire data model rests on one structural decision: everything is a tree. This page explains why trees are the right foundation for a cross-language platform. See [[vision]] for the broader project context. For the spec-level details, see [[concepts/data-is-trees]].

## Why Trees

Trees are the most natural way to represent structured data. Every programming language already uses them — JSON objects, XML documents, file systems, ASTs, database schemas. When Polyglot needed a universal representation that works across Python, Rust, JavaScript, Go, and any future language, trees were the obvious choice. They are composable, serializable, inspectable, and every language already knows how to work with them.

More importantly, trees are *deterministic structures*. A tree has a single root, a defined depth, named branches, and terminal leaves. There is no ambiguity about where data lives. The compiler can reason about tree shape at compile time — verifying that a Python pipeline's output tree matches the Rust pipeline's expected input tree — because tree topology is structural, not behavioural.

## The Three-Tier System

Polyglot organises its type system into three tiers, each describing a different aspect of the tree:

- **`#` — The Data Tree.** This is the data itself — the structs, pipelines, variables, errors, and packages that make up a Polyglot program. Every `{X}` definition block creates a branch on the data tree. A `#User` struct, a `-ProcessData` pipeline, a `$myVar` variable — all are nodes on the same unified tree rooted at `%`.

- **`##` — The Data Schema.** This describes the tree's *topology* — its shape, depth, ordering, and structural constraints. A `##Scalar` schema says the tree is flat (depth 1). A `##Contiguous` schema says all elements are adjacent in memory. Schemas are reusable bundles of metadata that constrain how a tree can be shaped, without dictating what data it holds.

- **`###` — The Data Value Properties.** This describes the tree's *leaves* — what kind of data sits at the terminals. A `###Value` leaf holds typed data (a string, a number, a date). A `###Enum` leaf holds a variant selector (one of a fixed set of choices). Leaf properties constrain the content nature of terminals, independent of the tree's shape.

These three tiers are independent. You can change a tree's schema without changing its leaf properties, and vice versa. This separation means the compiler can verify tree compatibility at each tier independently — catching shape mismatches, content mismatches, and constraint mismatches as separate classes of errors.

## Strings Are the Universal Value

At the bottom of every tree — at every leaf — sits a `RawString`: a sequence of literal raw characters. This is Polyglot's one true primitive. Everything else — `#String`, `int`, `float`, `#Boolean`, arrays, user structs — is built on top of `RawString` through schemas that constrain how the string is interpreted.

This is not a limitation. It is a deliberate design choice rooted in one observation: *strings are the universal interface between programming languages.* Every language reads strings, writes strings, serializes to strings, deserializes from strings. By making strings the foundation, Polyglot avoids the complexity of bespoke type marshalling between language runtimes. Python's `str`, Rust's `String`, JavaScript's `string`, Go's `string` — they all understand strings. The cross-language data exchange problem reduces to: send a tree of strings, let the receiving side interpret them according to the schema.

## Trees Enable Cross-Language Data Exchange

The tree model is what makes Polyglot's cross-language integration possible at compile time. Consider a pipeline chain where Python produces output that Rust consumes. In a traditional system, you would need runtime marshalling — converting Python objects to some intermediate format, then reconstructing Rust types on the other side, hoping the shapes match.

In Polyglot, the compiler already knows both tree topologies. It verified at compile time that the Python output tree and the Rust input tree have the same shape, the same schemas, and compatible leaf properties. The data flows through the Polyglot Service as a serialized tree of strings — a format every language already knows how to handle. No runtime type negotiation. No marshalling failures. The trees match because the compiler proved they match.

This is a direct consequence of the async-centric model described in [[core-philosophy]]. Because data flows through the platform rather than directly between runtimes, the platform can verify compatibility before any code runs.
