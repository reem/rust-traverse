#![feature(unboxed_closures)]
#![deny(missing_docs, warnings)]

//! Proof-of-concept trait for intrusive iterators.

/// Intrusive Iterators.
pub trait IntrusiveIterator<T> {
    /// Run this Iterator using the provided closure.
    fn traverse<F: FnMut(T) -> bool>(self, F);
}

/// Extension methods for Intrusive Iterators
pub trait IntrusiveIteratorExt<T> {
    /// Get another intrusive iterator with its contents modified by the closure.
    fn map<O, F: FnMut(T) -> O>(self, F) -> Map<T, O, Self, F>;
}

impl<T, I: IntrusiveIterator<T>> IntrusiveIteratorExt<T> for I {
    fn map<O, F: FnMut(T) -> O>(self, f: F) -> Map<T, O, I, F> {
        Map { iter: self, closure: f }
    }
}

/// An IntrusiveIterator that maps over the contents of
/// another IntrusiveIterator.
pub struct Map<T, O, I: IntrusiveIterator<T>, F: FnMut(T) -> O> {
    iter: I,
    closure: F
}

impl<T, O, I: IntrusiveIterator<T>, F: FnMut(T) -> O> IntrusiveIterator<O> for Map<T, O, I, F> {
    fn traverse<F1: FnMut(O) -> bool>(self, mut f: F1) {
        let mut closure = self.closure;
        self.iter.traverse(move |t: T| {
            f(closure(t))
        });
    }
}

