use std::{mem, raw};
use {Traversal};

impl<'a, T> Traversal<&'a T> for &'a [T] {
    #[inline]
    fn foreach<F: FnMut(&'a T) -> bool>(self, mut f: F) {
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

impl<'a, T> Traversal<&'a mut T> for &'a mut [T] {
    #[inline]
    fn foreach<F: FnMut(&'a mut T) -> bool>(self, mut f: F) {
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

#[cfg(test)]
mod test {
    use Traversal;
    use test::Bencher;

    #[test]
    fn test_basic() {
        let data = [1u, 2, 5, 4, 6, 7];
        let traversal: Vec<uint> = data.as_slice().map(|&x| x).collect();
        assert_eq!(&*traversal, data.as_slice());
    }

    #[test]
    fn test_zero_size() {
        let data = [(), (), ()];
        let traversal: Vec<()> = data.as_slice().map(|&x| x).collect();
        assert_eq!(&*traversal, data.as_slice());
    }

    #[bench]
    fn bench_internal (bench: &mut Bencher) {
        use std::rand::random;

        let data = Vec::from_fn(10000, |_| random::<uint>());
        bench.iter(|| {
            data.as_slice().run(|&: x| ::test::black_box(x));
        });
    }

    #[bench]
    fn bench_external (bench: &mut Bencher) {
        use std::rand::random;

        let data = Vec::from_fn(10000, |_| random::<uint>());
        bench.iter(|| {
            for datum in data.as_slice().iter() {
                ::test::black_box(datum);
            }
        });
    }
}

