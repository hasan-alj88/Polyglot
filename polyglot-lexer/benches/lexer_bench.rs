// Performance benchmarks for v0.0.4 Lexer

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use polyglot_lexer::Lexer;

fn generate_test_file(lines: usize) -> String {
    let mut source = String::from("[@] Local::BenchTest:1.0.0.0\n[|] BenchPipeline\n\n");

    for i in 0..lines {
        source.push_str(&format!("[r] .var{} :pg.string << \"value number {}\"\n", i, i));

        // Add some variety
        if i % 10 == 0 {
            source.push_str(&format!("<input{} :pg.int << .val\n", i));
        }
        if i % 15 == 0 {
            source.push_str(&format!(">output{} :pg.string >> .result\n", i));
        }
        if i % 20 == 0 {
            source.push_str("[~]\n  .item << 1\n  .item << 2\n[*]\n");
        }
    }

    source.push_str("\n[X]\n");
    source
}

fn bench_100_line_file(c: &mut Criterion) {
    let source = generate_test_file(100);

    c.bench_function("lex 100 lines", |b| {
        b.iter(|| {
            let mut lexer = Lexer::new(black_box(&source));
            lexer.tokenize().unwrap()
        })
    });
}

fn bench_1000_line_file(c: &mut Criterion) {
    let source = generate_test_file(1000);

    c.bench_function("lex 1000 lines", |b| {
        b.iter(|| {
            let mut lexer = Lexer::new(black_box(&source));
            lexer.tokenize().unwrap()
        })
    });
}

fn bench_10000_line_file(c: &mut Criterion) {
    let source = generate_test_file(10000);

    c.bench_function("lex 10000 lines", |b| {
        b.iter(|| {
            let mut lexer = Lexer::new(black_box(&source));
            lexer.tokenize().unwrap()
        })
    });
}

fn bench_operators(c: &mut Criterion) {
    let source = r#"
.x << .y
.x >> .y
.x <~ .y
.x ~> .y
.x |> .pipeline
.x =? .y
.x >? .y
.x <? .y
.x >=? .y
.x <=? .y
<input :pg.string << .value
>output :pg.int >> .result
"#;

    c.bench_function("operator disambiguation", |b| {
        b.iter(|| {
            let mut lexer = Lexer::new(black_box(source));
            lexer.tokenize().unwrap()
        })
    });
}

fn bench_indentation_tracking(c: &mut Criterion) {
    let source = r#"
[~]
  .item1 << 1
  .item2 << 2
    .nested1 << 3
    .nested2 << 4
      .deep1 << 5
      .deep2 << 6
    .nested3 << 7
  .item3 << 8
[*]
"#;

    c.bench_function("indentation tracking", |b| {
        b.iter(|| {
            let mut lexer = Lexer::new(black_box(source));
            lexer.tokenize().unwrap()
        })
    });
}

criterion_group!(
    benches,
    bench_100_line_file,
    bench_1000_line_file,
    bench_10000_line_file,
    bench_operators,
    bench_indentation_tracking
);
criterion_main!(benches);
