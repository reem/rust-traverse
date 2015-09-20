extern crate quickcheck;
extern crate traverse;

use traverse::Traversal;

#[test]
fn quickcheck_map() {
    fn add_one(x: &u32) -> u32 { *x + 1 }

    fn prop(vec: Vec<u32>) -> bool {
        let expected: Vec<_> = vec.iter().map(add_one).collect();
        let result: Vec<_> = vec.map(add_one).collect();
        expected == result
    }

    quickcheck::quickcheck(prop as fn(Vec<u32>) -> bool);
}

#[test]
fn quickcheck_filter() {
    fn is_even(x: & &u32) -> bool { **x % 2 == 0 }

    fn prop(vec: Vec<u32>) -> bool {
        let expected: Vec<&u32> = vec.iter().filter(is_even).collect();
        let result: Vec<&u32> = vec.filter(is_even).collect();
        expected == result
    }

    quickcheck::quickcheck(prop as fn(Vec<u32>) -> bool);
}
