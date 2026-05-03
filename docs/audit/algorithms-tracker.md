# Aljam3 Compiler Algorithms Tracker

This document tracks the implementation lifecycle of the compiler rule detection algorithms within `lib/aljam3/src/compiler/rules/algorithms/`. 
Each algorithm is responsible for traversing the token stream or AST to enforce one or more `JM3Ex/JM3Wx` compiler rules.

| Algorithm | Description | Target Rules | Status |
|---|---|---|---|
| `missing_token_detector` | Simplest algorithm: scans the token stream to detect missing structural tokens (e.g., missing pipeline triggers, missing brackets) | `JM3Ex00110001`, `JM3Ex0011...` | 🏗️ In Progress |
