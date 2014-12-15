#![feature(unboxed_closures, globs, phase)]
//#![deny(missing_docs, warnings)]

//! Proof-of-concept trait for intrusive iterators.

#[cfg(test)] #[phase(plugin)]
extern crate stainless;

#[cfg(test)]
extern crate test;

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

    fn map<O, F: FnMut(T) -> O>(self, f: F) -> Map<Self, F> {
        Map { iter: self, closure: f }
    }

    fn filter<F: FnMut(&T) -> bool>(self, pred: F) -> Filter<Self, F> {
        Filter { iter: self, predicate: pred }
    }

    fn filter_map<O, F: FnMut(T) -> Option<O>>(self, pred: F) -> FilterMap<Self, F> {
        FilterMap { iter: self, predicate: pred }
    }

    fn enumerate(self) -> Enumerate<Self> {
        Enumerate(self)
    }

    fn skip(self, n: uint) -> Skip<Self> {
        Skip { iter: self, n: n }
    }

    fn take(self, n: uint) -> Take<Self> {
        Take { iter: self, n: n }
    }

    fn skip_while<F: FnMut(T) -> bool>(self, pred: F) -> SkipWhile<Self, F> {
        SkipWhile { iter: self, predicate: pred }
    }

    fn take_while<F: FnMut(T) -> bool>(self, pred: F) -> TakeWhile<Self, F> {
        TakeWhile { iter: self, predicate: pred }
    }

    fn inspect<F: FnMut(&T)>(self, f: F) -> Inspect<Self, F> {
        Inspect { iter: self, closure: f }
    }

    fn flat_map<O, U: Iterator<O>, F: FnMut(T) -> U>(self, f: F) -> FlatMap<Self, F> {
        FlatMap { iter: self, producer: f }
    }

    fn chain<O: IntrusiveIterator<T>>(self, other: O) -> Chain<Self, O> {
        Chain { one: self, two: other }
    }

    fn count(self) -> uint {
        let mut count = 0;
        self.iterate(|_| { count += 1; });
        count
    }

    fn cloned(self) -> Cloned<Self> {
        Cloned { iter: self }
    }

    fn collect<D: FromIntrusiveIterator<T>>(self) -> D {
        FromIntrusiveIterator::collect(self)
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

/// An IntrusiveIterator that maps over the contents of
/// another IntrusiveIterator.
#[deriving(Copy, Clone)]
pub struct Map<I, F> {
    iter: I,
    closure: F
}

#[deriving(Copy, Clone)]
pub struct Filter<I, F> {
    iter: I,
    predicate: F
}

#[deriving(Copy, Clone)]
pub struct FilterMap<I, F> {
    iter: I,
    predicate: F
}

#[deriving(Copy, Clone)]
pub struct Enumerate<I>(I);

#[deriving(Copy, Clone)]
pub struct Skip<I> {
    iter: I,
    n: uint
}

#[deriving(Copy, Clone)]
pub struct Take<I> {
    iter: I,
    n: uint
}

#[deriving(Copy, Clone)]
pub struct SkipWhile<I, F> {
    iter: I,
    predicate: F
}

#[deriving(Copy, Clone)]
pub struct TakeWhile<I, F> {
    iter: I,
    predicate: F
}

#[deriving(Copy, Clone)]
pub struct Inspect<I, F> {
    iter: I,
    closure: F
}

#[deriving(Copy, Clone)]
pub struct Chain<I, O> {
    one: I,
    two: O
}

#[deriving(Copy, Clone)]
pub struct FlatMap<I, F> {
    iter: I,
    producer: F
}

#[deriving(Copy, Clone)]
pub struct Cloned<I> {
    iter: I,
}

mod ext;
mod impls;
pub mod utils;
