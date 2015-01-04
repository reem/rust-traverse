use super::*;
use std::ops::Deref;

impl<T, O, I: Traversal<Item=T>, F: FnMut(T) -> O> Traversal for Map<I, F> {
    type Item = O;

    fn foreach<F1>(self, mut f: F1) where F1: FnMut(O) -> bool {
        let mut closure = self.closure;
        self.iter.foreach(move |t: T| {
            f(closure(t))
        });
    }
}

impl<T, I, F> Traversal for Filter<I, F>
where I: Traversal<Item=T>, F: FnMut(&T) -> bool {
    type Item = T;

    fn foreach<F1>(self, mut f: F1) where F1: FnMut(T) -> bool{
        let mut predicate = self.predicate;
        self.iter.foreach(move |t: T| {
            if predicate(&t) { f(t) } else { false }
        });
    }
}

impl<T, O, I, F> Traversal for FilterMap<I, F>
where I: Traversal<Item=T>, F: FnMut(T) -> Option<O> {
    type Item = O;

    fn foreach<F1>(self, mut f: F1) where F1: FnMut(O) -> bool {
        let mut predicate = self.predicate;
        self.iter.foreach(move |t: T| {
            match predicate(t) {
                Some(o) => f(o),
                None => false
            }
        });
    }
}

impl<T, I> Traversal for Enumerate<I>
where I: Traversal<Item=T> {
    type Item = (uint, T);

    fn foreach<F1>(self, mut f: F1) where F1: FnMut((uint, T)) -> bool {
        let mut counter = 0;
        self.0.foreach(|t: T| {
            let res = f((counter, t));
            counter += 1;
            res
        })
    }
}

impl<T, I> Traversal for Skip<I>
where I: Traversal<Item=T> {
    type Item = T;

    fn foreach<F1>(self, mut f: F1) where F1: FnMut(T) -> bool {
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

impl<T, I> Traversal for Take<I>
where I: Traversal<Item=T> {
    type Item = T;

    fn foreach<F1>(self, mut f: F1) where F1: FnMut(T) -> bool {
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

impl<T, I, F> Traversal for SkipWhile<I, F>
where I: Traversal<Item=T>, F: FnMut(&T) -> bool {
    type Item = T;

    fn foreach<F1>(self, mut f: F1) where F1: FnMut(T) -> bool {
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

impl<T, I, F> Traversal for TakeWhile<I, F>
where I: Traversal<Item=T>, F: FnMut(&T) -> bool {
    type Item = T;

    fn foreach<F1>(self, mut f: F1) where F1: FnMut(T) -> bool {
        let mut predicate = self.predicate;
        self.iter.foreach(move |t: T| {
            if predicate(&t) { f(t) } else { true }
        });
    }
}

impl<T, I, F> Traversal for Inspect<I, F>
where I: Traversal<Item=T>, F: FnMut(&T) {
    type Item = T;

    fn foreach<F1>(self, mut f: F1) where F1: FnMut(T) -> bool {
        let mut closure = self.closure;
        self.iter.foreach(move |t: T| {
            closure(&t);
            f(t)
        });
    }
}

impl<T, I, O> Traversal for Chain<I, O>
where I: Traversal<Item=T>, O: Traversal<Item=T> {
    type Item = T;

    fn foreach<F1>(self, mut f: F1) where F1: FnMut(T) -> bool {
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

impl<T, O, U, I, F> Traversal for FlatMap<I, F>
where I: Traversal<Item=T>,
      F: FnMut(T) -> U,
      U: Traversal<Item=O> {
    type Item = O;

    fn foreach<F1>(self, mut f: F1) where F1: FnMut(O) -> bool {
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

impl<T, I, D> Traversal for Cloned<I>
where I: Traversal<Item=D>,
      D: Deref<Target=T>,
      T: Clone {
    type Item = T;

    fn foreach<F>(self, mut f: F) where F: FnMut(T) -> bool {
        self.iter.foreach(|d| {
            f(d.deref().clone())
        });
    }
}
