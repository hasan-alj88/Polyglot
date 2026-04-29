
use polyglot::lexer::Lexer;

fn main() {
    let code = "{-} -BatchJob\n[Q] -Q.Assign\"BatchQueue\"\n<maxInstances#int << 5";
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize();
    for token in tokens {
        println!("{:?}", token);
    }
}
