// Example: Test error detection and reporting
use polyglot_lexer::{lex, LexerError};

fn main() {
    println!("=== Polyglot Lexer v0.0.2 - Error Detection Test ===\n");

    let error_cases = vec![
        (
            "Unterminated String",
            r#"[|] Test
[r] .x: pg\string << "Hello
[X]"#,
            "UnterminatedString"
        ),
        (
            "Unterminated Multi-line Comment",
            r#"[|] Test
/* This comment never ends
[r] .x: pg\int << 42
[X]"#,
            "UnterminatedComment"
        ),
        (
            "Unknown Block Marker",
            r#"[|] Test
[z] This is not valid
[X]"#,
            "UnknownBlockMarker"
        ),
        (
            "Unterminated Block Marker",
            r#"[|] Test
[@
[X]"#,
            "UnterminatedBlockMarker"
        ),
        (
            "Invalid Escape Sequence",
            r#"[|] Test
[r] .x: pg\string << "Invalid \x escape"
[X]"#,
            "InvalidEscapeSequence"
        ),
    ];

    let mut detected = 0;
    let mut not_detected = 0;
    let mut wrong_error = 0;

    for (name, source, expected_error) in error_cases {
        print!("Testing: {:35} ... ", name);

        match lex(source) {
            Ok(_) => {
                println!("✗ FAIL (no error detected)");
                not_detected += 1;
            }
            Err(e) => {
                let error_type = match e {
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

                if error_type == expected_error {
                    println!("✓ PASS ({})", error_type);
                    detected += 1;
                } else {
                    println!("⚠️  PARTIAL (expected {}, got {})", expected_error, error_type);
                    wrong_error += 1;
                }
            }
        }
    }

    println!();
    println!("==========================================");
    println!("Error Detection Test Results:");
    println!("  Correctly detected: {}", detected);
    println!("  Wrong error type:   {}", wrong_error);
    println!("  Not detected:       {}", not_detected);
    println!("  Total tests:        {}", detected + wrong_error + not_detected);
    println!("==========================================");

    if not_detected == 0 && wrong_error == 0 {
        println!("\n🎉 All errors correctly detected and categorized!");
    } else if not_detected == 0 {
        println!("\n✓ All errors detected (some with different categorization)");
    } else {
        println!("\n⚠️  Some errors not detected");
    }
}
