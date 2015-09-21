use {Traversal, Internal};

impl<'a, T> Traversal for &'a [T] {
    type Item = &'a T;

    #[inline]
    fn foreach<F>(self, f: F) where F: FnMut(&'a T) -> bool {
        Internal::new(self).foreach(f)
    }
}

impl<'a, T> Traversal for &'a mut [T] {
    type Item = &'a mut T;

    #[inline]
    fn foreach<F>(self, f: F) where F: FnMut(&'a mut T) -> bool {
        Internal::new(self).foreach(f)
    }
}

#[cfg(test)]
mod test {
    use Traversal;

    #[test]
    fn test_basic() {
        let data = [1, 2, 5, 4, 6, 7];
        let traversal: Vec<usize> = data.map(|&x| x).collect();
        assert_eq!(traversal, data);
    }

    #[test]
    fn test_zero_size() {
        let data = [(), (), ()];
        let traversal: Vec<()> = data.map(|&x| x).collect();
        assert_eq!(traversal, data);
    }
}
