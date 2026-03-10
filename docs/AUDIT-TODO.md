# v0.0.4 Documentation Audit - TODO List

**Generated:** 2025-12-16
**Total Items:** 9 (2 CRITICAL, 3 MAJOR, 4 MINOR)

---

## CRITICAL - Must Fix Immediately

- [ ] **`[V]` vs `[v]`** | `_archive/design-history/loop-system/*.md` (7 files) | **COLLISION** | Archive files use uppercase `[V]` join marker while v0.0.4 spec uses lowercase `[v]`. Add prominent deprecation warnings: "⚠️ v0.0.3 syntax - v0.0.4 uses `[v]` not `[V]`". Verify NO production files use uppercase variant.

- [ ] **`#True` alias** | `language/advanced/inline-pipelines.md:455`, spec-wide | **DUPLICATE + AMBIGUITY** | Claims `#True` is alias for `#;Boolean;True` but NO formal `{A}` alias definition exists. **DECISION REQUIRED:** (1) Formalize alias and create `{A} #Boolean` block with mappings, OR (2) Remove alias claim and standardize all examples to `#;Boolean;True` full form only.

---

## MAJOR - Fix Soon

- [ ] **`%Inline.FormattedString`** | `stdlib/utilities/README.md`, `getting-started/core-principles.md`, `language/syntax/prefix-system.md` | **AMBIGUITY** | Special compiler-populated variable only documented in 2 files (`inline-pipelines.md`, `pipeline-structure.md`). Add documentation section to stdlib utilities README explaining how inline pipelines receive formatted strings. Add references to getting-started and prefix-system docs.

- [ ] **Pipeline chaining** | `language/control-flow/pipeline-structure.md:768-987` | **CONFLICT (verify)** | Manual verification needed: ensure ALL multi-pipeline chains use one segment per line format. Search for any remaining `[r] |A |> |B |> |C` single-line patterns that violate "one line = one marker + one expression" rule.

- [ ] **Boolean enum standardization** | `language/syntax/operators.md` (37×), spec-wide (89× full, 15× short) | **DUPLICATE** | Depends on CRITICAL `#True` alias decision. IF alias formalized: standardize examples to use `#True` for brevity. IF alias rejected: update all references to use `#;Boolean;True` exclusively. Create style guide entry.

---

## MINOR - Enhancements

- [ ] **Archive deprecation warnings** | `_archive/design-history/**/*.md` (11 files) | **AMBIGUITY** | Add to top of ALL archive files: "⚠️ **HISTORICAL DOCUMENT** - This file contains v0.0.3 syntax. For v0.0.4, see main docs (README.md)". Prevents outdated syntax copying.

- [ ] **Grammar expansion (optional)** | `reference/grammar.md:314-341` | **AMBIGUITY (enhancement)** | Inline template grammar correct but could expand: (1) Add explicit `literal_part ::= any_character_except_brace`, (2) Enumerate format_specifier options: `"hex" | "json" | "iso8601" | identifier`, (3) Add nested substitution examples.

- [x] **Serial type case check** | spec-wide | **VERIFICATION** | ✅ VERIFIED CLEAN - All serial type annotations use lowercase `:pg.serial`. No instances of `:pg.Serial` or `:Serial` found. Close as verified.

- [x] **Inline comma placement** | `stdlib/utilities/math/*.md`, spec-wide (47 examples) | **VERIFICATION** | ✅ VERIFIED CLEAN - All inline calls use consistent comma placement: `"{$x}, {$y}"` (comma OUTSIDE braces, INSIDE template string). Pattern documented in `inline-pipelines.md:154`. Close as verified.

---

## Implementation Priority

### Phase 1: Critical Decisions (1-2 items, blocks others)
1. ☐ **Decide:** Formalize `#True` alias or reject it (CRITICAL #2)
2. ☐ **Verify:** No production code uses `[V]` uppercase (CRITICAL #1)

### Phase 2: Critical Implementations (depends on Phase 1)
3. ☐ **Create:** `{A} #Boolean` alias definition (if decision = YES)
4. ☐ **Update:** All archive files with deprecation warnings (CRITICAL #1)
5. ☐ **Standardize:** Boolean enum usage spec-wide (MAJOR #3, depends on item 1)

### Phase 3: Major Fixes
6. ☐ **Expand:** `%Inline.FormattedString` documentation (MAJOR #1)
7. ☐ **Review:** Pipeline chaining in pipeline-structure.md (MAJOR #2)

### Phase 4: Minor Enhancements
8. ☐ **Optional:** Enhance grammar file (MINOR #2)

### Phase 5: Verification
9. ☑ **Close:** Serial type verification (MINOR #3) ✅
10. ☑ **Close:** Comma placement verification (MINOR #4) ✅

---

## Detailed TODO Items

### CRITICAL-001: Uppercase Join Marker in Archives

**Symbol:** `[V]` (uppercase) vs `[v]` (lowercase)

**Files:**
- `_archive/design-history/loop-system/v0.0.3.1-loop-system-specification.md` (21× `[V]`)
- `_archive/design-history/loop-system/loop-io-mini-pipelines.md` (13× `[V]`)
- `_archive/design-history/loop-system/loop-unpack-pack-final-design.md` (18× `[V]`)
- `_archive/design-history/loop-system/v0.0.3.1-blind-spots-analysis.md` (12× `[V]`)
- `_archive/design-history/loop-system/loop-pack-unpack-improvements.md` (9× `[V]`)
- `_archive/design-history/loop-system/README.md` (2× `[V]`)
- `_archive/design-history/syntax-refinement/archive/v0.0.4-final-decisions.md` (1× `[V]`)

**Type:** COLLISION

**Action Items:**
1. Run verification command:
   ```bash
   grep -r "\[V\]" --include="*.md" --exclude-dir="_archive" docs/User/specifications/v0.0.4/
   ```
   Expected: No results (confirms no production code uses uppercase)

2. Add to top of each affected archive file:
   ```markdown
   > ⚠️ **HISTORICAL DOCUMENT - DEPRECATED SYNTAX**
   >
   > This document contains **v0.0.3 syntax** including uppercase `[V]` join marker.
   >
   > **v0.0.4 uses lowercase `[v]`** for join operations.
   >
   > For current syntax, see v0.0.4 documentation (README.md).
   ```

3. Update archive README files to list syntax changes

**Verification:**
```bash
# After fix, verify warnings added:
head -n 10 docs/User/specifications/v0.0.4/_archive/design-history/loop-system/*.md | grep "DEPRECATED"
```

**References:**
- `language/syntax/markers.md:262` - Canonical `[v]` definition
- `reference/grammar.md:135` - Grammar defines lowercase only

---

### CRITICAL-002: Missing Boolean Alias Definition

**Symbol:** `#True` / `#False`

**Files:**
- `language/advanced/inline-pipelines.md:455` - Claims alias exists
- `language/advanced/inline-pipelines.md:442` - Uses `#True` in example
- `language/control-flow/pipeline-structure.md:818` - Uses `#True` in example
- Spec-wide: 89× full form `#;Boolean;True`, 15× short form `#True` in archives

**Type:** DUPLICATE + AMBIGUITY

**Decision Required:**

**OPTION A: Formalize the Alias**
1. Create alias definition file or add to reserved enum section:
   ```polyglot
   {A} #Boolean
   [A] #;Boolean;True >> #True
   [A] #;Boolean;False >> #False
   {x}
   ```

2. Location options:
   - Add to `language/types/enums-serial.md` after reserved enum definition
   - Create new `language/advanced/reserved-aliases.md`
   - Add to stdlib as `stdlib/aliases.pg` reference

3. Update references:
   - Confirm alias in `language/advanced/reserved-indication.md`
   - Add to `reference/ai-context.md` quick reference
   - Update grammar to note shorthand forms

**OPTION B: Reject the Alias**
1. Remove alias claims:
   - Update `language/advanced/inline-pipelines.md:455` to remove alias statement
   - Change example at line 442 from `#True` to `#;Boolean;True`

2. Update `language/control-flow/pipeline-structure.md:818` example

3. Standardize all examples to full form `#;Boolean;True`

**Recommended:** OPTION A (formalize) - shorter form improves readability, matches other language conventions

**Action Items:**
1. **Make decision** (document in decision log)
2. IF OPTION A:
   - Create alias definition
   - Update related documentation
   - Create validation test
3. IF OPTION B:
   - Remove alias claims
   - Update 2-3 examples
   - Standardize style guide

**Verification:**
```bash
# If OPTION A chosen:
grep -r "{A} #Boolean" --include="*.md" docs/User/specifications/v0.0.4/
# Expected: 1 result showing alias definition

# If OPTION B chosen:
grep -r "#True\>" --include="*.md" --exclude-dir="_archive" docs/User/specifications/v0.0.4/
# Expected: 0 results (all replaced with #;Boolean;True)
```

---

### MAJOR-001: Expand %Inline.FormattedString Documentation

**Symbol:** `%Inline.FormattedString`

**Files Currently Documenting:**
- `language/advanced/inline-pipelines.md:345` (complete)
- `language/control-flow/pipeline-structure.md:750` (brief mention)

**Files That Should Reference:**
- `stdlib/utilities/README.md` - Main utilities overview
- `getting-started/core-principles.md` - Inline syntax introduction
- `language/syntax/prefix-system.md` - Inline pipeline section

**Type:** AMBIGUITY (incomplete documentation)

**Action Items:**

1. **Update `stdlib/utilities/README.md`** (after line 79):
   ```markdown
   ### How Inline Calls Work

   When you use inline syntax like `|U.Math.Add"{$x}, {$y}"`:

   1. **Phase 1:** Variables converted to strings (parallel):
      - `$x` → `|U.String.Polyglot.Int.Default`
      - `$y` → `|U.String.Polyglot.Int.Default`

   2. **Phase 2:** Substitution into template:
      - Result: `"5, 3"` (example values)

   3. **Phase 3:** Formatter pipeline receives:
      - Special variable `%Inline.FormattedString` contains `"5, 3"`
      - Formatter parses and outputs to main pipeline's inputs

   **See:** Complete inline pipelines documentation (User/language/advanced/inline-pipelines.md)
   ```

2. **Update `getting-started/core-principles.md`** (after line 506):
   ```markdown
   **How it works behind the scenes:**

   The string `"{$num}"` is a **formatted string template**. The compiler:
   1. Converts `$num` to its string representation
   2. Substitutes into template
   3. Passes result to formatter pipeline via `%Inline.FormattedString` special variable
   4. Formatter parses and outputs to main pipeline

   This is NOT simple argument passing—it's a three-phase execution model.
   ```

3. **Update `language/syntax/prefix-system.md`** (after line 419):
   ```markdown
   ### The %Inline.FormattedString Special Variable

   Formatter pipelines receive the formatted string via compiler-populated variable:

   ```polyglot
   {|} |Pipeline.FormattedString.For.Pipeline
   [|] <formatted_string:pg.string << %Inline.FormattedString
   ```

   This special variable is ONLY available in formatter pipelines registered with `%Inline` metadata.
   ```

**Verification:**
```bash
# After updates:
grep -r "%Inline.FormattedString" --include="*.md" docs/User/specifications/v0.0.4/
# Expected: 5+ results (original 2 + new 3)
```

---

### MAJOR-002: Verify Pipeline Chaining Syntax

**Symbol:** `|>` (pipeline composition operator)

**Files:** `language/control-flow/pipeline-structure.md:768-987`

**Type:** CONFLICT (needs verification)

**Issue:**
Recent fix corrected most multi-pipeline chains from single-line to multi-line format. One file requires manual review to ensure ALL instances follow correct pattern.

**Incorrect Pattern:**
```polyglot
[r] |Step1 |> |Step2 |> |Step3  // ❌ Multiple chains on one line
```

**Correct Pattern:**
```polyglot
[r] |Step1 |> |Step2                      // Chain Step1 → Step2
[|] <input << $value
[|] >output1 >> <input2                  // Step1 output → Step2 input
[|] |> |Step3                             // Chain Step2 → Step3
[|] >output2 >> <input3
[|] |>
[|] >final >> $result
```

**Action Items:**

1. **Manual Review** of `language/control-flow/pipeline-structure.md`:
   - Open file, navigate to lines 768-987
   - Find all instances of `|>` operator
   - For each instance:
     - Count how many pipeline names appear on same line
     - If > 1 pipeline name, split to multi-line format
     - Verify I/O parameter mappings are explicit

2. **Search Pattern:**
   ```bash
   # Find potential violations:
   grep -n "\[r\].*|>.*|>" docs/User/specifications/v0.0.4/language/control-flow/pipeline-structure.md
   ```

3. **Automated Check:**
   ```bash
   # Should return NO results after fix:
   grep -r "^\[r\].*|>.*|>.*|>" --include="*.md" --exclude-dir="_archive" docs/User/specifications/v0.0.4/
   ```

**Verification:**
- Read through pipeline composition section (lines 768-987)
- Verify each chaining example follows multi-line format
- Verify explicit output → input mappings with comments
- Confirm consistency with core principle: "one line = one marker + one expression"

---

### MAJOR-003: Standardize Boolean Enum Usage

**Symbol:** `#;Boolean;True` vs `#True`

**Files:**
- `language/syntax/operators.md` - 37× full form
- Spec-wide: 89× full form, 15× short form (archives)
- `language/advanced/inline-pipelines.md:442` - 1× short form
- `language/control-flow/pipeline-structure.md:818` - 1× short form

**Type:** DUPLICATE

**Depends On:** CRITICAL-002 decision

**Action Items:**

**IF CRITICAL-002 = OPTION A (alias formalized):**
1. Create style guide entry:
   ```markdown
   ## Boolean Enum Usage

   **Full Form:** `#;Boolean;True` / `#;Boolean;False`
   **Short Form:** `#True` / `#False` (via alias)

   **Style:**
   - Use FULL form in: definitions, formal specifications, type examples
   - Use SHORT form in: code examples, inline expressions, %Inline.Output
   ```

2. Update examples for consistency:
   - Keep `language/syntax/operators.md` as full form (formal spec)
   - Change simple examples to short form where appropriate
   - Update `%Inline.Output << #True` examples to use short form

**IF CRITICAL-002 = OPTION B (alias rejected):**
1. Update 2 instances to full form:
   - `language/advanced/inline-pipelines.md:442`: `#True` → `#;Boolean;True`
   - `language/control-flow/pipeline-structure.md:818`: `#True` → `#;Boolean;True`

2. Document in style guide:
   ```markdown
   ## Boolean Enum Usage

   **Always use full form:** `#;Boolean;True` / `#;Boolean;False`

   The short forms `#True` / `#False` are NOT valid syntax.
   ```

**Verification:**
```bash
# Count usage after standardization:
grep -r "#;Boolean;True" --include="*.md" --exclude-dir="_archive" docs/User/specifications/v0.0.4/ | wc -l
grep -r "#True\>" --include="*.md" --exclude-dir="_archive" docs/User/specifications/v0.0.4/ | wc -l

# If OPTION A: Both should have counts
# If OPTION B: Second should be 0
```

---

### MINOR-001: Add Archive Deprecation Warnings

**Files:** All files in `_archive/design-history/` (11 files)

**Type:** AMBIGUITY (documentation clarity)

**Action Items:**

1. Create warning template:
   ```markdown
   ---
   > ⚠️ **HISTORICAL DOCUMENT - DEPRECATED SYNTAX**
   >
   > This document contains syntax and design decisions from v0.0.3 or earlier.
   >
   > **Syntax Changes in v0.0.4:**
   > - `[V]` → `[v]` (lowercase join marker)
   > - Additional prefix system changes
   > - Reserved indication with semicolon
   >
   > **For current v0.0.4 syntax, see:**
   > - Main Documentation (README.md)
   > - v0.0.4 Grammar (User/reference/grammar.md)
   > - Syntax Complete (User/language/syntax/)
   ---
   ```

2. Add to each file:
   - `_archive/design-history/loop-system/v0.0.3.1-loop-system-specification.md`
   - `_archive/design-history/loop-system/loop-io-mini-pipelines.md`
   - `_archive/design-history/loop-system/loop-unpack-pack-final-design.md`
   - `_archive/design-history/loop-system/v0.0.3.1-blind-spots-analysis.md`
   - `_archive/design-history/loop-system/loop-pack-unpack-improvements.md`
   - `_archive/design-history/loop-system/README.md`
   - `_archive/design-history/loop-system/variable-reassignment-pack-unpack.md`
   - `_archive/design-history/loop-system/pipelines-as-variables.md`
   - `_archive/design-history/syntax-refinement/archive/v0.0.4-final-decisions.md`
   - `_archive/design-history/syntax-refinement/archive/v0.0.4-complete-syntax.md`
   - `_archive/design-history/syntax-refinement/archive/v0.0.4-design-decisions-final.md`

3. Update `_archive/design-history/README.md` to list syntax changes

**Verification:**
```bash
# Check all files have warning:
for file in docs/User/specifications/v0.0.4/_archive/design-history/**/*.md; do
  if ! head -n 15 "$file" | grep -q "DEPRECATED SYNTAX"; then
    echo "Missing warning: $file"
  fi
done
```

---

### MINOR-002: Grammar Enhancement (Optional)

**File:** `reference/grammar.md:314-341`

**Type:** AMBIGUITY (enhancement)

**Current State:**
```ebnf
inline_args ::= formatted_string_template
formatted_string_template ::= '"' template_content '"'
template_content ::= { literal_part | substitution }
substitution ::= "{" ( variable_ref [ ":" format_specifier ] | literal | expression ) "}"
```

**Proposed Enhancements:**

1. **Add explicit literal_part:**
   ```ebnf
   literal_part ::= any_character_except_brace
   any_character_except_brace ::= (* Any character except '{' or '}' *)
   ```

2. **Enumerate format specifiers:**
   ```ebnf
   format_specifier ::= "hex"
                     | "json"
                     | "iso8601"
                     | "default"
                     | "rfc3339"
                     | identifier  (* Custom formats *)
   ```

3. **Add examples section:**
   ```ebnf
   (* Format Specifier Examples:
      {$value:hex}     - Hexadecimal integer
      {$data:json}     - JSON serialization
      {$time:iso8601}  - ISO 8601 timestamp
      {$text:default}  - Default string representation
   *)
   ```

**Action Items:**
1. Add explicit productions for `literal_part`
2. Expand `format_specifier` with known values
3. Add format specifier examples in comments
4. Cross-reference with `inline-pipelines.md` format registry

**Verification:**
- Grammar remains parseable
- Examples match actual implementation
- Cross-references are accurate

---

## Validation Scripts

Run these commands to verify fixes:

```bash
#!/bin/bash
# v0.0.4 Audit Validation

echo "=== CRITICAL-001: Verify no production code uses [V] ==="
grep -r "\[V\]" --include="*.md" --exclude-dir="_archive" docs/User/specifications/v0.0.4/
echo "Expected: No results"
echo ""

echo "=== CRITICAL-002: Check alias definition ==="
grep -r "{A} #Boolean" --include="*.md" docs/User/specifications/v0.0.4/
echo "Expected: 1 result if OPTION A, 0 if OPTION B"
echo ""

echo "=== MAJOR-001: Verify %Inline.FormattedString expansion ==="
grep -r "%Inline.FormattedString" --include="*.md" docs/User/specifications/v0.0.4/ | wc -l
echo "Expected: 5+ results (was 2)"
echo ""

echo "=== MAJOR-002: Verify no multi-chain on single line ==="
grep -r "^\[r\].*|>.*|>.*|>" --include="*.md" --exclude-dir="_archive" docs/User/specifications/v0.0.4/
echo "Expected: No results"
echo ""

echo "=== MINOR-001: Verify archive warnings ==="
head -n 10 docs/User/specifications/v0.0.4/_archive/design-history/loop-system/*.md | grep -c "DEPRECATED"
echo "Expected: 7+ (one per file)"
echo ""

echo "=== Verification Complete ==="
```

---

## Progress Tracking

**Updated:** 2025-12-16

### Critical
- [ ] 0/2 complete

### Major
- [ ] 0/3 complete

### Minor
- [x] 2/4 complete (verifications)

**Total:** 2/9 items complete (22%)

---

**End of TODO List**
