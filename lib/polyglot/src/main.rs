use std::env;
use std::fs;
use std::path::Path;
use polyglot::lexer::lex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run --bin polyglot -- <file.pg>");
        std::process::exit(1);
    }
    
    let file_path = &args[1];
    let path = Path::new(file_path);
    if !path.exists() || !path.is_file() {
        eprintln!("Error: Cannot find file {}", file_path);
        std::process::exit(1);
    }
    
    let script = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading {}: {}", file_path, e);
            std::process::exit(1);
        }
    };
    
    let tokens = lex(&script);
    
    let mut out_string = String::new();
    out_string.push_str("=== Polyglot Token Stream ===\n");
    for t in &tokens {
        // Output formatting mirrors the unit test layout precisely
        out_string.push_str(&format!("[L{:02}:C{:02}] {:?}\n", t.line, t.col, t.value));
    }
    out_string.push_str("=============================\n");
    
    let out_path = path.with_extension("pgts");
    match fs::write(&out_path, out_string) {
        Ok(_) => println!("Successfully saved token stream to {}", out_path.display()),
        Err(e) => {
            eprintln!("Error writing output file: {}", e);
            std::process::exit(1);
        }
    }
}
