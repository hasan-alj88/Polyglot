---
description: Train the model on the latest Polyglot docs, run a QA loop on the Lexer, and update system constraints
---

<objective>
Analyze the current state of Polyglot documentation and code, proactively uncover invalid patterns in the Lexer via generated synthetic edge cases, and update systemic project documentation/memory.
</objective>

<process>
Follow these sequential phases. **Do not skip explicitly defined pauses.**

1. **Phase 1: Generation & Baseline**
   - Review `docs/development-guide.md` and other recent documentation changes.
   - The AI must generate a synthetic `test_qa.pg` file in `lib/polyglot/tests/fixtures/qa/` (create the directory if it doesn't exist).
   - This `.pg` file must include both formally correct Polyglot syntax and intentionally complex, ambiguous, or invalid syntax blocks to stress-test the lexer accurately.
   // turbo-all
   - The AI runs the lexer via `cargo run --bin polyglot -- --lexer -c tests/fixtures/qa/test_qa.pg -t tests/fixtures/qa/test_qa.pgts` within the `lib/polyglot` directory.
   - The AI *explicitly PAUSES* and yields control immediately to the User to complete Phase 2.

2. **Phase 2: User Calibration (Manual)**
   - The User manually inspects the newly generated `test_qa.pg` and `test_qa.pgts`.
   - The User corrects the syntax directly in `test_qa.pg` and creates the exact "ground truth" token mapping in `test_qa.pgts`. 
   - The User resumes the AI by replying with instructions such as "I have corrected the syntax and token stream. Proceed to diff."

3. **Phase 3: Differential Analysis**
   - The AI regenerates the token stream inside a temporary output file `test_qa_ai.pgts` (do NOT overwrite the user's `test_qa.pgts`).
   - The AI performs a step-by-step comparative diff between its `test_qa_ai.pgts` map and the User's baseline `test_qa.pgts` map to find token differences or logic failures.
   - The AI lists out exactly what structural logic failed or was missed. 
   - The AI *explicitly PAUSES* and asks the User to confirm the planned changes.

4. **Phase 4: Engine Refinement**
   - After confirmation, the AI surgically modifies `token.rs`, `engine.rs`, and/or `patterns.rs` to close the gap.
   // turbo-all
   - The AI runs `cargo test` and the CLI generator again to confirm 100% equivalence, resolving any compilation errors if necessary.

5. **Phase 5: Documentation & Memory Integration**
   - The AI extracts the specific definitions, edge cases, and formatting quirks discovered during the run.
   - The AI updates the internal documentation and definitions (such as `docs/`) so future generations of Polyglot natively comply with the new standard.
   - Summarize what was learned and applied during this training step.
</process>
