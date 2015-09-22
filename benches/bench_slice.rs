#![feature(test)]

extern crate test;
extern crate traverse;
extern crate rand;

use traverse::Traversal;
use test::Bencher;

#[bench]
fn bench_internal(bench: &mut Bencher) {
    use rand::random;

    let data: Vec<usize> = (0..10000).map(|_| random()).collect();
    bench.iter(|| {
        data.run(|x| { ::test::black_box(x); });
    });
}

#[bench]
fn bench_external(bench: &mut Bencher) {
    use rand::random;

    let data: Vec<usize> = (0..10000).map(|_| random()).collect();
    bench.iter(|| {
        for datum in data.iter() {
            ::test::black_box(datum);
        }
    });
}
