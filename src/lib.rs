//#![deny(missing_docs, warnings)]

//! Proof-of-concept trait for internal iterators.

#![cfg_attr(all(test, feature = "nightly"), feature(test))]
#[cfg(all(test, feature = "nightly"))] extern crate test;
#[cfg(all(test, feature = "nightly"))] extern crate rand;

// For CheckedAdd
extern crate num;

mod ext;
pub mod utils;
mod impls;

/// An iterator that runs all at once
pub trait Traversal: Sized {
    type Item;

    /// Run this Iterator using the provided closure.
    ///
    /// Return true from the closure to end the iteration.
    fn foreach<F>(self, F) where F: FnMut(Self::Item) -> bool;

    /// Run this Iterator using the provided closure.
    ///
    /// This is a utility method for non-cancelling iterations.
    fn run<F>(self, mut f: F) where F: FnMut(Self::Item) {
        self.foreach(|t| { f(t); false })
    }

    fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }

    fn map<F, O>(self, f: F) -> Map<Self, F>
    where F: FnMut(Self::Item) -> O {
        Map { iter: self, closure: f }
    }

    fn filter<F>(self, pred: F) -> Filter<Self, F>
    where F: FnMut(&Self::Item) -> bool {
        Filter { iter: self, predicate: pred }
    }

    fn filter_map<F, O>(self, pred: F) -> FilterMap<Self, F>
    where F: FnMut(Self::Item) -> Option<O> {
        FilterMap { iter: self, predicate: pred }
    }

    fn enumerate(self) -> Enumerate<Self> {
        Enumerate(self)
    }

    fn skip(self, n: usize) -> Skip<Self> {
        Skip { iter: self, n: n }
    }

    fn take(self, n: usize) -> Take<Self> {
        Take { iter: self, n: n }
    }

    fn skip_while<F>(self, pred: F) -> SkipWhile<Self, F>
    where F: FnMut(&Self::Item) -> bool {
        SkipWhile { iter: self, predicate: pred }
    }

    fn take_while<F>(self, pred: F) -> TakeWhile<Self, F>
    where F: FnMut(&Self::Item) -> bool {
        TakeWhile { iter: self, predicate: pred }
    }

    fn inspect<F>(self, f: F) -> Inspect<Self, F>
    where F: FnMut(&Self::Item) {
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

    fn any<P>(self, mut predicate: P) -> bool where P: FnMut(Self::Item) -> bool {
        let mut result = false;
        self.foreach(|item| {
            let this = predicate(item);
            result |= this;
            this
        });
        result
    }

    fn all<P>(self, mut predicate: P) -> bool where P: FnMut(Self::Item) -> bool {
        let mut result = true;
        self.foreach(|item| {
            let this = predicate(item);
            result &= this;
            !this
        });
        result
    }

    fn count(self) -> usize {
        let mut count = 0;
        self.run(|_| { count += 1; });
        count
    }

    fn cloned(self) -> Cloned<Self> {
        Cloned { iter: self }
    }

    fn collect<D>(self) -> D
    where D: FromTraversal<Self::Item> {
        FromTraversal::from_traversal(self)
    }
}

pub trait FromTraversal<T> {
    fn from_traversal<I: IntoTraversal<Item=T>>(traversable: I) -> Self;
}

pub trait IntoTraversal {
    type IntoTrav: Traversal<Item=Self::Item>;
    type Item;
    fn into_traversal(self) -> Self::IntoTrav;
}

impl<T: Traversal> IntoTraversal for T {
    type IntoTrav = Self;
    type Item = <Self as Traversal>::Item;

    fn into_traversal(self) -> Self::IntoTrav {
        self
    }
}

pub struct Internal<I> {
    iter: I
}

impl<I: Iterator> Internal<I> {
    pub fn new<It: IntoIterator<IntoIter=I,Item=I::Item>>(iterable: It) -> Self {
        Internal { iter: iterable.into_iter() }
    }
}

impl<I: Iterator> Traversal for Internal<I> {
    type Item = I::Item;

    fn foreach<F>(self, mut f: F) where F: FnMut(I::Item) -> bool {
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
    closure: F,
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
    n: usize
}

#[derive(Copy, Clone)]
pub struct Take<I> {
    iter: I,
    n: usize
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

#[test]
fn test_any() {
    assert!(Internal::new(&[1, 2, 3, 4]).cloned().any(|i| i > 3));
    assert!(!Internal::new(&[1, 2, 3, 4]).cloned().any(|i| i > 4));
}

#[test]
fn test_all() {
    assert!(Internal::new(&[1, 2, 3, 4]).cloned().all(|i| i < 5));
    assert!(!Internal::new(&[1, 2, 3, 4]).cloned().all(|i| i < 4));
}
