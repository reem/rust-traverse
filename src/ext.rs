-use super::*;
use std::ops::Deref;

impl<I: Traversal, F: FnMut<(I::Item,)>>
Traversal for Map<I, F> {
    type Item = F::Output;

    fn foreach<F1>(self, mut f: F1) where F1: FnMut(F::Output) -> bool {
        let mut closure = self.closure;
        self.iter.foreach(move |t| {
            f(closure(t))
        });
    }
}

impl<I: Traversal, F: FnMut(&I::Item) -> bool>
Traversal for Filter<I, F> {
    type Item = I::Item;

    fn foreach<F1>(self, mut f: F1) where F1: FnMut(I::Item) -> bool{
        let mut predicate = self.predicate;
        self.iter.foreach(move |t| {
            if predicate(&t) { f(t) } else { false }
        });
    }
}

impl<O, I: Traversal, F: FnMut(I::Item) -> Option<O>>
Traversal for FilterMap<I, F> {
    type Item = O;

    fn foreach<F1>(self, mut f: F1) where F1: FnMut(O) -> bool {
        let mut predicate = self.predicate;
        self.iter.foreach(move |t| {
            match predicate(t) {
                Some(o) => f(o),
                None => false
            }
        });
    }
}

impl<I: Traversal>
Traversal for Enumerate<I> {
    type Item = (usize, I::Item);

    fn foreach<F1>(self, mut f: F1) where F1: FnMut((usize, I::Item)) -> bool {
        let mut counter = 0;
        self.0.foreach(|t| {
            let res = f((counter, t));
            counter += 1;
            res
        })
    }
}

impl<I: Traversal>
Traversal for Skip<I> {
    type Item = I::Item;

    fn foreach<F1>(self, mut f: F1) where F1: FnMut(I::Item) -> bool {
        let mut counter = 0;
        let n = self.n;

        self.iter.foreach(|t| {
            if counter != n {
                counter += 1;
                false
            } else {
                f(t)
            }
        })
    }
}

impl<I: Traversal>
Traversal for Take<I> {
    type Item = I::Item;

    fn foreach<F1>(self, mut f: F1) where F1: FnMut(I::Item) -> bool {
        let mut counter = 0;
        let n = self.n;

        self.iter.foreach(|t| {
            if counter != n {
                counter += 1;
                f(t)
            } else {
                false
            }
        })
    }
}

impl<I: Traversal, F: FnMut(&I::Item) -> bool>
Traversal for SkipWhile<I, F> {
    type Item = I::Item;

    fn foreach<F1>(self, mut f: F1) where F1: FnMut(I::Item) -> bool {
        let mut predicate = self.predicate;
        let mut flag = false;
        self.iter.foreach(move |t| {
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

impl<I: Traversal, F: FnMut(&I::Item) -> bool>
Traversal for TakeWhile<I, F> {
    type Item = I::Item;

    fn foreach<F1>(self, mut f: F1) where F1: FnMut(I::Item) -> bool {
        let mut predicate = self.predicate;
        self.iter.foreach(move |t| {
            if predicate(&t) { f(t) } else { true }
        });
    }
}

impl<I: Traversal, F: FnMut(&I::Item)>
Traversal for Inspect<I, F> {
    type Item = I::Item;

    fn foreach<F1>(self, mut f: F1) where F1: FnMut(I::Item) -> bool {
        let mut closure = self.closure;
        self.iter.foreach(move |t| {
            closure(&t);
            f(t)
        });
    }
}

impl<T, I: Traversal<Item=T>, O: Traversal<Item=T>>
Traversal for Chain<I, O> {
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

impl<I: Traversal, F: FnMut<(I::Item,)>>
Traversal for FlatMap<I, F>
where  F::Output: Traversal, {
    type Item = <F::Output as Traversal>::Item;

    fn foreach<F1>(self, mut f: F1) where F1: FnMut(<Self as Traversal>::Item) -> bool {
        let mut producer = self.producer;
        let mut flag = false;
        self.iter.foreach(|t| {
            producer(t).foreach(|o| {
                flag = f(o); flag
            });
            flag
        });
    }
}

impl<I: Traversal>
Traversal for Cloned<I>
where I::Item: Deref,
      <I::Item as Deref>::Target: Clone {
    type Item = <I::Item as Deref>::Target;

    fn foreach<F>(self, mut f: F) where F: FnMut(<Self as Traversal>::Item) -> bool {
        self.iter.foreach(|d| {
            f(d.deref().clone())
        });
    }
}
