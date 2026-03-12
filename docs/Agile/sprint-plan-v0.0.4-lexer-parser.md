# Sprint Plan: v0.0.4 Lexer & Parser Implementation

**Plan Created:** 2025-12-29
**Epic:** Epic 13 - v0.0.4 Syntax Migration
**Stories:** 13.1 (Lexer), 13.2 (Parser)
**Total Estimated Effort:** 5-7 weeks
**Target Completion:** Q1 2026

---

## Executive Summary

This sprint plan outlines the implementation of the v0.0.4 lexer and parser based on the completed architecture documents by Winston (Architect). The plan is structured as two sequential phases with well-defined milestones and acceptance criteria.

**Key Deliverables:**
1. Complete v0.0.4 lexer with 124 token types
2. Complete v0.0.4 parser with full AST support
3. Comprehensive test suites (unit + integration)
4. Performance benchmarks meeting targets
5. Documentation updates

**Risk Mitigation:**
- Architecture is complete and detailed
- Clear acceptance criteria for each task
- Incremental testing throughout
- Regular checkpoints for course correction

---

## Phase 1: Lexer Implementation (Story 13.1)

**Duration:** 2-3 weeks
**Story:** [Story 13.1 - Complete v0.0.4 Lexer Implementation](./stories/13-1-lexer-v0.0.4-complete.md)
**Architecture:** [Lexer Architecture v0.0.4](./architecture/lexer-architecture-v0.0.4.md)

### Sprint 1: Foundation & Core Tokens (Week 1)

**Focus:** Set up lexer foundation and implement core tokenization

#### Week 1 - Days 1-2: Token Definitions & Setup
- [ ] Update `polyglot-lexer/src/token.rs` with 3 new token types
  - `PipelineComposition` (|>)
  - `Input` (<param)
  - `Output` (>param)
- [ ] Add `Indent` and `Dedent` token types
- [ ] Verify all 124 token types compile
- [ ] Write basic unit tests for each token type

**Deliverable:** All 124 token types defined and compiling

#### Week 1 - Days 3-4: Lookahead Implementation
- [ ] Implement `peek_char()` method
- [ ] Implement `peek_char_at(offset)` method (3-character lookahead)
- [ ] Implement `peek_string(len)` helper
- [ ] Write unit tests for lookahead functions
- [ ] Performance check: ensure <5% overhead

**Deliverable:** 3-character lookahead working correctly

#### Week 1 - Day 5: Pipeline Composition Disambiguation
- [ ] Implement `lex_pipe_or_composition()` method
- [ ] Handle `|>` (composition)
- [ ] Handle `|Pipeline` (call)
- [ ] Handle `|?` (logical or)
- [ ] Write comprehensive unit tests
- [ ] Test edge cases (invalid pipes)

**Deliverable:** Pipeline composition tokenizes correctly

**Sprint 1 Checkpoint:**
- All 124 tokens defined ✅
- 3-character lookahead working ✅
- Pipeline composition disambiguation working ✅

---

### Sprint 2: Operator Disambiguation (Week 2)

**Focus:** Implement all multi-character operator disambiguation

#### Week 2 - Days 1-2: Angle Bracket Operators
- [ ] Implement `lex_angle_bracket()` for `<`
  - Handle `<<<`, `<<`, `<~<`, `<~`
  - Handle `<=?`, `<!?`, `<?`
  - Handle `<param` (input parameter)
- [ ] Implement `lex_greater_than()` for `>`
  - Handle `>>>`, `>>`, `~>~`, `~>`
  - Handle `>=?`, `>!?`, `>?`
  - Handle `>param` (output parameter)
- [ ] Write comprehensive unit tests
- [ ] Test all operator combinations

**Deliverable:** All angle bracket operators tokenize correctly

#### Week 2 - Day 3: Block Marker Disambiguation
- [ ] Implement `lex_paren_or_io()` method
- [ ] Handle `(|)` (ParenPipeline)
- [ ] Handle `(~)` (ParenLoop)
- [ ] Handle `(*)` (ParenPack)
- [ ] Handle regular `(` (ParenOpen)
- [ ] Write unit tests for all cases

**Deliverable:** Block markers `[|]` vs `(|)` vs `(~)` vs `(*)` disambiguate correctly

#### Week 2 - Days 4-5: Metadata & Reserved Indication
- [ ] Implement metadata prefix `%` tokenization
- [ ] Handle `%metadata_name` pattern
- [ ] Implement semicolon `;` tokenization
- [ ] Add context tracking for reserved vs. plain semicolon
- [ ] Write unit tests
- [ ] Integration test with enum definitions

**Deliverable:** Metadata and semicolon tokenize correctly

**Sprint 2 Checkpoint:**
- All operators disambiguate correctly ✅
- Input/Output parameters tokenize ✅
- Metadata prefix working ✅

---

### Sprint 3: Indentation & Testing (Week 3)

**Focus:** Implement indentation tracking and comprehensive testing

#### Week 3 - Days 1-2: Indentation Tracking
- [ ] Add `in_loop_body` flag to lexer state
- [ ] Add `indent_stack` for tracking levels
- [ ] Implement `enter_loop_body()` method
- [ ] Implement `exit_loop_body()` method
- [ ] Implement `lex_newline()` with INDENT/DEDENT generation
- [ ] Implement `count_leading_whitespace()`
- [ ] Handle tab detection (error on tabs)
- [ ] Write unit tests for indentation

**Deliverable:** Indentation tracking works (loop bodies only)

#### Week 3 - Day 3: Error Handling
- [ ] Update `LexerError` enum with all error types
- [ ] Implement helpful error messages (line/column numbers)
- [ ] Add error recovery (skip invalid token, continue)
- [ ] Write tests for all error cases
- [ ] Test multiple errors in single file

**Deliverable:** Comprehensive error messages

#### Week 3 - Days 4-5: Integration Testing & Performance
- [ ] Write integration tests for pipeline composition examples
- [ ] Write integration tests for loop examples with indentation
- [ ] Write integration tests for trigger I/O wiring
- [ ] Set up performance benchmarks (criterion)
- [ ] Benchmark 1,000-line file (<100ms target)
- [ ] Benchmark 10,000-line file (<1s target)
- [ ] Memory profiling (<10MB for 10k lines)
- [ ] Fix any performance issues

**Deliverable:** All integration tests pass, performance targets met

**Phase 1 Complete Checkpoint:**
- All 124 token types working ✅
- All disambiguation algorithms working ✅
- Indentation tracking working ✅
- Error handling comprehensive ✅
- Integration tests passing ✅
- Performance targets met ✅

**Phase 1 Deliverables:**
- Complete v0.0.4 lexer implementation
- >95% code coverage
- All documentation examples lex correctly
- Performance benchmarks green

---

## Phase 2: Parser Implementation (Story 13.2)

**Duration:** 3-4 weeks
**Story:** [Story 13.2 - Complete v0.0.4 Parser Implementation](./stories/13-2-parser-v0.0.4-complete.md)
**Architecture:** [Parser Architecture v0.0.4](./architecture/parser-architecture-v0.0.4.md)
**Dependency:** Phase 1 (Lexer) must be 100% complete

### Sprint 4: AST Foundation (Week 4)

**Focus:** Define all AST types and basic parsing infrastructure

#### Week 4 - Days 1-2: AST Type Definitions
- [ ] Update `polyglot-parser/src/ast.rs` with new node types
  - `PipelineComposition` struct
  - `LoopStatement` struct
  - `EnumBlock` with `SerialLoad`
  - `ErrorBlock` with `ErrorPattern`
  - Updated `PipelineBlock` with `OutputParameter` and `WiringTarget`
- [ ] Add all supporting types (wirings, unpack/pack calls, etc.)
- [ ] Ensure all types derive `Debug, Clone, PartialEq`
- [ ] Add `Span` tracking to all nodes
- [ ] Compile and verify all types

**Deliverable:** All AST node types defined

#### Week 4 - Days 3-5: Parser Foundation & Error Recovery
- [ ] Set up parser structure with token stream
- [ ] Implement `parse()` entry point
- [ ] Implement `parse_program()` (package, enums, pipelines)
- [ ] Add error recovery infrastructure:
  - `synchronize()` method
  - `parse_with_recovery()` wrapper
  - `parse_tolerant()` for multiple error collection
- [ ] Implement helper methods:
  - `expect(token_kind)`
  - `check(token_kind)`
  - `advance()`
  - `peek()`
- [ ] Write basic unit tests

**Deliverable:** Parser foundation with error recovery

**Sprint 4 Checkpoint:**
- All AST types defined ✅
- Parser foundation working ✅
- Error recovery infrastructure in place ✅

---

### Sprint 5: Core Parsing (Week 5)

**Focus:** Implement parsing for core language constructs

#### Week 5 - Days 1-2: Pipeline Composition Parsing
- [ ] Implement `parse_pipeline_composition()` method
- [ ] Parse `[|] |>` marker
- [ ] Parse optional next pipeline name
- [ ] Implement `parse_output_to_input_wiring()` method
- [ ] Parse `(|) >output >> <input` syntax
- [ ] Handle chains with multiple steps
- [ ] Write unit tests for each case
- [ ] Integration test: complete composition chain

**Deliverable:** Pipeline composition parsing works

#### Week 5 - Days 3-5: Loop Statement Parsing
- [ ] Implement `parse_loop_statement()` method
- [ ] Parse `[p] ~ForEach.Array` syntax
- [ ] Implement `parse_unpack_call()` method
- [ ] Implement `parse_loop_params()` method (with `(~)`)
- [ ] Parse indented loop body (handle INDENT/DEDENT tokens)
- [ ] Parse `[*] *Into.Array` syntax
- [ ] Implement `parse_pack_call()` method
- [ ] Implement `parse_pack_params()` method (with `(*)`)
- [ ] Handle nested loops
- [ ] Write comprehensive unit tests
- [ ] Integration test: loop with nested error blocks

**Deliverable:** Loop statement parsing works

**Sprint 5 Checkpoint:**
- Pipeline composition parsing complete ✅
- Loop parsing with indentation complete ✅

---

### Sprint 6: Advanced Features (Week 6)

**Focus:** Enum blocks, error blocks, and type validation

#### Week 6 - Days 1-2: Enum Block Parsing
- [ ] Implement `parse_enum_block()` method
- [ ] Parse `{#} EnumName ... {x}` structure
- [ ] Implement `parse_serial_load()` method
- [ ] Parse `[s] yaml|json|toml "path"` syntax
- [ ] Implement `parse_field_accessor()` method
- [ ] Parse `[.] field.path` syntax
- [ ] Implement `parse_alias_declaration()` method
- [ ] Handle scope-wide `[s][!]` error handler
- [ ] Write unit tests
- [ ] Integration test: enum with multiple serial loads

**Deliverable:** Enum block parsing works

#### Week 6 - Days 3-4: Error Block Parsing
- [ ] Implement `parse_error_block()` method
- [ ] Parse `[!]` marker
- [ ] Implement `parse_error_handler()` method
- [ ] Implement `parse_error_pattern()` method
  - Specific: `#Error.FileNotFound`
  - Category: `#Error.File*`
  - Conversion: `#Error.* #Warning.*`
  - Wildcard: `*?`
- [ ] Handle multiple handlers in one block
- [ ] Write unit tests for all pattern types
- [ ] Integration test: complete error handling

**Deliverable:** Error block parsing works

#### Week 6 - Day 5: Type Validation
- [ ] Implement `parse_type()` method
- [ ] Validate `pg.` prefix requirement
- [ ] Implement nested type parsing (arrays)
- [ ] **CRITICAL:** Validate double `pg.` prefix for arrays
  - `pg.array.pg.string` ✅ correct
  - `pg.array.string` ❌ error
- [ ] Write unit tests for all type cases
- [ ] Test edge cases (missing prefix, wrong prefix)

**Deliverable:** Type validation with nested prefix checking

**Sprint 6 Checkpoint:**
- Enum block parsing complete ✅
- Error block parsing complete ✅
- Type validation working ✅

---

### Sprint 7: Resolution & Testing (Week 7)

**Focus:** Three-phase resolution and comprehensive testing

#### Week 7 - Days 1-2: Three-Phase Pipeline Resolution
- [ ] Implement Phase 1: Current file namespace
- [ ] Implement Phase 2: Same package files (with `[#]` ordering)
- [ ] Implement `validate_file_ordering()` method
- [ ] Detect duplicate `[#]` markers
- [ ] Implement Phase 3: External registry lookup
- [ ] Add helpful error messages with suggestions
- [ ] Write unit tests for each phase
- [ ] Integration test: multi-file compilation

**Deliverable:** Three-phase resolution working

#### Week 7 - Days 3-5: Integration Testing & Performance
- [ ] Write integration tests for all documentation examples:
  - Pipeline composition chains
  - Loops with nested error blocks
  - Enums with serial loads
  - Trigger I/O wiring
  - Multi-file compilation
- [ ] Set up performance benchmarks
- [ ] Benchmark 1,000-line file (<500ms target)
- [ ] Benchmark 10,000-line file (<5s target)
- [ ] Memory profiling (<20MB for 10k lines)
- [ ] Fix any performance issues
- [ ] Code coverage analysis (>95% target)
- [ ] Fix any missing test coverage

**Deliverable:** All integration tests pass, performance targets met

**Phase 2 Complete Checkpoint:**
- All parsing features working ✅
- Three-phase resolution working ✅
- Error recovery working ✅
- Integration tests passing ✅
- Performance targets met ✅

**Phase 2 Deliverables:**
- Complete v0.0.4 parser implementation
- >95% code coverage
- All documentation examples parse correctly
- Performance benchmarks green

---

## Success Criteria

### Must-Have (Required for Completion)

**Lexer (Story 13.1):**
- [ ] All 124 token types implemented and tested
- [ ] 3-character lookahead working
- [ ] All disambiguation algorithms working
- [ ] Indentation tracking working (loop bodies only)
- [ ] Metadata prefix and semicolon tokenizing correctly
- [ ] >95% unit test coverage
- [ ] All integration tests passing
- [ ] Performance: <100ms for 1,000 lines, <1s for 10,000 lines
- [ ] Memory: <10MB for 10,000 lines

**Parser (Story 13.2):**
- [ ] All AST node types implemented and tested
- [ ] Pipeline composition parsing working
- [ ] Loop parsing with indentation working
- [ ] Enum block parsing with serial loads working
- [ ] Error block parsing with pattern matching working
- [ ] Type validation with nested prefix working
- [ ] Three-phase pipeline resolution working
- [ ] Error recovery with synchronization working
- [ ] >95% unit test coverage
- [ ] All integration tests passing
- [ ] Performance: <500ms for 1,000 lines, <5s for 10,000 lines
- [ ] Memory: <20MB for 10,000 lines

### Should-Have (Nice to Have)

- [ ] IDE-friendly error messages (with suggestions)
- [ ] Performance better than targets (>10% faster)
- [ ] Code documentation (rustdoc comments)
- [ ] Example fixtures for all language features

### Could-Have (Future Improvements)

- [ ] Incremental parsing support
- [ ] Syntax tree visitor pattern
- [ ] Pretty-printer for formatted output
- [ ] Syntax highlighting data export

---

## Risk Management

### Identified Risks

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|------------|
| Indentation parsing complexity | Medium | High | Architecture clearly defines algorithm, extensive tests |
| Type validation edge cases | Medium | Medium | Comprehensive test suite with all type combinations |
| Performance targets not met | Low | Medium | Architecture optimized for performance, benchmarks early |
| Multi-file resolution bugs | Medium | Medium | Phase 2 resolution clearly documented, unit tests |
| Integration test failures | Low | High | Architecture based on verified Polly examples |

### Contingency Plans

**If indentation parsing is too complex:**
- Fall back to explicit block delimiters for loops in v0.0.4
- Defer indentation to v0.0.5

**If performance targets not met:**
- Profile and optimize hot paths
- Consider `logos` crate for lexer
- Implement caching for parser

**If multi-file resolution has issues:**
- Start with Phase 1 and 3 only
- Add Phase 2 in Story 13.3

---

## Communication Plan

### Daily Updates
- Progress on current task
- Blockers encountered
- Tests passing/failing

### Weekly Checkpoints
- Sprint completion status
- Updated time estimates
- Risk assessment

### Phase Completion Reviews
- Demo of working features
- Test coverage report
- Performance benchmark results
- Decision on proceeding to next phase

---

## Resources

### Architecture Documents
- [Lexer Architecture v0.0.4](./architecture/lexer-architecture-v0.0.4.md)
- [Parser Architecture v0.0.4](./architecture/parser-architecture-v0.0.4.md)

### Implementation Stories
- [Story 13.1 - Lexer Implementation](./stories/13-1-lexer-v0.0.4-complete.md)
- [Story 13.2 - Parser Implementation](./stories/13-2-parser-v0.0.4-complete.md)

### Reference Materials
- [v0.0.4 Grammar](../User/reference/grammar.md)
- [Polly Examples - Pipeline Composition](../../bmad-polly/data/memory/syntax/pipeline-composition.yaml)
- [Polly Examples - Trigger I/O](../../bmad-polly/data/memory/patterns/trigger-io-wiring.yaml)

### Test Data
- Documentation examples from user guides
- Polly-verified syntax patterns
- Edge case scenarios

---

## Progress Tracking

### Overall Progress
- **Week 1:** Lexer Foundation (Sprint 1)
- **Week 2:** Lexer Disambiguation (Sprint 2)
- **Week 3:** Lexer Indentation & Testing (Sprint 3)
- **Week 4:** Parser AST Foundation (Sprint 4)
- **Week 5:** Parser Core Parsing (Sprint 5)
- **Week 6:** Parser Advanced Features (Sprint 6)
- **Week 7:** Parser Resolution & Testing (Sprint 7)

### Milestone Tracking

| Milestone | Target Date | Status |
|-----------|-------------|--------|
| Lexer Foundation Complete | End of Week 1 | Pending |
| Lexer Disambiguation Complete | End of Week 2 | Pending |
| Phase 1 (Lexer) Complete | End of Week 3 | Pending |
| Parser AST Foundation Complete | End of Week 4 | Pending |
| Parser Core Parsing Complete | End of Week 5 | Pending |
| Parser Advanced Features Complete | End of Week 6 | Pending |
| Phase 2 (Parser) Complete | End of Week 7 | Pending |
| **Epic 13 Stories 13.1-13.2 Complete** | **Q1 2026** | **Pending** |

---

## Next Actions

1. **Begin Sprint 1 (Week 1):** Lexer Foundation & Core Tokens
2. **Set up development environment:** Ensure all dependencies installed
3. **Create feature branch:** `feature/epic-13-v0.0.4-lexer-parser`
4. **Start with Story 13.1, Task 1:** Update token definitions

---

**Sprint Plan Created By:** John (Product Manager)
**Architecture By:** Winston (Architect)
**Date:** 2025-12-29
**Status:** Ready to Begin
