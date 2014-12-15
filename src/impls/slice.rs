use std::{mem, raw};
use {IntrusiveIterator};

pub trait SliceIntrusiveIter<T> for Sized? {
    fn intrusive_iter(&self) -> Items<T>;
    fn intrusive_iter_mut(&mut self) -> ItemsMut<T>;
}

pub struct Items<'a, T: 'a>(&'a [T]);
pub struct ItemsMut<'a, T: 'a>(&'a mut [T]);

impl<T> SliceIntrusiveIter<T> for [T] {
    #[inline]
    fn intrusive_iter(&self) -> Items<T> { Items(self) }

    #[inline]
    fn intrusive_iter_mut(&mut self) -> ItemsMut<T> { ItemsMut(self) }
}

impl<'a, T> IntrusiveIterator<&'a T> for Items<'a, T> {
    #[inline]
    fn traverse<F: FnMut(&'a T) -> bool>(self, mut f: F) {
        unsafe {
            let slice = mem::transmute::<&'a [T], raw::Slice<T>>(self.0);

            let is_zero_size = mem::size_of::<T>() == 0;

            if is_zero_size {
                for _ in range(0, slice.len) {
                    // Just give some pointer, doesn't matter what.
                    if f(mem::transmute(1u)) { break }
                }
            } else {
                let mut current = slice.data;
                let end = slice.data.offset(slice.len as int);
                while current != end {
                    if f(mem::transmute(current)) { break }
                    current = current.offset(1);
                }
            }
        }
    }
}

impl<'a, T> IntrusiveIterator<&'a mut T> for ItemsMut<'a, T> {
    #[inline]
    fn traverse<F: FnMut(&'a mut T) -> bool>(self, mut f: F) {
        unsafe {
            let slice = mem::transmute::<&'a mut [T], raw::Slice<T>>(self.0);

            let is_zero_size = mem::size_of::<T>() == 0;

            if is_zero_size {
                for _ in range(0, slice.len) {
                    // Just give some pointer, doesn't matter what.
                    if f(mem::transmute(1u)) { break }
                }
            } else {
                let mut current = slice.data;
                let end = slice.data.offset(slice.len as int);
                while current != end {
                    if f(mem::transmute(current)) { break }
                    current = current.offset(1);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    pub use super::*;
    pub use IntrusiveIterator;

    describe! intrusive_slice_iter {
        it "should yield all elements of a slice in order" {
            let data = [1u, 2, 5, 4, 6, 7];
            let intrusive: Vec<uint> = data.intrusive_iter().map(|&x| x).collect();
            assert_eq!(&*intrusive, data.as_slice());
        }

        it "should work with zero-sized types" {
            let data = [(), (), ()];
            let intrusive: Vec<()> = data.intrusive_iter().map(|&x| x).collect();
            assert_eq!(&*intrusive, data.as_slice());
        }

        bench "intrusive iteration" (bench) {
            use std::rand::random;

            let data = Vec::from_fn(10000, |_| random::<uint>());
            bench.iter(|| {
                data.intrusive_iter().iterate(|&: x| ::test::black_box(x));
            });
        }

        bench "external iteration" (bench) {
            use std::rand::random;

            let data = Vec::from_fn(10000, |_| random::<uint>());
            bench.iter(|| {
                for datum in data.as_slice().iter() {
                    ::test::black_box(datum);
                }
            });
        }
    }
}

