use super::*;
use std::ops::Deref;

impl<I: Traversal, O, F: FnMut(I::Item) -> O>
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
                true
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
            if !flag {
                if predicate(&t) {
                    false
                } else {
                    flag = true;
                    f(t)
                }
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

impl<I: Traversal, O: Traversal, F: FnMut(I::Item) -> O>
Traversal for FlatMap<I, F> {
    type Item = <F::Output as Traversal>::Item;

    fn foreach<F1>(self, mut f: F1)
    where F1: FnMut(Self::Item) -> bool {
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

    fn foreach<F>(self, mut f: F) where F: FnMut(Self::Item) -> bool {
        self.iter.foreach(|d| {
            f(d.deref().clone())
        });
    }
}

#[cfg(test)]
mod test {
    use utils::*;
    use Traversal;

    #[test]
    fn map() {
        let vec: Vec<_> = range(0, 5).map(|x| x * 2).collect();
        assert_eq!(vec, &[0, 2, 4, 6, 8]);
    }

    #[test]
    fn filter() {
        let vec: Vec<_> = range(0, 10).filter(|x| x % 2 == 0).collect();
        assert_eq!(vec, &[0, 2, 4, 6, 8]);
    }

    #[test]
    fn filter_map() {
        let vec: Vec<_> = range(0, 10).filter_map(|x| if x % 2 == 0 {
            Some(x / 2)
        } else {
            None
        }).collect();
        assert_eq!(vec, &[0, 1, 2, 3, 4]);
    }

    #[test]
    fn enumerate() {
        let vec: Vec<_> = range(1, 5).enumerate().collect();
        assert_eq!(vec, &[(0, 1), (1, 2), (2, 3), (3, 4)]);
    }

    #[test]
    fn take() {
        let vec: Vec<_> = range(0, 10).take(5).collect();
        assert_eq!(vec, &[0, 1, 2, 3, 4]);
    }

    #[test]
    fn skip() {
        let vec: Vec<_> = range(0, 10).skip(5).collect();
        assert_eq!(vec, &[5, 6, 7, 8, 9]);
    }

    #[test]
    fn take_while() {
        let vec: Vec<_> = range(0, 10).take_while(|&x| x < 5).collect();
        assert_eq!(vec, &[0, 1, 2, 3, 4]);
    }

    #[test]
    fn skip_while() {
        let vec: Vec<_> = range(0, 10).skip_while(|&x| x < 5).collect();
        assert_eq!(vec, &[5, 6, 7, 8, 9]);
    }

    #[test]
    fn inspect() {
        let mut x = 0;
        let vec: Vec<_> = range(0, 5).inspect(|&y| x += y).collect();
        assert_eq!(vec, &[0, 1, 2, 3, 4]);
        assert_eq!(x, 10);
    }

    #[test]
    fn chain() {
        let vec: Vec<_> = range(5, 10).chain(range(0, 5)).collect();
        assert_eq!(vec, &[5, 6, 7, 8, 9, 0, 1, 2, 3, 4]);
    }

    #[test]
    fn cloned() {
        let x = 0;
        let vec: Vec<_> = repeat(&x).cloned().take(5).collect();
        assert_eq!(vec, &[0, 0, 0, 0, 0]);
    }
}
