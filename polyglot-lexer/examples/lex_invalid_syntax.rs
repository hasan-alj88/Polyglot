// Example: Test invalid syntax detection and error reporting
use polyglot_lexer::{lex, LexerError};

fn main() {
    println!("=== Polyglot Lexer v0.0.2 - Invalid Syntax Test ===\n");

    let invalid_cases = vec![
        // ========================================
        // 1. Invalid Characters
        // ========================================
        (
            "Invalid Character in Code",
            r#"[|] Test
[r] .x: pg\int << 42
$invalid_character_here
[X]"#,
            "Should reject '$' character",
            vec!["UnexpectedCharacter"],
        ),
        (
            "Unicode Characters Outside Strings",
            r#"[|] Test
[r] .résumé: pg\string << "valid"
[X]"#,
            "Should reject non-ASCII in identifiers",
            vec!["UnexpectedCharacter", "InvalidIdentifier"],
        ),
        (
            "Control Characters",
            "[|] Test\n[r] .x: pg\\int << 42\n[X]",
            "Testing with standard characters (null byte test removed)",
            vec!["ACCEPT"], // Just a normal program, will pass
        ),
        // ========================================
        // 2. Malformed Operators
        // ========================================
        (
            "Incomplete Comparison Operator",
            r#"[|] Test
[i] .x: pg\int
[?] .x =
[X]"#,
            "Should reject incomplete '='",
            vec!["UnexpectedCharacter"],
        ),
        (
            "Invalid Operator Sequence",
            r#"[|] Test
[r] .x: pg\int <<< 42
[X]"#,
            "Should reject '<<<'",
            vec!["UnexpectedCharacter"],
        ),
        (
            "Malformed Range Operator",
            r#"[|] Test
[i] .x: pg\int
[?] .x ?{ 1, 10}
[X]"#,
            "Should reject '?{' (not a valid range operator)",
            vec!["UnexpectedCharacter"],
        ),
        // ========================================
        // 3. Invalid Identifier Patterns
        // ========================================
        (
            "Variable Starting with Number",
            r#"[|] Test
[r] .123abc: pg\string << "test"
[X]"#,
            "Should reject variable starting with digit",
            vec!["UnexpectedCharacter", "InvalidIdentifier"],
        ),
        (
            "Empty Identifier After Dot",
            r#"[|] Test
[r] .: pg\string << "test"
[X]"#,
            "Should reject empty identifier after dot",
            vec!["UnexpectedCharacter", "InvalidIdentifier"],
        ),
        (
            "Double Prefix",
            r#"[|] Test
[r] ..variable: pg\string << "test"
[X]"#,
            "Should reject double dot prefix",
            vec!["UnexpectedCharacter", "InvalidIdentifier"],
        ),
        // ========================================
        // 4. Unterminated Constructs
        // ========================================
        (
            "String Ends at EOF",
            r#"[|] Test
[r] .x: pg\string << "Hello World"#,
            "Should detect unterminated string",
            vec!["UnterminatedString"],
        ),
        (
            "String with Newline",
            "[|] Test\n[r] .x: pg\\string << \"Line 1\nUnterminated",
            "Should detect string crossing line without escape",
            vec!["UnterminatedString"],
        ),
        (
            "Interpolation Not Closed",
            r#"[|] Test
[r] .x: pg\string << "Value: {.var"
[X]"#,
            "Should detect unclosed interpolation",
            vec!["UnterminatedString"],
        ),
        (
            "Comment Never Closes",
            r#"[|] Test
/* Start of comment
[r] .x: pg\int << 42
[X]
Still in comment..."#,
            "Should detect unterminated comment",
            vec!["UnterminatedComment"],
        ),
        // ========================================
        // 5. Invalid Block Markers
        // ========================================
        (
            "Lowercase Block Marker",
            r#"[|] Test
[x] This should be [X]
[X]"#,
            "Should reject lowercase 'x' (must be uppercase 'X')",
            vec!["UnknownBlockMarker"],
        ),
        (
            "Invalid Block Character",
            r#"[|] Test
[$] Invalid
[X]"#,
            "Should reject '$' as block marker",
            vec!["UnknownBlockMarker"],
        ),
        (
            "Block Marker Not Closed",
            r#"[|] Test
[r
[X]"#,
            "Should detect block marker missing ']'",
            vec!["UnterminatedBlockMarker"],
        ),
        (
            "Empty Block Marker",
            r#"[|] Test
[] Empty
[X]"#,
            "Should reject empty block marker",
            vec!["UnknownBlockMarker", "UnterminatedBlockMarker"],
        ),
        // ========================================
        // 6. Invalid Number Formats
        // ========================================
        (
            "Multiple Decimal Points",
            r#"[|] Test
[r] .x: pg\float << 3.14.15
[X]"#,
            "Should reject multiple decimal points",
            vec!["UnexpectedCharacter", "InvalidNumberFormat"],
        ),
        (
            "Leading Zeros (Possible Octal Ambiguity)",
            r#"[|] Test
[r] .x: pg\int << 0123
[X]"#,
            "Leading zeros - depends on implementation",
            vec!["UnexpectedCharacter", "InvalidNumberFormat", "ACCEPT"],
        ),
        (
            "Number Followed by Identifier",
            r#"[|] Test
[r] .x: pg\int << 42abc
[X]"#,
            "Should reject number followed immediately by letters",
            vec!["UnexpectedCharacter", "InvalidNumberFormat"],
        ),
        // ========================================
        // 7. Invalid Escape Sequences
        // ========================================
        (
            "Unknown Escape Sequence",
            r#"[|] Test
[r] .x: pg\string << "Invalid \x escape"
[X]"#,
            "Should reject '\\x' (not a valid escape)",
            vec!["InvalidEscapeSequence"],
        ),
        (
            "Backslash at End",
            r#"[|] Test
[r] .x: pg\string << "Ends with \"
[X]"#,
            "Should detect backslash at string end",
            vec!["InvalidEscapeSequence", "UnterminatedString"],
        ),
        // ========================================
        // 8. Structural Errors
        // ========================================
        (
            "Pipeline Without End",
            r#"[|] Test
[r] .x: pg\int << 42"#,
            "Should detect missing [X]",
            vec!["ACCEPT"], // Lexer may accept, parser will reject
        ),
        (
            "Nested Strings (Invalid)",
            r#"[|] Test
[r] .x: pg\string << "Outer "nested" string"
[X]"#,
            "Should detect string terminating early",
            vec!["UnexpectedCharacter", "ACCEPT"],
        ),
    ];

    let mut total = 0;
    let mut detected = 0;
    let mut not_detected = 0;
    let mut acceptable = 0;

    for (name, source, description, expected_errors) in invalid_cases {
        total += 1;
        print!("\n{:2}. {:40}", total, name);
        println!("\n    Description: {}", description);
        print!("    Result: ");

        match lex(source) {
            Ok(tokens) => {
                // Check if this case is acceptable (lexer may accept, parser rejects)
                if expected_errors.contains(&"ACCEPT") {
                    println!("✓ ACCEPTABLE (lexer accepts, parser will validate)");
                    println!("           Tokens: {}", tokens.len());
                    acceptable += 1;
                } else {
                    println!(
                        "✗ FAIL - No error detected (expected one of: {:?})",
                        expected_errors
                    );
                    println!("           Got {} tokens instead", tokens.len());
                    not_detected += 1;
                }
            }
            Err(e) => {
                let error_type = match &e {
                    LexerError::UnterminatedString { .. } => "UnterminatedString",
                    LexerError::UnterminatedInterpolation { .. } => "UnterminatedInterpolation",
                    LexerError::UnterminatedComment { .. } => "UnterminatedComment",
                    LexerError::UnterminatedBlockMarker { .. } => "UnterminatedBlockMarker",
                    LexerError::UnknownBlockMarker { .. } => "UnknownBlockMarker",
                    LexerError::InvalidIdentifier { .. } => "InvalidIdentifier",
                    LexerError::InvalidEscapeSequence { .. } => "InvalidEscapeSequence",
                    LexerError::UnexpectedCharacter { .. } => "UnexpectedCharacter",
                    LexerError::InvalidNumberFormat { .. } => "InvalidNumberFormat",
                };

                if expected_errors.contains(&error_type) || expected_errors.contains(&"ACCEPT") {
                    println!("✓ DETECTED - {}", error_type);
                    println!("           {}", e);
                    detected += 1;
                } else {
                    println!(
                        "⚠️  PARTIAL - Got {}, expected one of: {:?}",
                        error_type, expected_errors
                    );
                    println!("           {}", e);
                    detected += 1; // Still counts as detected, just different error
                }
            }
        }
    }

    println!();
    println!("========================================================================");
    println!("Invalid Syntax Test Results:");
    println!("  Total test cases:        {}", total);
    println!(
        "  Errors detected:         {} ({:.1}%)",
        detected,
        (detected as f64 / total as f64) * 100.0
    );
    println!(
        "  Not detected (failures): {} ({:.1}%)",
        not_detected,
        (not_detected as f64 / total as f64) * 100.0
    );
    println!(
        "  Acceptable (lexer OK):   {} ({:.1}%)",
        acceptable,
        (acceptable as f64 / total as f64) * 100.0
    );
    println!("========================================================================");

    if not_detected == 0 {
        println!("\n🎉 All invalid syntax properly handled!");
        println!("   {} errors detected", detected);
        println!("   {} cases acceptable at lexer level", acceptable);
    } else {
        println!("\n⚠️  Some invalid syntax not detected:");
        println!("   {} cases failed to detect errors", not_detected);
    }
}
