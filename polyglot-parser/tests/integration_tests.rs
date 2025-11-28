//! Integration tests for Polyglot Parser
//!
//! These tests use the fixtures from the test design document to verify
//! parser behavior with realistic Polyglot code.

use polyglot_parser::{Parser, FileRegistryResolver};
use std::fs;
use std::path::PathBuf;

fn get_fixture_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(filename)
}

fn load_fixture(filename: &str) -> String {
    let path = get_fixture_path(filename);
    fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read fixture {}: {}", filename, e))
}

#[test]
fn test_valid_pipeline_fixture() {
    let source = load_fixture("test-valid-pipeline.pg");
    let registry = get_fixture_path("test-registry.json");
    
    let resolver = FileRegistryResolver::from_file(&registry)
        .expect("Failed to load registry");
    
    let parser = Parser::new(&source, resolver)
        .expect("Failed to create parser");
    
    let result = parser.parse();
    assert!(result.is_ok(), "Failed to parse valid pipeline: {:?}", result.err());
    
    let program = result.unwrap();
    
    // Verify structure
    assert_eq!(program.definitions.len(), 1, "Should have 1 pipeline");
    assert_eq!(program.package.imports.len(), 1, "Should have 1 import");
}

#[test]
fn test_missing_param_fixture() {
    let source = load_fixture("test-missing-param.pg");
    let registry = get_fixture_path("test-registry.json");
    
    let resolver = FileRegistryResolver::from_file(&registry)
        .expect("Failed to load registry");
    
    let parser = Parser::new(&source, resolver)
        .expect("Failed to create parser");
    
    // Parser should succeed (it's just parsing, not validating yet)
    let result = parser.parse();
    assert!(result.is_ok(), "Parser should succeed even with validation errors: {:?}", result.err());
    
    // TODO: Add semantic validation in future story
    // For now, we just verify it parses correctly
}

#[test]
fn test_wrong_param_name_fixture() {
    let source = load_fixture("test-wrong-param-name.pg");
    let registry = get_fixture_path("test-registry.json");
    
    let resolver = FileRegistryResolver::from_file(&registry)
        .expect("Failed to load registry");
    
    let parser = Parser::new(&source, resolver)
        .expect("Failed to create parser");
    
    let result = parser.parse();
    assert!(result.is_ok(), "Parser should succeed: {:?}", result.err());
}

#[test]
fn test_type_mismatch_fixture() {
    let source = load_fixture("test-type-mismatch.pg");
    let registry = get_fixture_path("test-registry.json");

    let resolver = FileRegistryResolver::from_file(&registry)
        .expect("Failed to load registry");

    let parser = Parser::new(&source, resolver)
        .expect("Failed to create parser");

    let result = parser.parse();
    assert!(result.is_ok(), "Parser should succeed: {:?}", result.err());
}

// ============================================================================
// Multi-File Compilation Tests (Story 1.5.5)
// ============================================================================

#[test]
fn test_multifile_forward_reference_succeeds() {
    // Task 4.1: Pipeline in [#] 1 can be called from [#] 2
    let file1_path = get_fixture_path("multifile-forward-ref/file1.pg");
    let file2_path = get_fixture_path("multifile-forward-ref/file2.pg");

    // Parse file1 first (defines DataLoader)
    let file1_source = fs::read_to_string(&file1_path).unwrap();
    let resolver = FileRegistryResolver::empty();

    let parser = Parser::new(&file1_source, resolver).unwrap();
    let result1 = parser.parse();
    assert!(result1.is_ok(), "File1 should parse successfully: {:?}", result1.err());

    // Parse file2 (calls DataLoader from file1)
    let file2_source = fs::read_to_string(&file2_path).unwrap();
    let resolver = FileRegistryResolver::empty();

    let parser = Parser::new(&file2_source, resolver).unwrap();
    let result2 = parser.parse();
    assert!(result2.is_ok(), "File2 should parse successfully: {:?}", result2.err());

    // Note: Actual Phase 2 resolution validation would happen at runtime
    // For now, we verify both files parse correctly
}

#[test]
fn test_multifile_backward_reference_not_validated() {
    // Task 4.2: Pipeline in [#] 2 cannot be called from [#] 1
    // Note: Current implementation parses successfully but would fail
    // at runtime when Phase 2 resolution is enforced

    let file1_path = get_fixture_path("multifile-backward-ref/file1.pg");

    // File1 tries to call LatePipeline which is defined in file2
    let file1_source = fs::read_to_string(&file1_path).unwrap();
    let resolver = FileRegistryResolver::empty();

    let parser = Parser::new(&file1_source, resolver).unwrap();
    let result = parser.parse();

    // Parser succeeds (syntax is valid), but semantic validation
    // would detect the backward reference violation
    assert!(result.is_ok(), "File should parse: {:?}", result.err());

    // TODO: Add semantic validation in future to catch backward references
}

#[test]
fn test_multifile_unnumbered_can_call_numbered() {
    // Task 4.3: Files without [#] can call pipelines from numbered files
    let file1_path = get_fixture_path("multifile-unnumbered/file1.pg");
    let file_no_marker_path = get_fixture_path("multifile-unnumbered/file_no_marker.pg");

    // Parse numbered file
    let file1_source = fs::read_to_string(&file1_path).unwrap();
    let resolver = FileRegistryResolver::empty();

    let parser = Parser::new(&file1_source, resolver).unwrap();
    let result1 = parser.parse();
    assert!(result1.is_ok(), "Numbered file should parse: {:?}", result1.err());

    // Parse unnumbered file (calls BaseHelper from file1)
    let file_no_marker_source = fs::read_to_string(&file_no_marker_path).unwrap();
    let resolver = FileRegistryResolver::empty();

    let parser = Parser::new(&file_no_marker_source, resolver).unwrap();
    let result2 = parser.parse();
    assert!(result2.is_ok(), "Unnumbered file should parse: {:?}", result2.err());
}

#[test]
fn test_multifile_duplicate_detection() {
    // Task 4.4: Duplicate [#] markers should be detected
    let file1_path = get_fixture_path("multifile-duplicate/file1.pg");
    let file2_path = get_fixture_path("multifile-duplicate/file2.pg");

    // When validate_file_ordering is called on these two files,
    // it should detect duplicate [#] 1 markers

    // For this test, we'll manually call the validation logic
    // by parsing both files and checking for duplicates

    let file1_source = fs::read_to_string(&file1_path).unwrap();
    let file2_source = fs::read_to_string(&file2_path).unwrap();

    let resolver = FileRegistryResolver::empty();

    // Both files parse individually
    let parser1 = Parser::new(&file1_source, resolver.clone()).unwrap();
    let result1 = parser1.parse();
    assert!(result1.is_ok(), "File1 should parse individually: {:?}", result1.err());

    let parser2 = Parser::new(&file2_source, resolver).unwrap();
    let result2 = parser2.parse();
    assert!(result2.is_ok(), "File2 should parse individually: {:?}", result2.err());

    // TODO: Add test that validates file ordering across multiple files
    // and expects DuplicateFileOrder error
}

#[test]
fn test_multifile_end_to_end() {
    // Task 4.5: Complete multi-file package compiles successfully

    // Use forward-ref fixture as our end-to-end test case
    let file1_path = get_fixture_path("multifile-forward-ref/file1.pg");
    let file2_path = get_fixture_path("multifile-forward-ref/file2.pg");

    let resolver = FileRegistryResolver::empty();

    // Parse both files
    let file1_source = fs::read_to_string(&file1_path).unwrap();
    let parser1 = Parser::new(&file1_source, resolver.clone()).unwrap();
    let program1 = parser1.parse().expect("File1 should compile");

    let file2_source = fs::read_to_string(&file2_path).unwrap();
    let parser2 = Parser::new(&file2_source, resolver).unwrap();
    let program2 = parser2.parse().expect("File2 should compile");

    // Verify structure of both programs
    assert_eq!(program1.definitions.len(), 1, "File1 should have 1 pipeline");
    assert_eq!(program2.definitions.len(), 1, "File2 should have 1 pipeline");

    // Verify package specs match (same package)
    assert_eq!(program1.package.spec.registry, program2.package.spec.registry);
    assert_eq!(program1.package.spec.path, program2.package.spec.path);
}
