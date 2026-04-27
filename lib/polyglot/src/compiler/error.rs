pub fn format_caret_line(line_str: &str, col: usize, message: &str) -> String {
    let padding = " ".repeat(line_str.len() + if col > 1 { col - 1 } else { 0 });
    format!("  | {}^^^ {}", padding, message)
}

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub code: String,
    pub name: String,
    pub message: String,
    pub line: usize,
    pub col: usize,
    pub snippet: Option<String>,
    pub context_snippets: Vec<(usize, String)>,
    pub help: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub status: String,
    pub total_errors: usize,
    pub file: String,
    pub violations: Vec<ValidationError>,
}

impl ValidationReport {
    pub fn new(file: String) -> Self {
        Self {
            status: "passed".to_string(),
            total_errors: 0,
            file,
            violations: Vec::new(),
        }
    }

    pub fn add_error(&mut self, err: ValidationError) {
        self.violations.push(err);
        self.total_errors += 1;
        self.status = "failed".to_string();
    }

    pub fn print_report(&self) {
        if self.total_errors == 0 {
            println!("Validation Passed: {}", self.file);
            return;
        }

        println!("Validation Failed: {} errors found in {}\n", self.total_errors, self.file);

        for violation in &self.violations {
            println!("error[{}]: {}", violation.code, violation.name);
            println!(" --> {}:{}:{}", self.file, violation.line, violation.col);
            
            if let Some(snippet) = &violation.snippet {
                println!("  |");
                let mut last_line = 0;
                
                for (ctx_line, ctx_text) in &violation.context_snippets {
                    if last_line != 0 && *ctx_line > last_line + 1 {
                        println!("...");
                    }
                    println!("{}| {}", ctx_line, ctx_text);
                    last_line = *ctx_line;
                }
                
                if last_line != 0 && violation.line > last_line + 1 {
                    println!("...");
                }
                
                let line_str = format!("{}", violation.line);
                println!("{}| {}", line_str, snippet);
                println!("{}", format_caret_line(&line_str, violation.col, &violation.message));
            } else {
                println!("  | {}", violation.message);
            }
            
            if let Some(help) = &violation.help {
                println!("  = help: {}", help);
            }
            println!(); // Blank line between errors
        }
    }
}
