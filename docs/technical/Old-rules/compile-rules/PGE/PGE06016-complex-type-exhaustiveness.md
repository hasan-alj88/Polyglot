# PGE06016: Complex Type Exhaustiveness

code: PGE06016
description: Branching on complex types requires a catch-all branch.
category: Syntax and Structure

## Description

**Statement:** Branching on arrays, structs, paths, datetime, or any dynamic type without a statically provable discrete domain requires a `*?` catch-all branch. If a conditional `[?]` evaluates a complex or open-domain type and lacks a `*?` wildcard, `PGE06016` is raised.

## Rationale
Since complex types possess an infinite or unprovable domain space at compile time, exhaustive static checking is impossible. A fallback is mandatory to ensure runtime safety.
