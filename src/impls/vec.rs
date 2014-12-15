use {IntrusiveIterator, FromIntrusiveIterator};

impl<T> FromIntrusiveIterator<T> for Vec<T> {
    fn collect<I: IntrusiveIterator<T>>(iter: I) -> Vec<T> {
        let mut vec = Vec::new();
        iter.iterate(|&mut: elem| vec.push(elem));
        vec
    }
}

