use super::*;

impl<T, O, I: IntrusiveIterator<T>, F: FnMut(T) -> O> IntrusiveIterator<O> for Map<I, F> {
    fn traverse<F1: FnMut(O) -> bool>(self, mut f: F1) {
        let mut closure = self.closure;
        self.iter.traverse(move |t: T| {
            f(closure(t))
        });
    }
}

impl<T, I, F> IntrusiveIterator<T> for Filter<I, F>
where I: IntrusiveIterator<T>, F: FnMut(&T) -> bool {
    fn traverse<F1: FnMut(T) -> bool>(self, mut f: F1) {
        let mut predicate = self.predicate;
        self.iter.traverse(move |t: T| {
            if predicate(&t) { f(t) } else { false }
        });
    }
}

impl<T, O, I, F> IntrusiveIterator<O> for FilterMap<I, F>
where I: IntrusiveIterator<T>, F: FnMut(T) -> Option<O> {
    fn traverse<F1: FnMut(O) -> bool>(self, mut f: F1) {
        let mut predicate = self.predicate;
        self.iter.traverse(move |t: T| {
            match predicate(t) {
                Some(o) => f(o),
                None => false
            }
        });
    }
}

impl<T, I> IntrusiveIterator<(uint, T)> for Enumerate<I>
where I: IntrusiveIterator<T> {
    fn traverse<F1: FnMut((uint, T)) -> bool>(self, mut f: F1) {
        let mut counter = 0;
        self.0.traverse(|t: T| {
            let res = f((counter, t));
            counter += 1;
            res
        })
    }
}

impl<T, I> IntrusiveIterator<T> for Skip<I>
where I: IntrusiveIterator<T> {
    fn traverse<F1: FnMut(T) -> bool>(self, mut f: F1) {
        let mut counter = 0;
        let n = self.n;

        self.iter.traverse(|t: T| {
            if counter != n {
                counter += 1;
                true
            } else {
                f(t)
            }
        })
    }
}

impl<T, I> IntrusiveIterator<T> for Take<I>
where I: IntrusiveIterator<T> {
    fn traverse<F1: FnMut(T) -> bool>(self, mut f: F1) {
        let mut counter = 0;
        let n = self.n;

        self.iter.traverse(|t: T| {
            if counter != n {
                counter += 1;
                f(t)
            } else {
                true
            }
        })
    }
}

impl<T, I, F> IntrusiveIterator<T> for SkipWhile<I, F>
where I: IntrusiveIterator<T>, F: FnMut(&T) -> bool {
    fn traverse<F1: FnMut(T) -> bool>(self, mut f: F1) {
        let mut predicate = self.predicate;
        let mut flag = false;
        self.iter.traverse(move |t: T| {
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

impl<T, I, F> IntrusiveIterator<T> for TakeWhile<I, F>
where I: IntrusiveIterator<T>, F: FnMut(&T) -> bool {
    fn traverse<F1: FnMut(T) -> bool>(self, mut f: F1) {
        let mut predicate = self.predicate;
        self.iter.traverse(move |t: T| {
            if predicate(&t) { f(t) } else { true }
        });
    }
}

impl<T, I, F> IntrusiveIterator<T> for Inspect<I, F>
where I: IntrusiveIterator<T>, F: FnMut(&T) {
    fn traverse<F1: FnMut(T) -> bool>(self, mut f: F1) {
        let mut closure = self.closure;
        self.iter.traverse(move |t: T| {
            closure(&t);
            f(t)
        });
    }
}

impl<T, I, O> IntrusiveIterator<T> for Chain<I, O>
where I: IntrusiveIterator<T>, O: IntrusiveIterator<T> {
    fn traverse<F1: FnMut(T) -> bool>(self, mut f: F1) {
        let mut flag = false;
        self.one.traverse(|t: T| {
            flag = f(t); flag
        });

        if !flag {
            self.two.traverse(|t: T| {
                f(t)
            });
        }
    }
}

impl<T, O, U, I, F> IntrusiveIterator<O> for FlatMap<I, F>
where I: IntrusiveIterator<T>,
      F: FnMut(T) -> U,
      U: IntrusiveIterator<O> {
    fn traverse<F1: FnMut(O) -> bool>(self, mut f: F1) {
        let mut producer = self.producer;
        let mut flag = false;
        self.iter.traverse(|t: T| {
            producer(t).traverse(|o: O| {
                flag = f(o); flag
            });
            flag
        });
    }
}

impl<T, I, D> IntrusiveIterator<T> for Cloned<I>
where I: IntrusiveIterator<D>,
      D: Deref<T>,
      T: Clone {
    fn traverse<F: FnMut(T) -> bool>(self, mut f: F) {
        self.iter.traverse(|d| {
            f(d.deref().clone())
        });
    }
}