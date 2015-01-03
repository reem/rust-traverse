use super::*;
use std::ops::Deref;

impl<T, O, I: Traversal<T>, F: FnMut(T) -> O> Traversal<O> for Map<I, F> {
    fn foreach<F1: FnMut(O) -> bool>(self, mut f: F1) {
        let mut closure = self.closure;
        self.iter.foreach(move |t: T| {
            f(closure(t))
        });
    }
}

impl<T, I, F> Traversal<T> for Filter<I, F>
where I: Traversal<T>, F: FnMut(&T) -> bool {
    fn foreach<F1: FnMut(T) -> bool>(self, mut f: F1) {
        let mut predicate = self.predicate;
        self.iter.foreach(move |t: T| {
            if predicate(&t) { f(t) } else { false }
        });
    }
}

impl<T, O, I, F> Traversal<O> for FilterMap<I, F>
where I: Traversal<T>, F: FnMut(T) -> Option<O> {
    fn foreach<F1: FnMut(O) -> bool>(self, mut f: F1) {
        let mut predicate = self.predicate;
        self.iter.foreach(move |t: T| {
            match predicate(t) {
                Some(o) => f(o),
                None => false
            }
        });
    }
}

impl<T, I> Traversal<(uint, T)> for Enumerate<I>
where I: Traversal<T> {
    fn foreach<F1: FnMut((uint, T)) -> bool>(self, mut f: F1) {
        let mut counter = 0;
        self.0.foreach(|t: T| {
            let res = f((counter, t));
            counter += 1;
            res
        })
    }
}

impl<T, I> Traversal<T> for Skip<I>
where I: Traversal<T> {
    fn foreach<F1: FnMut(T) -> bool>(self, mut f: F1) {
        let mut counter = 0;
        let n = self.n;

        self.iter.foreach(|t: T| {
            if counter != n {
                counter += 1;
                true
            } else {
                f(t)
            }
        })
    }
}

impl<T, I> Traversal<T> for Take<I>
where I: Traversal<T> {
    fn foreach<F1: FnMut(T) -> bool>(self, mut f: F1) {
        let mut counter = 0;
        let n = self.n;

        self.iter.foreach(|t: T| {
            if counter != n {
                counter += 1;
                f(t)
            } else {
                true
            }
        })
    }
}

impl<T, I, F> Traversal<T> for SkipWhile<I, F>
where I: Traversal<T>, F: FnMut(&T) -> bool {
    fn foreach<F1: FnMut(T) -> bool>(self, mut f: F1) {
        let mut predicate = self.predicate;
        let mut flag = false;
        self.iter.foreach(move |t: T| {
            // Done skipping
            if flag {
                if !predicate(&t) {
                    flag = true;
                }
                false
            } else {
                f(t)
            }
        });
    }
}

impl<T, I, F> Traversal<T> for TakeWhile<I, F>
where I: Traversal<T>, F: FnMut(&T) -> bool {
    fn foreach<F1: FnMut(T) -> bool>(self, mut f: F1) {
        let mut predicate = self.predicate;
        self.iter.foreach(move |t: T| {
            if predicate(&t) { f(t) } else { true }
        });
    }
}

impl<T, I, F> Traversal<T> for Inspect<I, F>
where I: Traversal<T>, F: FnMut(&T) {
    fn foreach<F1: FnMut(T) -> bool>(self, mut f: F1) {
        let mut closure = self.closure;
        self.iter.foreach(move |t: T| {
            closure(&t);
            f(t)
        });
    }
}

impl<T, I, O> Traversal<T> for Chain<I, O>
where I: Traversal<T>, O: Traversal<T> {
    fn foreach<F1: FnMut(T) -> bool>(self, mut f: F1) {
        let mut flag = false;
        self.one.foreach(|t: T| {
            flag = f(t); flag
        });

        if !flag {
            self.two.foreach(|t: T| {
                f(t)
            });
        }
    }
}

impl<T, O, U, I, F> Traversal<O> for FlatMap<I, F>
where I: Traversal<T>,
      F: FnMut(T) -> U,
      U: Traversal<O> {
    fn foreach<F1: FnMut(O) -> bool>(self, mut f: F1) {
        let mut producer = self.producer;
        let mut flag = false;
        self.iter.foreach(|t: T| {
            producer(t).foreach(|o: O| {
                flag = f(o); flag
            });
            flag
        });
    }
}

impl<T, I, D> Traversal<T> for Cloned<I>
where I: Traversal<D>,
      D: Deref<Target=T>,
      T: Clone {
    fn foreach<F: FnMut(T) -> bool>(self, mut f: F) {
        self.iter.foreach(|d| {
            f(d.deref().clone())
        });
    }
}
