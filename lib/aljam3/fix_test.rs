use std::fs;

fn main() {
    let output = std::process::Command::new("cargo")
        .args(&["test", "--test", "lexer_engine_tests", "test_lex_valid_code", "--", "--nocapture"])
        .output()
        .unwrap();

    let out_str = String::from_utf8_lossy(&output.stdout);
    
    // The test prints:
    // === Aljam3 Token Stream ===
    // ...
    // =============================
    
    if let Some(start) = out_str.find("=== Aljam3 Token Stream ===") {
        if let Some(end) = out_str[start..].find("=============================\n") {
            let token_stream = &out_str[start..start + end + 30];
            fs::write("tests/fixtures/valid_code.jm3ts", token_stream).unwrap();
            println!("Updated valid_code.jm3ts!");
        }
    }
}
