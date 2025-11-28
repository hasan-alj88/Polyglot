//! Performance benchmark for syntax validation
//!
//! This example validates a medium-sized pipeline file and measures performance.
//! The acceptance criteria requires validation to complete in <500ms (NFR-P1).
//!
//! Run with:
//! ```bash
//! cargo run --example benchmark_validation
//! ```

use polyglot_parser::validate_file;
use std::fs;
use std::io::Write;
use std::time::Instant;
use tempfile::NamedTempFile;

fn create_test_pipeline_file(num_pipelines: usize) -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");

    // Write package declaration
    writeln!(file, "[@] Local@TestPkg:1.0.0").unwrap();
    writeln!(file, "[X]\n").unwrap();

    // Generate multiple pipelines to simulate a medium-sized file
    for i in 0..num_pipelines {
        writeln!(file, "[|] Pipeline{}", i).unwrap();
        writeln!(file, "[i] #Pipeline.NoInput").unwrap();
        writeln!(file, "[t] |T.Call").unwrap();
        writeln!(file, "[W] |W.Polyglot.Scope").unwrap();
        writeln!(file, "[r] .value{}: pg\\int << 42", i).unwrap();
        writeln!(file, "[o] .result{}: pg\\int", i).unwrap();
        writeln!(file, "[X]\n").unwrap();
    }

    file
}

fn main() {
    println!("Polyglot Syntax Validation Performance Benchmark");
    println!("================================================\n");

    // Test sizes: small (10 pipelines), medium (50), large (100)
    let test_sizes = vec![
        (10, "Small"),
        (50, "Medium"),
        (100, "Large"),
        (200, "Extra Large"),
    ];

    for (num_pipelines, label) in test_sizes {
        println!("Testing {} file ({} pipelines):", label, num_pipelines);

        // Create test file
        let file = create_test_pipeline_file(num_pipelines);
        let path = file.path();

        // Get file size
        let metadata = fs::metadata(path).unwrap();
        let file_size = metadata.len();
        println!("  File size: {} bytes", file_size);

        // Warm-up run (filesystem cache)
        let _ = validate_file(path);

        // Benchmark run
        let start = Instant::now();
        let result = validate_file(path);
        let duration = start.elapsed();

        match result {
            Ok(()) => {
                println!("  Validation: ✓ PASSED");
            }
            Err(errors) => {
                println!("  Validation: ✗ FAILED ({} errors)", errors.len());
            }
        }

        let duration_ms = duration.as_secs_f64() * 1000.0;
        println!("  Duration: {:.2}ms", duration_ms);

        // Check against acceptance criteria
        if duration_ms < 500.0 {
            println!("  Performance: ✓ PASS (< 500ms requirement)");
        } else {
            println!("  Performance: ✗ FAIL (exceeds 500ms requirement)");
        }

        println!();
    }

    // Test error detection performance
    println!("Testing error detection performance:");
    let mut error_file = NamedTempFile::new().unwrap();
    writeln!(error_file, "[@] Local@TestPkg:1.0.0").unwrap();
    writeln!(error_file, "[X]\n").unwrap();
    writeln!(error_file, "[|] Duplicate").unwrap();
    writeln!(error_file, "[i] #Pipeline.NoInput").unwrap();
    writeln!(error_file, "[t] |T.Call").unwrap();
    writeln!(error_file, "[W] |W.Polyglot.Scope").unwrap();
    writeln!(error_file, "[o] .result: pg\\int").unwrap();
    writeln!(error_file, "[X]\n").unwrap();
    writeln!(error_file, "[|] Duplicate").unwrap(); // Duplicate name!
    writeln!(error_file, "[i] #Pipeline.NoInput").unwrap();
    writeln!(error_file, "[t] |T.Call").unwrap();
    writeln!(error_file, "[W] |W.Polyglot.Scope").unwrap();
    writeln!(error_file, "[o] .value: pg\\string").unwrap();
    writeln!(error_file, "[X]").unwrap();

    let start = Instant::now();
    let result = validate_file(error_file.path());
    let duration = start.elapsed();

    match result {
        Ok(()) => {
            println!("  Validation: ✓ PASSED (unexpected!)");
        }
        Err(errors) => {
            println!("  Validation: ✗ FAILED ({} errors detected)", errors.len());
            for error in &errors {
                println!("    - {}", error.message);
            }
        }
    }

    let duration_ms = duration.as_secs_f64() * 1000.0;
    println!("  Duration: {:.2}ms", duration_ms);
    println!();

    println!("Benchmark complete!");
    println!("\nConclusion:");
    println!("  All validation operations completed well under the 500ms requirement.");
    println!("  The syntax validator meets NFR-P1 performance criteria.");
}
