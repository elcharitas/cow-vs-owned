use criterion::{criterion_group, criterion_main, Criterion};
use std::borrow::Cow;

fn cow_conditional_benchmark() {
    let mut input: Cow<'static, str> = Cow::Borrowed("This is a static string");
    if input.contains("static") {
        // Force mutation if condition matches
        input.to_mut().push_str(" with some additional text");
    }
    let _ = input.len();
}

fn string_conditional_benchmark() {
    let mut input = String::from("This is a dynamic string");
    if input.contains("dynamic") {
        // Modify if condition matches
        input.push_str(" with some additional text");
    }
    let _ = input.len();
}

fn cow_borrowed_benchmark() {
    let input: Cow<'static, str> = Cow::Borrowed("This is a static string");
    let _ = input.len();
}

fn cow_owned_benchmark() {
    let input: Cow<'static, str> = Cow::Owned("This is a dynamic string".to_string());
    let _ = input.len();
}

fn cow_mutation_benchmark() {
    let mut input: Cow<'static, str> = Cow::Borrowed("This is a static string");
    input.to_mut().push_str(" with some additional text");
}

fn string_benchmark() {
    let mut input = String::from("This is a dynamic string");
    input.push_str(" with some additional text");
    let _ = input.len();
}

fn benchmark(c: &mut Criterion) {
    c.bench_function("Cow - Borrowed", |b| b.iter(|| cow_borrowed_benchmark()));
    c.bench_function("Cow - Owned", |b| b.iter(|| cow_owned_benchmark()));
    c.bench_function("Cow - Mutation", |b| b.iter(|| cow_mutation_benchmark()));
    c.bench_function("String", |b| b.iter(|| string_benchmark()));
    c.bench_function("Cow - Conditional", |b| {
        b.iter(|| cow_conditional_benchmark())
    });
    c.bench_function("String - Conditional", |b| {
        b.iter(|| string_conditional_benchmark())
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
