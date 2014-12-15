#![feature(unboxed_closures)]
extern crate intrusive;

use std::default::Default;
use intrusive::IntrusiveIterator;

pub struct Streaming<T: Default>;

impl<'a, T: Default + 'a> IntrusiveIterator<&'a mut T> for Streaming<T> {
    fn traverse<F: FnMut(&'a mut T) -> bool>(self, mut f: F) {
        loop {
            let t: T = Default::default();
            if f(&t) { return }
        }
    }
}

fn main() {
    let stream: Streaming<uint> = Streaming;
    stream.traverse(|x| if *x > 10 { true } else { println!("{}", x); false });
}

