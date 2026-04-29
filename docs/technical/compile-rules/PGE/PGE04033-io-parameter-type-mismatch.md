# PGE04033: IO Parameter Type Mismatch

code: PGE04033
description: Type mismatch when wiring a pipeline input.
category: Wrappers and External Connections

## Description

**Statement:** Type mismatch when wiring a pipeline input (`(-)`). Supplying an argument of the wrong type to a declared pipeline IO port raises `PGE04033`.

## Rationale
Guarantees that sub-pipelines strictly receive the data contracts they explicitly declare.
