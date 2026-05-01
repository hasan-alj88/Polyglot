# PGE02017: Unreachable Branch Code

code: PGE02017
description: Code placed after a terminal operation within a conditional branch is unreachable.
category: Variables and Data Types

## Description

**Statement:** Code placed after a terminal operation *within* a conditional branch (like `[!] >>` or `>>` to final outputs) is unreachable and produces a compile error.

## Rationale
Statically identifies dead code blocks, reducing confusion and enforcing clean exit paths from conditional operations.
