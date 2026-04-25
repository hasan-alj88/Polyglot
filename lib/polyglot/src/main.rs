use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use polyglot::lexer::lex;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut is_lexer = false;
    let mut input_file: Option<String> = None;
    let mut target_file: Option<String> = None;
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--lexer" => {
                is_lexer = true;
                i += 1;
            }
            "-c" => {
                if i + 1 < args.len() {
                    input_file = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: -c requires a file path");
                    std::process::exit(1);
                }
            }
            "-t" => {
                if i + 1 < args.len() {
                    target_file = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: -t requires a file path");
                    std::process::exit(1);
                }
            }
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                std::process::exit(1);
            }
        }
    }
    
    if !is_lexer || input_file.is_none() {
        eprintln!("Usage: polyglot --lexer -c <input.pg> [-t <output.pgts>]");
        std::process::exit(1);
    }
    
    let input_path_str = input_file.unwrap();
    let path = Path::new(&input_path_str);
    
    if !path.exists() || !path.is_file() {
        eprintln!("Error: Cannot find file {}", input_path_str);
        std::process::exit(1);
    }
    
    let out_path = if let Some(t) = target_file {
        PathBuf::from(t)
    } else {
        path.with_extension("pgts")
    };
    
    let script = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading {}: {}", input_path_str, e);
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
    
    match fs::write(&out_path, out_string) {
        Ok(_) => println!("Successfully saved token stream to {}", out_path.display()),
        Err(e) => {
            eprintln!("Error writing output file: {}", e);
            std::process::exit(1);
        }
    }
}
