# Missing Token Detector Algorithm

This algorithm implements a simple forward pass over the lexical token stream. It detects missing physical structure tokens (such as missing brackets, missing pipeline markers, missing triggers, etc.) before the AST is fully constructed or as a preliminary validation step.

## Detected Rules

This algorithm is responsible for asserting the following structural/syntax `JM3Ex` rules:

*   `JM3Ex00110001` - Missing Pipeline Trigger (formerly `PGE01005`)
*   `JM3Ex00110002` - Unmarked Execution Line (formerly `PGE01016`)
*   `JM3Ex00110003` - Wrong Block Element Marker (formerly `PGE01017`)
*   `JM3Ex00110004` - Empty Data Definition (formerly `PGE01021`)
*   `JM3Ex00110005` - Incompatible Operation Marker (formerly `PGE01024`)
*   `JM3Ex00110006` - Missing Trigger Boolean Output (formerly `PGE01032`)

*(This list is non-exhaustive and will grow as more syntax rules are migrated).*
