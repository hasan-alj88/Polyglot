# PGE04029: Undefined Target/Source

code: PGE04029
description: Variable references in push/pull expressions must explicitly resolve to a defined reference in scope.
category: Wrappers and External Connections

## Description

**Statement:** Every `$variable` used in a push/pull expression, `<input`, or `>output` port must explicitly resolve to a defined reference in the current scope. If a reference does not map to any known declaration, `PGE04029` is raised.

## Rationale
Prevents typos and ensures all memory locations exist before read/write operations.
