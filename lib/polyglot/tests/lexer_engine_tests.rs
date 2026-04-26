use polyglot::lexer::lexer_engine::lex;
use polyglot::lexer::token::PolyglotToken;

#[test]
fn test_lex_basic_pipeline() {
    let script = std::fs::read_to_string("tests/fixtures/basic_pipeline.pg").unwrap();
    let tokens = lex(&script);

    println!("\n=== Polyglot Token Stream ===");
    for t in &tokens {
        println!("[L{:02}:C{:02}] {:?}", t.line, t.col, t.value);
    }
    println!("=============================\n");

    assert!(!tokens.is_empty(), "Token stream should not be empty");
}

#[test]
fn test_lex_incorrect_indent() {
    let script = std::fs::read_to_string("tests/fixtures/incorrect_indent.pg").unwrap();
    let tokens = lex(&script);

    println!("\n=== Incorrect Indent Stream ===");
    for t in &tokens {
        println!("[L{:02}:C{:02}] {:?}", t.line, t.col, t.value);
    }
    println!("===============================\n");

    // Assertions for exact coordinates to prove algorithmic safety
    assert_eq!(tokens[0].value, PolyglotToken::Scope(1));
    assert_eq!(
        tokens[1].value,
        PolyglotToken::IncorrectIndent(" ".to_string())
    );
    assert_eq!(tokens[1].col, 4); // The extra space

    assert_eq!(tokens[5].value, PolyglotToken::Scope(0)); // Start of line 2
    assert_eq!(
        tokens[6].value,
        PolyglotToken::IncorrectIndent("\t".to_string())
    ); // The \t on line 2
}

#[test]
fn test_lex_comments() {
    let script = std::fs::read_to_string("tests/fixtures/comments.pg").unwrap();
    let tokens = lex(&script);
    println!("\n=== Polyglot Comments Stream ===");
    for t in &tokens {
        println!("[L{:02}:C{:02}] {:?}", t.line, t.col, t.value);
    }
    println!("================================\n");
}

// Merged comment functionality into test_lex_comments fixture.
// Removing redundant inline string test test_lex_comment_nested_patterns.

#[test]
fn test_lex_edge_cases() {
    let script = std::fs::read_to_string("tests/fixtures/edge_cases.pg").unwrap();
    let tokens = lex(&script);
    println!("\n=== Polyglot Edge Cases Stream ===");
    for t in &tokens {
        println!("[L{:02}:C{:02}] {:?}", t.line, t.col, t.value);
    }
    println!("======================================\n");

    // Assert ghost line (L02) skip
    assert_eq!(tokens[0].line, 3);

    // Assert contiguous InvalidPattern slurp for @@@
    assert_eq!(
        tokens[3].value,
        PolyglotToken::InvalidPattern("@@@".to_string())
    );

    // Assert missing spaces triggers standalone and pushes invalid operators!
    assert_eq!(tokens[4].value, PolyglotToken::Variable("var".to_string()));

    // Since there is NO space, the algorithm previously slurped `<<#Config._database`.
    // Now it correctly splits `<<` as PullFrom, `#Config.` as Data, and `_database` as InvalidIdentifier!
    assert_eq!(tokens[5].value, PolyglotToken::PullFrom);
    assert_eq!(tokens[6].value, PolyglotToken::Data("Config.".to_string()));
    assert_eq!(
        tokens[7].value,
        PolyglotToken::InvalidPattern("_database".to_string())
    );
}

#[test]
fn test_cli_execution() {
    use std::fs;
    use std::path::Path;
    use std::process::Command;

    let output_file = "tests/fixtures/cli_output_test.pgts";
    let _ = fs::remove_file(output_file); // Ensure clean state before start

    let status = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "polyglot",
            "--",
            "--lexer",
            "-c",
            "tests/fixtures/basic_pipeline.pg",
            "-t",
            output_file,
        ])
        .status()
        .expect("Failed to execute CLI binary via cargo");

    assert!(
        status.success(),
        "CLI command returned a non-zero exit code"
    );
    assert!(
        Path::new(output_file).exists(),
        "CLI failed to generate the .pgts output file"
    );

    let generated_content =
        fs::read_to_string(output_file).expect("Failed to read generated .pgts");

    // Assert that the CLI fundamentally parses and formats identically to the organic lex() core
    assert!(generated_content.contains("[L02:C01] ActionExecSeq"));
    assert!(generated_content.contains("Pipeline(\"Transform.Data\")"));

    // Cleanup organic test artifact
    let _ = fs::remove_file(output_file);
}
#[test]
fn test_lex_new_tokens() {
    let script =
        "[T] -T.Daily\"3AM\"\n[*] *All\n(-) >SendStatus >> >SendEmails\n\"price is {$Price}$\"";
    let tokens = lex(script);

    let mut found_trigger = false;
    let mut found_trigger_string = false;
    let mut found_collector = false;
    let mut found_substitute = false;
    let mut found_push = false;

    for t in tokens {
        match t.value {
            PolyglotToken::InlineInstruction(ref id) if id == "T.Daily" => found_trigger = true,
            PolyglotToken::InlineString(ref s) if s == "3AM" => found_trigger_string = true,
            PolyglotToken::ActionCollector => found_collector = true,
            PolyglotToken::SubstituteVariable(ref v) if v == "Price" => found_substitute = true,
            PolyglotToken::PushInto => found_push = true,
            _ => {}
        }
    }

    assert!(found_trigger, "InlineInstruction missing");
    assert!(found_trigger_string, "InlineString missing");
    assert!(found_collector, "ActionCollector missing");
    assert!(found_substitute, "SubstituteVariable missing");
    assert!(found_push, "PushInto missing");
}

#[test]
fn test_lex_advanced_operators() {
    // Includes Default IO, Constructor with inline string, compression, ranging, and new markers
    let script = "(#) <~ $DT\"Now\" ~>\n[~] =? >!? *?\n(=) ?[1,10)";
    let tokens = lex(script);

    let mut found_data_input = false;
    let mut found_default_pull = false;
    let mut found_constructor = false;
    let mut found_inline_str = false;
    let mut found_default_push = false;

    let mut found_continue_action = false;
    let mut found_is_equal = false;
    let mut found_is_not_gr = false;
    let mut found_otherwise = false;

    let mut found_expander_io = false;
    let mut found_range_inc_from = false;
    let mut found_range_from_val = false;
    let mut found_range_sep = false;
    let mut found_range_to_val = false;
    let mut found_range_exc_to = false;

    for t in tokens {
        match t.value {
            PolyglotToken::DataInput => found_data_input = true,
            PolyglotToken::DefaultPullFrom => found_default_pull = true,
            PolyglotToken::Constructor(ref id) if id == "DT" => found_constructor = true,
            PolyglotToken::InlineString(ref s) if s == "Now" => found_inline_str = true,
            PolyglotToken::DefaultPushInto => found_default_push = true,

            PolyglotToken::ContinueActionLine => found_continue_action = true,
            PolyglotToken::IsItEqual => found_is_equal = true,
            PolyglotToken::IsItNotGreaterThan => found_is_not_gr = true,
            PolyglotToken::IsItOtherwise => found_otherwise = true,

            PolyglotToken::ExpanderIO => found_expander_io = true,
            PolyglotToken::IsItInRangeInclusiveFrom => found_range_inc_from = true,
            PolyglotToken::RangeFrom(ref v) if v == "1" => found_range_from_val = true,
            PolyglotToken::RangeSeparator => found_range_sep = true,
            PolyglotToken::RangeTo(ref v) if v == "10" => found_range_to_val = true,
            PolyglotToken::IsItInRangeExclusiveTo => found_range_exc_to = true,
            _ => {}
        }
    }

    assert!(found_data_input, "DataInput marker missing");
    assert!(found_default_pull, "DefaultPullFrom missing");
    assert!(found_constructor, "Constructor missing");
    assert!(found_inline_str, "InlineString missing");
    assert!(found_default_push, "DefaultPushInto missing");

    assert!(found_continue_action, "ContinueActionLine missing");
    assert!(found_is_equal, "IsItEqual missing");
    assert!(found_is_not_gr, "IsItNotGreaterThan missing");
    assert!(found_otherwise, "IsItOtherwise missing");

    assert!(found_expander_io, "ExpanderIO missing");
    assert!(found_range_inc_from, "IsItInRangeInclusiveFrom missing");
    assert!(found_range_from_val, "RangeFrom missing");
    assert!(found_range_sep, "RangeSeparator missing");
    assert!(found_range_to_val, "RangeTo missing");
    assert!(found_range_exc_to, "IsItInRangeExclusiveTo missing");
}

#[test]
fn test_lex_valid_code() {
    let script = std::fs::read_to_string("tests/fixtures/valid_code.pg").unwrap();
    let tokens = lex(&script);

    let mut generated_string = String::new();
    generated_string.push_str("=== Polyglot Token Stream ===\n");

    for t in &tokens {
        // Assert absolutely zero fallback patterns were generated!
        if let PolyglotToken::InvalidPattern(s) = &t.value {
            panic!("Lexer generated InvalidPattern: {}", s);
        }
        if let PolyglotToken::IncorrectIndent(s) = &t.value {
            panic!("Lexer generated IncorrectIndent: {}", s);
        }
        if let PolyglotToken::MissingMarker = t.value {
            panic!("Lexer generated MissingMarker on completely valid semantic code.");
        }

        generated_string.push_str(&format!("[L{:02}:C{:02}] {:?}\n", t.line, t.col, t.value));
    }
    generated_string.push_str("=============================\n");
    println!("{}", generated_string);

    // Verify the emitted token stream exactly matches the expected spec
    let expected_string = std::fs::read_to_string("tests/fixtures/valid_code.pgts").unwrap();
    assert_eq!(
        generated_string, expected_string,
        "Generated token stream does not match tests/fixtures/valid_code.pgts"
    );
}
