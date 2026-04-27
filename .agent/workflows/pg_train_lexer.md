---
description: Train the model on the Polyglot lexer by validating token streams.
---

<objective>
Apply the lexer to Polyglot code, review the token stream with the user, and iteratively correct the lexer engine or documentation based on lessons learned.
</objective>

<process>
Follow these sequential steps. **Do not skip explicitly defined pauses.**

1. **Phase 1: Lexical Analysis**
   - Apply the lexer to Polyglot code (either from the `pg_train_code` workflow or from an existing Polyglot file).
   // turbo-all
   - Generate the token stream and prepare it for review.

2. **Phase 2: User Review**
   - Submit the token stream for the user's review.
   - *PAUSE* and wait for the user to accept or reject the stream.

3. **Phase 3: Acceptance**
   - If the user **accepts**:
     - Save the valid `.pgts` token stream file to `lib/polyglot/tests/fixtures/token_streams/valid/`.
     - Close the workflow or start another round.

4. **Phase 4: Correction & Documentation**
   - If the token stream is **incorrect**:
     - Read the review comments and confirm the lessons learned.
     - Document the specific lesson learned as a new markdown file in `docs/training_lessons/` and link it in `docs/training_lessons/INDEX.md`.
     - Save the incorrect `.pgts` token stream (with comments explaining why it is wrong) to `lib/polyglot/tests/fixtures/token_streams/invalid/`.
     - Check the consistency of the lessons in both the codebase and the documentation.
     - Make a plan to update them for the user's approval.
     - *PAUSE* and wait for the user's approval on the plan.
     - After corrections are applied, regenerate the lexer output and confirm its correctness.
     - Save the final, corrected `.pgts` token stream to `lib/polyglot/tests/fixtures/token_streams/valid/`.
     - Close the workflow or start another round.
</process>
