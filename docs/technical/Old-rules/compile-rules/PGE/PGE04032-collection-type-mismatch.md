# PGE04032: Collection Type Mismatch

code: PGE04032
description: Type mismatch when gathering items into a collector.
category: Wrappers and External Connections

## Description

**Statement:** Type mismatch when gathering items into a `(*)` collector array. If the collected items deviate from the array's internal type declaration, `PGE04032` is raised.

## Rationale
Enforces homogeneous data structures across parallel processing aggregation.
