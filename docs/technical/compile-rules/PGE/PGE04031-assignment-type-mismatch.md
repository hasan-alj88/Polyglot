# PGE04031: Assignment Type Mismatch

code: PGE04031
description: Type mismatch when pushing a value to a variable or output port.
category: Wrappers and External Connections

## Description

**Statement:** Type mismatch when pushing a value to a variable or output port (`<<`, `>>`). The assigned value's type must strictly align with the declared type of the target variable. Failing to align types raises `PGE04031`.

## Rationale
Maintains data integrity across pipeline executions without relying on implicit coercions.
