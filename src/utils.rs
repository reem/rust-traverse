use Traversal;
use std::ops::Add;

use num::traits::PrimInt;

/// An infinite iterator starting at `start` and advancing by `step` with each
/// iteration
#[derive(Copy, Clone)]
pub struct Counter<A> {
    /// The current state the counter is at (next value to be yielded)
    start: A,
    /// The amount that this iterator is stepping by
    step: A,
}

/// Creates a new counter with the specified start/step
#[inline]
pub fn count<A>(start: A, step: A) -> Counter<A> {
    Counter{ start: start, step: step }
}

impl<A: Add<Output=A> + Clone> Traversal for Counter<A> {
    type Item = A;

    #[inline]
    fn foreach<F>(self, mut f: F) where F: FnMut(A) -> bool {
        let mut i = self.start;
        loop {
            let old = i;
            // This is what std does, so I guess it's legit...
            i = old.clone() + self.step.clone();
            if f(old) { return; }
        }
    }
}

/// An iterator over the range [start, stop)
#[derive(Copy, Clone)]
pub struct Range<A> {
    start: A,
    stop: A,
}

/// Returns an iterator over the given range [start, stop) (that is, starting
/// at start (inclusive), and ending at stop (exclusive)).
#[inline]
pub fn range<A: PrimInt>(start: A, stop: A) -> Range<A> {
    Range { start: start, stop: stop }
}

// FIXME: rust-lang/rust#10414: Unfortunate type bound
impl<A: PrimInt> Traversal for Range<A> {
    type Item = A;

    #[inline]
    fn foreach<F>(self, mut f: F) where F: FnMut(A) -> bool {
        let mut i = self.start;
        let one = A::one();
        while i < self.stop {
            let old = i;
            i = old + one;
            if f(old) { return; }
        }
    }
}

/// An iterator over the range [start, stop]
#[derive(Copy, Clone)]
pub struct RangeInclusive<A> {
    start: A,
    stop: A,
}

/// Return an iterator over the range [start, stop]
#[inline]
pub fn range_inclusive<A: PrimInt>(start: A, stop: A) -> RangeInclusive<A> {
    RangeInclusive { start: start, stop: stop }
}

impl<A: PrimInt> Traversal for RangeInclusive<A> {
    type Item = A;

    #[inline]
    fn foreach<F>(self, mut f: F) where F: FnMut(A) -> bool {
        let mut i = self.start;
        let one = A::one();
        while i <= self.stop {
            let old = i;
            i = old + one;
            if f(old) { return; }
        }
    }
}

/// An iterator over the range [start, stop) by `step`. It handles overflow by stopping.
#[derive(Copy, Clone)]
pub struct RangeStep<A> {
    start: A,
    stop: A,
    step: A,
}

/// Return an iterator over the range [start, stop) by `step`. It handles overflow by stopping.
#[inline]
pub fn range_step<A: PrimInt>(start: A, stop: A, step: A) -> RangeStep<A> {
    RangeStep { start: start, stop: stop, step: step }
}

impl<A: PrimInt> Traversal for RangeStep<A> {
    type Item = A;

    #[inline]
    fn foreach<F>(self, mut f: F) where F: FnMut(A) -> bool {
        let mut i = self.start;
        // branch once and duplicate trivial logic for the perf
        if self.step > A::zero() {
            while i < self.stop {
                let old = i;
                let temp = i.checked_add(&self.step);
                if f(old) { return; }
                i = match temp { None => return, Some(x) => x }
            }
        } else {
            while i > self.stop {
                let old = i;
                let temp = i.checked_add(&self.step);
                if f(old) { return; }
                i = match temp { None => return, Some(x) => x }
            }
        }
    }
}

/// An iterator over the range [start, stop] by `step`. It handles overflow by stopping.
#[derive(Copy, Clone)]
pub struct RangeStepInclusive<A> {
    start: A,
    stop: A,
    step: A,
}

/// Return an iterator over the range [start, stop] by `step`. It handles overflow by stopping.
#[inline]
pub fn range_step_inclusive<A: PrimInt>(start: A, stop: A, step: A) -> RangeStepInclusive<A> {
    RangeStepInclusive { start: start, stop: stop, step: step }
}

impl<A: PrimInt> Traversal for RangeStepInclusive<A> {
    type Item = A;

    #[inline]
    fn foreach<F>(self, mut f: F) where F: FnMut(A) -> bool {
        let mut i = self.start;
        // branch once and duplicate trivial logic for the perf
        if self.step > A::zero() {
            while i <= self.stop {
                let old = i;
                let temp = i.checked_add(&self.step);
                if f(old) { return; }
                i = match temp { None => return, Some(x) => x }
            }
        } else {
            while i >= self.stop {
                let old = i;
                let temp = i.checked_add(&self.step);
                if f(old) { return; }
                i = match temp { None => return, Some(x) => x }
            }
        }
    }
}

/// Create a new iterator that endlessly repeats the element `elt`.
#[inline]
pub fn repeat<T: Clone>(elt: T) -> Repeat<T> {
    Repeat{ element: elt }
}

/// An iterator that repeats an element endlessly
#[derive(Copy, Clone)]
pub struct Repeat<A> {
    element: A
}

impl<A: Clone> Traversal for Repeat<A> {
    type Item = A;

    #[inline]
    fn foreach<F>(self, mut f: F) where F: FnMut(A) -> bool {
        loop {
            if f(self.element.clone()) { return; }
        }
    }
}

/// An iterator that repeatedly applies a given function, starting
/// from a given seed value.
#[derive(Copy, Clone)]
pub struct Iterate<T, F> {
    seed: T,
    iter: F,
}

/// Create a new iterator that produces an infinite sequence of
/// repeated applications of the given function `f`.
#[inline]
pub fn iterate<T, F>(seed: T, f: F) -> Iterate<T, F> where
    T: Clone,
    F: FnMut(T) -> T
{
    Iterate { seed: seed, iter: f }
}

impl<A, I> Traversal for Iterate<A, I> where
    A: Clone,
    I: FnMut(A) -> A {
    type Item = A;

    #[inline]
    fn foreach<F>(mut self, mut f: F) where F: FnMut(A) -> bool {
        if !f(self.seed.clone()) {
            let mut cur = self.seed;
            loop {
                let next = (self.iter)(cur);
                if f(next.clone()) { return; }
                cur = next;
            }
        }
    }
}



#[cfg(test)]
mod test {
    use super::*;
    use Traversal;

    #[test]
    fn test_range() {
        assert_eq!(range(0, 5).collect::<Vec<i32>>(), vec![0, 1, 2, 3, 4]);
        assert_eq!(range(-10, -1).collect::<Vec<i32>>(),
            vec![-10, -9, -8, -7, -6, -5, -4, -3, -2]);
        assert_eq!(range(200i32, -5).count(), 0);
        assert_eq!(range(200i32, 200).count(), 0);
    }

    #[test]
    fn test_range_inclusive() {
        assert_eq!(range_inclusive(0, 5).collect::<Vec<i32>>(), vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(range_inclusive(200i32, -5).count(), 0);
        assert_eq!(range_inclusive(200, 200).collect::<Vec<i32>>(), vec![200]);
    }

    #[test]
    fn test_range_step() {
        assert_eq!(range_step(0, 20, 5).collect::<Vec<i32>>(), vec![0, 5, 10, 15]);
        assert_eq!(range_step(20, 0, -5).collect::<Vec<i32>>(), vec![20, 15, 10, 5]);
        assert_eq!(range_step(20, 0, -6).collect::<Vec<i32>>(), vec![20, 14, 8, 2]);
        assert_eq!(range_step(200u8, 255, 50).collect::<Vec<u8>>(), vec![200u8, 250]);
        assert_eq!(range_step(200, -5, 1).collect::<Vec<i32>>(), vec![]);
        assert_eq!(range_step(200, 200, 1).collect::<Vec<i32>>(), vec![]);
    }

    #[test]
    fn test_range_step_inclusive() {
        assert_eq!(range_step_inclusive(0, 20, 5).collect::<Vec<i32>>(), vec![0, 5, 10, 15, 20]);
        assert_eq!(range_step_inclusive(20, 0, -5).collect::<Vec<i32>>(), vec![20, 15, 10, 5, 0]);
        assert_eq!(range_step_inclusive(20, 0, -6).collect::<Vec<i32>>(), vec![20, 14, 8, 2]);
        assert_eq!(range_step_inclusive(200, 255, 50).collect::<Vec<u8>>(), vec![200, 250]);
        assert_eq!(range_step_inclusive(200, -5, 1).collect::<Vec<i32>>(), vec![]);
        assert_eq!(range_step_inclusive(200, 200, 1).collect::<Vec<i32>>(), vec![200]);
    }


    #[test]
    fn test_iterate() {
        assert_eq!(iterate(1, |x| x * 2).take(5).collect::<Vec<i32>>(), vec![1, 2, 4, 8, 16]);
    }

    #[test]
    fn test_repeat() {
        assert_eq!(repeat(42).take(5).collect::<Vec<i32>>(), vec![42, 42, 42, 42, 42]);
    }
}
