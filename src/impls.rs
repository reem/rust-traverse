use std::{mem, raw};
use {IntrusiveIterator, FromIntrusiveIterator};

impl<'a, T> IntrusiveIterator<&'a T> for &'a [T] {
    #[inline]
    fn traverse<F: FnMut(&'a T) -> bool>(self, mut f: F) {
        unsafe {
            let slice = mem::transmute::<&'a [T], raw::Slice<T>>(self);

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

impl<'a, T> IntrusiveIterator<&'a mut T> for &'a mut [T] {
    #[inline]
    fn traverse<F: FnMut(&'a mut T) -> bool>(self, mut f: F) {
        unsafe {
            let slice = mem::transmute::<&'a mut [T], raw::Slice<T>>(self);

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

impl<T> FromIntrusiveIterator<T> for Vec<T> {
    fn collect<I: IntrusiveIterator<T>>(iter: I) -> Vec<T> {
        let mut vec = Vec::new();
        iter.traverse(|&mut: elem| { vec.push(elem); false });
        vec
    }
}

#[cfg(test)]
mod test {
    pub use super::*;
    pub use IntrusiveIterator;
    pub use IntrusiveIteratorExt;

    describe! intrusive_slice_iter {
        it "should yield all elements of a slice in order" {
            let data = [1u, 2, 5, 4, 6, 7];
            let intrusive: Vec<uint> = data.as_slice().map(|&x| x).collect();
            assert_eq!(&*intrusive, data.as_slice());
        }

        it "should work with zero-sized types" {
            let data = [(), (), ()];
            let intrusive: Vec<()> = data.as_slice().map(|&x| x).collect();
            assert_eq!(&*intrusive, data.as_slice());
        }

        bench "intrusive iteration" (bench) {
            use std::rand::random;

            let data = Vec::from_fn(10000, |_| random::<uint>());
            bench.iter(|| {
                data.as_slice().traverse(|&: x| { ::test::black_box(x); false });
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

