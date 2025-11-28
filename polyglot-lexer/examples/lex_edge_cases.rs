// Example: Test edge cases and boundary conditions
use polyglot_lexer::lex;

fn main() {
    println!("=== Polyglot Lexer v0.0.2 - Edge Cases Test ===\n");

    let test_cases = vec![
        (
            "Empty Pipeline",
            r#"[|] EmptyPipeline
[i] #Pipeline.NoInput
[t] |T.Call
[W] |W.Polyglot.Scope
[o] !NoError
[X]"#,
            true
        ),
        (
            "Very Long Identifier",
            r#"[|] Pipeline
[i] #Pipeline.NoInput
[t] |T.Call
[W] |W.Polyglot.Scope
[r] .this_is_a_very_long_variable_name_that_exceeds_normal_length_expectations: pg\string << "test"
[o] !NoError
[X]"#,
            true
        ),
        (
            "Large Numbers",
            r#"[|] Numbers
[i] #Pipeline.NoInput
[t] |T.Call
[W] |W.Polyglot.Scope
[r] .big: pg\int << 999999999
[r] .negative: pg\int << -123456789
[r] .float: pg\float << 123.456789
[o] !NoError
[X]"#,
            true
        ),
        (
            "Multiple Newlines",
            r#"[|] Newlines
[i] #Pipeline.NoInput


[t] |T.Call


[W] |W.Polyglot.Scope


[o] !NoError
[X]"#,
            true
        ),
        (
            "Empty String",
            r#"[|] EmptyString
[i] #Pipeline.NoInput
[t] |T.Call
[W] |W.Polyglot.Scope
[r] .empty: pg\string << ""
[o] !NoError
[X]"#,
            true
        ),
        (
            "String Only Whitespace",
            r#"[|] Whitespace
[i] #Pipeline.NoInput
[t] |T.Call
[W] |W.Polyglot.Scope
[r] .spaces: pg\string << "   "
[o] !NoError
[X]"#,
            true
        ),
        (
            "Adjacent Operators",
            r#"[|] Operators
[i] .a: pg\int
[i] .b: pg\int
[t] |T.Call
[W] |W.Polyglot.Scope
[r] .x: pg\int << .a
[r] .y: pg\int << .b
[?] .x >? .y
[&] .x <? 100
[~][o] !NoError
[~]
[?] *?
[~][o] !NoError
[~]
[X]"#,
            true
        ),
        (
            "Deeply Nested Conditionals",
            r#"[|] Nested
[i] .x: pg\int
[t] |T.Call
[W] |W.Polyglot.Scope
[?] .x >? 0
[~][?] .x >? 10
[~][~][?] .x >? 20
[~][~][~][r] .msg: pg\string << "deep"
[~][~][~]
[~][~][?] *?
[~][~][~][r] .msg: pg\string << "medium"
[~][~][~]
[~][~]
[~][?] *?
[~][~][r] .msg: pg\string << "shallow"
[~][~]
[~]
[?] *?
[~][r] .msg: pg\string << "default"
[~]
[o] .msg: pg\string
[X]"#,
            true
        ),
    ];

    let mut passed = 0;
    let mut failed = 0;

    for (name, source, should_succeed) in test_cases {
        print!("Testing: {:30} ... ", name);

        match lex(source) {
            Ok(tokens) => {
                if should_succeed {
                    println!("✓ PASS ({} tokens)", tokens.len());
                    passed += 1;
                } else {
                    println!("✗ FAIL (expected error, got success)");
                    failed += 1;
                }
            }
            Err(e) => {
                if !should_succeed {
                    println!("✓ PASS (error detected: {})", e);
                    passed += 1;
                } else {
                    println!("✗ FAIL (unexpected error: {})", e);
                    failed += 1;
                }
            }
        }
    }

    println!();
    println!("==========================================");
    println!("Edge Case Test Results:");
    println!("  Passed: {}/{}", passed, passed + failed);
    println!("  Failed: {}/{}", failed, passed + failed);
    println!("==========================================");

    if failed == 0 {
        println!("\n🎉 All edge case tests passed!");
    } else {
        println!("\n⚠️  Some edge cases failed");
    }
}
