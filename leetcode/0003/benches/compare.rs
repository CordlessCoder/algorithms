use criterion::{black_box, criterion_group, criterion_main, Criterion};
use stuff::length_of_longest_substring;

use std::collections::VecDeque;

fn sample(s: String) -> i32 {
    let mut res = 0;
    let mut queue = VecDeque::new();
    for (i, c) in s.chars().enumerate() {
        if let Some(idx) = queue.iter().position(|&d| c == d) {
            queue.drain(..=idx);
        }
        queue.push_back(c);
        res = res.max(queue.len());
    }
    res as i32
}

pub fn criterion_benchmark(c: &mut Criterion) {
    // let (i_long, i_realistic) = input
    //     .split_once('\n')
    //     .expect("test_inputs should contain two lines, a long and realistic sentence.");
    let input = String::from("a".repeat(1000) + "bcd" + &"a".repeat(997));
    c.bench_function("sample 3ms solution", |b| {
        b.iter(|| {
            black_box(sample(input.clone()));
        })
    });
    c.bench_function("my solution", |b| {
        b.iter(|| {
            black_box(length_of_longest_substring(input.clone()));
        })
    });
    // c.bench_function("real world\tnoalloc cleanup", |b| {
    //     b.iter(|| string_cleanup_noalloc(black_box(i_realistic.to_string())))
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
