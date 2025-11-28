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
