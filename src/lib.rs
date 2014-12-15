#![feature(unboxed_closures, globs, phase)]
//#![deny(missing_docs, warnings)]

//! Proof-of-concept trait for internal iterators.

#[cfg(test)] #[phase(plugin)]
extern crate stainless;

#[cfg(test)]
extern crate test;

/// An iterator that runs all at once
pub trait Traversal<T> {
    /// Run this Iterator using the provided closure.
    ///
    /// Return false from the closure to end the iteration.
    fn foreach<F: FnMut(T) -> bool>(self, F);

    /// Run this Iterator using the provided closure.
    ///
    /// This is a utility method for non-cancelling iterations.
    fn run<F: FnMut(T)>(self, mut f: F) {
        self.foreach(|&mut: t: T| { f(t); false })
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

    fn chain<O: Traversal<T>>(self, other: O) -> Chain<Self, O> {
        Chain { one: self, two: other }
    }

    fn count(self) -> uint {
        let mut count = 0;
        self.run(|_| { count += 1; });
        count
    }

    fn cloned(self) -> Cloned<Self> {
        Cloned { iter: self }
    }

    fn collect<D: FromTraversal<T>>(self) -> D {
        FromTraversal::collect(self)
    }
}

pub trait FromTraversal<T> {
    fn collect<I: Traversal<T>>(I) -> Self;
}

pub trait IntoTraversal<T> {
    fn into_traversal(self) -> Internal<Self>;
}

impl<T, I: Iterator<T>> IntoTraversal<T> for I {
    fn into_traversal(self) -> Internal<I> {
        Internal { iter: self }
    }
}

pub struct Internal<I> {
    iter: I
}

impl<T, I: Iterator<T>> Traversal<T> for Internal<I> {
    fn foreach<F: FnMut(T) -> bool>(mut self, mut f: F) {
        for elem in self.iter {
            if f(elem) { break }
        }
    }
}

/// An Traversal that maps over the contents of
/// another Traversal.
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
