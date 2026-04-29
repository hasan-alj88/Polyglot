# PGE01063: LHS Mutability Check

code: PGE01063
description: The left-hand side of an assignment must be a mutable memory location.
category: Syntax and Structure

## Description

**Statement:** The left-hand side of any assignment operator must be a mutable memory location (`$var`, `>output`, field path). Value literals, pipeline references, or IO markers cannot be targets. Using an invalid construct as the assignment target raises `PGE01063`.

## Rationale
Assignments fundamentally require a writable memory address. Attempting to overwrite static identifiers or syntax tokens is logically invalid.
