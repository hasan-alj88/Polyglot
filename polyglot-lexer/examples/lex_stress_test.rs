// Example: Comprehensive stress test with complex program
use polyglot_lexer::{lex, TokenKind};

fn main() {
    // Large, complex program combining many features
    let source = r#"// ============================================================================
// Comprehensive Stress Test - Combines Multiple Features
// ============================================================================

[@] Local@StressTest:1.0.0
[#] 1
[X]

// ============================================================================
// Main Data Processing Pipeline
// ============================================================================

[|] ProcessUserData
[i] .user_id: pg\string
[i] .include_history: pg\bool
[i] .max_records: pg\int
[t] |T.Call
[W] |W.Polyglot.Scope

// Fetch user from database (no error handling for now - simplified)
[r] |FetchUser
[<] .id: pg\string << .user_id
[>] .user: pg\serial >> .user_data

// Check user state
[?] .user_data.pgvar.state =? #PgVar.States.Faulted
[~][r] .errors: pg\array{!} << .user_data.pgvar.errors
[~][o] !DatabaseError
[~]

[?] *?
[~]// Continue with valid user
[~]

// Conditional history fetching
[?] .include_history =? #Boolean.True
[&] .max_records >? 0
[~][r] |FetchHistory
[~][<] .user_id: pg\string << .user_data.id
[~][<] .limit: pg\int << .max_records
[~][>] .history: pg\array{pg\serial} >> .user_history
[~]

[?] *?
[~][r] .user_history: pg\array{pg\serial} << {}
[~]

// Process results with string interpolation
[r] .summary: pg\string << "User: {.user_data.name}, Records: {.user_history.length:Decimal}"

// Range-based validation
[?] .max_records ?[1, 100]
[~][r] .valid: pg\bool << #Boolean.True
[~]

[?] .max_records ?(100, 1000]
[~][r] .valid: pg\bool << #Boolean.True
[~][r] .warning: pg\string << "Large result set"
[~]

[?] *?
[~][r] .valid: pg\bool << #Boolean.False
[~]

// Output results
[o] .summary: pg\string
[o] .valid: pg\bool
[o] .user_history: pg\array{pg\serial}
[X]

// ============================================================================
// Helper Pipeline: Fetch User
// ============================================================================

[|] FetchUser
[i] .id: pg\string
[t] |T.Call
[W] RT.Python"fetch_user.py"

// Datetime tracking
[r] .query_time: pg\dt << DT.Now
[r] .formatted_time: pg\string << DT.Format

[o] .user: pg\serial
[X]

// ============================================================================
// Helper Pipeline: Fetch History
// ============================================================================

[|] FetchHistory
[i] .user_id: pg\string
[i] .limit: pg\int
[t] |T.Call
[W] |W.Python3.11

// Parallel processing of recent and archived data
[p] |FetchRecent
[<] .id: pg\string << .user_id
[<] .count: pg\int << .limit
[>] .recent: pg\array{pg\serial} >> recent_data

[p] |FetchArchived
[<] .id: pg\string << .user_id
[<] .count: pg\int << .limit
[>] .archived: pg\array{pg\serial} >> archived_data

// Join results
[Y] |Y.JoinAll
[>] recent_data
[>] archived_data

// Merge arrays
[r] |U.Array.Concat
[<] .arrays: pg\array{pg\array{pg\serial}} << {recent_data, archived_data}
[>] .merged: pg\array{pg\serial} >> .history

[o] .history: pg\array{pg\serial}
[X]

// ============================================================================
// Deeply nested conditional logic
// ============================================================================

[|] NestedValidation
[i] .age: pg\int
[i] .income: pg\int
[i] .credit: pg\int
[t] |T.Call
[W] |W.Polyglot.Scope

[?] .age =>? 18
[&] .age =<? 65
[~][?] .income >? 50000
[~][&] .credit >? 700
[~][~][?] .income >? 100000
[~][~][~][r] .tier: pg\string << "platinum"
[~][~][~]
[~][~][?] .income >? 75000
[~][~][~][r] .tier: pg\string << "gold"
[~][~][~]
[~][~][?] *?
[~][~][~][r] .tier: pg\string << "silver"
[~][~][~]
[~][~]
[~][?] *?
[~][~][r] .tier: pg\string << "bronze"
[~][~]
[~]

[?] *?
[~][r] .tier: pg\string << "ineligible"
[~]

[o] .tier: pg\string
[X]"#;

    println!("=== Polyglot Lexer v0.0.2 - Comprehensive Stress Test ===\n");
    println!("Program size: {} bytes", source.len());
    println!("Program lines: {}", source.lines().count());
    println!();

    let start = std::time::Instant::now();
    match lex(source) {
        Ok(tokens) => {
            let duration = start.elapsed();

            // Analyze token distribution
            let mut token_counts: std::collections::HashMap<String, usize> =
                std::collections::HashMap::new();
            for token in &tokens {
                if token.kind != TokenKind::Eof {
                    let key = format!("{:?}", token.kind);
                    *token_counts.entry(key).or_insert(0) += 1;
                }
            }

            println!("✅ Lexing completed successfully!");
            println!("   Duration: {:?}", duration);
            println!("   Total tokens: {}", tokens.len());
            println!();

            // Top token types
            let mut sorted: Vec<_> = token_counts.iter().collect();
            sorted.sort_by(|a, b| b.1.cmp(a.1));

            println!("📊 Token Distribution (Top 10):");
            for (i, (kind, count)) in sorted.iter().take(10).enumerate() {
                println!("   {:2}. {:30} {}", i + 1, kind, count);
            }
            println!();

            // Feature coverage
            let has_strings = tokens.iter().any(|t| t.kind == TokenKind::StringStart);
            let has_interpolation = tokens
                .iter()
                .any(|t| t.kind == TokenKind::InterpolationStart);
            let has_conditionals = tokens.iter().any(|t| t.kind == TokenKind::BlockConditional);
            let has_ranges = tokens
                .iter()
                .any(|t| matches!(t.kind, TokenKind::OpRangeClosed | TokenKind::OpRangeOpen));
            let has_reserved = tokens.iter().any(|t| {
                matches!(
                    t.kind,
                    TokenKind::ReservedBooleanTrue
                        | TokenKind::ReservedBooleanFalse
                        | TokenKind::ReservedPgVarReady
                        | TokenKind::ReservedPgVarFaulted
                )
            });
            let has_errors = tokens.iter().any(|t| t.kind == TokenKind::IdentifierError);
            let has_parallel = tokens.iter().any(|t| t.kind == TokenKind::BlockParallel);
            let has_join = tokens.iter().any(|t| t.kind == TokenKind::BlockJoin);

            println!("🎯 Feature Coverage:");
            println!("   {} String literals", if has_strings { "✓" } else { "✗" });
            println!(
                "   {} String interpolation",
                if has_interpolation { "✓" } else { "✗" }
            );
            println!(
                "   {} Conditional logic",
                if has_conditionals { "✓" } else { "✗" }
            );
            println!("   {} Range operators", if has_ranges { "✓" } else { "✗" });
            println!(
                "   {} Reserved enumerations",
                if has_reserved { "✓" } else { "✗" }
            );
            println!("   {} Error handling", if has_errors { "✓" } else { "✗" });
            println!(
                "   {} Parallel execution",
                if has_parallel { "✓" } else { "✗" }
            );
            println!("   {} Join operations", if has_join { "✓" } else { "✗" });

            println!();
            if has_strings
                && has_interpolation
                && has_conditionals
                && has_ranges
                && has_reserved
                && has_errors
                && has_parallel
                && has_join
            {
                println!("🎉 All advanced features present and tokenized correctly!");
                println!();
                println!("Performance: {} tokens in {:?}", tokens.len(), duration);
                println!(
                    "             {:.2} tokens/ms",
                    tokens.len() as f64 / duration.as_millis() as f64
                );
            } else {
                println!("⚠️  Some features missing");
            }
        }
        Err(e) => {
            eprintln!("❌ Lexer Error: {}", e);
            eprintln!("   at line {}, column {}", e.line(), e.column());
        }
    }
}
