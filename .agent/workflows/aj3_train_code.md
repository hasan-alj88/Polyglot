---
description: Train the model on Aljam3 code syntax and semantics.
---

<objective>
Generate Aljam3 code scenarios, review with the user, and add to valid or invalid examples to train the model's understanding of the language syntax.
</objective>

<process>
Follow these sequential steps. **Do not skip explicitly defined pauses.**

1. **Phase 1: Generation**
   - First, read `docs/training_lessons/INDEX.md` and review the documented lessons to ensure the generated code incorporates past learning and avoids previously corrected mistakes.
   - Generate a complete Aljam3 (`.jm3`) file based on a random scenario if the user has not specified one.
   - Save the generated file to the disk in a temporary location (e.g., `lib/aljam3/tests/fixtures/qa/test_qa.jm3`) for review.

2. **Phase 2: User Review**
   - Inform the user of the generated file's path for their review.
   - *PAUSE* and explicitly wait for the user's feedback or acceptance.

3. **Phase 3: Acceptance & Documentation**
   - If the user **accepts** the code:
     - Move or save the accepted code into the valid examples folder (`lib/aljam3/tests/fixtures/examples/valid/`).
     - Update the relevant documentation to reference or include the valid example.
   - If the code is **incorrect**:
     - Inspect the user's review comments and confirm the lessons learned.
     - Document the lessons learned in a highly modular way (strictly 1 lesson = 1 file) as new markdown files in `docs/training_lessons/` and link each of them in `docs/training_lessons/INDEX.md`.
     - Save the original incorrect code (with comments explaining why it is wrong) into the invalid examples folder (`lib/aljam3/tests/fixtures/examples/invalid/`).
     - Save the corrected version of the code to the valid examples folder (`lib/aljam3/tests/fixtures/examples/valid/`).
     - Check if the lessons learned are consistent with the current documentation and recommend changes.
     - Ask questions to the user to ensure a full picture of what to update in the docs.

4. **Phase 4: Next Steps**
   - Repeat the workflow for a new scenario, close the workflow, or transition to the `jm3_train_lexer` workflow using the newly generated examples.
</process>
