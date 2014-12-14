#![feature(unboxed_closures, globs, phase)]
//#![deny(missing_docs, warnings)]

//! Proof-of-concept trait for intrusive iterators.

#[cfg(test)] #[phase(plugin)]
extern crate stainless;

#[cfg(test)]
extern crate test;

pub use ext::{IntrusiveIteratorExt,
              Map, Filter, FilterMap,
              Enumerate, Skip, Take,
              SkipWhile, TakeWhile,
              Inspect, FlatMap, Chain};

/// Intrusive Iterators.
pub trait IntrusiveIterator<T> {
    /// Run this Iterator using the provided closure.
    ///
    /// Return false from the closure to end the iteration.
    fn traverse<F: FnMut(T) -> bool>(self, F);

    /// Run this Iterator using the provided closure.
    ///
    /// This is a utility method for non-cancelling iterations.
    fn iterate<F: FnMut(T)>(self, mut f: F) {
        self.traverse(|&mut: t: T| { f(t); false })
    }
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
mod impls;
pub mod utils;
