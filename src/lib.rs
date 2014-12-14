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
    fn map<O, F: FnMut(T) -> O>(self, F) -> Map<T, O, Self, F>;
    fn filter<F: FnMut(T) -> bool>(self, f: F) -> Filter<T, I, F>;
    fn filter_map<O, F: FnMut(T) -> Option<O>>(self, F) -> FilterMap<T, O, Self, F>;
    fn enumerate(self) -> Enumerate<Self>;
    fn skip(self, uint) -> Skip<Self>;
    fn take(self, uint) -> Take<Self>;
    fn skip_while<F: FnMut(T) -> bool>(self, F) -> SkipWhile<T, Self, F>;
    fn take_while<F: FnMut(T) -> bool>(self, F) -> TakeWhile<T, Self, F>;
    fn inspect<F: FnMut(T)>(self, F) -> Inspect<T, Self, F>;
    fn flat_map<O, U: Iterator<O>, F: FnMut(T) -> U>(self, F) -> FlatMap<T, O, U, Self, F>;
    fn collect<D: FromIntrusiveIterator<T>>(self) -> D;
    fn chain<O: IntrusiveIterator<T>>(self, O) -> Chain<T, Self, O>;
}

trait FromIntrusiveIterator<T> {
    fn collect<I: IntrusiveIterator<T>>(I) -> Self;
}

impl<T, I: IntrusiveIterator<T>> IntrusiveIteratorExt<T> for I {
    fn map<O, F: FnMut(T) -> O>(self, f: F) -> Map<T, O, I, F> {
        Map { iter: self, closure: f }
    }

    fn filter<F: FnMut(T) -> bool>(self, pred: F) -> Filter<T, I, F> {
        Filter { iter: self, predicate: pred }
    }

    fn filter_map<O, F: FnMut(T) -> Option<O>>(self, pred: F) -> FilterMap<T, O, Self, F> {
        FilterMap { iter: self, predicate: pred }
    }

    fn enumerate(self) -> Enumerate<Self> {
        Enumerate { iter: self }
    }

    fn skip(self, n: uint) -> Skip<Self> {
        Skip { iter: self, n: n }
    }

    fn take(self, n: uint) -> Take<Self> {
        Take { iter: self, n: n }
    }

    fn skip_while<F: FnMut(T) -> bool>(self, pred: F) -> SkipWhile<T, Self, F> {
        SkipWhile { iter: self, predicate: pred }
    }

    fn take_while<F: FnMut(T) -> bool>(self, pred: F) -> TakeWhile<T, Self, F> {
        TakeWhile { iter: self, predicate: pred }
    }

    fn inspect<F: FnMut(T)>(self, f: F) -> Inspect<T, Self, F> {
        Inspect { iter: self, closure: f }
    }

    fn flat_map<O, U: Iterator<O>, F: FnMut(T) -> U>(self, f: F) -> FlatMap<T, O, U, Self, F> {
        FlatMap { iter: self, producer: other }
    }

    fn chain<O: IntrusiveIterator<T>>(self, other: O) -> Chain<T, Self, O> {
        Chain { one: self, two: other }
    }

    fn collect<D: FromIntrusiveIterator<T>>(self) -> D {
        FromIntrusiveIterator::collect(self)
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

