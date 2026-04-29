# PGE02016: Backwards Transition

code: PGE02016
description: Reverting a variable from Final to Default is a compile error.
category: Variables and Data Types

## Description

**Statement:** A variable in the `Final` state cannot be pushed into with a default assignment operator (`<~`, `~>`). Attempting to revert a variable from Final to Default is a compile error.

## Rationale
Variables that have received their final, authoritative value must not be overridden with a tentative or default value. This guarantees the unidirectional flow of certainty in Aljam3.
