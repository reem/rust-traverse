#![feature(unboxed_closures)]
//#![deny(missing_docs, warnings)]

//! Proof-of-concept trait for intrusive iterators.

/// Intrusive Iterators.
pub trait IntrusiveIterator<T> {
    /// Run this Iterator using the provided closure.
    fn traverse<F: FnMut(T) -> bool>(self, F);
}

/// Extension methods for Intrusive Iterators
pub trait IntrusiveIteratorExt<T> {
    /// Get another intrusive iterator with its contents modified by the closure.
    fn map<O, F: FnMut(T) -> O>(self, F) -> Map<Self, F>;
    fn filter<F: FnMut(T) -> bool>(self, f: F) -> Filter<Self, F>;
    fn filter_map<O, F: FnMut(T) -> Option<O>>(self, F) -> FilterMap<Self, F>;
    fn enumerate(self) -> Enumerate<Self>;
    fn skip(self, uint) -> Skip<Self>;
    fn take(self, uint) -> Take<Self>;
    fn skip_while<F: FnMut(T) -> bool>(self, F) -> SkipWhile<Self, F>;
    fn take_while<F: FnMut(T) -> bool>(self, F) -> TakeWhile<Self, F>;
    fn inspect<F: FnMut(T)>(self, F) -> Inspect<Self, F>;
    fn flat_map<O, U: Iterator<O>, F: FnMut(T) -> U>(self, F) -> FlatMap<Self, F>;
    fn chain<O: IntrusiveIterator<T>>(self, O) -> Chain<Self, O>;
    fn collect<D: FromIntrusiveIterator<T>>(self) -> D;
}

pub trait FromIntrusiveIterator<T> {
    fn collect<I: IntrusiveIterator<T>>(I) -> Self;
}

impl<T, I: IntrusiveIterator<T>> IntrusiveIteratorExt<T> for I {
    fn map<O, F: FnMut(T) -> O>(self, f: F) -> Map<I, F> {
        Map { iter: self, closure: f }
    }

    fn filter<F: FnMut(T) -> bool>(self, pred: F) -> Filter<I, F> {
        Filter { iter: self, predicate: pred }
    }

    fn filter_map<O, F: FnMut(T) -> Option<O>>(self, pred: F) -> FilterMap<I, F> {
        FilterMap { iter: self, predicate: pred }
    }

    fn enumerate(self) -> Enumerate<I> {
        Enumerate(self)
    }

    fn skip(self, n: uint) -> Skip<I> {
        Skip { iter: self, n: n }
    }

    fn take(self, n: uint) -> Take<I> {
        Take { iter: self, n: n }
    }

    fn skip_while<F: FnMut(T) -> bool>(self, pred: F) -> SkipWhile<I, F> {
        SkipWhile { iter: self, predicate: pred }
    }

    fn take_while<F: FnMut(T) -> bool>(self, pred: F) -> TakeWhile<I, F> {
        TakeWhile { iter: self, predicate: pred }
    }

    fn inspect<F: FnMut(T)>(self, f: F) -> Inspect<I, F> {
        Inspect { iter: self, closure: f }
    }

    fn flat_map<O, U: Iterator<O>, F: FnMut(T) -> U>(self, f: F) -> FlatMap<I, F> {
        FlatMap { iter: self, producer: f }
    }

    fn chain<O: IntrusiveIterator<T>>(self, other: O) -> Chain<I, O> {
        Chain { one: self, two: other }
    }

    fn collect<D: FromIntrusiveIterator<T>>(self) -> D {
        FromIntrusiveIterator::collect(self)
    }
}

/// An IntrusiveIterator that maps over the contents of
/// another IntrusiveIterator.
pub struct Map<I, F> {
    iter: I,
    closure: F
}

impl<T, O, I: IntrusiveIterator<T>, F: FnMut(T) -> O> IntrusiveIterator<O> for Map<I, F> {
    fn traverse<F1: FnMut(O) -> bool>(self, mut f: F1) {
        let mut closure = self.closure;
        self.iter.traverse(move |t: T| {
            f(closure(t))
        });
    }
}

pub struct Filter<I, F> {
    iter: I,
    predicate: F
}

pub struct FilterMap<I, F> {
    iter: I,
    predicate: F
}

pub struct Enumerate<I>(I);

pub struct Skip<I> {
    iter: I,
    n: uint
}

pub struct Take<I> {
    iter: I,
    n: uint
}

pub struct SkipWhile<I, F> {
    iter: I,
    predicate: F
}

pub struct TakeWhile<I, F> {
    iter: I,
    predicate: F
}

pub struct Inspect<I, F> {
    iter: I,
    closure: F
}

pub struct FlatMap<I, F> {
    iter: I,
    producer: F
}

pub struct Chain<I, O> {
    one: I,
    two: O
}

