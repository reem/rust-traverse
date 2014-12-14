#![feature(unboxed_closures, globs)]
//#![deny(missing_docs, warnings)]

//! Proof-of-concept trait for intrusive iterators.

pub use ext::{IntrusiveIteratorExt,
              Map, Filter, FilterMap,
              Enumerate, Skip, Take,
              SkipWhile, TakeWhile,
              Inspect, FlatMap, Chain};

/// Intrusive Iterators.
pub trait IntrusiveIterator<T> {
    /// Run this Iterator using the provided closure.
    fn traverse<F: FnMut(T) -> bool>(self, F);
}

pub trait FromIntrusiveIterator<T> {
    fn collect<I: IntrusiveIterator<T>>(I) -> Self;
}

pub trait IntoIntrusive<T> {
    fn into_intrusive(self) -> Intrusive<Self>;
}

impl<T, I: Iterator<T>> IntoIntrusive<T> for I {
    fn into_intrusive(self) -> Intrusive<I> {
        Intrusive { iter: self }
    }
}

pub struct Intrusive<I> {
    iter: I
}

impl<T, I: Iterator<T>> IntrusiveIterator<T> for Intrusive<I> {
    fn traverse<F: FnMut(T) -> bool>(mut self, mut f: F) {
        for elem in self.iter {
            if f(elem) { break }
        }
    }
}

mod ext;

