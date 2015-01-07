//#![deny(missing_docs, warnings)]

//! Proof-of-concept trait for internal iterators.

#[cfg(test)]
extern crate test;

/// An iterator that runs all at once
pub trait Traversal: Sized {
    type Item;

    /// Run this Iterator using the provided closure.
    ///
    /// Return false from the closure to end the iteration.
    fn foreach<F>(self, F) where F: FnMut(Self::Item) -> bool;

    /// Run this Iterator using the provided closure.
    ///
    /// This is a utility method for non-cancelling iterations.
    fn run<F>(self, mut f: F) where F: FnMut(Self::Item) {
        self.foreach(|&mut: t: Self::Item| { f(t); false })
    }

    fn map<O, F>(self, f: F) -> Map<Self, F> where F: FnMut(Self::Item) -> O {
        Map { iter: self, closure: f }
    }

    fn filter<F>(self, pred: F) -> Filter<Self, F>
    where F: FnMut(&Self::Item) -> bool {
        Filter { iter: self, predicate: pred }
    }

    fn filter_map<O, F>(self, pred: F) -> FilterMap<Self, F>
    where F: FnMut(Self::Item) -> Option<O> {
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

    fn skip_while<F>(self, pred: F) -> SkipWhile<Self, F>
    where F: FnMut(Self::Item) -> bool {
        SkipWhile { iter: self, predicate: pred }
    }

    fn take_while<F>(self, pred: F) -> TakeWhile<Self, F>
    where F: FnMut(Self::Item) -> bool {
        TakeWhile { iter: self, predicate: pred }
    }

    fn inspect<F: FnMut(&Self::Item)>(self, f: F) -> Inspect<Self, F> {
        Inspect { iter: self, closure: f }
    }

    fn flat_map<A, U, F>(self, f: F) -> FlatMap<Self, F>
    where U: Traversal<Item=A>,
          F: FnMut(Self::Item) -> U {
        FlatMap { iter: self, producer: f }
    }

    fn chain<O>(self, other: O) -> Chain<Self, O>
    where O: Traversal<Item=Self::Item> {
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

    fn collect<D>(self) -> D where D: FromTraversal<Self::Item> {
        FromTraversal::collect(self)
    }
}

pub trait FromTraversal<T> {
    fn collect<I: Traversal<Item=T>>(I) -> Self;
}

pub trait IntoTraversal<T> {
    fn into_traversal(self) -> Internal<Self>;
}

impl<I: Iterator> IntoTraversal<I::Item> for I {
    fn into_traversal(self) -> Internal<I> {
        Internal { iter: self }
    }
}

pub struct Internal<I> {
    iter: I
}

impl<I: Iterator> Traversal for Internal<I> {
    type Item = I::Item;

    fn foreach<F>(mut self, mut f: F) where F: FnMut(I::Item) -> bool {
        for elem in self.iter {
            if f(elem) { break }
        }
    }
}

/// An Traversal that maps over the contents of
/// another Traversal.
#[derive(Copy, Clone)]
pub struct Map<I, F> {
    iter: I,
    closure: F
}

#[derive(Copy, Clone)]
pub struct Filter<I, F> {
    iter: I,
    predicate: F
}

#[derive(Copy, Clone)]
pub struct FilterMap<I, F> {
    iter: I,
    predicate: F
}

#[derive(Copy, Clone)]
pub struct Enumerate<I>(I);

#[derive(Copy, Clone)]
pub struct Skip<I> {
    iter: I,
    n: uint
}

#[derive(Copy, Clone)]
pub struct Take<I> {
    iter: I,
    n: uint
}

#[derive(Copy, Clone)]
pub struct SkipWhile<I, F> {
    iter: I,
    predicate: F
}

#[derive(Copy, Clone)]
pub struct TakeWhile<I, F> {
    iter: I,
    predicate: F
}

#[derive(Copy, Clone)]
pub struct Inspect<I, F> {
    iter: I,
    closure: F
}

#[derive(Copy, Clone)]
pub struct Chain<I, O> {
    one: I,
    two: O
}

#[derive(Copy, Clone)]
pub struct FlatMap<I, F> {
    iter: I,
    producer: F
}

#[derive(Copy, Clone)]
pub struct Cloned<I> {
    iter: I,
}

mod ext;
mod impls;
pub mod utils;
